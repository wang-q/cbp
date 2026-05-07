# HomeDir

---

## 概述

CBP 中其他命令管理 `~/.cbp/` 下的可执行文件，而 `dot` 和 `snap` 管理 `$HOME` 下的配置和文档。

| 命令 | 职责 | 粒度 |
|------|------|------|
| `cbp dot` | 单文件模板管线：前缀解析 → 模板渲染 → 权限设置 | 单文件 |
| `cbp snap` | 批量快照/还原：以 `$HOME` 为基准打包/解压 | 批量文件/目录 |

---

# Part 1：cbp dot

`dot` 已基本实现。起初它同时承担单文件模板和批量压缩包两个职责（通过 `--tar` 切换），但压缩包操作全程不碰前缀解析、模板渲染和权限设置——两条管线的交接点为零，现已拆分 `snap`。

## 快速开始

```bash
# 1. 创建源目录
mkdir ~/dotfiles

# 2. 从现有配置创建模板
cbp dot ~/.bashrc --dir ~/dotfiles/
# → ~/dotfiles/dot_bashrc.tmpl

# 3. 预览（默认）
cbp dot ~/dotfiles/dot_bashrc.tmpl

# 4. 确认后应用
cbp dot -a ~/dotfiles/dot_bashrc.tmpl

# 5. 用你喜欢的版本控制管理 ~/dotfiles
```

> `--dir` 模式下会自动推断前缀：
> - 隐藏文件（`.bashrc`）→ `dot_` 前缀
> - 位于 `.config/` 或 `AppData/Roaming/` 下 → `xdg_config/` 前缀
> - 位于 `.local/share/` 或 `AppData/Local/` 下 → `xdg_data/` 前缀
> - 位于 `.cache/` 下 → `xdg_cache/` 前缀
> - Unix 上有可执行权限 → `executable_` 前缀
>
> 生成后也可手动调整文件名。

## 文件命名约定

通过文件名前缀决定如何处理，处理顺序：**属性前缀 → 路径前缀 → 后缀**。

**完整示例：**

```
源文件：private_executable_dot_myscript.tmpl

处理步骤：
  1. private_      → 0600 权限
  2. executable_   → 0755 权限（优先级高于 private）
  3. dot_          → 目标路径 ~/.myscript
  4. .tmpl         → 模板渲染

最终结果：~/.myscript (权限 0755，已渲染模板)
```

**组合顺序约束：**

> 属性前缀必须出现在目录前缀之前：
> - ✅ `executable_dot_script.sh`
> - ✅ `private_xdg_config/myapp/config.tmpl`
> - ❌ `dot_executable_bashrc` — `dot_` 先被作为路径前缀消费

### 属性前缀（影响权限）

- `private_` → 0600（敏感文件），示例：`private_dot_ssh_config` → `~/.ssh/config`
- `executable_` → 0755（可执行脚本），示例：`executable_script.sh` → `~/script.sh`

> `private_`/`executable_` 仅在 Unix 上生效，Windows 静默忽略。

### 路径前缀（单文件模式，`_` 分隔）

- `dot_` → `~/.name`，示例：`dot_bashrc` → `~/.bashrc`

### 目录前缀（目录模式，`/` 分隔）

| 前缀 | Linux/Mac | Windows |
|------|-----------|---------|
| `dot_config/` | `~/.config/{path}` | `~/.config/{path}` |
| `xdg_config/` | `~/.config/{path}` | `%APPDATA%/{path}` |
| `xdg_data/` | `~/.local/share/{path}` | `%LOCALAPPDATA%/{path}` |
| `xdg_cache/` | `~/.cache/{path}` | `%LOCALAPPDATA%/Temp/{path}` |

**`dot_config/` 与 `xdg_config/` 的选择：**

> | 前缀 | 适用场景 |
> |------|---------|
> | `dot_config/` | 不关心 Windows 平台惯例 |
> | `xdg_config/` | 遵循各平台原生路径标准 |
>
> 跨平台共享推荐 `xdg_` 系列。

### 模板后缀

- `.tmpl` → Tera 引擎渲染（Jinja2 兼容语法）

> 无 `.tmpl` 后缀的文件跳过渲染，直接复制到目标位置。

## 命令格式

```bash
# 应用模板（默认模式）
cbp dot [OPTIONS] <template_file...>

# 从现有配置创建模板
cbp dot [OPTIONS] <source_file> --dir <template_dir>
```

| 选项 | 说明 |
|------|------|
| `-a, --apply` | 实际应用（默认仅预览） |
| `-v, --verbose` | 详细输出 |
| `-d, --dir <dir>` | 指定模板存储目录 |

> `--apply` 和 `--dir` 互斥。apply 模式支持多个模板文件，`--dir` 模式只接受单个源文件。

## 使用示例

```bash
# 添加新文件
cbp dot ~/.gitconfig --dir ~/dotfiles/
cbp dot -a ~/dotfiles/dot_gitconfig.tmpl

# 编辑已有模板
vim ~/dotfiles/dot_bashrc.tmpl
cbp dot -a ~/dotfiles/dot_bashrc.tmpl

# 批量应用
for f in ~/dotfiles/dot_* ~/dotfiles/dot_config/**/*; do
    [ -f "$f" ] && cbp dot -a "$f"
done

# 在新机器上部署
git clone https://github.com/username/dotfiles.git ~/dotfiles
for f in ~/dotfiles/dot_* ~/dotfiles/dot_config/**/*; do
    [ -f "$f" ] && cbp dot -a "$f"
done
```

> 批量操作中 `**` 递归匹配需先开启 globstar：`shopt -s globstar`（bash）或 `setopt globstar`（zsh）。

## 模板系统

使用 Tera 引擎（Jinja2 兼容）：

```bash
# dot_bashrc.tmpl
{% if os == "linux" %}
alias ls='ls --color=auto'
{% elif os == "macos" %}
alias ls='ls -G'
{% endif %}

{% if hostname == "work-laptop" %}
export HTTP_PROXY=http://proxy.company.com:8080
{% endif %}
```

**可用变量：**

| 变量 | 说明 | 示例 |
|------|------|------|
| `os` | 操作系统 | linux, macos, windows |
| `arch` | 架构 | x86_64, aarch64 |
| `hostname` | 主机名 | myhost |
| `user` | 用户名 | username |
| `distro` | 发行版 | Ubuntu |
| `env.*` | 环境变量 | `env.HOME`, `env.PATH` |

> 渲染失败时以错误码退出，打印 Tera 错误信息（含行号），不写入任何文件。

---

# Part 2：cbp snap

`snap` 还在设计中。它从 `dot` 拆分而来，专注于批量文件的打包/还原，不涉及前缀解析、模板渲染和权限设置。

## 设计思路

**为什么以 $HOME 为基准？**

`snap` 存的不是"某个目录下的文件"，而是"这些文件在 HOME 里的位置"。压缩包内以源路径本身为根（不含上层父目录），完整路径由 gzip 注释记录。`load` 时根据注释将内容还原到目标目录下的对应位置。这样压缩包简洁内聚，路径语义完整保留。

**gzip 注释的作用**

注释不是可选的辅助信息，而是路径还原的关键——它记录了每个源的完整路径（以 `~` 表示相对于 HOME）。即使源路径指向 HOME 之外（如 `~/../../etc/fstab`），注释中仍以 HOME 为基准表达。注释存的就是源路径列表，不附加版本号、主机名等元数据。

### dot 与 snap 对比

| | `cbp dot` | `cbp snap` |
|------|-----------|------------|
| 粒度 | 单文件 | 批量文件/目录 |
| 处理 | 前缀解析 + 模板渲染 + 权限 | 路径打包 + 解压 |
| 后缀 | `.tmpl` | `.snap.tar.gz` |
| 场景 | 日常编辑、版本控制 | 备份、迁移、分享 |

## 快速开始

```bash
# 备份 nvim 配置
cbp snap save ~/.config/nvim
# → nvim.snap.tar.gz

# 还原到当前 HOME
cbp snap load nvim.snap.tar.gz

# 还原到测试目录
cbp snap load nvim.snap.tar.gz -t /tmp/test-home
```

## 核心概念

**路径约定：**

`save` 时压缩包的根目录就是源路径本身，不含上层的父目录，源内部的目录结构完整保留：

```
输入：~/.config/nvim/
压缩包内：
  nvim/
    init.vim

输入：~/.bashrc
压缩包内：
  .bashrc
```

完整的目标路径由 gzip 注释提供——注释记录了源的完整路径（`~/.config/nvim`），`load` 时据此将压缩包内的 `nvim/` 还原到目标目录下的 `.config/nvim/`。

> 源路径不限于 `$HOME` 内。指向 HOME 外的路径（如 `/etc/fstab`）在注释中记为 `~/../../etc/fstab`，`load` 时按相同规则还原。

**快照签名（gzip comment）：**

嵌入源路径列表，统一以 HOME 为基准（`~` 记法）：

`~/.config/nvim` `~/../../etc/fstab`

预览时展示此信息。

**文件扩展名：** 推荐 `.snap.tar.gz`。`load` 不做后缀校验。

## 命令格式

```bash
cbp snap save <paths...> [-o <output>] [-v]
cbp snap load <archive>  [-t <target>] [-v]
```

### save

| 选项 | 说明 |
|------|------|
| `<paths...>` | 要保存的文件或目录 |
| `-o, --output <file>` | 输出路径，默认 `<name>.snap.tar.gz` |
| `-v, --verbose` | 详细输出 |

### load

| 选项 | 说明 |
|------|------|
| `<archive>` | 要还原的快照压缩包 |
| `-t, --target <dir>` | 目标根目录，默认 `$HOME` |
| `-v, --verbose` | 详细输出 |

> 解压时目标路径已有文件直接覆盖，不会预览或提示确认。

## 使用示例

```bash
# 备份单个目录
cbp snap save ~/.config/nvim

# 备份多个路径
cbp snap save ~/.config/nvim ~/.bashrc ~/.gitconfig -o dev-env.snap.tar.gz

# 还原到当前 HOME
cbp snap load dev-env.snap.tar.gz

# 还原到沙箱目录
cbp snap load nvim.snap.tar.gz -t /tmp/test
```

---

# 附录

## 代码结构

```
src/
├── cmd_cbp/
│   ├── mod.rs
│   ├── dot.rs             # dot — 单文件模板管线
│   └── snap.rs            # snap — 批量快照/还原
├── libs/
│   ├── mod.rs
│   ├── dirs.rs            # 目录管理
│   ├── utils.rs           # 工具函数、系统检测
│   └── dot.rs             # 系统信息 + 文件名解析 + 模板渲染
└── ...
```

## 技术栈

| 功能 | 实现 |
|------|------|
| CLI 解析 | `clap` |
| 模板引擎 | `tera` |
| 压缩/解压 | `flate2` + `tar` |
| gzip 注释 | `flate2::GzBuilder::comment()` / `GzDecoder::header().comment()` |
| 路径处理 | `std::path::Path` + `dunce` |
| 目录遍历 | `walkdir` |

## 与类似工具对比

| 特性 | HomeDir | chezmoi | YADM |
|------|---------|---------|------|
| 定位 | 极简单命令工具 | 完整管理器 | Git 包装器 |
| Git 集成 | 外部化 | 内置 | 内置 |
| 模板 | 支持 | 支持 | 支持 |
| 批量快照 | snap 命令 | 无 | 无 |
| 加密 | 不支持 | 支持 | 支持 |
| 配置文件 | 无 | 有 | 有 |
| 学习曲线 | 极低 | 中等 | 低 |

## 参考资源

- [YADM](https://github.com/yadm-dev/yadm) — 参考项目 (v3.5.0)
- [chezmoi 文档](https://www.chezmoi.io/)
- [Tera 模板引擎](https://keats.github.io/tera/)
- [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
