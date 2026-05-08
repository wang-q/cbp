Show files modified since a snapshot was taken.

Compares files on disk against a snapshot archive and lists files
that have changed.

Examples:
1. Show modified files:
   `cbp snap delta dotfiles.snap.tar.gz`

2. Pack modified files into a delta snapshot:
   `cbp snap delta dotfiles.snap.tar.gz -p`