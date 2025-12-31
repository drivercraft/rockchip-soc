# RK3588 PLL ID 双重映射说明

## 问题背景

RK3588 PLL ID 存在两套编号系统,容易混淆。

## 两套 ID 系统

### 1. C 代码枚举 (内部索引)

**文件**: `u-boot-orangepi/arch/arm/include/asm/arch-rockchip/cru_rk3588.h`

```c
enum rk3588_pll_id {
    B0PLL,    // 0
    B1PLL,    // 1
    LPLL,     // 2
    CPLL,     // 3
    GPLL,     // 4
    NPLL,     // 5
    V0PLL,    // 6
    AUPLL,    // 7
    PPLL,     // 8
    PLL_COUNT,
};
```

**用途**: 内部数组索引,从 **0** 开始

### 2. 设备树绑定 (时钟 ID)

**文件**: `u-boot-orangepi/include/dt-bindings/clock/rk3588-cru.h`

```c
#define PLL_B0PLL    1
#define PLL_B1PLL    2
#define PLL_LPLL     3
#define PLL_V0PLL    4
#define PLL_AUPLL    5
#define PLL_CPLL     6
#define PLL_GPLL     7
#define PLL_NPLL     8
#define PLL_PPLL     9
```

**用途**: 时钟框架标识符,从 **1** 开始

## Rust 实现

### PllId 枚举 (数组索引)

```rust
#[repr(usize)]
pub enum PllId {
    B0PLL,   // 0 - 数组索引
    B1PLL,   // 1
    LPLL,    // 2
    CPLL,    // 3
    GPLL,    // 4
    NPLL,    // 5
    V0PLL,   // 6
    AUPLL,   // 7
    PPLL,    // 8
    _Len,
}
```

### PllClock 结构体

```rust
pub struct PllClock {
    pub id: u32,              // 时钟 ID (1-9, 匹配设备树)
    pub con_offset: u32,
    pub mode_offset: u32,
    // ...
}
```

### 宏定义

```rust
macro_rules! pll {
    ($id:ident, $con:expr, $mode:expr, $mshift:expr, $lshift:expr, $pflags:expr) => {
        PllClock {
            // 时钟 ID: 从 1 开始 (匹配设备树绑定 rk3588-cru.h)
            id: PllId::$id as u32 + 1,  // 0+1=1, 1+1=2, ..., 8+1=9
            // ...
        }
    };
}
```

## 完整映射表

| PLL   | PllId 枚举 | 数组索引 | PllClock.id | 设备树宏     |
|-------|-----------|---------:|------------:|-------------|
| B0PLL | `PllId::B0PLL` | 0 | 1 | `PLL_B0PLL` |
| B1PLL | `PllId::B1PLL` | 1 | 2 | `PLL_B1PLL` |
| LPLL  | `PllId::LPLL`  | 2 | 3 | `PLL_LPLL`  |
| CPLL  | `PllId::CPLL`  | 3 | 4 | `PLL_CPLL`  |
| GPLL  | `PllId::GPLL`  | 4 | 5 | `PLL_GPLL`  |
| NPLL  | `PllId::NPLL`  | 5 | 6 | `PLL_NPLL`  |
| V0PLL | `PllId::V0PLL` | 6 | 7 | `PLL_V0PLL` |
| AUPLL | `PllId::AUPLL` | 7 | 8 | `PLL_AUPLL` |
| PPLL  | `PllId::PPLL`  | 8 | 9 | `PLL_PPLL`  |

## 使用示例

### 数组访问 (使用枚举值)

```rust
// 通过 PllId 枚举访问数组 (索引 0-8)
let gpll = &RK3588_PLL_CLOCKS[PllId::GPLL as usize];  // 索引 4
```

### 获取时钟 ID (用于时钟框架)

```rust
// 获取时钟框架标识符 (1-9)
let clock_id = gpll.id;  // 5 (匹配 PLL_GPLL)
```

### 完整示例

```rust
use rockchip_soc::rk3588::cru::pll::{PllId, get_pll};

// 1. 使用枚举访问数组
let pll_idx = PllId::GPLL as usize;  // 4
let pll = &RK3588_PLL_CLOCKS[pll_idx];

// 2. 或使用辅助函数
let pll = get_pll(PllId::GPLL);

// 3. 获取时钟 ID (用于时钟框架)
println!("时钟 ID: {}", pll.id);  // 5

// 4. 获取 PLL 名称
println!("PLL 名称: {}", PllId::GPLL.name());  // "GPLL"
```

## 为什么这样设计?

### 1. 数组索引从 0 开始

Rust/C 数组索引天然从 0 开始,使用枚举直接作为索引最自然:

```rust
pub const RK3588_PLL_CLOCKS: [PllClock; PllId::_Len as usize] = [
    // [0] B0PLL
    pll!(B0PLL, ...),
    // [1] B1PLL
    pll!(B1PLL, ...),
    ...
];
```

访问时:
```rust
let b0pll = &RK3588_PLL_CLOCKS[PllId::B0PLL as usize];  // 索引 0
```

### 2. 时钟 ID 从 1 开始

Linux/u-boot 时钟框架约定 0 保留/无效,有效 ID 从 1 开始:

```c
// rk3588-cru.h
#define PLL_B0PLL    1  // 不是 0!
```

这符合 Linux 时钟框架的规范,其中 0 通常表示"无效"或"未指定"。

### 3. 两者分离

- `PllId` 枚举值: **内部使用**,用于数组索引
- `PllClock.id` 字段: **外部使用**,用于时钟框架标识

## 验证

所有测试都验证了这个映射关系:

```rust
#[test]
fn test_gpll_config() {
    let pll = &RK3588_PLL_CLOCKS[PllId::GPLL as usize];  // 索引 4
    assert_eq!(pll.id, 5, "GPLL ID should be 5");        // 时钟 ID 5
    // ...
}

#[test]
fn test_pll_ids() {
    // 验证枚举索引
    assert_eq!(PllId::B0PLL as usize, 0);
    assert_eq!(PllId::GPLL as usize, 4);
    assert_eq!(PllId::PPLL as usize, 8);
}
```

运行测试:
```bash
cargo test --lib pll
```

结果:
```
test result: ok. 21 passed; 0 failed
```

## 常见错误

### ❌ 错误 1: 混淆数组索引和时钟 ID

```rust
// 错误: 认为数组索引等于时钟 ID
let pll = &RK3588_PLL_CLOCKS[5];  // 这是 NPLL,不是 GPLL!
```

**正确**:
```rust
let pll = &RK3588_PLL_CLOCKS[PllId::GPLL as usize];  // 使用枚举
assert_eq!(pll.id, 5);  // 时钟 ID 是 5
```

### ❌ 错误 2: 直接使用时钟 ID 访问数组

```rust
// 错误: 使用时钟 ID 访问数组
let clock_id = 5;  // GPLL 的时钟 ID
let pll = &RK3588_PLL_CLOCKS[clock_id];  // 越界! 应该用索引 4
```

**正确**:
```rust
// 如果只有时钟 ID,需要转换
fn clock_id_to_pll_id(clock_id: u32) -> Option<PllId> {
    match clock_id {
        1 => Some(PllId::B0PLL),
        2 => Some(PllId::B1PLL),
        // ...
        5 => Some(PllId::GPLL),
        _ => None,
    }
}

if let Some(pll_id) = clock_id_to_pll_id(5) {
    let pll = &RK3588_PLL_CLOCKS[pll_id as usize];
}
```

## 总结

- **PllId 枚举值** (0-8): 用于数组访问
- **PllClock.id** (1-9): 用于时钟框架
- 不要混淆两者!
- 使用 `get_pll()` 辅助函数更安全

---
**版本**: 1.0
**更新时间**: 2025-12-31
