use crate::pim::{
    continuous_array::Matrix, interleaved_array, operation::PimOperand, vector::F16x16,
};
use aarch64_cpu::asm::barrier;
use nalgebra::SVector;
use pim_isa::{File, Instruction, Kernel};

pub const KERNEL: Kernel = Kernel([
    Instruction::MOV {
        src: File::Bank,
        dst: File::GrfA { index: 0 },
    },
    Instruction::MOV {
        src: File::Bank,
        dst: File::GrfA { index: 1 },
    },
    Instruction::MOV {
        src: File::Bank,
        dst: File::GrfA { index: 2 },
    },
    Instruction::MOV {
        src: File::Bank,
        dst: File::GrfA { index: 3 },
    },
    Instruction::MOV {
        src: File::Bank,
        dst: File::GrfA { index: 4 },
    },
    Instruction::MOV {
        src: File::Bank,
        dst: File::GrfA { index: 5 },
    },
    Instruction::MOV {
        src: File::Bank,
        dst: File::GrfA { index: 6 },
    },
    Instruction::MOV {
        src: File::Bank,
        dst: File::GrfA { index: 7 },
    },
    Instruction::MAC {
        src0: File::Bank,
        src1: File::GrfA { index: 0 },
        src2: File::GrfB { index: 0 },
        dst: File::GrfB { index: 0 },
        aam: true,
    },
    Instruction::JUMP {
        offset: -1,
        count: 7,
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
]);

pub fn execute<const X16R: usize, const R: usize>(
    matrix: &Matrix<X16R, 8>,
    input_vector: &interleaved_array::Vector<8>,
    output_partial_sum_vector: &mut SVector<F16x16, R>,
    dummy: &impl PimOperand,
) {
    for block in input_vector.0.iter() {
        block.execute_read();
    }

    for sub_matrix in matrix.0.iter() {
        for column_block in sub_matrix.fixed_rows::<1>(0).iter() {
            column_block.execute_read_async();
        }
    }

    barrier::dsb(barrier::SY);

    for chunk in output_partial_sum_vector
        .fixed_rows_with_step_mut::<X16R>(0, 16)
        .iter_mut()
    {
        chunk.execute_write();
    }

    dummy.execute_read();
}
