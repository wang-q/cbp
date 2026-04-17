# CBP Dot 设计文档

参考项目: [YADM](https://github.com/yadm-dev/yadm) (Yet Another Dotfiles Manager) v3.5.0

---

## 1. 概述

CBP Dot 是一个极简的 dotfiles 管理工具，核心功能：

- **单命令设计**：`cbp dot <file>` 完成所有操作
- **模板支持**：使用 Tera 引擎，根据系统信息动态渲染
- **路径自动转换**：通过文件名前缀自动确定目标位置
- **Git 外部化**：用户自行管理 Git，工具只负责文件处理

---

## 2. 快速开始

**第一步：创建源目录**

```bash
# 创建你的 dotfiles 目录（示例：~/dotfiles）
mkdir ~/dotfiles
```

**第二步：从现有配置创建模板**

```bash
cbp dot ~/.bashrc --dir ~/dotfiles/
# 创建：~/dotfiles/dot_bashrc.tmpl

cbp dot ~/.ssh/config --dir ~/dotfiles/
# 创建：~/dotfiles/private_dot_ssh_config.tmpl
```

**第三步：应用配置**

```bash
# 预览（默认）
cbp dot ~/dotfiles/dot_bashrc.tmpl

# 确认无误后，实际应用
cbp dot -a ~/dotfiles/dot_bashrc.tmpl
```

**第四步：管理你的 dotfiles**

使用你喜欢的版本控制工具管理 `~/dotfiles` 目录。

---

## 3. 命令参考

### 3.1 命令格式

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

**示例：**

```bash
# 预览（默认）
cbp dot ~/dotfiles/dot_bashrc.tmpl
# 输出：[预览] 将会覆盖 ~/.bashrc

# 实际应用
cbp dot -a ~/dotfiles/dot_bashrc.tmpl
# 输出：已覆盖 ~/.bashrc

# 从现有配置创建模板
cbp dot ~/.bashrc --dir ~/dotfiles/
```

### 3.2 文件命名约定

CBP Dot 通过文件名前缀决定如何处理文件。前缀可以组合使用，处理顺序为：
1. 属性前缀（`private_`, `executable_`）
2. 路径前缀（`dot_`, `xdg_*`）
3. 后缀（`.tmpl`）

#### 属性前缀（影响权限）

- `private_` - 设置 0600 权限（敏感文件）
  - 示例：`private_dot_ssh_config` → `~/.ssh/config` (权限 0600)
  
- `executable_` - 设置 0755 权限（可执行脚本）
  - 示例：`executable_script.sh` → `~/script.sh` (权限 0755)

#### 路径前缀（单文件模式，用 `_` 分隔）

- `dot_` - 转为隐藏文件（`~/.` 开头）
  - 示例：`dot_bashrc` → `~/.bashrc`

#### 目录前缀（目录模式，用 `/` 分隔）

| 前缀 | Linux/Mac 目标 | Windows 目标 | 示例 |
|------|----------------|--------------|------|
| `dot_config/` | `~/.config/{path}` | `~/.config/{path}` | `dot_config/nvim/init.vim` |
| `xdg_config/` | `~/.config/{path}` | `%APPDATA%/{path}` | `xdg_config/myapp/config` |
| `xdg_data/` | `~/.local/share/{path}` | `%LOCALAPPDATA%/{path}` | `xdg_data/myapp/data.db` |
| `xdg_cache/` | `~/.cache/{path}` | `%LOCALAPPDATA%/Temp/{path}` | `xdg_cache/myapp/tmp` |

#### 模板后缀

- `.tmpl` - 模板文件，使用 Tera 引擎渲染
  - 示例：`dot_bashrc.tmpl` → 渲染后 → `~/.bashrc`

#### 完整示例

```
源文件：private_executable_dot_myscript.tmpl

处理步骤：
  1. private_      → 设置权限为 0600
  2. executable_   → 设置权限为 0755（executable 优先级高于 private）
  3. dot_          → 目标路径 ~/.myscript
  4. .tmpl         → 使用模板渲染

最终结果：~/.myscript (权限 0755，已渲染模板)
```

### 3.3 使用示例

**添加新文件：**
```bash
# 从现有配置创建模板
cbp dot ~/.gitconfig --dir ~/dotfiles/

# 预览并应用
cbp dot ~/dotfiles/dot_gitconfig.tmpl
cbp dot -a ~/dotfiles/dot_gitconfig.tmpl

# 提交
cd ~/dotfiles && git add . && git commit -m "Add gitconfig"
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

## 4. 模板系统

### 4.1 模板语法

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

### 4.2 可用变量

| 变量 | 说明 | 示例 |
|------|------|------|
| `os` | 操作系统 | linux, macos, windows |
| `arch` | 架构 | x86_64, aarch64 |
| `hostname` | 主机名 | myhost |
| `user` | 用户名 | username |
| `distro` | 发行版 | Ubuntu |
| `env.*` | 环境变量 | `env.HOME`, `env.PATH` |

---

## 5. 实现要点

### 5.1 代码结构

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

### 5.2 关键功能

1. **源文件解析**：从输入路径提取文件名，识别特殊前缀
2. **模板渲染**：使用 `tera` 渲染 `.tmpl` 文件
3. **权限设置**：根据 `private_`/`executable_` 前缀设置权限
4. **路径转换**：根据前缀自动转换目标路径
5. **安全默认**：默认预览模式，加 `-a` 才实际写入文件

### 5.3 技术栈

| 功能 | 实现 |
|------|------|
| 模板处理 | `tera` |
| 路径处理 | `std::path::Path` + `dunce` |
| CLI 解析 | `clap` |
| 系统信息 | `sysinfo` |

---

## 6. 对比与参考

### 6.1 与类似工具对比

| 特性 | CBP Dot | chezmoi | YADM |
|------|---------|---------|------|
| 定位 | 极简单命令工具 | 完整 dotfiles 管理器 | Git 包装器 |
| Git 集成 | 外部化 | 内置 | 内置 |
| 模板 | 支持 | 支持 | 支持 |
| 加密 | 不支持 | 支持 | 支持 |
| 配置文件 | 无 | 有 | 有 |
| 学习曲线 | 极低 | 中等 | 低 |

### 6.2 参考资源

- [YADM 官方文档](https://yadm.io/docs/overview)
- [chezmoi 文档](https://www.chezmoi.io/)
- [Tera 模板引擎](https://keats.github.io/tera/)
- [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
