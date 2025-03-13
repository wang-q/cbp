# Get script directory
$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location "${ScriptRoot}\.."

function New-TemporaryDirectory {
    $parent = [System.IO.Path]::GetTempPath()
    $name = [System.IO.Path]::GetRandomFileName()
    New-Item -ItemType Directory -Path (Join-Path $parent $name)
}

# Check if project name is provided
if ($args.Count -eq 0) {
    Write-Host "Usage: $($MyInvocation.MyCommand.Name) <PROJECT_NAME>"
    Write-Host "Example: $($MyInvocation.MyCommand.Name) intspan"
    exit 1
}
$PROJECT_NAME = $args[0]

# Check for source tarball
$SOURCE_FILE = "sources\${PROJECT_NAME}.tar.gz"
if (-not (Test-Path $SOURCE_FILE)) {
    Write-Error "Error: Source file $SOURCE_FILE not found"
    exit 1
}

# Create temp directory
$TEMP_DIR = New-TemporaryDirectory
Push-Location $TEMP_DIR

try {
    # Copy and extract source
    Copy-Item (Join-Path $ScriptRoot "..\sources\${PROJECT_NAME}.tar.gz") (Join-Path $TEMP_DIR "${PROJECT_NAME}.tar.gz")
    7z x "${PROJECT_NAME}.tar.gz" -y
    7z x "${PROJECT_NAME}.tar" -y

    # Change to project directory
    $PROJECT_DIR = Get-ChildItem -Directory -Filter "${PROJECT_NAME}*" | Select-Object -First 1
    if (-not $PROJECT_DIR) {
        throw "Error: Cannot find source directory ${PROJECT_NAME}"
    }
    Set-Location $PROJECT_DIR

    # Build project
    cargo build --release
    if ($LASTEXITCODE -ne 0) { throw "Build failed" }

    # List release directory contents
    Get-ChildItem "target\release"

    # Get binary targets from Cargo.toml
    $BINS = cargo metadata --no-deps --format-version 1 |
        ConvertFrom-Json |
        Select-Object -ExpandProperty packages |
        Select-Object -ExpandProperty targets |
        Where-Object { $_.kind[0] -eq "bin" } |
        Select-Object -ExpandProperty name

    # Copy binaries
    New-Item -ItemType Directory -Path "collect\bin" -Force
    foreach ($BIN in $BINS) {
        $BinPath = "target\release\$BIN.exe"
        if (Test-Path $BinPath) {
            Copy-Item $BinPath "collect\bin\"
        } else {
            Write-Warning "Binary $BIN.exe not found in target\release"
        }
    }

    # Create and move archive
    $FN_TAR = "${PROJECT_NAME}.windows.tar.gz"
    cbp tar collect -o $FN_TAR
    Move-Item -Force $FN_TAR "${ScriptRoot}\..\binaries\"

} finally {
    # Cleanup
    Pop-Location
    if ($TEMP_DIR -and (Test-Path $TEMP_DIR)) {
        Remove-Item -Recurse -Force $TEMP_DIR
    }
}
