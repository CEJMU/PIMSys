use crate::pim::{operation::PimOperand, vector::F16x1};
use nalgebra::SVector;
use pim_isa::{File, Instruction, Kernel};

pub const KERNEL: Kernel = Kernel([
    Instruction::MOV {
        src: File::Bank,
        dst: File::GrfA { index: 0 },
    },
    Instruction::MUL {
        src0: File::Bank,
        src1: File::GrfA { index: 0 },
        dst: File::GrfB { index: 0 },
        aam: false,
    },
    Instruction::FILL {
        src: File::GrfB { index: 0 },
        dst: File::Bank,
    },
    Instruction::EXIT,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
]);

pub fn execute<const R: usize, const BLOCKS: usize>(
    a: &SVector<F16x1, R>,
    b: &SVector<F16x1, R>,
    c: &mut SVector<F16x1, R>,
    dummy: &impl PimOperand,
) {
    a.fixed_rows_with_step::<BLOCKS>(0, 256)
        .iter()
        .for_each(|entry| entry.execute_read());
    b.fixed_rows_with_step::<BLOCKS>(0, 256)
        .iter()
        .for_each(|entry| entry.execute_read());
    c.fixed_rows_with_step_mut::<BLOCKS>(0, 256)
        .iter_mut()
        .for_each(|entry| entry.execute_write());

    dummy.execute_read();
}
