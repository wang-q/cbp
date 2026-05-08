Download and install pre-built binary packages from the GitHub release repository.
Checks for existing installations to avoid duplicates and handles platform-specific
package selection automatically.

[Release page](https://github.com/wang-q/cbp/releases/tag/Binaries)

Network proxy support (priority high to low):
* `--proxy` argument
* Environment variables: `ALL_PROXY`, `HTTP_PROXY`, `all_proxy`, `http_proxy`

Examples:
1. Install a single package:
   cbp install zlib

2. Install multiple packages:
   cbp install zlib bzip2

3. Install fonts:
   cbp install -t font arial

4. Use proxy:
   cbp install --proxy socks5://127.0.0.1:7890 zlib