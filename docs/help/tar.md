Create a compressed archive from a software package directory.
This command is specifically designed for packaging software installations,
not as a general-purpose tar replacement.

The command will:
* Filter out system files (.DS_Store, etc.)
* Preserve relative paths within archive
* Preserve symbolic links with relative targets
* Clean up documentation directories (optional)

Examples:
1. Package current directory:
   cbp tar .

2. Package specific directory:
   cbp tar path/to/dir

3. Custom output:
   cbp tar path/to/dir -o output.tar.gz

4. Clean up docs:
   cbp tar path/to/dir --cleanup
