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

**属性前缀（影响权限）：**

- `private_` - 设置 0600 权限（敏感文件）
  - 示例：`private_dot_ssh_config` → `~/.ssh/config` (权限 0600)
  
- `executable_` - 设置 0755 权限（可执行脚本）
  - 示例：`executable_script.sh` → `~/script.sh` (权限 0755)

**路径前缀（单文件模式，用 `_` 分隔）：**

- `dot_` - 转为隐藏文件（`~/.` 开头）
  - 示例：`dot_bashrc` → `~/.bashrc`

**目录前缀（目录模式，用 `/` 分隔）：**

| 前缀 | Linux/Mac 目标 | Windows 目标 | 示例 |
|------|----------------|--------------|------|
| `dot_config/` | `~/.config/{path}` | `~/.config/{path}` | `dot_config/nvim/init.vim` |
| `xdg_config/` | `~/.config/{path}` | `%APPDATA%/{path}` | `xdg_config/myapp/config` |
| `xdg_data/` | `~/.local/share/{path}` | `%LOCALAPPDATA%/{path}` | `xdg_data/myapp/data.db` |
| `xdg_cache/` | `~/.cache/{path}` | `%LOCALAPPDATA%/Temp/{path}` | `xdg_cache/myapp/tmp` |

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
```

**选项：**

| 选项 | 说明 |
|------|------|
| `-a, --apply` | 实际应用更改（默认仅预览，应用模式） |
| `-v, --verbose` | 显示详细操作信息 |
| `-d, --dir <dir>` | 指定模板存储目录（创建模式） |

**注意：** `--apply` 和 `--dir` 是互斥选项，不能同时使用。

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
A: 可以。普通文件（不含 `.tmpl` 后缀）会直接复制，不会经过模板渲染。

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

### 7.2 与类似工具对比

| 特性 | CBP Dot | chezmoi | YADM |
|------|---------|---------|------|
| 定位 | 极简单命令工具 | 完整 dotfiles 管理器 | Git 包装器 |
| Git 集成 | 外部化 | 内置 | 内置 |
| 模板 | 支持 | 支持 | 支持 |
| 加密 | 不支持 | 支持 | 支持 |
| 配置文件 | 无 | 有 | 有 |
| 学习曲线 | 极低 | 中等 | 低 |

### 7.3 参考资源

- [YADM 官方文档](https://yadm.io/docs/overview)
- [chezmoi 文档](https://www.chezmoi.io/)
- [Tera 模板引擎](https://keats.github.io/tera/)
- [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
