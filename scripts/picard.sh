#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Set download URL based on OS type
DL_URL="https://github.com/broadinstitute/picard/releases/download/3.3.0/picard.jar"

# Download jar
curl -L ${DL_URL} -o ${PROJ}.jar ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
cd ${TEMP_DIR} || { echo "Error: Failed to enter temp directory"; exit 1; }

# Collect binaries and scripts
mkdir -p collect/libexec/
cp picard.jar collect/libexec/

# Create wrapper script
cat > collect/picard << 'EOF'
#!/bin/bash
SCRIPT_DIR=$(dirname $(readlink -f "$0"))
exec java -jar "${SCRIPT_DIR}/libexec/picard.jar" "$@"
EOF

chmod +x collect/picard

# Use build_tar function from common.sh
build_tar
