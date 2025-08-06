#![no_std]
#![no_main]

extern crate alloc;

use aarch64_cpu::asm::barrier;
use alloc::{boxed::Box, rc::Rc};
use core::{cell::RefCell, fmt::Write};
use half::f16;
use nalgebra::Matrix;
use pim_isa::BankMode;
use pim_os::{
    pim::{
        self,
        array::{DummyArray, PimMatrixArena, PimStorage, NUMBER_OF_BANKS},
        kernel::matrix_vector_mul,
        vector::{F16x1, F16x16},
    },
    uart::Uart0,
};

#[no_mangle]
pub extern "C" fn main() {
    pim::state::set_kernel(&matrix_vector_mul::KERNEL);

    let pim_matrix_arena0 = Rc::new(RefCell::new(PimMatrixArena(
        [[[F16x16::default(); NUMBER_OF_BANKS]; 8]; 8],
    )));
    let pim_matrix_arena1 = Rc::new(RefCell::new(PimMatrixArena(
        [[[F16x16::default(); NUMBER_OF_BANKS]; 8]; 1],
    )));
    let pim_matrix_arena2 = Rc::new(RefCell::new(PimMatrixArena(
        [[[F16x16::default(); NUMBER_OF_BANKS]; 8]; 1],
    )));

    let mut matrix0 = Matrix::from_data(PimStorage {
        arena: &pim_matrix_arena0,
        index: 0,
    });
    matrix0.fill_lower_triangle(F16x1(f16::ONE), 0);

    let mut matrix1 = Matrix::from_data(PimStorage {
        arena: &pim_matrix_arena1,
        index: 0,
    });
    matrix1.fill_lower_triangle(F16x1(f16::ONE), 0);

    let matrix2 = Matrix::from_data(PimStorage {
        arena: &pim_matrix_arena2,
        index: 0,
    });

    writeln!(Uart0, "{matrix0} * {matrix1}\n=").unwrap();

    let dummy_array = Box::new(DummyArray([F16x16::default(); NUMBER_OF_BANKS]));

    // Verify everything is correctly initialized before PIM operation
    barrier::dsb(barrier::SY);

    // Execute kernel
    {
        let pim_matrix_arena0 = &pim_matrix_arena0.borrow();
        let pim_matrix_arena1 = &pim_matrix_arena1.borrow();
        let pim_matrix_arena2 = &mut pim_matrix_arena2.borrow_mut();

        pim::state::set_bank_mode(BankMode::PimAllBank);

        matrix_vector_mul::execute(
            pim_matrix_arena0,
            pim_matrix_arena1,
            pim_matrix_arena2,
            dummy_array.as_ref(),
        );

        pim::state::set_bank_mode(BankMode::SingleBank);
    }

    writeln!(Uart0, "{matrix2}").unwrap();
}
