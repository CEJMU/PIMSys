use core::arch::global_asm;

global_asm!(include_str!("m5op.s"));

extern "C" {
    fn m5_exit(delay_ns: u64);
    fn m5_reset_stats(delay_ns: u64, period_ns: u64);
    fn m5_dump_stats(delay_ns: u64, period_ns: u64);
    fn m5_dump_reset_stats(delay_ns: u64, period_ns: u64);
}

pub fn exit(delay_ns: u64) {
    unsafe { m5_exit(delay_ns) }
}

pub fn reset_stats(delay_ns: u64, period_ns: u64) {
    unsafe { m5_reset_stats(delay_ns, period_ns) }
}

pub fn dump_stats(delay_ns: u64, period_ns: u64) {
    unsafe { m5_dump_stats(delay_ns, period_ns) }
}

pub fn dump_reset_stats(delay_ns: u64, period_ns: u64) {
    unsafe { m5_dump_reset_stats(delay_ns, period_ns) }
}
