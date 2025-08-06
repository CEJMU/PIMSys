use aarch64_cpu::asm::barrier;

pub trait PimOperand {
    fn ptr(&self) -> *const u8;
    fn ptr_mut(&mut self) -> *mut u8;

    fn execute_read(&self) {
        unsafe { core::ptr::read_volatile(self.ptr()) };
        barrier::dsb(barrier::SY);
    }

    fn execute_read_async(&self) {
        unsafe { core::ptr::read_volatile(self.ptr()) };
    }

    fn execute_write(&mut self) {
        unsafe { core::ptr::write_volatile(self.ptr_mut(), Default::default()) };
        barrier::dsb(barrier::SY);
    }
}

impl<T> PimOperand for T {
    fn ptr(&self) -> *const u8 {
        core::ptr::addr_of!(*self) as *const _
    }

    fn ptr_mut(&mut self) -> *mut u8 {
        core::ptr::addr_of_mut!(*self) as *mut _
    }
}
