# PLL 结构体使用示例

## 概述

本项目实现了 Rockchip PLL (Phase-Locked Loop) 时钟结构的 Rust 版本,对应 C 语言的 `struct rockchip_pll_clock` 和 `struct rockchip_pll_rate_table`。

## 核心结构

### 1. PllClock - PLL 时钟控制结构

```rust
pub struct PllClock {
    pub id: u32,                          // 时钟 ID
    pub con_offset: u32,                   // PLL 控制寄存器偏移
    pub mode_offset: u32,                  // 模式寄存器偏移
    pub mode_shift: u32,                   // 模式位偏移
    pub lock_shift: u32,                   // 锁定位偏移
    pub pll_type: RockchipPllType,         // PLL 类型
    pub pll_flags: u32,                    // PLL 标志位
    pub rate_table: Option<&'static PllRateTable>, // 速率表
    pub mode_mask: u32,                    // 模式掩码
}
```

### 2. PllRateTable - PLL 速率表

支持两种不同的参数集:

```rust
pub struct PllRateTable {
    pub rate: u32,                         // 输出频率 (Hz)
    pub nr: u32,                           // 参考分频系数
    pub nf: u32,                           // 反馈分频系数
    pub no: u32,                           // 输出分频系数
    pub nb: u32,                           // 带宽分频系数
    pub params: PllRateParams,             // 特定参数 (枚举)
}

pub enum PllRateParams {
    // RK3036/RK3399 参数
    Rk3036 {
        fbdiv: u32,    // 反馈分频
        postdiv1: u32, // 后分频器 1
        refdiv: u32,   // 参考分频
        postdiv2: u32, // 后分频器 2
        dsmpd: u32,    // 小数分频使能
        frac: u32,     // 小数分频系数
    },

    // RK3588 参数
    Rk3588 {
        m: u32,       // M 分频系数
        p: u32,       // P 分频系数
        s: u32,       // S 分频系数
        k: u32,       // K 小数分频系数
    },
}
```

## 使用示例

### 示例 1: 创建 RK3588 GPLL

```rust
use rockchip_soc::clock::pll::{PllClock, PllRateTable, RockchipPllType, pll_flags};

// 创建 RK3588 类型的速率表
static GPLL_RATE: PllRateTable = PllRateTable::rk3588(
    1188 * 1_000_000,  // rate: 1188 MHz
    1,                  // nr
    99,                 // nf
    1,                  // no
    0,                  // nb
    99,                 // m
    3,                  // p
    1,                  // s
    0,                  // k
);

// 创建 GPLL 实例
let gpll = PllClock::new(
    1,                          // id
    0x0000,                     // con_offset
    0x0100,                     // mode_offset
    8,                          // mode_shift
    10,                         // lock_shift
    RockchipPllType::PllRk3588, // pll_type
    pll_flags::PLL_RK3588,      // pll_flags
    Some(&GPLL_RATE),           // rate_table
    0x3,                        // mode_mask
);

// 检查 PLL 是否锁定
if gpll.is_locked(cru_base) {
    println!("GPLL 已锁定");
}

// 获取 PLL 模式
let mode = gpll.get_mode(cru_base);

// 设置 PLL 模式
gpll.set_mode(cru_base, 0x1);
```

### 示例 2: 创建 RK3399 PLL

```rust
// 创建 RK3399 类型的速率表
static RK3399_RATE: PllRateTable = PllRateTable::rk3036(
    600 * 1_000_000,   // rate: 600 MHz
    1,                  // nr
    50,                 // nf
    1,                  // no
    0,                  // nb
    50,                 // fbdiv
    1,                  // postdiv1
    1,                  // refdiv
    1,                  // postdiv2
    1,                  // dsmpd
    0,                  // frac
);

let pll = PllClock::new(
    2,
    0x0020,
    0x0100,
    8,
    10,
    RockchipPllType::PllRk3399,
    pll_flags::PLL_RK3399,
    Some(&RK3399_RATE),
    0x3,
);
```

### 示例 3: 匹配不同的 PLL 参数

```rust
match rate_table.params {
    PllRateParams::Rk3036 { fbdiv, postdiv1, postdiv2, .. } => {
        println!("RK3036/RK3399 PLL:");
        println!("  fbdiv = {}", fbdiv);
        println!("  postdiv1 = {}", postdiv1);
        println!("  postdiv2 = {}", postdiv2);
    }

    PllRateParams::Rk3588 { m, p, s, k } => {
        println!("RK3588 PLL:");
        println!("  m = {}", m);
        println!("  p = {}", p);
        println!("  s = {}", s);
        println!("  k = {}", k);
    }
}
```

## PLL 标志位

可用的 PLL 标志位 (按位或组合使用):

```rust
pub mod pll_flags {
    pub const PLL_SYNC: u32 = 1 << 0;      // 同步模式
    pub const PLL_FRAC: u32 = 1 << 1;      // 小数分频
    pub const PLL_POSTDIV4: u32 = 1 << 2;  // 4 个后分频器
    pub const PLL_RK3588: u32 = 1 << 3;    // RK3588 类型
    pub const PLL_RK3399: u32 = 1 << 4;    // RK3399 类型
}

// 组合使用多个标志
let flags = pll_flags::PLL_SYNC | pll_flags::PLL_FRAC;
```

## C 语言对应

### 原始 C 结构体

```c
struct rockchip_pll_clock {
    unsigned int           id;
    unsigned int           con_offset;
    unsigned int           mode_offset;
    unsigned int           mode_shift;
    unsigned int           lock_shift;
    enum rockchip_pll_type type;
    unsigned int           pll_flags;
    struct rockchip_pll_rate_table *rate_table;
    unsigned int           mode_mask;
};

struct rockchip_pll_rate_table {
    unsigned long rate;
    unsigned int nr;
    unsigned int nf;
    unsigned int no;
    unsigned int nb;
    /* for RK3036/RK3399 */
    unsigned int fbdiv;
    unsigned int postdiv1;
    unsigned int refdiv;
    unsigned int postdiv2;
    unsigned int dsmpd;
    unsigned int frac;
    /* for RK3588 */
    unsigned int m;
    unsigned int p;
    unsigned int s;
    unsigned int k;
};
```

### Rust 实现改进

1. **类型安全**: 使用枚举 `PllRateParams` 区分不同芯片类型的参数
2. **零成本抽象**: 枚举编译后与 C 联合体具有相同的内存布局
3. **构造函数**: 提供 `rk3036()` 和 `rk3588()` 便捷构造函数
4. **文档注释**: 完整的中文文档说明每个字段的含义

## 注意事项

1. 速率表必须是 `'static` 生命周期,通常使用 `static` 声明
2. 不同芯片类型使用不同的参数集,不能混用
3. 寄存器操作需要 `unsafe` 块,已封装在 `is_locked()`, `get_mode()`, `set_mode()` 方法中
4. 标志位可以通过按位或 (`|`) 组合使用

## 编译验证

```bash
cargo check
cargo test pll
```
