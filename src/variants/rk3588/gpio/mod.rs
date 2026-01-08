pub mod consts;

use consts::*;

use crate::{GpioDirection, Mmio};

mod reg;

use core::fmt;
use reg::*;

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
        // 清除方向寄存器的对应位（设置为输入）
        unsafe {
            let reg_ptr = &self.reg().swport_ddr as *const _ as *mut u32;
            let current = reg_ptr.read_volatile();
            reg_ptr.write_volatile(current & !mask);
        }

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

        unsafe {
            // 先设置输出值
            let dr_ptr = &self.reg().swport_dr as *const _ as *mut u32;
            let current_dr = dr_ptr.read_volatile();
            let new_dr = if value {
                current_dr | mask
            } else {
                current_dr & !mask
            };
            dr_ptr.write_volatile(new_dr);

            // 再设置方向为输出
            let ddr_ptr = &self.reg().swport_ddr as *const _ as *mut u32;
            let current_ddr = ddr_ptr.read_volatile();
            ddr_ptr.write_volatile(current_ddr | mask);
        }

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
        let value = unsafe {
            let reg_ptr = &self.reg().ext_port as *const _ as *const u32;
            reg_ptr.read_volatile()
        };

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

        unsafe {
            let reg_ptr = &self.reg().swport_dr as *const _ as *mut u32;
            let current = reg_ptr.read_volatile();
            let new_value = if value {
                current | mask
            } else {
                current & !mask
            };
            reg_ptr.write_volatile(new_value);
        }

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
        let ddr_value = unsafe {
            let reg_ptr = &self.reg().swport_ddr as *const _ as *const u32;
            reg_ptr.read_volatile()
        };

        if (ddr_value & mask) != 0 {
            // 输出方向：同时读取输出值
            let dr_value = unsafe {
                let reg_ptr = &self.reg().swport_dr as *const _ as *const u32;
                reg_ptr.read_volatile()
            };
            Ok(GpioDirection::Output((dr_value & mask) != 0))
        } else {
            // 输入方向
            Ok(GpioDirection::Input)
        }
    }
}
