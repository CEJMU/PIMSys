#![no_std]
#![no_main]

extern crate alloc;

use aarch64_cpu::asm::barrier;
use alloc::boxed::Box;
use core::fmt::Write;
use half::f16;
use nalgebra::SVector;
use pim_isa::BankMode;
use pim_os::{
    kernel::haxpy,
    pim::{
        self, interleaved_array,
        vector::{F16x1, F16x16},
    },
    uart::Uart0,
};

const ROWS: usize = 256;
const ELEMENTS_PER_BANK: usize = 16;
const BANKS: usize = 16;
const BLOCKS: usize = ROWS / (ELEMENTS_PER_BANK * BANKS);

#[no_mangle]
pub extern "C" fn main() {
    pim::state::set_kernel(&haxpy::KERNEL);

    let a = Box::new(pim::continuous_array::Vector(
        SVector::<F16x1, ROWS>::from_fn(|i, _| F16x1(f16::from_f32(i as _))),
    ));
    let b = Box::new(pim::continuous_array::Vector(
        SVector::<F16x1, ROWS>::from_fn(|i, _| F16x1(f16::from_f32((ROWS - i) as _))),
    ));

    let scalar_vector = SVector::<F16x16, 1>::from_element(F16x16([F16x1(f16::NEG_ONE); 16]));
    let interleaved_scalar_vector = Box::new(interleaved_array::Vector::from(scalar_vector));

    writeln!(Uart0, "{}+{}=", a.0, b.0).unwrap();

    let mut c = Box::new(pim::continuous_array::Vector(
        SVector::<F16x1, ROWS>::zeros(),
    ));

    let dummy = Box::new(0);

    // Verify everything is correctly initialized before PIM operation
    barrier::dsb(barrier::SY);

    // Execute kernel
    pim::state::set_bank_mode(BankMode::PimAllBank);
    haxpy::execute::<ROWS, BLOCKS>(
        &a.0,
        &b.0,
        &interleaved_scalar_vector,
        &mut c.0,
        dummy.as_ref(),
    );
    pim::state::set_bank_mode(BankMode::SingleBank);

    writeln!(Uart0, "{}", c.0).unwrap();
    writeln!(Uart0, "Done").unwrap();
}
