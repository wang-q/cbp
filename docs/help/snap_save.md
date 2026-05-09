Save files and directories as a snapshot archive.

Creates a `.snap.tar.gz` file containing the specified paths.
Source paths are stored in the gzip comment for reliable restoration.

Examples:
1. Save a directory:
   `cbp snap save ~/.config/nvim`

2. Save multiple directories to a named archive:
   `cbp snap save ~/.config/nvim ~/.config/alacritty -o configs.snap.tar.gz`

3. Save a single file:
   `cbp snap save ~/.bashrc`

4. Verbose output:
   `cbp snap save -v ~/.config/nvim`

5. Windows (PowerShell):
   `cbp snap save $env:APPDATA/alacritty -o alacritty.snap.tar.gz`

6. Windows (CMD):
   `cbp snap save %APPDATA%\alacritty -o alacritty.snap.tar.gz`
