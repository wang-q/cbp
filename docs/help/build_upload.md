Upload package files to GitHub release and update checksums.

Requirements:
* GitHub CLI (`gh`) must be installed and authenticated
* For proxy support, use `HTTPS_PROXY` environment variable

The command will:
* Calculate MD5 checksums
* Upload files to GitHub release
* Update release notes with checksums

Examples:
1. Upload a single file:
   `cbp build upload binaries/zlib.macos.tar.gz`

2. Upload multiple files:
   `cbp build upload binaries/*.tar.gz`

3. Force upload (skip MD5 check):
   `cbp build upload --force binaries/*.tar.gz`
