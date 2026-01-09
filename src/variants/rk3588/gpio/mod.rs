use crate::{GpioDirection, Mmio, PinId, PinctrlResult, pinctrl::Iomux, pinctrl::PinctrlError};

mod reg;

use reg::*;
use tock_registers::interfaces::{Readable, Writeable};

pub struct GpioBank {
    base: usize,
    iomux: [Iomux; 4],
}

impl GpioBank {
    pub fn new(base: Mmio, iomux: [Iomux; 4]) -> Self {
        GpioBank {
            base: base.as_ptr() as usize,
            iomux,
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

        if self.iomux[iomux_num as usize].contains(Iomux::UNROUTED) {
            debug!("verify_mux: pin {:?} does not support routing", pin);
            return Err(PinctrlError::Unsupported);
        }

        if self.iomux[iomux_num as usize].contains(Iomux::GPIO_ONLY) && mux != Iomux::GPIO_ONLY {
            debug!("verify_mux: pin {:?} only supports GPIO function", pin);
            return Err(PinctrlError::Unsupported);
        }

        Ok(())
    }

    pub fn iomux_gpio_only(&self, pin: PinId) -> bool {
        let iomux_num = pin.pin_in_bank() / 8;
        self.iomux[iomux_num as usize].contains(Iomux::GPIO_ONLY)
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
