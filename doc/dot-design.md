# CBP Dot 设计文档

参考项目: [YADM](https://github.com/yadm-dev/yadm) (Yet Another Dotfiles Manager) v3.5.0

---

## 核心设计

### 1. 模板系统

**设计原则：完全依赖模板，不使用 ALT 文件系统**

采用类似 chezmoi 的单文件模板方式，通过文件名前缀和模板变量处理差异：

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

#### 1.1 模板处理器

使用 `tera` crate（Jinja2 兼容）：

| 功能 | 说明 |
|------|------|
| 条件判断 | `{% if os == "linux" %}` |
| 变量输出 | `{{ hostname }}`, `{{ env.HOME }}` |
| 包含文件 | `{% include "common.conf" %}` |

#### 1.2 模板变量（运行时检测）

```rust
let context = Context::new();
context.insert("os", std::env::consts::OS);
context.insert("arch", std::env::consts::ARCH);
context.insert("hostname", get_hostname());
context.insert("user", std::env::var("USER").unwrap_or_default());
context.insert("distro", get_distro());  // 读取 /etc/os-release
context.insert("env", &env_vars);
```

---

### 2. 命令设计

**Git 完全外部化**：用户直接使用原生 Git，`cbp dot` 只负责文件处理。

| 命令 | 功能 | 说明 |
|------|------|------|
| `cbp dot <file>` | 应用单个文件 | 根据文件名前缀自动处理，复制/渲染到 `$HOME` |

#### 2.1 文件命名约定（参考 chezmoi）

CBP Dot 通过文件名前缀来决定如何处理文件。前缀可以组合使用（如 `private_dot_bashrc.tmpl`），处理顺序为：
1. 先处理属性前缀（`private_`, `executable_`）
2. 再处理路径前缀（`dot_`, `xdg_*`）
3. 最后处理后缀（`.tmpl`）

##### 属性前缀（影响文件权限）

| 前缀 | 作用 | 说明 |
|------|------|------|
| `private_` | 设置 0600 权限 | 敏感配置文件，如 SSH 密钥、密码文件 |
| `executable_` | 设置可执行权限（0755） | 脚本文件 |

**示例：**
```
private_ssh_config          → ~/.ssh/config (权限 0600)
executable_script.sh        → ~/script.sh (权限 0755)
```

##### 路径前缀（影响目标位置）

| 前缀/目录 | Linux/Mac 目标 | Windows 目标 | 说明 |
|-----------|----------------|--------------|------|
| `dot_` | `~/.{name}` | `~/.{name}` | 隐藏文件/目录，前缀 `dot_` 替换为 `.` |
| `dot_config/` | `~/.config/{path}` | `~/.config/{path}` | 配置目录（仅 $HOME） |
| `xdg_config/` | `~/.config/{path}` | `%APPDATA%/{path}` | XDG 配置目录（跨平台） |
| `xdg_data/` | `~/.local/share/{path}` | `%LOCALAPPDATA%/{path}` | XDG 数据目录（跨平台） |
| `xdg_cache/` | `~/.cache/{path}` | `%LOCALAPPDATA%/Temp/{path}` | XDG 缓存目录（跨平台） |

**详细示例：**

```
# dot_ 前缀 - 隐藏文件
dot_bashrc                  → ~/.bashrc
dot_vimrc                   → ~/.vimrc

# dot_config/ 目录 - 配置目录（仅 $HOME）
dot_config/nvim/init.vim    → ~/.config/nvim/init.vim
dot_config/myapp/config     → ~/.config/myapp/config

# xdg_config/ 目录 - 配置目录（跨平台）
xdg_config/myapp/settings.json
                            → ~/.config/myapp/settings.json (Linux/Mac)
                            → %APPDATA%/myapp/settings.json (Windows)

xdg_config/git/config       → ~/.config/git/config (Linux/Mac)
                            → %APPDATA%/git/config (Windows)

# xdg_data/ 目录 - 数据目录（跨平台）
xdg_data/myapp/data.db      → ~/.local/share/myapp/data.db (Linux/Mac)
                            → %LOCALAPPDATA%/myapp/data.db (Windows)

# xdg_cache/ 目录 - 缓存目录（跨平台）
xdg_cache/myapp/cache.tmp   → ~/.cache/myapp/cache.tmp (Linux/Mac)
                            → %LOCALAPPDATA%/Temp/myapp/cache.tmp (Windows)
```

##### 模板后缀

| 后缀 | 作用 | 说明 |
|------|------|------|
| `.tmpl` | 模板文件 | 使用 Tera 引擎渲染，可与其他前缀组合 |

**组合示例：**
```
dot_bashrc.tmpl                     → 渲染模板 → ~/.bashrc
private_dot_ssh_config.tmpl         → 渲染模板 → ~/.ssh/config (权限 0600)
xdg_config/myapp/config.tmpl        → 渲染模板 → ~/.config/myapp/config (Linux/Mac)
                                    → %APPDATA%/myapp/config (Windows)
```

##### 完整示例

```
源文件：
  private_executable_dot_myscript.tmpl

处理步骤：
  1. private_      → 设置权限为 0600
  2. executable_   → 设置权限为 0755（executable 优先级高于 private）
  3. dot_          → 目标路径 ~/.myscript
  4. .tmpl         → 使用模板渲染

最终结果：
  ~/.myscript (权限 0755，已渲染模板)
```

#### 2.2 使用示例

```bash
# 普通文件（自动转为隐藏文件）
cbp dot ~/.dotfiles/dot_bashrc
# 生成：~/.bashrc

# 模板文件（渲染后复制）
cbp dot ~/.dotfiles/dot_bashrc.tmpl
# 生成：~/.bashrc

# 可执行脚本
cbp dot ~/.dotfiles/executable_script.sh
# 生成：~/script.sh（带可执行权限）

# 批量处理（用 shell 循环）
for f in ~/.dotfiles/dot_*; do cbp dot "$f"; done
```

#### 2.3 Git 工作流（用户自行管理）

```bash
# 设置 Git 别名（可选）
alias dotf='git --git-dir=$HOME/.dotfiles.git --work-tree=$HOME'

# 日常使用
dotf add dot_bashrc
dotf commit -m "update"
dotf push

# 应用配置
cbp dot ~/.dotfiles/dot_bashrc.tmpl
```

---

### 3. 配置系统

**无配置文件**：运行时动态检测系统信息，无需维护配置。

| 变量 | 检测方式 | 示例 |
|------|----------|------|
| `os` | `std::env::consts::OS` | linux, macos, windows |
| `arch` | `std::env::consts::ARCH` | x86_64, aarch64 |
| `hostname` | `hostname` crate | myhost |
| `user` | `std::env::var("USER")` | username |
| `distro` | 读取 `/etc/os-release` | Ubuntu |
| `env.*` | `std::env::var()` | HOME, PATH |

---

### 4. 实现要点

#### 4.1 代码结构

```
src/
├── cmd_cbp/
│   ├── mod.rs
│   └── dot.rs             # dot 功能模块（单命令）
├── libs/
│   ├── mod.rs
│   └── dot.rs             # 系统信息检测 + 文件名解析
└── ...
```

#### 4.2 关键功能

1. **单文件处理**：`cbp dot <file>` 根据文件名前缀自动处理
2. **模板渲染**：使用 `tera` 渲染 `.tmpl` 文件
3. **权限设置**：根据 `private_`/`executable_` 前缀设置权限
4. **路径转换**：根据前缀自动转换目标路径

##### 特殊前缀列表

程序按以下顺序识别前缀（长前缀优先）：

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
  - 示例：`dot_config/nvim/init.vim` → `~/.config/nvim/init.vim`

- `xdg_config/` - 配置目录（跨平台）
  - Linux/Mac: `~/.config/{path}`
  - Windows: `%APPDATA%/{path}`
  - 示例：`xdg_config/myapp/config` → `~/.config/myapp/config`

- `xdg_data/` - 数据目录（跨平台）
  - Linux/Mac: `~/.local/share/{path}`
  - Windows: `%LOCALAPPDATA%/{path}`
  - 示例：`xdg_data/myapp/data.db`

- `xdg_cache/` - 缓存目录（跨平台）
  - Linux/Mac: `~/.cache/{path}`
  - Windows: `%LOCALAPPDATA%/Temp/{path}`
  - 示例：`xdg_cache/myapp/tmp`

**处理规则：**
1. 检查路径第一段是否为目录前缀（含 `/`）
2. 如果不是，从文件名中提取属性前缀和路径前缀（含 `_`）
3. 长前缀优先匹配（如 `private_dot_` 优先于 `dot_`）
4. 移除 `.tmpl` 后缀后确定最终目标路径

#### 4.3 技术栈

| 功能 | 实现 |
|------|------|
| 模板处理 | `tera`（Jinja2 兼容） |
| 路径处理 | `std::path::Path` + `dunce` |
| CLI 解析 | `clap` |

---

### 5. 与 YADM 的差异

| 特性 | YADM | CBP Dot |
|------|------|---------|
| Git 集成 | 内置 | 外部化 |
| 配置文件 | 有 | 无（运行时检测） |
| ALT 系统 | 支持 | 不支持（用模板替代） |
| 加密 | 支持 | 不支持 |
| 钩子 | 支持 | 不支持 |
| 命令数量 | 多个 | 单个 |

---

## 参考资源

- [YADM 官方文档](https://yadm.io/docs/overview)
- [chezmoi 文档](https://www.chezmoi.io/)
- [Tera 模板引擎](https://keats.github.io/tera/)
