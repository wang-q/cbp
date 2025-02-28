#!/bin/bash

if [ $# -eq 0 ]; then
    echo "Usage: $0 <files...>"
    exit 1
fi

# Get current release notes
NOTES=$(gh release view Binaries --json body -q .body)

# Process each file
for file in "$@"; do
    # Calculate hash
    HASH=$(shasum -a 256 "$file" | cut -d' ' -f1)
    NAME=$(basename "$file")
    
    echo "==> Processing ${NAME}..."
    
    # Remove old hash for this file from notes
    NOTES=$(echo "$NOTES" | sed -E "/${NAME}: [a-f0-9]{64}/d")
    
    # Add new hash
    NOTES="${NOTES}${NAME}: ${HASH}\n"
    
    # Upload file
    echo "==> Uploading ${NAME}..."
    gh release upload Binaries "$file" --clobber
done

# Update release notes
echo -e "$NOTES" | gh release edit Binaries --notes-file -

echo "==> All files processed successfully"
