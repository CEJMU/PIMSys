use crate::{
    memory_config::NUMBER_OF_BANKS,
    pim::vector::{F16x1, F16x16},
};
use aarch64_cpu::asm::barrier;
use core::{arch::asm, cell::RefCell};
use half::f16;
use nalgebra::{Const, Dyn, RawStorage, RawStorageMut};

const EVEN_BANK_INDEX: usize = 0;
const ODD_BANK_INDEX: usize = 8;

#[derive(Clone, Debug)]
#[repr(C, align(65536))]
pub struct PimMatrixArena<const R: usize, const C: usize>(pub [[[F16x16; NUMBER_OF_BANKS]; R]; C]);

impl<const R: usize, const C: usize> PimRegion for PimMatrixArena<R, C> {
    const OCCUPIED_CACHE_LINES: usize = R * C * NUMBER_OF_BANKS;

    fn bank_ptr(&self, bank_index: usize) -> *const f16 {
        unsafe { (self.0.as_ptr() as *const F16x16).add(bank_index) as *const f16 }
    }

    fn bank_ptr_mut(&mut self, bank_index: usize) -> *mut f16 {
        unsafe { (self.0.as_mut_ptr() as *mut F16x16).add(bank_index) as *mut f16 }
    }
}

#[derive(Clone, Debug)]
#[repr(C, align(1024))]
pub struct PimScalarArena(pub [F16x16; NUMBER_OF_BANKS]);

impl PimRegion for PimScalarArena {
    const OCCUPIED_CACHE_LINES: usize = NUMBER_OF_BANKS;

    fn bank_ptr(&self, bank_index: usize) -> *const f16 {
        unsafe { (self.0.as_ptr() as *const F16x16).add(bank_index) as *const f16 }
    }

    fn bank_ptr_mut(&mut self, bank_index: usize) -> *mut f16 {
        unsafe { (self.0.as_mut_ptr() as *mut F16x16).add(bank_index) as *mut f16 }
    }
}

#[derive(Debug)]
pub struct PimStorage<'a, const R: usize, const C: usize> {
    pub arena: &'a RefCell<PimMatrixArena<R, C>>,
    pub index: usize,
}

unsafe impl<'a, const R: usize, const C: usize> RawStorage<F16x1, Const<R>, Const<C>>
    for PimStorage<'a, R, C>
{
    type RStride = Dyn;
    type CStride = Dyn;

    fn ptr(&self) -> *const F16x1 {
        unsafe { (&self.arena.borrow().0[0][0] as *const F16x16 as *const F16x1).add(self.index) }
    }

    fn shape(&self) -> (Const<R>, Const<C>) {
        (Const::<R>, Const::<C>)
    }

    fn strides(&self) -> (Self::RStride, Self::CStride) {
        (Dyn(16 * NUMBER_OF_BANKS), Dyn(16 * R * NUMBER_OF_BANKS))
    }

    fn is_contiguous(&self) -> bool {
        false
    }

    unsafe fn as_slice_unchecked(&self) -> &[F16x1] {
        panic!("PimStorage is not contiguous!");
    }
}

unsafe impl<'a, const R: usize, const C: usize> RawStorageMut<F16x1, Const<R>, Const<C>>
    for PimStorage<'a, R, C>
{
    fn ptr_mut(&mut self) -> *mut F16x1 {
        unsafe {
            (&mut self.arena.borrow_mut().0[0][0] as *mut F16x16 as *mut F16x1).add(self.index)
        }
    }

    unsafe fn as_mut_slice_unchecked(&mut self) -> &mut [F16x1] {
        panic!("PimStorage is not contiguous!");
    }
}

pub trait PimRegion {
    const OCCUPIED_CACHE_LINES: usize;

    fn bank_ptr(&self, bank_index: usize) -> *const f16;
    fn bank_ptr_mut(&mut self, bank_index: usize) -> *mut f16;

    fn execute_instruction_read_single_bank(&self, i: usize) {
        if !cfg!(feature = "cacheless") {
            self.invalidate_bank(EVEN_BANK_INDEX + i * NUMBER_OF_BANKS);
            barrier::dsb(barrier::SY);
        }

        // Read from first bank
        self.read_data_bank(EVEN_BANK_INDEX + i * NUMBER_OF_BANKS);

        barrier::dsb(barrier::SY);
    }

    fn execute_instruction_read_single_bank_unsynchronized(&self, i: usize) {
        self.read_data_bank(EVEN_BANK_INDEX + i * NUMBER_OF_BANKS);
    }

    fn execute_instruction_read_dual_bank(&self) {
        let i = 0;
        if !cfg!(feature = "cacheless") {
            self.invalidate_bank(EVEN_BANK_INDEX + i * NUMBER_OF_BANKS);
            self.invalidate_bank(ODD_BANK_INDEX + i * NUMBER_OF_BANKS);

            barrier::dsb(barrier::SY);
        }

        // Read from first and second bank
        self.read_data_bank(EVEN_BANK_INDEX + i * NUMBER_OF_BANKS);
        self.read_data_bank(ODD_BANK_INDEX + i * NUMBER_OF_BANKS);

        barrier::dsb(barrier::SY);
    }

    fn read_data_bank(&self, bank_index: usize) {
        let bank = self.bank_ptr(bank_index);
        // writeln!(&mut crate::uart::Uart0 {}, "Read data {:?}", bank).unwrap();
        unsafe { core::ptr::read_volatile(bank) };
    }

    fn execute_instruction_write_single_bank(&mut self, i: usize) {
        if !cfg!(feature = "cacheless") {
            self.preload_zero_bank(EVEN_BANK_INDEX + i * NUMBER_OF_BANKS);
            barrier::dsb(barrier::SY);
        }

        // Write to first bank
        self.write_data_bank(EVEN_BANK_INDEX + i * NUMBER_OF_BANKS);

        if !cfg!(feature = "cacheless") {
            self.invalidate_flush_bank(EVEN_BANK_INDEX + i * NUMBER_OF_BANKS);
        }

        barrier::dsb(barrier::SY);
    }

    fn execute_instruction_write_dual_bank(&mut self) {
        let i = 0;
        if !cfg!(feature = "cacheless") {
            self.preload_zero_bank(EVEN_BANK_INDEX + i * NUMBER_OF_BANKS);
            self.preload_zero_bank(ODD_BANK_INDEX + i * NUMBER_OF_BANKS);
            barrier::dsb(barrier::SY);
        }

        // Write to first and second bank
        self.write_data_bank(EVEN_BANK_INDEX + i * NUMBER_OF_BANKS);
        self.write_data_bank(ODD_BANK_INDEX + i * NUMBER_OF_BANKS);

        if !cfg!(feature = "cacheless") {
            self.invalidate_flush_bank(EVEN_BANK_INDEX + i * NUMBER_OF_BANKS);
            self.invalidate_flush_bank(ODD_BANK_INDEX + i * NUMBER_OF_BANKS);
        }

        barrier::dsb(barrier::SY);
    }

    fn write_data_bank(&mut self, bank_index: usize) {
        let bank = self.bank_ptr_mut(bank_index);
        unsafe {
            core::ptr::write_volatile(bank, Default::default());
        }
    }

    fn invalidate(&self) {
        (0..Self::OCCUPIED_CACHE_LINES).for_each(|idx| self.invalidate_bank(idx));
    }

    fn invalidate_bank(&self, bank_index: usize) {
        let bank = self.bank_ptr(bank_index);
        unsafe {
            asm!("dc ivac, {val}", val = in(reg) bank);
        }
    }

    fn invalidate_flush(&self) {
        (0..Self::OCCUPIED_CACHE_LINES).for_each(|idx| self.invalidate_flush_bank(idx));
    }

    fn invalidate_flush_bank(&self, bank_index: usize) {
        let bank = self.bank_ptr(bank_index);
        unsafe {
            asm!("dc civac, {val}", val = in(reg) bank);
        }
    }

    fn preload_zero(&self) {
        (0..Self::OCCUPIED_CACHE_LINES).for_each(|idx| self.preload_zero_bank(idx));
    }

    fn preload_zero_bank(&self, bank_index: usize) {
        let bank = self.bank_ptr(bank_index);
        unsafe {
            // Preload first bank
            asm!("dc zva, {val}", val = in(reg) bank);
        }
    }
}

#[repr(C, align(1024))]
pub struct DummyArray(pub [F16x16; NUMBER_OF_BANKS]);

impl PimRegion for DummyArray {
    const OCCUPIED_CACHE_LINES: usize = NUMBER_OF_BANKS;

    fn bank_ptr(&self, bank_index: usize) -> *const f16 {
        &self.0[bank_index] as *const F16x16 as *const f16
    }

    fn bank_ptr_mut(&mut self, bank_index: usize) -> *mut f16 {
        &mut self.0[bank_index] as *mut F16x16 as *mut f16
    }
}
