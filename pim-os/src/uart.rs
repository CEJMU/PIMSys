use core::{fmt::Write, ptr::write_volatile};

const UART0_ADDR: *mut u32 = 0x1c090000 as _;

#[derive(Debug)]
pub struct Uart0;

impl Write for Uart0 {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &byte in s.as_bytes() {
            unsafe {
                write_volatile(UART0_ADDR, byte as _);
            }
        }
        Ok(())
    }
}
