use crate::{GpioDirection, Mmio, PinId, PinctrlResult, pinctrl::Iomux, pinctrl::PinctrlError};

mod reg;

use reg::*;
use tock_registers::interfaces::{Readable, Writeable};

#[derive(Debug, Clone, Copy)]
pub(crate) struct IomuxReg {
    pub ty: Iomux,
    pub offset: usize,
}

pub struct GpioBank {
    base: usize,
    pub(crate) iomux: [IomuxReg; 4],
}

impl GpioBank {
    pub fn new(base: Mmio, bank_id: usize, iomux: [Iomux; 4]) -> Self {
        // 根据 u-boot rockchip_pinctrl_get_soc_data 函数实现 offset 初始化
        //
        // RK3588 硬件特性：
        // - GPIO0 使用 PMU1_IOC (0x0000)
        // - GPIO1-4 使用 BUS_IOC (0x8000)
        // - 4-bit IOMUX，每组占用 8 字节（2 个寄存器 × 4 字节）
        let base_offset = if bank_id == 0 {
            0x0000 // GPIO0 使用 PMU1_IOC
        } else {
            0x8000 // GPIO1-4 使用 BUS_IOC
        };

        // 每个 iomux 组占用 8 字节（2 个寄存器 × 4 字节）
        let iomux_regs = [
            IomuxReg {
                ty: iomux[0],
                offset: base_offset + 0x00, // 引脚 0-7
            },
            IomuxReg {
                ty: iomux[1],
                offset: base_offset + 0x08, // 引脚 8-15
            },
            IomuxReg {
                ty: iomux[2],
                offset: base_offset + 0x10, // 引脚 16-23
            },
            IomuxReg {
                ty: iomux[3],
                offset: base_offset + 0x18, // 引脚 24-31
            },
        ];

        GpioBank {
            base: base.as_ptr() as usize,
            iomux: iomux_regs,
        }
    }

    fn reg(&self) -> &Registers {
        unsafe { &*(self.base as *const Registers) }
    }

    pub fn verify_mux(&self, pin: PinId, mux: Iomux) -> PinctrlResult<()> {
        let pin_in_bank = pin.pin_in_bank();
        if pin_in_bank >= 32 {
            return Err(PinctrlError::InvalidPinId(pin));
        }
        let iomux_num = pin_in_bank / 8;

        if self.iomux[iomux_num as usize].ty.contains(Iomux::UNROUTED) {
            debug!("verify_mux: pin {:?} does not support routing", pin);
            return Err(PinctrlError::Unsupported);
        }

        if self.iomux[iomux_num as usize].ty.contains(Iomux::GPIO_ONLY) && mux != Iomux::GPIO_ONLY {
            debug!("verify_mux: pin {:?} only supports GPIO function", pin);
            return Err(PinctrlError::Unsupported);
        }

        Ok(())
    }

    pub fn iomux_gpio_only(&self, pin: PinId) -> bool {
        let iomux_num = pin.pin_in_bank() / 8;
        self.iomux[iomux_num as usize].ty.contains(Iomux::GPIO_ONLY)
    }

    /// 设置引脚方向（统一接口）
    ///
    /// # 参数
    ///
    /// * `pin_in_bank` - Bank 内的引脚编号 (0-31)
    /// * `config` - 方向配置（输入或输出带初始值）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// bank.set_direction(5, DirectionConfig::Input)?;
    /// bank.set_direction(5, DirectionConfig::Output(true))?;  // 输出，初始值 HIGH
    /// ```
    pub fn set_direction(&self, pin: PinId, direction: GpioDirection) -> PinctrlResult<()> {
        match direction {
            GpioDirection::Input => self.set_direction_input(pin),
            GpioDirection::Output(value) => self.set_direction_output(pin, value),
        }
    }

    /// 设置引脚为输入方向
    ///
    /// # 参数
    ///
    /// * `pin_in_bank` - Bank 内的引脚编号 (0-31)
    #[inline]
    pub fn set_direction_input(&self, pin: PinId) -> PinctrlResult<()> {
        let pin_in_bank = pin.pin_in_bank();
        let mask = 1u32 << pin_in_bank;
        let current = self.reg().swport_ddr.get();
        self.reg().swport_ddr.set(current & !mask);

        Ok(())
    }

    /// 设置引脚为输出方向并设置初始值
    ///
    /// # 参数
    ///
    /// * `pin_in_bank` - Bank 内的引脚编号 (0-31)
    /// * `value` - 初始输出值
    #[inline]
    pub fn set_direction_output(&self, pin: PinId, value: bool) -> PinctrlResult<()> {
        let pin_in_bank = pin.pin_in_bank();
        if pin_in_bank >= 32 {
            return Err(PinctrlError::InvalidPinId(pin));
        }

        let mask = 1u32 << pin_in_bank;

        // 设置初始输出值
        let current_dr = self.reg().swport_dr.get();
        let new_dr = if value {
            current_dr | mask
        } else {
            current_dr & !mask
        };
        self.reg().swport_dr.set(new_dr);

        // 设置为输出
        let current_ddr = self.reg().swport_ddr.get();
        self.reg().swport_ddr.set(current_ddr | mask);

        Ok(())
    }

    /// 读取引脚值
    ///
    /// # 参数
    ///
    /// * `pin_in_bank` - Bank 内的引脚编号 (0-31)
    pub fn read(&self, pin: PinId) -> PinctrlResult<bool> {
        let pin_in_bank = pin.pin_in_bank();
        if pin_in_bank >= 32 {
            return Err(PinctrlError::InvalidPinId(pin));
        }

        let mask = 1u32 << pin_in_bank;
        let value = self.reg().ext_port.get();
        Ok((value & mask) != 0)
    }

    /// 写入引脚值
    ///
    /// 引脚必须已配置为输出方向。
    ///
    /// # 参数
    ///
    /// * `pin_in_bank` - Bank 内的引脚编号 (0-31)
    /// * `value` - 输出值
    pub fn write(&self, pin: PinId, value: bool) -> PinctrlResult<()> {
        let pin_in_bank = pin.pin_in_bank();
        if pin_in_bank >= 32 {
            return Err(PinctrlError::InvalidPinId(pin));
        }

        let mask = 1u32 << pin_in_bank;
        let current = self.reg().swport_dr.get();
        let new_value = if value {
            current | mask
        } else {
            current & !mask
        };
        self.reg().swport_dr.set(new_value);

        Ok(())
    }

    /// 获取引脚方向配置
    ///
    /// 如果引脚配置为输出，同时返回当前输出值。
    ///
    /// # 参数
    ///
    /// * `pin_in_bank` - Bank 内的引脚编号 (0-31)
    ///
    /// # 返回
    ///
    /// 返回 `DirectionConfig`：
    /// - `Input` - 引脚配置为输入
    /// - `Output(value)` - 引脚配置为输出，value 为当前输出值
    pub fn get_direction(&self, pin: PinId) -> PinctrlResult<GpioDirection> {
        let pin_in_bank = pin.pin_in_bank();
        if pin_in_bank >= 32 {
            return Err(PinctrlError::InvalidPinId(pin));
        }

        let mask = 1u32 << pin_in_bank;

        // 读取方向寄存器
        let ddr_value = self.reg().swport_ddr.get();

        if (ddr_value & mask) != 0 {
            // 输出方向：同时读取输出值
            let dr_value = self.reg().swport_dr.get();
            Ok(GpioDirection::Output((dr_value & mask) != 0))
        } else {
            // 输入方向
            Ok(GpioDirection::Input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iomux_offset_gpio0() {
        // GPIO0 应使用 PMU1_IOC 偏移
        let base = unsafe { Mmio::new_unchecked(0xfd8a0000 as *mut u8) };
        let iomux = [Iomux::WIDTH_4BIT; 4];
        let bank = GpioBank::new(base, 0, iomux);

        assert_eq!(bank.iomux[0].offset, 0x0000);
        assert_eq!(bank.iomux[1].offset, 0x0008);
        assert_eq!(bank.iomux[2].offset, 0x0010);
        assert_eq!(bank.iomux[3].offset, 0x0018);
    }

    #[test]
    fn test_iomux_offset_gpio1() {
        // GPIO1 应使用 BUS_IOC 偏移
        let base = unsafe { Mmio::new_unchecked(0xfec20000 as *mut u8) };
        let iomux = [Iomux::WIDTH_4BIT; 4];
        let bank = GpioBank::new(base, 1, iomux);

        assert_eq!(bank.iomux[0].offset, 0x8000);
        assert_eq!(bank.iomux[1].offset, 0x8008);
        assert_eq!(bank.iomux[2].offset, 0x8010);
        assert_eq!(bank.iomux[3].offset, 0x8018);
    }

    #[test]
    fn test_offset_increment() {
        // 验证每个 iomux 组占用 8 字节
        let base = unsafe { Mmio::new_unchecked(0xfec20000 as *mut u8) };
        let iomux = [Iomux::WIDTH_4BIT; 4];
        let bank = GpioBank::new(base, 1, iomux);

        assert_eq!(bank.iomux[1].offset - bank.iomux[0].offset, 0x8);
        assert_eq!(bank.iomux[2].offset - bank.iomux[1].offset, 0x8);
        assert_eq!(bank.iomux[3].offset - bank.iomux[2].offset, 0x8);
    }
}
