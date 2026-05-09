Display detailed package information from the GitHub repository.
Information is sourced from JSON files in the `packages/` directory.

Network proxy support (priority high to low):
* `--proxy` argument
* Environment variables: `ALL_PROXY`, `HTTP_PROXY`, `all_proxy`, `http_proxy`

Examples:
1. View package information:
   `cbp info newick-utils`

2. Output in JSON format:
   `cbp info bwa --json`

3. Use proxy:
   `cbp info newick-utils --proxy socks5://127.0.0.1:7890`
