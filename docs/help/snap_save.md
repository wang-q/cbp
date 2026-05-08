Save files and directories as a snapshot archive.

Creates a `.snap.tar.gz` file containing the specified paths.
Source paths are stored in the gzip comment for reliable restoration.

Examples:
1. Save a single file:
   `cbp snap save ~/.bashrc`

2. Save multiple files:
   `cbp snap save ~/.bashrc ~/.vimrc -o dotfiles.snap.tar.gz`

3. Save a directory:
   `cbp snap save ~/.config/nvim -o nvim.snap.tar.gz`

4. Verbose output:
   `cbp snap save -v ~/.bashrc`