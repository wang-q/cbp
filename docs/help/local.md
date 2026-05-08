Install packages from local binaries or downloaded cache.

Search locations:
* `./binaries/` — Pre-built binary directory (primary)
* `~/.cbp/cache/` — Downloaded packages (fallback)

Package format: `<package_name>.<type>.tar.gz`

Features:
* Installation status checking
* Automatic location selection
* Package record management
* File extraction to `~/.cbp`

Examples:
1. Install a single package:
   `cbp local zlib`

2. Install multiple packages:
   `cbp local zlib bzip2`

3. Install fonts:
   `cbp local -t font arial`

4. List package contents without installing:
   `cbp local -l zlib`

5. Cross-platform install (developer option):
   `cbp local -t windows zlib`