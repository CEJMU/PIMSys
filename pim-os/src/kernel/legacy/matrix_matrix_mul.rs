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
        count: 63,
    },
    Instruction::FILL {
        src: File::GrfB { index: 0 },
        dst: File::Bank,
    },
    Instruction::FILL {
        src: File::GrfB { index: 1 },
        dst: File::Bank,
    },
    Instruction::FILL {
        src: File::GrfB { index: 2 },
        dst: File::Bank,
    },
    Instruction::FILL {
        src: File::GrfB { index: 3 },
        dst: File::Bank,
    },
    Instruction::FILL {
        src: File::GrfB { index: 4 },
        dst: File::Bank,
    },
    Instruction::FILL {
        src: File::GrfB { index: 5 },
        dst: File::Bank,
    },
    Instruction::FILL {
        src: File::GrfB { index: 6 },
        dst: File::Bank,
    },
    Instruction::FILL {
        src: File::GrfB { index: 7 },
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
]);

// Vlt in der Thesis kurz erwähnen und dann zu AAM überleiten
// pub fn execute_matrix_multiply_elementwise<const R: usize, const C: usize>(
//     pim_state: &mut PimState,
//     pim_matrix_arena0: &mut PimMatrixArena<R, C>,
//     pim_matrix_arena1: &mut PimMatrixArena<R, C>,
//     pim_matrix_arena2: &mut PimMatrixArena<R, C>,
//     dummy_array: &mut DummyArray,
// ) {
//     set_bank_mode(BankMode::PimAllBank);

//     for i in 0..(R * C) {
//         let start_column = i % R;
//         let start_row = (i / R) * R;

//         for j in 0..C {
//             pim_matrix_arena0.execute_instruction_read_single_bank(start_column + R * j);
//         }

//         for j in 0..R {
//             pim_matrix_arena1.execute_instruction_read_single_bank(start_row + j);
//         }

//         pim_matrix_arena2.execute_instruction_write_single_bank(i);

//         dummy_array.execute_instruction_read_single_bank(0);
//     }

//     set_bank_mode(BankMode::SingleBank);
// }

const MATRIX_DIMENSION: usize = 8;

pub fn execute(
    pim_matrix_arena0: &PimMatrixArena<MATRIX_DIMENSION, MATRIX_DIMENSION>,
    pim_matrix_arena1: &PimMatrixArena<MATRIX_DIMENSION, MATRIX_DIMENSION>,
    pim_matrix_arena2: &mut PimMatrixArena<MATRIX_DIMENSION, MATRIX_DIMENSION>,
    dummy_array: &DummyArray,
) {
    for row in 0..MATRIX_DIMENSION {
        for i in 0..MATRIX_DIMENSION {
            pim_matrix_arena0.execute_instruction_read_single_bank(row + MATRIX_DIMENSION * i);
        }

        for column in 0..MATRIX_DIMENSION {
            for i in 0..MATRIX_DIMENSION {
                pim_matrix_arena1.execute_instruction_read_single_bank_unsynchronized(
                    column * MATRIX_DIMENSION + i,
                );
            }
        }

        for column in 0..MATRIX_DIMENSION {
            pim_matrix_arena2
                .execute_instruction_write_single_bank(column * MATRIX_DIMENSION + row);
        }

        dummy_array.execute_instruction_read_single_bank(0);
    }
}
