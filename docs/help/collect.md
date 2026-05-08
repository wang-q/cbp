Collect and package files into a tar.gz archive.
Supports multiple processing modes for different file organization strategies.

Mode options:
* `files` — Collect files as-is (default)
* `list` — Process a list file containing file paths
* `vcpkg` — Process vcpkg-style list file
* `bin` — Place files in `bin/` directory
* `font` — Place files in `share/fonts/` directory

Examples:
1. Process files (default mode):
   `cbp collect program.exe`

2. Process list file:
   `cbp collect list.txt --mode list`

3. Process vcpkg list:
   `cbp collect pkg.list --mode vcpkg`

4. Create file aliases:
   `cbp collect program.exe --copy libz.so=libz.so.1`

5. Ignore specific files:
   `cbp collect src/ --ignore .dll --ignore .exe`

6. Specify output file:
   `cbp collect program.exe -o output.tar.gz`

7. Collect binaries:
   `cbp collect program.exe --mode bin`

8. Collect fonts:
   `cbp collect font.ttf --mode font`