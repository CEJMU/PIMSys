use critical_section::RawRestoreState;

struct CriticalSection;
critical_section::set_impl!(CriticalSection);

unsafe impl critical_section::Impl for CriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        // no special implementation as interrupts are not used in the project
    }

    unsafe fn release(_token: RawRestoreState) {
        // no special implementation as interrupts are not used in the project
    }
}
