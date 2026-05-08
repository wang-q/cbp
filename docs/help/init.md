Initialize cbp environment and configure shell settings.

Operations:
* Create `~/.cbp` directory structure
* Install cbp executable
* Update `$PATH` on Bash, Zsh, or Windows

Configuration:
* Default: Uses `~/.cbp` for everything
* Custom: Separates config and packages

Examples:
1. Default installation:
   `cbp init`

2. Custom package directory:
   `cbp init /opt/cbp`