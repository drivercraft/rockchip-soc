use crate::{Mmio, grf::GrfMmio};

mod syscon;

#[derive(Debug, Clone)]
pub struct Cru {
    base: usize,
    grf: usize,
}

impl Cru {
    pub fn new(base: Mmio, sys_grf: Mmio) -> Self {
        Cru {
            base: base.as_ptr() as usize,
            grf: sys_grf.as_ptr() as usize,
        }
    }

    pub fn init(&mut self) {
        
    }

    pub fn grf_mmio_ls() -> &'static [GrfMmio] {
        &[syscon::grf_mmio::SYS_GRF]
    }

    fn reg(&self, offset: usize) -> *mut u32 {
        (self.base + offset) as *mut u32
    }

    fn read(&self, offset: usize) -> u32 {
        unsafe { core::ptr::read_volatile(self.reg(offset)) }
    }

    fn write(&self, offset: usize, value: u32) {
        unsafe { core::ptr::write_volatile(self.reg(offset), value) }
    }
}
