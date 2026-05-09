Manage file snapshots for backup and restore.

Snapshots store the original file paths in the gzip comment,
allowing files to be restored to their correct locations.

Behavior:
* Snapshots are stored as `.snap.tar.gz` files
* Source paths are saved relative to HOME when possible
* Files outside HOME are stored with absolute paths
* Delta snapshots capture only modified files

Path handling:
* `~` expands to the user's home directory (`$HOME` on Unix, `%USERPROFILE%` on Windows)
* In PowerShell, `~` works natively; in CMD, use `%USERPROFILE%` directly

Windows directory mapping:
* `~/AppData/Roaming/` — Roaming application data (`%APPDATA%`)
* `~/AppData/Local/` — Local application data (`%LOCALAPPDATA%`)
* `~/AppData/Local/Temp/` — Temporary files (`%TEMP%`)

Subcommands:
* `save` — Save files and directories as a snapshot archive
* `load` — Restore files from a snapshot archive
* `list` — List contents of a snapshot archive
* `delta` — Show files modified since a snapshot was taken
