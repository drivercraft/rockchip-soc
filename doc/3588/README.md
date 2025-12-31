# RK3588 文档索引

RK3588 SoC 相关的技术文档和实现说明。

## 文档列表

### [PLL.md](PLL.md) - PLL 时钟配置

RK3588 PLL (Phase-Locked Loop) 时钟配置完整说明,包括:
- 9 个 PLL 列表和用途
- 寄存器映射表
- 频率计算公式 (整数和小数分频)
- 17 个预设频率表
- 使用示例和代码片段
- ID 映射说明

**适合**: 了解 RK3588 PLL 硬件配置和使用

### [PLL_ID_MAPPING.md](PLL_ID_MAPPING.md) - PLL ID 映射说明

详细解释 RK3588 PLL 的双重 ID 系统:
- C 代码枚举 (数组索引,从 0 开始)
- 设备树绑定 (时钟 ID,从 1 开始)
- Rust 实现如何处理两套 ID
- 完整映射表和使用示例
- 常见错误和正确用法

**适合**: 理解 PLL ID 的设计和使用,避免混淆

### [TEST_REPORT.md](TEST_REPORT.md) - PLL 测试报告

RK3588 PLL 配置的完整测试报告:
- 21 个单元测试覆盖说明
- 每个测试的验证内容
- 关键修正历史
- 与 u-boot C 代码对比
- 测试执行结果

**适合**: 验证实现正确性,了解测试覆盖

## 快速导航

### 我想了解...

| 需求 | 推荐文档 |
|------|---------|
| 了解 RK3588 PLL 硬件配置 | [PLL.md](PLL.md) |
| 理解 PLL ID 的两套编号系统 | [PLL_ID_MAPPING.md](PLL_ID_MAPPING.md) |
| 查看测试验证结果 | [TEST_REPORT.md](TEST_REPORT.md) |
| 计算 PLL 输出频率 | [PLL.md](PLL.md#频率计算公式) |
| 使用 PLL 配置 API | [PLL.md](PLL.md#使用示例) |
| 理解为什么时钟 ID 从 1 开始 | [PLL_ID_MAPPING.md](PLL_ID_MAPPING.md#为什么这样设计) |
| 排查 PLL ID 相关错误 | [PLL_ID_MAPPING.md](PLL_ID_MAPPING.md#常见错误) |

## 相关代码

| 代码文件 | 说明 |
|---------|------|
| `src/variants/rk3588/cru/pll.rs` | RK3588 PLL 配置实现 |
| `src/clock/pll.rs` | 通用 PLL 基础类型定义 |
| `src/variants/rk3588/cru/consts.rs` | CRU 寄存器常量定义 |
| `src/variants/rk3588/cru/mod.rs` | CRU 模块入口 |

## 测试

运行所有 RK3588 PLL 测试:

```bash
cd rockchip-soc
cargo test --lib pll
```

预期结果:
```
test result: ok. 21 passed; 0 failed
```

## 外部参考

1. **u-boot 源码**
   - `drivers/clk/rockchip/clk_rk3588.c` - PLL 配置
   - `drivers/clk/rockchip/clk_pll.c` - PLL 驱动
   - `arch/arm/include/asm/arch-rockchip/cru_rk3588.h` - CRU 定义

2. **设备树绑定**
   - `include/dt-bindings/clock/rk3588-cru.h` - 时钟 ID 定义

3. **硬件文档**
   - RK3588 TRM (Technical Reference Manual)

## 贡献

如需添加或更新文档,请遵循:
1. 使用 Markdown 格式
2. 保持与代码同步
3. 添加示例代码
4. 包含验证测试

---
**文档版本**: 1.0
**最后更新**: 2025-12-31
