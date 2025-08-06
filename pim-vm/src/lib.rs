use half::f16;
use pim_isa::{BankMode, File, Instruction, Kernel};

#[cxx::bridge(namespace = "pim_vm")]
mod ffi {
    pub enum BankMode {
        SingleBank,
        AllBank,
        PimAllBank,
    }

    extern "Rust" {
        type PimVM;

        fn new_pim_vm(num_banks: u32) -> Box<PimVM>;
        fn reset(&mut self);
        fn apply_config(&mut self, config: &str);
        fn bank_mode(&self) -> BankMode;
        fn execute_read(
            &mut self,
            bank_index: u32,
            address: u32,
            row: u32,
            column: u32,
            bank_data: &[u8],
        );
        fn execute_write(&mut self, bank_index: u32) -> [u8; 32];

        fn init_logger();
    }
}

fn init_logger() {
    env_logger::init();
}

const GRF_A_BIT_OFFSET: usize = 2;
const GRF_B_BIT_OFFSET: usize = 5;
const COLUMN_BITS : usize = 7;

const BURST_LENGTH: usize = 32;

const GRF_NUM_REGISTERS: usize = 8;
const SRF_A_NUM_REGISTERS: usize = 8;
const SRF_M_NUM_REGISTERS: usize = 8;

const FP_UNITS: usize = 16;
type GrfRegister = [f16; FP_UNITS];

#[derive(Clone, Debug)]
struct PimUnit {
    grf_a: [GrfRegister; GRF_NUM_REGISTERS],
    grf_b: [GrfRegister; GRF_NUM_REGISTERS],
    srf_a: [f16; SRF_A_NUM_REGISTERS],
    srf_m: [f16; SRF_A_NUM_REGISTERS],
    pc: u8,
    jump_counter: Option<u16>,
}

impl PimUnit {
    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Default for PimUnit {
    fn default() -> Self {
        Self {
            grf_a: [[f16::ZERO; FP_UNITS]; GRF_NUM_REGISTERS],
            grf_b: [[f16::ZERO; FP_UNITS]; GRF_NUM_REGISTERS],
            srf_a: [f16::ZERO; SRF_A_NUM_REGISTERS],
            srf_m: [f16::ZERO; SRF_M_NUM_REGISTERS],
            pc: 0,
            jump_counter: None,
        }
    }
}

#[derive(Debug)]
struct PimVM {
    pim_units: Vec<PimUnit>,
    bank_mode: pim_isa::BankMode,
    kernel: pim_isa::Kernel,
}

impl PimVM {
    fn reset(&mut self) {
        for unit in self.pim_units.iter_mut() {
            unit.reset();
        }
    }

    fn apply_config(&mut self, config_str: &str) {
        let config = serde_json::from_str::<pim_isa::PimConfig>(config_str).unwrap();

        if let Some(kernel) = config.kernel {
            self.kernel = kernel;
        }

        if let Some(bank_mode) = config.bank_mode {
            self.bank_mode = bank_mode;
        }
    }

    fn bank_mode(&self) -> ffi::BankMode {
        match self.bank_mode {
            BankMode::SingleBank => ffi::BankMode::SingleBank,
            BankMode::AllBank => ffi::BankMode::AllBank,
            BankMode::PimAllBank => ffi::BankMode::PimAllBank,
        }
    }
}

fn new_pim_vm(num_banks: u32) -> Box<PimVM> {
    let num_pim_units = if cfg!(feature = "shared_pim_units") {
        num_banks / 2
    } else {
        num_banks
    };

    Box::new(PimVM {
        pim_units: vec![PimUnit::default(); num_pim_units as _],
        bank_mode: BankMode::SingleBank,
        kernel: Kernel::NOP,
    })
}

#[repr(C)]
struct BankData([f16; FP_UNITS]);

impl PimVM {
    pub fn execute_read(
        &mut self,
        bank_index: u32,
        address: u32,
        row: u32,
        column: u32,
        bank_data: &[u8],
    ) {
        assert_eq!(bank_data.len(), BURST_LENGTH);

        let pim_unit_index = if cfg!(feature = "shared_pim_units") {
            bank_index / 2
        } else {
            bank_index
        };

        let pim_unit = &mut self.pim_units[pim_unit_index as usize];

        let inst = self.kernel.0[pim_unit.pc as usize];

        let row_column_bits = (row << COLUMN_BITS) | column;
        let aam_grf_a_index = (row_column_bits >> GRF_A_BIT_OFFSET) & 0b111;
        let aam_grf_b_index = (row_column_bits >> GRF_B_BIT_OFFSET) & 0b111;

        if pim_unit_index == 0 {
            log::debug!(
                "PimUnit {pim_unit_index} at {address:#x} (B{aam_grf_b_index}, A{aam_grf_a_index}) Execute Read PC {}: {inst:?}",
                pim_unit.pc
            );
        }

        match inst {
            Instruction::NOP => (),
            Instruction::EXIT => {
                pim_unit.reset();
                return;
            }
            Instruction::JUMP { .. } => unreachable!(),
            Instruction::MOV { src, dst } | Instruction::FILL { src, dst } => {
                let data = PimVM::load(src, pim_unit, &bank_data);
                PimVM::store(dst, pim_unit, &data);
            }
            Instruction::ADD {
                src0,
                mut src1,
                mut dst,
                aam,
            } => {
                if aam {
                    src1 = if let File::GrfA { index: _ } = src1 {
                        File::GrfA {
                            index: aam_grf_a_index as _,
                        }
                    } else {
                        panic!("Invalid operand in address-aligned-mode");
                    };

                    dst = if let File::GrfB { index: _ } = dst {
                        File::GrfB {
                            index: aam_grf_b_index as _,
                        }
                    } else {
                        panic!("Invalid operand in address-aligned-mode");
                    };
                }

                let data0 = PimVM::load(src0, pim_unit, &bank_data);
                let data1 = PimVM::load(src1, pim_unit, &bank_data);

                let sum: [f16; FP_UNITS] = data0
                    .into_iter()
                    .zip(data1)
                    .map(|(src0, src1)| src0 + src1)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                PimVM::store(dst, pim_unit, &sum);
            }
            Instruction::MUL {
                src0,
                mut src1,
                mut dst,
                aam,
            } => {
                if aam {
                    src1 = if let File::GrfA { index: _ } = src1 {
                        File::GrfA {
                            index: aam_grf_a_index as _,
                        }
                    } else {
                        panic!("Invalid operand in address-aligned-mode");
                    };

                    dst = if let File::GrfB { index: _ } = dst {
                        File::GrfB {
                            index: aam_grf_b_index as _,
                        }
                    } else {
                        panic!("Invalid operand in address-aligned-mode");
                    };
                }

                let data0 = PimVM::load(src0, pim_unit, &bank_data);
                let data1 = PimVM::load(src1, pim_unit, &bank_data);

                let product: [f16; FP_UNITS] = data0
                    .into_iter()
                    .zip(data1)
                    .map(|(src0, src1)| src0 * src1)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                PimVM::store(dst, pim_unit, &product);
            }
            Instruction::MAC {
                src0,
                mut src1,
                mut src2,
                mut dst,
                aam,
            }
            | Instruction::MAD {
                src0,
                mut src1,
                mut src2,
                mut dst,
                aam,
            } => {
                if aam {
                    src1 = if let File::GrfA { index: _ } = src1 {
                        // if pim_unit_index == 0 {
                        //     log::debug!("AAM index GrfA {aam_grf_a_index}");
                        // }
                        File::GrfA {
                            index: aam_grf_a_index as _,
                        }
                    } else {
                        panic!("Invalid operand in address-aligned-mode");
                    };

                    src2 = if let File::GrfB { index: _ } = src2 {
                        // if pim_unit_index == 0 {
                        //     log::debug!("AAM index GrfB {aam_grf_a_index}");
                        // }
                        File::GrfB {
                            index: aam_grf_b_index as _,
                        }
                    } else {
                        panic!("Invalid operand in address-aligned-mode");
                    };

                    dst = if let File::GrfB { index: _ } = dst {
                        File::GrfB {
                            index: aam_grf_b_index as _,
                        }
                    } else {
                        panic!("Invalid operand in address-aligned-mode");
                    };
                }

                assert_eq!(src2, dst);

                let data0 = PimVM::load(src0, pim_unit, &bank_data);
                let data1 = PimVM::load(src1, pim_unit, &bank_data);
                let data2 = PimVM::load(src2, pim_unit, &bank_data);

                let product: [f16; FP_UNITS] = data0
                    .into_iter()
                    .zip(data1)
                    .map(|(src0, src1)| src0 * src1)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                let sum: [f16; FP_UNITS] = product
                    .into_iter()
                    .zip(data2)
                    .map(|(product, src2)| product + src2)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                // if pim_unit_index == 0 {
                //     log::debug!(
                //         "\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
                //         data0[0],
                //         data1[0],
                //         data2[0],
                //         product[0],
                //         sum[0]
                //     );
                // }
                PimVM::store(dst, pim_unit, &sum);
            }
        }

        pim_unit.pc += 1;

        // The JUMP instruction is zero-cycle and not actually executed
        while let Instruction::JUMP { offset, count } = self.kernel.0[pim_unit.pc as usize] {
            pim_unit.jump_counter = match pim_unit.jump_counter {
                Some(jump_counter) => jump_counter.checked_sub(1),
                None => count.checked_sub(1),
            };

            if pim_unit.jump_counter != None {
                let new_pc = pim_unit.pc as i32 + offset as i32;

                if new_pc < 0 || new_pc >= 32 {
                    panic!("Invalid PC {new_pc} after JUMP: {inst:?}");
                }

                pim_unit.pc = new_pc as _;
            } else {
                pim_unit.pc += 1;
            }

            // if pim_unit_index == 0 {
            //     log::debug!(
            //         "PimUnit {pim_unit_index} JUMP to PC {}: {:?}",
            //         pim_unit.pc,
            //         self.kernel.0[pim_unit.pc as usize]
            //     );
            // }
        }
    }

    pub fn execute_write(&mut self, bank_index: u32) -> [u8; BURST_LENGTH] {
        let pim_unit_index = if cfg!(feature = "shared_pim_units") {
            bank_index / 2
        } else {
            bank_index
        };

        let pim_unit = &mut self.pim_units[pim_unit_index as usize];
        let inst = self.kernel.0[pim_unit.pc as usize];

        if pim_unit_index == 0 {
            log::debug!(
                "PimUnit {pim_unit_index} Execute Write PC {}: {inst:?}",
                pim_unit.pc
            );
        }

        let data = match inst {
            Instruction::FILL { src, dst } => {
                let data: [f16; FP_UNITS] = match src {
                    File::GrfA { index } => pim_unit.grf_a[index as usize],
                    File::GrfB { index } => pim_unit.grf_b[index as usize],
                    _ => panic!("Unsupported src operand: {src:?}"),
                };

                if dst != File::Bank {
                    panic!("Unsupported dst operand: {dst:?}")
                }

                // if pim_unit_index == 0 {
                //     log::debug!("Store {data:?}");
                // }

                data
            }
            _ => panic!("Unsupported instruction for write: {inst:?}"),
        };

        pim_unit.pc += 1;

        // The JUMP instruction is zero-cycle and not actually executed
        while let Instruction::JUMP { offset, count } = self.kernel.0[pim_unit.pc as usize] {
            pim_unit.jump_counter = match pim_unit.jump_counter {
                Some(jump_counter) => jump_counter.checked_sub(1),
                None => count.checked_sub(1),
            };

            if pim_unit.jump_counter != None {
                let new_pc = pim_unit.pc as i32 + offset as i32;

                if new_pc < 0 || new_pc >= 32 {
                    panic!("Invalid PC {new_pc} after JUMP: {inst:?}");
                }

                pim_unit.pc = new_pc as _;
            } else {
                pim_unit.pc += 1;
            }

            if pim_unit_index == 0 {
                log::debug!(
                    "PimUnit {pim_unit_index} JUMP to PC {}: {:?}",
                    pim_unit.pc,
                    self.kernel.0[pim_unit.pc as usize]
                );
            }
        }

        unsafe { std::mem::transmute(data) }
    }

    fn load(src: File, pim_unit: &PimUnit, bank_data: &[u8]) -> [f16; FP_UNITS] {
        match src {
            File::GrfA { index } => pim_unit.grf_a[index as usize],
            File::GrfB { index } => pim_unit.grf_b[index as usize],
            File::SrfM { index } => [pim_unit.srf_m[index as usize]; FP_UNITS],
            File::SrfA { index } => [pim_unit.srf_a[index as usize]; FP_UNITS],
            File::Bank => unsafe { std::ptr::read(bank_data.as_ptr() as *const BankData).0 },
        }
    }

    fn store(dst: File, pim_unit: &mut PimUnit, data: &[f16; FP_UNITS]) {
        match dst {
            File::GrfA { index } => pim_unit.grf_a[index as usize] = data.clone(),
            File::GrfB { index } => pim_unit.grf_b[index as usize] = data.clone(),
            File::SrfM { index } => pim_unit.srf_m[index as usize] = data[0],
            File::SrfA { index } => pim_unit.srf_a[index as usize] = data[0],
            File::Bank => panic!("Unsupported dst operand: {dst:?}"),
        }
    }
}
