use aarch64_cpu::asm::barrier;
use core::{
    arch::asm,
    ptr::{addr_of_mut, write_volatile},
};

#[link_section = ".pim_config"]
static mut PIM_CONFIG_REGION: [u8; 0x4000] = [0; 0x4000];

pub fn write(s: &str) {
    unsafe {
        let mut index = 0;
        for &byte in s.as_bytes() {
            write_volatile(
                (addr_of_mut!(PIM_CONFIG_REGION) as *mut u8).offset(index),
                byte as _,
            );
            barrier::dsb(barrier::SY);
            index += 1;
        }
        write_volatile(
            (addr_of_mut!(PIM_CONFIG_REGION) as *mut u8).offset(index),
            b'\0',
        );

        // PIM_CONFIG_REGION[..s.len()].copy_from_slice(s.as_bytes());
        // PIM_CONFIG_REGION[s.len()] = b'\0';

        if cfg!(feature = "cacheless") {
            // Be pessimistic so that config region is not optimized away
            core::hint::black_box(PIM_CONFIG_REGION);
        } else {
            // Flush all cache lines that were affected by write operation
            for element in PIM_CONFIG_REGION[..s.len()].iter() {
                asm!("dc civac, {val}", val = in(reg) element);
            }

            barrier::dsb(barrier::SY);
        }
    }
}
