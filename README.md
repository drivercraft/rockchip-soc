# rockchip-soc

Rockchip SoC 的 Rust 实现,提供时钟和复位单元 (CRU) 驱动。

## 当前支持

### RK3588

- ✅ PLL 时钟配置 (9 个 PLL)
- ✅ 频率计算 (整数和小数分频)
- ✅ 17 个预设频率
- ✅ 完整的单元测试覆盖

**详细文档**: [doc/3588/](doc/3588/)

- [PLL 时钟配置](doc/3588/PLL.md)
- [PLL ID 映射说明](doc/3588/PLL_ID_MAPPING.md)
- [测试报告](doc/3588/TEST_REPORT.md)

## 快速开始

### 运行测试

```bash
# 运行所有库测试
cargo test --lib

# 运行 PLL 相关测试
cargo test --lib pll

# 运行集成测试 (需要 ostool)
cargo install ostool
cargo test --test test

# 带 u-boot 的开发板测试
cargo test --test test -- -- uboot
```

## 项目结构

```text
rockchip-soc/
├── src/
│   ├── clock/
│   │   └── pll.rs              # 通用 PLL 类型定义
│   ├── variants/
│   │   ├── rk3588/
│   │   │   └── cru/
│   │   │       ├── pll.rs      # RK3588 PLL 配置
│   │   │       ├── consts.rs   # CRU 寄存器常量
│   │   │       └── mod.rs      # CRU 模块
│   │   └── mod.rs
│   └── lib.rs
├── doc/
│   └── 3588/                   # RK3588 文档
│       ├── README.md           # 文档索引
│       ├── PLL.md              # PLL 配置说明
│       ├── PLL_ID_MAPPING.md   # ID 映射说明
│       └── TEST_REPORT.md      # 测试报告
└── README.md
```

## 设计原则

- **SOLID**: 单一职责,开闭原则,里氏替换,接口隔离,依赖倒置
- **KISS**: 保持简单直观
- **DRY**: 避免重复代码
- **YAGNI**: 只实现必需功能

## 参考资料

1. [u-boot RK3588 时钟驱动](https://source.denx.de/u-boot/u-boot/-/blob/master/drivers/clk/rockchip)
2. [RK3588 TRM](https://www.rockchip.com) (需要 NDA)

## 许可证

[待定]

---
**版本**: 0.1.0
**更新时间**: 2025-12-31
