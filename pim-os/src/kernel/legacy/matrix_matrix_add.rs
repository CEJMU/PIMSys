use crate::pim::legacy::array::{DummyArray, PimMatrixArena, PimRegion};
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
    Instruction::ADD {
        src0: File::Bank,
        src1: File::GrfA { index: 0 },
        dst: File::GrfA { index: 0 },
        aam: false,
    },
    Instruction::ADD {
        src0: File::Bank,
        src1: File::GrfA { index: 1 },
        dst: File::GrfA { index: 1 },
        aam: false,
    },
    Instruction::ADD {
        src0: File::Bank,
        src1: File::GrfA { index: 2 },
        dst: File::GrfA { index: 2 },
        aam: false,
    },
    Instruction::ADD {
        src0: File::Bank,
        src1: File::GrfA { index: 3 },
        dst: File::GrfA { index: 3 },
        aam: false,
    },
    Instruction::ADD {
        src0: File::Bank,
        src1: File::GrfA { index: 4 },
        dst: File::GrfA { index: 4 },
        aam: false,
    },
    Instruction::ADD {
        src0: File::Bank,
        src1: File::GrfA { index: 5 },
        dst: File::GrfA { index: 5 },
        aam: false,
    },
    Instruction::ADD {
        src0: File::Bank,
        src1: File::GrfA { index: 6 },
        dst: File::GrfA { index: 6 },
        aam: false,
    },
    Instruction::ADD {
        src0: File::Bank,
        src1: File::GrfA { index: 7 },
        dst: File::GrfA { index: 7 },
        aam: false,
    },
    Instruction::FILL {
        src: File::GrfA { index: 0 },
        dst: File::Bank,
    },
    Instruction::FILL {
        src: File::GrfA { index: 1 },
        dst: File::Bank,
    },
    Instruction::FILL {
        src: File::GrfA { index: 2 },
        dst: File::Bank,
    },
    Instruction::FILL {
        src: File::GrfA { index: 3 },
        dst: File::Bank,
    },
    Instruction::FILL {
        src: File::GrfA { index: 4 },
        dst: File::Bank,
    },
    Instruction::FILL {
        src: File::GrfA { index: 5 },
        dst: File::Bank,
    },
    Instruction::FILL {
        src: File::GrfA { index: 6 },
        dst: File::Bank,
    },
    Instruction::FILL {
        src: File::GrfA { index: 7 },
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
]);

pub fn execute<const R: usize, const C: usize>(
    pim_matrix_arena0: &PimMatrixArena<R, C>,
    pim_matrix_arena1: &PimMatrixArena<R, C>,
    pim_matrix_arena2: &mut PimMatrixArena<R, C>,
    dummy_array: &DummyArray,
) {
    for column in 0..C {
        for row in 0..R {
            pim_matrix_arena0.execute_instruction_read_single_bank(column * R + row);
        }

        for row in 0..R {
            pim_matrix_arena1.execute_instruction_read_single_bank(column * R + row);
        }

        for row in 0..R {
            pim_matrix_arena2.execute_instruction_write_single_bank(column * R + row);
        }

        dummy_array.execute_instruction_read_single_bank(0);
    }
}