mod consts;

use consts::*;

use crate::Mmio;

mod reg;

use reg::*;

pub struct GpioBank {
    base: usize,
}

impl GpioBank {
    pub fn new(base: Mmio) -> Self {
        GpioBank {
            base: base.as_ptr() as usize,
        }
    }

    fn reg(&self) -> &Registers {
        unsafe { &*(self.base as *const Registers) }
    }
}
