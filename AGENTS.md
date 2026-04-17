# CLAUDE.md

此文件为 AI 助手在处理本仓库代码时提供指南与上下文。

## 项目概览

**当前状态**: 活跃开发中 | **主要语言**: Rust

**语言约定**: 为了便于指导，本文件 (`AGENTS.md`) 使用中文编写，且**与用户交流时请使用中文**。但项目代码中的**所有文档注释 (doc comments)**、**行内注释**以及**提交信息**必须使用**英文**。

**目录约定**: 任何被 `.gitignore` 完全忽略的目录，均仅作为参考资料，**不是本项目的一部分**。

`cbp` 是一个跨平台二进制包管理器，专注于简化 CLI 工具的分发，特别是生物信息学软件。

`cbp` = **C**ross-platform **B**inary **P**ackage manager

## 构建命令

### 构建

```bash
# 开发构建
cargo build

# 发布构建 (高性能)
cargo build --release
```

### 测试

```bash
# 运行所有测试
cargo test -- --test-threads=1
```

## 架构

### 源代码组织

- **`src/cbp.rs`** - 主程序入口，负责命令行解析和分发。
    - 使用 `clap` 进行参数解析。
    - 在 `main` 函数中注册所有子命令模块。
- **`src/lib.rs`** - 库入口，导出模块。
- **`src/cmd_cbp/`** - 命令实现模块。按功能分组：
    - **Package Management**: `init`, `install`, `local`, `list`, `remove`, `info`, `avail`, `check`.
    - **Utilities**: `tar`, `prefix`, `kb`, `collect`, `dot`.
    - **Build**: `build` (包含多个子命令).
- **`src/libs/`** - 共享工具库和核心逻辑。
  - **`dirs.rs`** - 目录结构管理。
  - **`utils.rs`** - 通用工具函数。
  - **`build.rs`** - 构建相关逻辑。
  - **`dot.rs`** - Dotfiles 管理。

### 命令结构 (Command Structure)

每个命令在 `src/cmd_cbp/` 下作为一个独立的模块实现，通常包含两个公开函数：

1. **`make_subcommand`**: 定义命令行接口。
    - 返回 `clap::Command`。
    - 使用 `.about(...)` 设置简短描述。
    - 推荐使用 `.after_help(...)` 引入详细文档。
2. **`execute`**: 命令执行逻辑。
    - 接收 `&clap::ArgMatches`。
    - 返回 `anyhow::Result<()>`。

## 开发工作流

### 添加新命令

1.  在 `src/cmd_cbp/` 下创建新文件 (或新建目录用于复杂命令)。
2.  在 `src/cmd_cbp/mod.rs` 中声明该模块。
3.  在 `src/cbp.rs` 中注册该子命令。
4.  实现 `make_subcommand` 和 `execute`。
5.  添加测试文件 `tests/cli_<command>.rs`。

### 测试约定

- 集成测试位于 `tests/` 目录下，通常命名为 `cli_<category>.rs` (如 `cli_dot.rs`).
- 测试数据通常放在 `tests/` 下的相关子目录中。
- **推荐使用 `assert_cmd`** 来编写集成测试，以验证二进制文件的行为。
- **稳定性原则 (Zero Panic)**: 任何用户输入（包括畸形数据）都不应导致程序 Panic。必须捕获所有错误并返回友好的错误信息。
- **Mock 环境**: 测试应使用临时目录，避免影响用户本地系统。

## 代码规范

- 使用 `cargo fmt` 格式化代码。
- 使用 `cargo clippy` 检查潜在问题。
- 优先使用标准库和项目中已引入的 crate。
- 保持代码简洁，注重性能。

## 帮助文本规范 (Help Text Style Guide)

### Rust 实现规范 (Implementation)

* **`about`**: 使用第三人称单数动词，简要描述操作。
* **`after_help`**: 使用字符串字面量或 `include_str!` 引入外部文档。

### 外部帮助文档规范

* **内容原则**: 只包含 clap 不会自动生成的补充说明，避免重复。
* **可选章节**:
    * `Behavior:` - 命令行为说明、特殊逻辑、注意事项。
    * `Input:` - 输入格式、来源说明。
    * `Output:` - 输出格式、目标说明。
    * `Examples:` - 使用示例，使用反引号包裹命令。
* **避免内容**:
    * `Options:` 章节（clap 自动生成参数列表）。
    * 参数默认值（clap 自动生成）。
