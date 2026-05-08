# HomeDir

---

## Overview

While other CBP commands manage executables under `~/.cbp/`, `dot` and `snap` manage configurations and documents under `$HOME`.

| Command | Responsibility | Granularity |
|---------|---------------|-------------|
| `cbp dot` | Single-file template pipeline: prefix parsing -> template rendering -> permission setting | Single file |
| `cbp snap` | Batch snapshot/restore: save/load/list/delta relative to `$HOME` | Batch files/directories |

---

# Part 1: cbp dot

`dot` is essentially implemented. It originally handled both single-file templates and batch archives (switched via `--tar`), but archive operations never touched prefix parsing, template rendering, or permission setting — the two pipelines had zero intersection points. The archive functionality has now been split into `snap`.

## Quick Start

```bash
# 1. Create source directory
mkdir ~/dotfiles

# 2. Create template from existing config
cbp dot ~/.bashrc --dir ~/dotfiles/
# -> ~/dotfiles/dot_bashrc.tmpl

# 3. Preview (default)
cbp dot ~/dotfiles/dot_bashrc.tmpl

# 4. Apply after confirmation
cbp dot -a ~/dotfiles/dot_bashrc.tmpl

# 5. Manage ~/dotfiles with your preferred version control
```

> `--dir` mode automatically infers prefixes:
> - Hidden files (`.bashrc`) -> `dot_` prefix
> - Located in `.config/` or `AppData/Roaming/` -> `xdg_config/` prefix
> - Located in `.local/share/` or `AppData/Local/` -> `xdg_data/` prefix
> - Located in `.cache/` -> `xdg_cache/` prefix
> - Executable permission on Unix -> `executable_` prefix
>
> You can manually adjust filenames after generation.

## Filename Conventions

Processing is determined by filename prefixes, in order: **attribute prefix -> path prefix -> suffix**.

**Complete Example:**

```
Source: private_executable_dot_myscript.tmpl

Processing steps:
  1. private_      -> 0600 permissions
  2. executable_   -> 0755 permissions (takes precedence over private)
  3. dot_          -> target path ~/.myscript
  4. .tmpl         -> template rendering

Final result: ~/.myscript (permissions 0755, rendered template)
```

**Combination Order Constraints:**

> Attribute prefixes must appear before directory prefixes:
> - ✅ `executable_dot_script.sh`
> - ✅ `private_xdg_config/myapp/config.tmpl`
> - ❌ `dot_executable_bashrc` — `dot_` is consumed as path prefix first

### Attribute Prefixes (affect permissions)

- `private_` -> 0600 (sensitive files), example: `private_dot_ssh_config` -> `~/.ssh/config`
- `executable_` -> 0755 (executable scripts), example: `executable_script.sh` -> `~/script.sh`

> `private_`/`executable_` only take effect on Unix; silently ignored on Windows.

### Path Prefixes (single-file mode, `_` separated)

- `dot_` -> `~/.name`, example: `dot_bashrc` -> `~/.bashrc`

### Directory Prefixes (directory mode, `/` separated)

`dot_config/`  ->  Linux/Mac: `~/.config/{path}`  |  Windows: `~/.config/{path}`
`xdg_config/`  ->  Linux/Mac: `~/.config/{path}`  |  Windows: `%APPDATA%/{path}`
`xdg_data/`    ->  Linux/Mac: `~/.local/share/{path}`  |  Windows: `%LOCALAPPDATA%/{path}`
`xdg_cache/`   ->  Linux/Mac: `~/.cache/{path}`  |  Windows: `%LOCALAPPDATA%/Temp/{path}`

`dot_config/` does not follow Windows platform conventions; `xdg_config/` follows native platform standards. The `xdg_` series is recommended for cross-platform sharing.

### Template Suffix

- `.tmpl` -> Tera engine rendering (Jinja2 compatible syntax)

> Files without `.tmpl` suffix skip rendering and are copied directly to the target location.

## Command Format

```bash
# Apply template (default mode)
cbp dot [OPTIONS] <template_file...>

# Create template from existing config
cbp dot [OPTIONS] <source_file> --dir <template_dir>
```

`-a, --apply`  Actually apply (default is preview only)
`-v, --verbose`  Verbose output
`-d, --dir <dir>`  Specify template storage directory

> `--apply` and `--dir` are mutually exclusive. Apply mode supports multiple template files; `--dir` mode only accepts a single source file.

## Usage Examples

```bash
# Add new file
cbp dot ~/.gitconfig --dir ~/dotfiles/
cbp dot -a ~/dotfiles/dot_gitconfig.tmpl

# Edit existing template
vim ~/dotfiles/dot_bashrc.tmpl
cbp dot -a ~/dotfiles/dot_bashrc.tmpl

# Batch apply
for f in ~/dotfiles/dot_* ~/dotfiles/dot_config/**/*; do
    [ -f "$f" ] && cbp dot -a "$f"
done

# Deploy on new machine
git clone https://github.com/username/dotfiles.git ~/dotfiles
for f in ~/dotfiles/dot_* ~/dotfiles/dot_config/**/*; do
    [ -f "$f" ] && cbp dot -a "$f"
done
```

> Batch operations with `**` recursive matching require globstar: `shopt -s globstar` (bash) or `setopt globstar` (zsh).

## Template System

Uses Tera engine (Jinja2 compatible):

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

**Available Variables:**

`os`  Operating system (linux, macos, windows)
`arch`  Architecture (x86_64, aarch64)
`hostname`  Hostname
`user`  Username
`distro`  Distribution (Ubuntu)
`env.*`  Environment variables (env.HOME, env.PATH)

> Rendering failures exit with error code, printing Tera error information (including line numbers), without writing any files.

---

# Part 2: cbp snap

`snap` manages batch file snapshots for backup, migration, and sharing. It does not handle prefix parsing or template rendering — use `dot` for those.

### dot vs snap Comparison

`dot` manages single files, doing prefix parsing, template rendering, and permission setting, using `.tmpl` suffix, suitable for daily editing and version control. `snap` manages batch files, only doing path packing and unpacking, using `.snap.tar.gz` suffix, suitable for backup, migration, and sharing.

## Quick Start

```bash
# Backup nvim config
cbp snap save ~/.config/nvim
# -> nvim.snap.tar.gz

# Restore to current HOME
cbp snap load nvim.snap.tar.gz

# Restore to test directory
cbp snap load nvim.snap.tar.gz -t /tmp/test-home

# View snapshot contents
cbp snap list nvim.snap.tar.gz

# Compare snapshot with current disk differences
cbp snap delta nvim.snap.tar.gz

# Pack modified files into delta snapshot
cbp snap delta nvim.snap.tar.gz -p

# Multi-path backup
cbp snap save ~/.config/nvim ~/.bashrc ~/.gitconfig -o dev-env.snap.tar.gz
```

> `*.snap.tar.gz` files are also recommended for version control, facilitating tracking configuration change history and cross-machine synchronization.

## Core Concepts

snap stores not "files in a directory" but "where these files are in HOME" — the archive uses source paths as root, target paths are recorded by gzip comment, and `load` restores to corresponding locations based on the comment.

**Path Conventions:**

During `save`, the archive root is the source path itself, without parent directories, while internal directory structure is fully preserved:

```
Input: ~/.config/nvim/
Inside archive:
  nvim/
    init.vim

Input: ~/.bashrc
Inside archive:
  .bashrc
```

The complete target path is provided by the gzip comment — the comment records the source's complete path (`~/.config/nvim`), and `load` uses this to restore `nvim/` inside the archive to `.config/nvim/` under the target directory.

> Source paths are not limited to `$HOME`. Paths pointing outside HOME (e.g., `/etc/fstab`) are recorded in comments as `~/../../etc/fstab`, and `load` restores using the same rule.

**Snapshot Signature (gzip comment):**

Embeds source path list, unified with HOME as base (`~` notation), without additional version numbers, hostnames, or other metadata:

`~/.config/nvim` `~/../../etc/fstab`

**File Extension:** Recommended `.snap.tar.gz`. `load` does not validate suffixes.

> snap is pure file operations, does not preserve file permissions, ACLs, or extended attributes. For permission management, use `cbp dot`'s `private_`/`executable_` prefixes.

## Command Format

```bash
cbp snap save <paths...> [-o <output>] [-v]
cbp snap load <archive>  [-t <target>] [-v]
cbp snap list <archive>  [-v]
cbp snap delta <archive> [-p]
```

### save

`<paths...>`  Files or directories to save
`-o, --output <file>`  Output path. Defaults to `<basename>.snap.tar.gz` for single path, required for multiple paths
`-v, --verbose`  Verbose output

### load

`<archive>`  Snapshot archive to restore
`-t, --target <dir>`  Target root directory, default `$HOME`
`-v, --verbose`  Verbose output

> When extracting, existing files at target paths are directly overwritten without preview or confirmation prompt.

### list

`<archive>`  Snapshot archive to inspect
`-v, --verbose`  Verbose output

> Reads gzip comment and archive contents, displaying source paths and file lists under each path.

### delta

`<archive>`  Snapshot archive to compare
`-p, --pack`  Pack modified files into a delta snapshot

By default lists files modified since snapshot (files that exist in both snapshot and disk but have different content):

```
.config/nvim/init.vim
.config/nvim/lua/plugins.lua
```

> With `-p`, packs modified files into output in the same directory as input file, default name `<name>.delta.tar.gz` (e.g., `nvim.snap.tar.gz` -> `nvim.delta.tar.gz`). Gzip comment reuses original paths. New files (on disk but not in snapshot) and deleted files are not included.

---

# Appendix

## Code Structure

```
src/
├── cmd_cbp/
│   ├── mod.rs
│   ├── dot.rs             # dot — single-file template pipeline
│   └── snap/              # snap — batch snapshot/restore
│       ├── mod.rs
│       ├── save.rs
│       ├── load.rs
│       ├── list.rs
│       └── delta.rs
├── libs/
│   ├── mod.rs
│   ├── dirs.rs            # directory management
│   ├── utils.rs           # utility functions, system detection
│   └── dot.rs             # system info + filename parsing + template rendering
└── ...
```

## Tech Stack

| Feature | Implementation |
|---------|---------------|
| CLI parsing | `clap` |
| Template engine | `tera` |
| Compression/decompression | `flate2` + `tar` |
| gzip comment | `flate2::GzBuilder::comment()` / `GzDecoder::header().comment()` |
| Path handling | `std::path::Path` + `dunce` |
| Directory traversal | `walkdir` |

## Comparison with Similar Tools

| Feature | HomeDir | chezmoi | YADM |
|---------|---------|---------|------|
| Positioning | Extremely simple CLI tool | Complete manager | Git wrapper |
| Git integration | Externalized | Built-in | Built-in |
| Templates | Supported | Supported | Supported |
| Batch snapshots | snap command | No | No |
| Encryption | Not supported | Supported | Supported |
| Config files | None | Yes | Yes |
| Learning curve | Extremely low | Medium | Low |

## References

- [YADM](https://github.com/yadm-dev/yadm) — Reference project (v3.5.0)
- [chezmoi docs](https://www.chezmoi.io/)
- [Tera template engine](https://keats.github.io/tera/)
- [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
