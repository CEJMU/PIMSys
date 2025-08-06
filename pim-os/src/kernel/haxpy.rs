use crate::pim::{interleaved_array, operation::PimOperand, vector::F16x1};
use nalgebra::SVector;
use pim_isa::{File, Instruction, Kernel};

pub const KERNEL: Kernel = Kernel([
    Instruction::MOV {
        src: File::Bank,
        dst: File::SrfM { index: 0 },
    },
    Instruction::MOV {
        src: File::Bank,
        dst: File::GrfA { index: 0 },
    },
    Instruction::MAD {
        src0: File::Bank,
        src1: File::SrfA { index: 0 },
        src2: File::GrfA { index: 0 },
        dst: File::GrfA { index: 0 },
        aam: false,
    },
    Instruction::FILL {
        src: File::GrfA { index: 0 },
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
]);

pub fn execute<const R: usize, const BLOCKS: usize>(
    a: &SVector<F16x1, R>,
    b: &SVector<F16x1, R>,
    interleaved_scalar: &interleaved_array::Vector<1>,
    c: &mut SVector<F16x1, R>,
    dummy: &impl PimOperand,
) {
    interleaved_scalar.execute_read();

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
