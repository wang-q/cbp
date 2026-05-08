Restore files from a snapshot archive.

Extracts files from the archive to their original locations using
the source path information stored in the gzip comment.

Examples:
1. Restore to HOME:
   `cbp snap load dotfiles.snap.tar.gz`

2. Restore to a custom directory:
   `cbp snap load dotfiles.snap.tar.gz -t /tmp/restore`

3. Verbose output:
   `cbp snap load -v dotfiles.snap.tar.gz`