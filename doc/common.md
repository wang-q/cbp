# Common Shell Library Documentation

The `scripts/common.sh` script provides a set of common variables and utility functions for building
and packaging projects. It serves as a shared library and must be sourced by other scripts rather
than executed directly.

## Usage

```bash
# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Optional command line arguments
# $1: OS type (linux or macos, defaults based on current system)
# $2: Test mode flag (optional)
```

## Variables

The script defines the following variables:

| Variable      | Description                                        |
|---------------|----------------------------------------------------|
| `BASH_DIR`    | Directory of the calling script                    |
| `PROJ`        | Name of the calling script (without .sh extension) |
| `OS_TYPE`     | Operating system type (`linux` or `macos`)         |
| `TARGET_ARCH` | Target architecture for compilation                |
|               | - linux: `x86_64-linux-gnu.2.17`                   |
|               | - macos: `aarch64-macos-none`                      |
| `TEMP_DIR`    | Temporary working directory (auto-cleaned on exit) |
| `RUN_TEST`    | Test mode flag                                     |

## Functions

### Source Code Management

| Function                      | Description                                                                                    |
|-------------------------------|------------------------------------------------------------------------------------------------|
| Source Code Management        |                                                                                                |
| `extract_source`              | Extracts `${PROJ}.tar.gz` from the `sources/` directory and changes to the extracted directory |
| Binary Collection             |                                                                                                |
| `collect_make_bins`           | Collects binaries listed in Makefile's `all` target into `collect/bin/`                        |
| `collect_bins bin1 [bin2...]` | Collects specified binary files into `collect/bin/`                                            |
| Archive Management            |                                                                                                |
| `build_tar [name] [os_type]`  | Creates a compressed archive from files in `collect/` directory                                |
|                               | - `name`: Archive name (defaults to `$PROJ`)                                                   |
|                               | - `os_type`: OS type for archive name (defaults to `$OS_TYPE`)                                 |
| Script Utilities              |                                                                                                |
| `fix_shebang file`            | Fixes shebang lines in script files:                                                           |
|                               | - Perl: `#!/usr/bin/env perl`                                                                  |
|                               | - Python: `#!/usr/bin/env python3`                                                             |
| `run_test prog`               | Runs test program and verifies for "PASSED" in output                                          |

## Error Handling

The script implements comprehensive error checking:

- Prevents direct execution of the script
- Validates OS type and command line arguments
- Checks for source files and directories
- Verifies binary collection and archive creation

## Cleanup

The script implements automatic cleanup:

- Creates temporary directory in `TEMP_DIR`
- Sets up EXIT trap to remove temporary files
- Cleans up even if the script fails
