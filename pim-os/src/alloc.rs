extern crate alloc;

use core::mem::MaybeUninit;
use embedded_alloc::Heap;

#[global_allocator]
static PIM_ALLOC: Heap = Heap::empty();

const PIM_ARENA_SIZE: usize = 0x2000000;

#[link_section = ".pim_data"]
static mut PIM_ARENA: [MaybeUninit<u8>; PIM_ARENA_SIZE] = [MaybeUninit::uninit(); PIM_ARENA_SIZE];

pub fn init() {
    unsafe {
        PIM_ALLOC.init(PIM_ARENA.as_ptr() as usize, PIM_ARENA_SIZE);
    }
}
