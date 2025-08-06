#![feature(iter_array_chunks)]
#![no_std]

use core::sync::atomic::{compiler_fence, Ordering};

mod alloc;
mod panic;

pub mod boot;
pub mod critical_section;
pub mod kernel;
pub mod m5op;
pub mod memory_config;
pub mod pim;
pub mod uart;

extern "C" {
    fn main();
}

#[no_mangle]
pub extern "C" fn entry() -> ! {
    alloc::init();

    unsafe { main() }

    m5op::exit(0);

    loop {
        compiler_fence(Ordering::SeqCst);
    }
}
