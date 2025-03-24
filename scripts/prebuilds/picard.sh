#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

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
mkdir -p collect/bin/
cat > collect/bin/picard << 'EOF'
#!/bin/bash

# Find the real path of the script
if [ -L "$0" ]; then
    SCRIPT_PATH=$(readlink "$0")
else
    SCRIPT_PATH="$0"
fi
SCRIPT_DIR=$(cd "$(dirname "$SCRIPT_PATH")" && pwd)
BASE_DIR=$(dirname "$SCRIPT_DIR")

# Set memory options if needed
JAVA_OPTS=${JAVA_OPTS:-"-Xmx2g"}

exec java $JAVA_OPTS -jar "${BASE_DIR}/libexec/picard.jar" "$@"
EOF

chmod +x collect/bin/picard

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp collect -o "${FN_TAR}" collect ||
    { echo "==> Error: Failed to create archive"; exit 1; }
mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
    { echo "==> Error: Failed to move archive"; exit 1; }
