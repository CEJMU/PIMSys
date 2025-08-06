#![no_std]

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Instruction {
    NOP,
    EXIT,
    JUMP {
        offset: i16,
        count: u16,
    },
    MOV {
        src: File,
        dst: File,
    },
    FILL {
        src: File,
        dst: File,
    },
    ADD {
        src0: File,
        src1: File,
        dst: File,
        aam: bool,
    },
    MUL {
        src0: File,
        src1: File,
        dst: File,
        aam: bool,
    },
    MAC {
        src0: File,
        src1: File,
        src2: File,
        dst: File,
        aam: bool,
    },
    MAD {
        src0: File,
        src1: File,
        src2: File,
        dst: File,
        aam: bool,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum File {
    GrfA { index: u8 },
    GrfB { index: u8 },
    SrfM { index: u8 },
    SrfA { index: u8 },
    Bank,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kernel(pub [Instruction; 32]);

impl Kernel {
    pub const NOP: Kernel = Kernel([Instruction::NOP; 32]);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PimConfig {
    pub bank_mode: Option<BankMode>,
    pub kernel: Option<Kernel>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BankMode {
    SingleBank,
    AllBank,
    PimAllBank,
}
