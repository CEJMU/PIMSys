use crate::uart::Uart0;
use core::{
    fmt::Write,
    panic::PanicInfo,
    sync::atomic::{compiler_fence, Ordering},
};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    writeln!(Uart0, "{info}").unwrap();

    loop {
        compiler_fence(Ordering::SeqCst);
    }
}
