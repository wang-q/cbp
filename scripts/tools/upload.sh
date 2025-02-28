#!/bin/bash

if [ $# -eq 0 ]; then
    echo "Usage: $0 <files...>"
    exit 1
fi

# Get current release notes
NOTES=$(gh release view Binaries --json body -q .body)

# Create temp file for hashes
TEMP_HASHES=$(mktemp)
trap 'rm -f $TEMP_HASHES' EXIT

# Extract existing hashes
if echo "$NOTES" | grep -q '```text'; then
    echo "$NOTES" | sed -n '/^```text$/,/^```$/p' | grep -v '^```' > "$TEMP_HASHES"
fi

# Process each file
for file in "$@"; do
    NAME=$(basename "$file")
    HASH=$(shasum -a 256 "$file" | cut -d' ' -f1)
    
    echo "==> Processing ${NAME}..."
    # Update hash in temp file
    sed -i '' -E "/${NAME}: [a-f0-9]{64}/d" "$TEMP_HASHES"
    echo "${NAME}: ${HASH}" >> "$TEMP_HASHES"
    
    # Upload file
    echo "==> Uploading ${NAME}..."
    gh release upload Binaries "$file" --clobber
done

# Sort hashes
sort "$TEMP_HASHES" > "${TEMP_HASHES}.sorted"
mv "${TEMP_HASHES}.sorted" "$TEMP_HASHES"

# Update notes with new hash block
if echo "$NOTES" | grep -q '```text'; then
    NOTES=$(echo "$NOTES" | sed '/^```text$/,/^```$/d')
fi
NOTES="${NOTES}\n\`\`\`text\n$(cat "$TEMP_HASHES")\n\`\`\`"

# Update release notes
echo -e "$NOTES" | gh release edit Binaries --notes-file -

echo "==> All files processed successfully"
