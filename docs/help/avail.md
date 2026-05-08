Query and list available packages from the GitHub release repository.
Results are displayed in a formatted table, grouped alphabetically for better readability.

[Release page](https://github.com/wang-q/cbp/releases/tag/Binaries)

Network proxy support (priority high to low):
* `--proxy` argument
* Environment variables: `ALL_PROXY`, `HTTP_PROXY`, `all_proxy`, `http_proxy`

Examples:
1. List all packages:
   `cbp avail`

2. Platform-specific filtering:
   `cbp avail linux`

3. List fonts:
   `cbp avail font`

4. Use proxy:
   `cbp avail --proxy socks5://127.0.0.1:7890`