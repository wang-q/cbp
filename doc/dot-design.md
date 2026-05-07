# CBP Dot 设计文档

参考项目: [YADM](https://github.com/yadm-dev/yadm) (Yet Another Dotfiles Manager) v3.5.0

---

## 1. 概述

CBP Dot 是一个极简的 dotfiles 管理工具。通过文件名前缀自动确定目标位置，使用模板引擎实现跨平台配置。

---

## 2. 快速开始

**第一步：创建源目录**

```bash
mkdir ~/dotfiles
```

**第二步：从现有配置创建模板**

```bash
cbp dot ~/.bashrc --dir ~/dotfiles/
# 创建：~/dotfiles/dot_bashrc.tmpl
```

> CBP Dot 会根据源文件路径自动推断合适的前缀：
> - 隐藏文件（如 `.bashrc`）→ 自动添加 `dot_` 前缀
> - 位于 `.config/` 或 `AppData/Roaming/` 下的文件 → 自动使用 `xdg_config/` 前缀
> - 位于 `.local/share/` 或 `AppData/Local/` 下的文件 → 自动使用 `xdg_data/` 前缀
> - 位于 `.cache/` 下的文件 → 自动使用 `xdg_cache/` 前缀
> - Unix 上有可执行权限的文件 → 自动添加 `executable_` 前缀
>
> 生成模板后你也可以手动调整文件名来满足特定需求。

**第三步：预览并应用**

```bash
# 预览（默认）
cbp dot ~/dotfiles/dot_bashrc.tmpl

# 确认无误后，实际应用
cbp dot -a ~/dotfiles/dot_bashrc.tmpl
```

**第四步：管理你的 dotfiles**

使用你喜欢的版本控制工具管理 `~/dotfiles` 目录。

---

## 3. 核心概念：文件命名约定

CBP Dot 通过文件名前缀决定如何处理文件。

### 完整示例

```
源文件：private_executable_dot_myscript.tmpl

处理步骤：
  1. private_      → 设置权限为 0600
  2. executable_   → 设置权限为 0755（executable 优先级高于 private）
  3. dot_          → 目标路径 ~/.myscript
  4. .tmpl         → 使用模板渲染

最终结果：~/.myscript (权限 0755，已渲染模板)
```

### 前缀类型

前缀可以组合使用，处理顺序为：属性前缀 → 路径前缀 → 后缀

**组合顺序约束：**

> 属性前缀（`private_`/`executable_`）必须出现在目录前缀之前，否则解析会出错：
> - ✅ `executable_dot_script.sh` — 正确：属性前缀 → 路径前缀
> - ✅ `private_xdg_config/myapp/config.tmpl` — 正确
> - ❌ `dot_executable_bashrc` — 错误：`dot_` 会先被作为路径前缀消费

**`dot_config/` 与 `xdg_config/` 的选择指南：**

> | 前缀 | Linux/Mac 目标 | Windows 目标 | 适用场景 |
> |------|---------------|-------------|---------|
> | `dot_config/` | `~/.config/` | `~/.config/` | 不关心 Windows 平台惯例，始终用 `~/.config/` |
> | `xdg_config/` | `~/.config/` | `%APPDATA%/` | 遵循各平台原生路径标准 |
>
> 同理，`xdg_data/` 和 `xdg_cache/` 也会根据平台自动适配。如果你的配置文件需要在多平台间共享，推荐使用 `xdg_` 系列前缀。

**属性前缀（影响权限）：**

- `private_` - 设置 0600 权限（敏感文件）
  - 示例：`private_dot_ssh_config` → `~/.ssh/config` (权限 0600)
  
- `executable_` - 设置 0755 权限（可执行脚本）
  - 示例：`executable_script.sh` → `~/script.sh` (权限 0755)

**路径前缀（单文件模式，用 `_` 分隔）：**

- `dot_` - 转为隐藏文件（`~/.` 开头）
  - 示例：`dot_bashrc` → `~/.bashrc`

**目录前缀（目录模式，用 `/` 分隔）：**

- `dot_config/` - 配置目录（仅 $HOME）
  - Linux/Mac: `~/.config/{path}`
  - Windows: `~/.config/{path}`
  - 示例：`dot_config/nvim/init.vim`

- `xdg_config/` - 配置目录（跨平台）
  - Linux/Mac: `~/.config/{path}`
  - Windows: `%APPDATA%/{path}`
  - 示例：`xdg_config/myapp/config`

- `xdg_data/` - 数据目录（跨平台）
  - Linux/Mac: `~/.local/share/{path}`
  - Windows: `%LOCALAPPDATA%/{path}`
  - 示例：`xdg_data/myapp/data.db`

- `xdg_cache/` - 缓存目录（跨平台）
  - Linux/Mac: `~/.cache/{path}`
  - Windows: `%LOCALAPPDATA%/Temp/{path}`
  - 示例：`xdg_cache/myapp/tmp`

**模板后缀：**

- `.tmpl` - 模板文件，使用 Tera 引擎渲染
  - 示例：`dot_bashrc.tmpl` → 渲染后 → `~/.bashrc`

---

## 4. 命令参考

### 4.1 命令格式

```bash
# 应用模板（默认模式）
cbp dot [OPTIONS] <template_file>

# 从现有配置创建模板
cbp dot [OPTIONS] <source_file> --dir <template_dir>

# 应用压缩包配置
cbp dot [OPTIONS] <archive.tar.gz>

# 导出配置为压缩包
cbp dot [OPTIONS] <source> --tar <archive.tar.gz>
```

**选项：**

| 选项 | 说明 |
|------|------|
| `-a, --apply` | 实际应用更改（默认仅预览，应用模式） |
| `-v, --verbose` | 显示详细操作信息 |
| `-d, --dir <dir>` | 指定模板存储目录（创建模式） |
| `--tar <file>` | 导出为 tar.gz 压缩包 |

**注意：**
- `--apply` 和 `--dir` 是互斥选项，不能同时使用
- 压缩包内使用最终路径（如 `.config/nvim/init.vim`），不需要前缀转换
- `--tar` 和 `--dir` 是互斥选项，不能同时使用

### 4.2 使用示例

**添加新文件：**
```bash
# 从现有配置创建模板
cbp dot ~/.gitconfig --dir ~/dotfiles/

# 预览并应用
cbp dot ~/dotfiles/dot_gitconfig.tmpl
cbp dot -a ~/dotfiles/dot_gitconfig.tmpl
```

**编辑文件：**
```bash
vim ~/dotfiles/dot_bashrc.tmpl
cbp dot -a ~/dotfiles/dot_bashrc.tmpl
```

**批量应用：**
```bash
for f in ~/dotfiles/dot_* ~/dotfiles/dot_config/**/*; do 
    [ -f "$f" ] && cbp dot -a "$f"
done
```

**在新机器上部署：**
```bash
git clone https://github.com/username/dotfiles.git ~/dotfiles
for f in ~/dotfiles/dot_* ~/dotfiles/dot_config/**/*; do 
    [ -f "$f" ] && cbp dot -a "$f"
done
```

**使用压缩包：**
```bash
# 导出单个软件配置为压缩包
cbp dot ~/.config/nvim --tar nvim.tar.gz
# 压缩包内：.config/nvim/init.vim（最终路径，无前缀）

# 应用压缩包配置
cbp dot -a nvim.tar.gz
# 直接解压到 ~/.config/nvim/init.vim
```

---

## 5. 模板系统

### 5.1 模板语法

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

### 5.2 可用变量

| 变量 | 说明 | 示例 |
|------|------|------|
| `os` | 操作系统 | linux, macos, windows |
| `arch` | 架构 | x86_64, aarch64 |
| `hostname` | 主机名 | myhost |
| `user` | 用户名 | username |
| `distro` | 发行版 | Ubuntu |
| `env.*` | 环境变量 | `env.HOME`, `env.PATH` |

---

## 6. 常见问题

**Q: 为什么默认是预览模式？**  
A: 为了防止意外覆盖重要配置文件。先预览确认无误后，再加 `-a` 应用。

**Q: `--apply` 和 `--dir` 有什么区别？**  
A: `--apply` 用于应用模板到目标位置，`--dir` 用于从现有配置创建模板。两者是互斥的。

**Q: Windows 和 Linux 的路径怎么处理？**  
A: 使用 `xdg_` 前缀（如 `xdg_config/`）会自动根据平台转换路径。Linux 用 `~/.config/`，Windows 用 `%APPDATA%/`。

**Q: 可以不用模板吗？**  
A: 可以。普通文件（不含 `.tmpl` 后缀）会直接复制，不会经过模板渲染。例如：

```bash
# 这个源文件没有 .tmpl 后缀，内容直接复制到目标
cbp dot -a ~/dotfiles/dot_bashrc
# 等价于：cp ~/dotfiles/dot_bashrc ~/.bashrc
```

**Q: 前缀中的权限设置在 Windows 上生效吗？**  
A: 不生效。`private_`（0600）和 `executable_`（0755）权限仅在 Unix/Linux/Mac 上设置，Windows 上会静默忽略。你仍然可以使用这些前缀（方便跨平台共享同一套模板），但在 Windows 上它们不影响最终文件的权限。

**Q: 模板渲染失败会怎样？**  
A: 命令会以错误码退出并打印 Tera 的错误信息（包括行号）。例如模板中有语法错误或引用了不存在的变量时，不会写入任何文件。

**Q: 目标路径的父目录不存在怎么办？**  
A: 会自动创建（`mkdir -p` 语义），无需手动创建中间目录。

**Q: 压缩包解压时目标文件已存在会怎样？**  
A: 会直接覆盖，不会提示。建议先用预览模式确认影响范围：

```bash
# 先预览
cbp dot nvim.tar.gz
# 确认无误后再应用
cbp dot -a nvim.tar.gz
```

**Q: 压缩包内的路径格式是什么？**  
A: 压缩包内使用相对于 `$HOME` 的最终路径（不是前缀格式）。例如导出 `~/.config/nvim/` 生成的压缩包内文件路径是 `.config/nvim/init.vim`，不含 `xdg_config/` 或 `dot_` 前缀。

**Q: 一次可以处理多个源文件吗？**  
A: 可以。apply 模式下可以指定多个源文件，会逐个处理：

```bash
cbp dot -a ~/dotfiles/dot_bashrc.tmpl ~/dotfiles/dot_gitconfig.tmpl
```

但 `--dir`（创建模式）只支持单个源文件。

**Q: 批量操作时 `**` 递归匹配不生效？**  
A: 某些 shell 需要先开启 globstar 选项：

```bash
# bash
shopt -s globstar

# zsh
setopt globstar
```

---

## 7. 附录

### 7.1 实现要点

**代码结构：**
```
src/
├── cmd_cbp/
│   ├── mod.rs
│   └── dot.rs             # dot 功能模块
├── libs/
│   ├── mod.rs
│   ├── dirs.rs            # 目录管理
│   ├── utils.rs           # 工具函数
│   └── dot.rs             # 系统信息检测 + 文件名解析
└── ...
```

**关键功能：**
1. **源文件解析**：从输入路径提取文件名，识别特殊前缀
2. **模板渲染**：使用 `tera` 渲染 `.tmpl` 文件
3. **权限设置**：根据 `private_`/`executable_` 前缀设置权限
4. **路径转换**：根据前缀自动转换目标路径
5. **安全默认**：默认预览模式，加 `-a` 才实际写入文件

**技术栈：**

| 功能 | 实现 |
|------|------|
| 模板处理 | `tera` |
| 路径处理 | `std::path::Path` + `dunce` |
| CLI 解析 | `clap` |
| 系统信息 | `sysinfo` |
| 压缩包 | `tar` + `flate2` |

### 7.2 可复用的现有代码

CBP 项目中已有以下代码可直接复用：

**目录管理 (`libs/dirs.rs`)：**
- `CbpDirs` - 目录结构管理
- `to_absolute_path()` - 相对路径转绝对路径

**压缩包操作 (`libs/utils.rs`)：**
- `list_archive_files()` - 列出 tar.gz 内文件列表
- `read_file_from_archive()` - 读取压缩包内指定文件内容
- `find_files()` - 递归查找目录内所有文件
- `match_files()` - 使用 glob 模式匹配文件

**文件操作 (`libs/utils.rs`)：**
- `copy_dir_all()` - 递归复制目录
- `move_file_or_dir()` - 移动文件/目录（支持跨设备）
- `writer()` - 创建缓冲写入器（支持 stdout 或文件）

**系统检测 (`libs/utils.rs`)：**
- `get_os_type()` - 获取操作系统类型（linux/macos/windows）
- `is_system_file()` - 检测系统文件（.DS_Store 等）
- `is_cbp_file()` - 检测 CBP 管理文件

**tar 命令 (`cmd_cbp/tar.rs`)：**
- 创建 tar.gz 压缩包
- 保留符号链接
- 过滤系统文件
- 支持清理文档目录

**HTTP 代理 (`libs/utils.rs`)：**
- `create_http_agent()` - 创建带代理支持的 HTTP 客户端
- 支持 ALL_PROXY, HTTP_PROXY 等环境变量

**CLI 模式 (`cmd_cbp/` 各模块)：**
- `make_subcommand()` - 创建子命令参数
- `execute()` - 执行命令逻辑

### 7.3 与类似工具对比

| 特性 | CBP Dot | chezmoi | YADM |
|------|---------|---------|------|
| 定位 | 极简单命令工具 | 完整 dotfiles 管理器 | Git 包装器 |
| Git 集成 | 外部化 | 内置 | 内置 |
| 模板 | 支持 | 支持 | 支持 |
| 加密 | 不支持 | 支持 | 支持 |
| 配置文件 | 无 | 有 | 有 |
| 学习曲线 | 极低 | 中等 | 低 |

### 7.4 参考资源

- [YADM 官方文档](https://yadm.io/docs/overview)
- [chezmoi 文档](https://www.chezmoi.io/)
- [Tera 模板引擎](https://keats.github.io/tera/)
- [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
