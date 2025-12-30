#![no_std]
#![no_main]
#![feature(used_with_arg)]

extern crate alloc;
extern crate bare_test;

#[bare_test::tests]
mod tests {
    use bare_test::{mem::iomap, println};
    use log::info;
    use rockchip_soc::rk3588::*;

    #[test]
    fn it_works() {
        let cru3588 = 0xfd7c0000usize;
        let sys_grf = Cru::grf_mmio_ls()[0];

        let base = iomap(cru3588.into(), 0x5c000);
        let sys_grf = iomap(sys_grf.base.into(), sys_grf.size);

        let mut cru = Cru::new(base, sys_grf);

        cru.init();
    }
}
