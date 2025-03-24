# Common Shell Library

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

| Function                      | Description                                                    |
|-------------------------------|----------------------------------------------------------------|
| Source Code Management        |                                                                |
| `extract_source`              | Extracts `${PROJ}.tar.gz` from the `sources/` directory and    |
|                               | changes to the extracted directory                             |
| Archive Management            |                                                                |
| `build_tar [name] [os_type]`  | Creates a compressed archive from files in `collect/`          |
|                               | - `name`: Archive name (defaults to `$PROJ`)                   |
|                               | - `os_type`: OS type for archive name (defaults to `$OS_TYPE`) |

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
