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

pub fn execute<const R: usize, const C: usize>(
    pim_matrix_arena0: &PimMatrixArena<R, C>,
    pim_matrix_arena1: &PimMatrixArena<C, 1>,
    pim_matrix_arena2: &mut PimMatrixArena<C, 1>,
    dummy_array: &DummyArray,
) {
    for row in 0..R {
        for i in 0..C {
            pim_matrix_arena0.execute_instruction_read_single_bank(row + R * i);
        }

        for i in 0..R {
            pim_matrix_arena1.execute_instruction_read_single_bank(i);
        }

        pim_matrix_arena2.execute_instruction_write_single_bank(row);

        dummy_array.execute_instruction_read_single_bank(0);
    }
}
