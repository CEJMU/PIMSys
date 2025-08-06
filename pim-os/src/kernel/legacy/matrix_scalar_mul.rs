use crate::pim::legacy::array::{DummyArray, PimMatrixArena, PimRegion, PimScalarArena};
use pim_isa::{File, Instruction, Kernel};

pub const KERNEL: Kernel = Kernel([
    Instruction::MOV {
        src: File::Bank,
        dst: File::SrfM { index: 0 },
    },
    Instruction::MUL {
        src0: File::Bank,
        src1: File::SrfM { index: 0 },
        dst: File::GrfA { index: 0 },
        aam: false,
    },
    Instruction::MUL {
        src0: File::Bank,
        src1: File::SrfM { index: 0 },
        dst: File::GrfA { index: 1 },
        aam: false,
    },
    Instruction::MUL {
        src0: File::Bank,
        src1: File::SrfM { index: 0 },
        dst: File::GrfA { index: 2 },
        aam: false,
    },
    Instruction::MUL {
        src0: File::Bank,
        src1: File::SrfM { index: 0 },
        dst: File::GrfA { index: 3 },
        aam: false,
    },
    Instruction::MUL {
        src0: File::Bank,
        src1: File::SrfM { index: 0 },
        dst: File::GrfA { index: 4 },
        aam: false,
    },
    Instruction::MUL {
        src0: File::Bank,
        src1: File::SrfM { index: 0 },
        dst: File::GrfA { index: 5 },
        aam: false,
    },
    Instruction::MUL {
        src0: File::Bank,
        src1: File::SrfM { index: 0 },
        dst: File::GrfA { index: 6 },
        aam: false,
    },
    Instruction::MUL {
        src0: File::Bank,
        src1: File::SrfM { index: 0 },
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
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
    Instruction::NOP,
]);

pub fn execute<const R: usize, const C: usize>(
    pim_scalar_arena: &PimScalarArena,
    pim_matrix_arena0: &PimMatrixArena<R, C>,
    pim_matrix_arena1: &mut PimMatrixArena<R, C>,
    dummy_array: &DummyArray,
) {
    for column in 0..C {
        pim_scalar_arena.execute_instruction_read_single_bank(0);

        for i in 0..R {
            pim_matrix_arena0.execute_instruction_read_single_bank(column * R + i);
        }

        for i in 0..R {
            pim_matrix_arena1.execute_instruction_write_single_bank(column * R + i);
        }

        dummy_array.execute_instruction_read_single_bank(0);
    }
}
