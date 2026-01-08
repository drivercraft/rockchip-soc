pub mod consts;

use consts::*;

use crate::{GpioDirection, Mmio};

mod reg;

use core::fmt;
use reg::*;
use tock_registers::interfaces::{Readable, Writeable};

/// GPIO 错误类型
#[derive(Debug)]
pub enum GpioError {
    /// 无效的引脚编号（必须 < 32）
    InvalidPin(u32),

    /// 尝试写入未配置为输出的引脚
    NotConfiguredAsOutput,
}

impl fmt::Display for GpioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidPin(pin) => write!(f, "无效的 GPIO 引脚编号: {} (必须 < 32)", pin),
            Self::NotConfiguredAsOutput => write!(f, "引脚未配置为输出方向"),
        }
    }
}

/// GPIO 操作 Result 类型
pub type GpioResult<T> = core::result::Result<T, GpioError>;

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
    pub fn set_direction(&self, pin_in_bank: u32, direction: GpioDirection) -> GpioResult<()> {
        match direction {
            GpioDirection::Input => self.set_direction_input(pin_in_bank),
            GpioDirection::Output(value) => self.set_direction_output(pin_in_bank, value),
        }
    }

    /// 设置引脚为输入方向
    ///
    /// # 参数
    ///
    /// * `pin_in_bank` - Bank 内的引脚编号 (0-31)
    #[inline]
    pub fn set_direction_input(&self, pin_in_bank: u32) -> GpioResult<()> {
        if pin_in_bank >= 32 {
            return Err(GpioError::InvalidPin(pin_in_bank));
        }

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
    pub fn set_direction_output(&self, pin_in_bank: u32, value: bool) -> GpioResult<()> {
        if pin_in_bank >= 32 {
            return Err(GpioError::InvalidPin(pin_in_bank));
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
    pub fn read(&self, pin_in_bank: u32) -> GpioResult<bool> {
        if pin_in_bank >= 32 {
            return Err(GpioError::InvalidPin(pin_in_bank));
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
    pub fn write(&self, pin_in_bank: u32, value: bool) -> GpioResult<()> {
        if pin_in_bank >= 32 {
            return Err(GpioError::InvalidPin(pin_in_bank));
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
    pub fn get_direction(&self, pin_in_bank: u32) -> GpioResult<GpioDirection> {
        if pin_in_bank >= 32 {
            return Err(GpioError::InvalidPin(pin_in_bank));
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
