Scan ~/.cbp directory for files not managed by any package.
Helps identify and clean up redundant files.

Scan scope:
* Files not listed in any package records
* Files outside cbp system directories (`records/`, `cache/`)
* Files not required by cbp itself

Auto-ignored files:
* macOS: `.DS_Store`, `__MACOSX/`, `.AppleDouble`, `._*` (resource fork files)
* Linux: backup files (`*~`), Vim swap files (`.swp`)
* Windows: `Thumbs.db`, `desktop.ini`

Examples:
1. Check for unmanaged files:
   cbp check