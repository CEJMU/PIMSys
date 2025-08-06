#![no_std]
#![no_main]

extern crate alloc;

use aarch64_cpu::asm::barrier;
use alloc::boxed::Box;
use core::fmt::Write;
use nalgebra::{SMatrix, SVector};
use num_traits::{One, Zero};
use pim_isa::BankMode;
use pim_os::{
    kernel::gemv,
    pim::{
        self, interleaved_array,
        vector::{F16x1, F16x16},
    },
    uart::Uart0,
};

const ROWS: usize = 128;
const COLUMNS: usize = 128;
const X16_ROWS: usize = ROWS / 16;
const X16_COLUMNS: usize = COLUMNS / 16;

#[no_mangle]
pub extern "C" fn main() {
    pim::state::set_kernel(&gemv::KERNEL);

    let mut matrix = SMatrix::<_, ROWS, COLUMNS>::zeros();
    matrix.fill_lower_triangle(F16x1::one(), 0);

    let pim_matrix = Box::new(pim::continuous_array::Matrix::<X16_ROWS, X16_COLUMNS>::from(matrix));

    let input_vector = SVector::<_, X16_COLUMNS>::from_element(F16x16::one());
    let interleaved_input_vector = Box::new(interleaved_array::Vector::from(input_vector));

    let mut output_partial_sum_vector = Box::new(SVector::<F16x16, ROWS>::zeros());

    let dummy = Box::new(0);

    // Verify everything is correctly initialized before PIM operation
    barrier::dsb(barrier::SY);

    // Execute kernel
    pim::state::set_bank_mode(BankMode::PimAllBank);
    gemv::execute(
        pim_matrix.as_ref(),
        interleaved_input_vector.as_ref(),
        output_partial_sum_vector.as_mut(),
        dummy.as_ref(),
    );
    pim::state::set_bank_mode(BankMode::SingleBank);

    writeln!(Uart0, "{output_partial_sum_vector}").unwrap();

    let output_vector = SVector::<F16x1, ROWS>::from_fn(|r, _| {
        output_partial_sum_vector[r]
            .0
            .iter()
            .fold(F16x1::zero(), |acc, val| acc + *val)
    });
    core::hint::black_box(output_vector);

    writeln!(Uart0, "{output_vector}").unwrap();
    writeln!(Uart0, "Done").unwrap();
}
