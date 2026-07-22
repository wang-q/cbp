# CLAUDE.md

此文件是我（AI 助手）在本仓库工作时的行为准则。所有规则都是硬性要求，除非用户明确覆盖。

## 项目概览

- **与用户交流**: 中文
- 本文件 (`CLAUDE.md`) : 使用中文编写
- **代码注释 (doc comments `///` `//!` 和行内 `//`)**: 英文
- **Git 提交信息**: 英文
- **文档正文** (如 `docs/*.md` 中的说明文字): 英文

**目录约定**: 任何被 `.gitignore` 完全忽略的目录，均仅作为参考资料，**不是本项目的一部分**。

`cbp` 是一个跨平台二进制包管理器，专注于简化 CLI 工具的分发，特别是生物信息学软件。

`cbp` = **C**ross-platform **B**inary **P**ackage manager

## 代码风格

**权衡：** 这些准则偏向谨慎而非速度。对于简单任务，自行判断。

### 编码前先思考

**不要假设。不要隐藏困惑。呈现权衡。**

在实现之前：
- 明确陈述你的假设。如果不确定，就问。
- 如果存在多种理解，把它们都列出来 — 不要默默地选一个。
- 如果有更简单的方法，说出来。必要时提出反对意见。
- 如果有不明白的地方，停下来。指出困惑之处。问。

### 简洁优先

**用最少的代码解决问题。不做任何推测性设计。**

- 不添加未被要求的功能。
- 不添加未被要求的"灵活性"或"可配置性"。
- 不为不可能发生的场景写错误处理。
- 如果你写了 200 行但其实 50 行就够了，重写它。

问自己："资深工程师会觉得这过于复杂吗？" 如果是，简化。

### 精准修改

**只改必须改的。只清理自己造成的混乱。**

编辑现有代码时：
- 不要"改进"相邻的代码、注释或格式。
- 不要重构没有坏的东西。
- 匹配现有风格，即使你不会这样写。
- 如果你注意到无关的死代码，提出来 — 不要删除它。

当你的修改产生了孤立代码时：
- 删除因你的修改而变得未使用的 import/变量/函数。
- 不要删除之前就存在的死代码，除非被要求。

检验标准：每一行改动都应该能追溯到用户的请求。

### 目标驱动执行

**定义成功标准。循环直到验证通过。**

将任务转化为可验证的目标：
- "添加验证" → "为无效输入写测试，然后让它们通过"
- "修复 bug" → "写一个能复现它的测试，然后让它通过"
- "重构 X" → "确保重构前后测试都通过"

对于多步骤任务，陈述简要计划：
```
1. [步骤] → 验证: [检查]
2. [步骤] → 验证: [检查]
3. [步骤] → 验证: [检查]
```

强有力的成功标准让你可以独立循环。薄弱的标准（"让它能用"）需要不断澄清。

### 必须遵守

- 每个 PR / commit 跑 `cargo fmt` 和 `cargo clippy -- -D warnings`，clean 之后再提交
- 日志用 `tracing` 宏 (`info!`, `debug!`, `warn!`, `error!`)，**禁止** `println!` / `eprintln!`
- 公共 API (pub fn / pub struct / pub trait) 必须写 doc comment (英文，一行即可)
- 不写冗余注释 — 如果函数名和类型签名已经说明了行为，不要画蛇添足
- 用 `anyhow::Result<T>` 做函数返回值，`anyhow::bail!` / `anyhow::anyhow!` 构造错误

### 禁止

- 不要引入新依赖，除非用户明确要求
- 不要为了"可能"的未来需求写抽象 — 三次相似代码出现之后再考虑提取
- 不要写半成品实现 — stub / TODO 必须有明确的后续任务链接
- 不要用 `unsafe`，除非有充分理由且用户同意
- 不要写超过一行的 doc comment，除非是 trait 定义或复杂不变量
- 不要反向兼容的 shim（rename `_vars`、re-export 旧类型等）

## 构建与测试

```bash
# 开发构建
cargo build

# 发布构建 (高性能)
cargo build --release

# 运行所有测试
cargo test -- --test-threads=1

# 代码质量
cargo fmt
cargo clippy -- -D warnings
```

## 架构

### 源代码组织

- **`src/cbp.rs`** - 主程序入口，负责命令行解析和分发。
    - 使用 `clap` 进行参数解析。
    - 在 `main` 函数中注册所有子命令模块。
- **`src/lib.rs`** - 库入口，导出模块。
- **`src/cmd_cbp/`** - 命令实现模块。按功能分组：
    - **Package Management**: `init`, `install`, `local`, `list`, `remove`, `info`, `avail`, `check`.
    - **Utilities**: `tar`, `prefix`, `collect`, `dot`, `snap`.
    - **Build**: `build` (`font`, `source`, `prebuild`, `test`, `upload`, `validate`).
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


## 软件包 (Packages)

`cbp` 管理的软件包以 JSON 配置文件定义，存放在 `packages/` 目录下；构建产物为 `.tar.gz` 压缩包，存放在 `binaries/` 目录下。

### 软件包配置文件

每个软件包对应一个 JSON 文件，字段定义参见 `docs/schema/schema.json`：

* 必填字段：`name`、`version`、`description`、`homepage`、`license`、`type`
* `name`：小写字母开头，仅含小写字母、数字、点、连字符
* `version`：数字格式，可带可选后缀（如 `x.y.z`、`x.y`、`2.4pre`）
* `type`：构建类型，可选值为
    * `prebuild`：从官方预构建二进制分发
    * `rust`：使用 Rust 构建
    * `make`：使用 Makefile 构建
    * `cmake`：使用 CMake 构建
    * `autotools`：使用 Autotools 构建
    * `vcpkg`：使用 vcpkg 构建
    * `font`：字体包
    * `source`：通用源码构建
* `downloads`：下载配置，按平台分组（`source`、`linux`、`macos`、`windows`、`font`），每个平台可指定 `url`、`binary`、`extract` 等
* `tests`：安装后的验证测试，包含 `name`、`command`、`pattern`，可选 `args` 和 `ignore_exit_code`

### 构建产物命名

构建完成的二进制包按平台命名：

* Linux：`{package}.linux.tar.gz`
* macOS：`{package}.macos.tar.gz`
* Windows：`{package}.windows.tar.gz`
* 字体：`{package}.font.tar.gz`

### 分发方式

二进制包通过 GitHub Releases 的 `Binaries` 标签页统一发布，URL 格式为：

```text
https://github.com/wang-q/cbp/releases/download/Binaries/{package}.{platform}.tar.gz
```

发布时使用 `cbp build upload binaries/{package}.{platform}.tar.gz`，该命令会自动计算 MD5 并更新 Release notes。

### 动态库依赖

构建目标尽量静态链接，允许的动态依赖仅限系统库（如 `libc.so.6`、`libstdc++.so.6`、`libm.so.6`、`libgcc_s.so.1` 等）。

### 更新 prebuild 软件包

对于 `type` 为 `prebuild` 的软件包，按以下步骤更新到最新版本：

1. 打开 `packages/{name}.json`，确认当前 `version` 与各平台 `downloads` URL。
2. 访问上游 Release 页面（通常为 GitHub `releases/latest`），获取最新版本号与资产下载地址。
3. 更新 `version` 字段。
4. 更新 `downloads` 下各平台的 `url`；若资产名称或架构发生变化，同步修改 `binary` 等字段。
5. 使用 `python3 -m json.tool packages/{name}.json` 校验 JSON 语法。
6. 构建本地安装包：
   `cbp build prebuild {name}`
   该命令会读取更新后的 JSON，从上游下载各平台资产，并生成 `binaries/{name}.{platform}.tar.gz`。
7. 安装到本地 `~/.cbp` 进行验证：
   `cbp local {name}`
8. 运行配置中的测试，确认新版本可正常执行：
   `cbp build test --dir ~/.cbp {name}`
9. 测试通过后，上传二进制包到 GitHub Release：
   `cbp build upload binaries/{name}.{platform}.tar.gz`

## 帮助文本规范 (Help Text Style Guide)

### Rust 实现规范 (Implementation)

* **`about`**: 使用第三人称单数动词，简要描述 TSV 操作。
  (e.g., "Converts TSV to markdown table", "Deduplicates TSV rows").
* **`after_help`**: 使用 `include_str!("../../docs/help/<cmd>.md")` 引入外部文档。

### 文档内容规范 (Markdown Content)

所有子命令的帮助文档 (`docs/help/<cmd>.md`) 必须遵循以下统一风格：

1. **简述 (Description)**:
    * 标题后紧跟简洁的功能描述，可以比 .about(...) 更详细，但不能超过两行。

2. **分节结构**（按顺序）:
    * **Behavior** (可选): 命令的核心行为说明。
    * **Input**: 输入源说明（使用标准格式）。
    * **Output** (如适用): 输出说明。
    * **Header behavior** (如适用): Header 处理说明（使用标准格式）。
    * **Examples**: 使用示例。

3. **节标题格式**:
    * 使用 `Section Name:`（首字母大写，后跟冒号）。
    * **不要**使用 Markdown 的 `##`。
    * **不要**包含 `Usage:` 或 `Options:` 小节（由 `clap` 自动生成）。

4. **内容格式**:
    * **列表**: 使用 `* `（星号 + 1 个空格）引导无序列表。
        * 子项使用 `    * `（4 空格缩进 + 星号 + 1 个空格）。
    * **代码示例**: 使用缩进（4 空格）而非 ` ``` `。
    * 例外：多行命令示例（如包含 `\` 换行的命令）可使用 ` ``` ` 代码块。
    * **参数引用**: 使用反引号包裹，如 `` `--header` / `-H` ``。

5. **示例 (Examples)**:
    * 使用 `Examples:` 作为标题。
    * 采用编号列表 (`1. `, `2. `)， 秆尾不要句号或冒号。
    * 下一行命令示例（3 空格缩进, 用 `` 包裹单行命令）。

6. **文件末尾必须以一个空行结尾**（trailing newline）。
