# Common Shell Library Documentation

This document describes the functionality provided by the `common.sh` script, which serves as a
shared library for build scripts.

## Overview

The `common.sh` script provides a set of common variables and utility functions for building and
packaging projects. It is designed to be sourced by other scripts, not executed directly.

## Usage

```bash
# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

```

## Variables

The script defines the following variables:

| Variable      | Description                                        |
|---------------|----------------------------------------------------|
| `BASH_DIR`    | Directory of the calling script                    |
| `PROJ`        | Name of the calling script (without .sh extension) |
| `OS_TYPE`     | Operating system type (`linux` or `macos`)         |
| `TARGET_ARCH` | Target architecture for compilation                |
| `TEMP_DIR`    | Temporary working directory (auto-cleaned on exit) |

## Functions

| Function                      | Description                                                    |
|-------------------------------|----------------------------------------------------------------|
| `extract_source`              | Extracts `${PROJ}.tar.gz` from the `sources/` directory        |
|                               | andcChanges to the extracted directory                         |
| `build_tar [name]  [os_type]` | Creates a compressed archive from collected files              |
|                               | * `name`: Archive name (defaults to `$PROJ`)                   |
|                               | * `os_type`: OS type for archive name (defaults to `$OS_TYPE`) |
| `collect_make_bins`           | Collects binaries listed in Makefile's `all` target            |
| `collect_bins bin1 [bin2...]` | Collects specified binary files                                |
|                               | * `bin1, bin2...`: List of binary files to collect             |
| `fix_shebang file`            | Fixes shebang lines in script files                            |
|                               | * Replaces perl paths with `#!/usr/bin/env perl`               |
|                               | * Replaces python paths with `#!/usr/bin/env python3`          |

## Error Handling

The script includes error checking for critical operations and will exit with an error message if
any of these operations fail.

## Cleanup

The script creates a temporary directory (`TEMP_DIR`) and sets up a trap to automatically remove
this directory when the script exits.
