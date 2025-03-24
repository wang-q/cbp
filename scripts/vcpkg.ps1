# Get script directory
$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location "${ScriptRoot}\.."

# Check if running on Windows
if (-not $IsWindows) {
    Write-Error "This script is for Windows only. Please use vcpkg.sh on Unix-like systems."
    exit 1
}

# Check if package name is provided
if ($args.Count -eq 0) {
    Write-Host "Usage: $($MyInvocation.MyCommand.Name) <PACKAGE_NAME> [TRIPLET] [COPY_PAIRS...]"
    Write-Host "Example: $($MyInvocation.MyCommand.Name) zlib"
    Write-Host "Example: $($MyInvocation.MyCommand.Name) zlib x64-windows-zig"
    Write-Host "Example with copy: $($MyInvocation.MyCommand.Name) pkgconf x64-windows-zig pkgconf=pkg-config"
    exit 1
}

# Get package name and triplet
$PROJ = $args[0]
$BASE_PROJ = $PROJ -split '\[' | Select-Object -First 1
$TRIPLET = if ($args.Count -gt 1) { $args[1] } else { "x64-windows-zig" }
$OS_TYPE = "windows"

# Install the package using vcpkg and clean after build
vcpkg install --debug --recurse --allow-unsupported `
    --clean-buildtrees-after-build `
    --overlay-ports=ports `
    --overlay-triplets="$(cbp prefix triplets)" `
    --x-buildtrees-root=vcpkg/buildtrees `
    --downloads-root=vcpkg/downloads `
    --x-install-root=vcpkg/installed `
    --x-packages-root=vcpkg/packages `
    "${PROJ}:${TRIPLET}"
if ($LASTEXITCODE -ne 0) { throw "vcpkg install failed" }

# Find the package list file
$LIST_FILE = Get-ChildItem -Path "vcpkg/installed/vcpkg/info" -Filter "${BASE_PROJ}_*_${TRIPLET}.list" |
    Select-Object -First 1 -ExpandProperty FullName

if (-not $LIST_FILE) {
    Write-Error "Package list file not found for ${BASE_PROJ}:${TRIPLET}"
    exit 1
}
Write-Host "Found package list: $LIST_FILE"

# Process copy arguments
$COPY_ARGS = @()
if ($args.Count -gt 2) {
    $args[2..($args.Count-1)] | ForEach-Object {
        $COPY_ARGS += "--copy"
        $COPY_ARGS += $_
    }
}

# Create archive from the package list
cbp collect --mode vcpkg $LIST_FILE $COPY_ARGS
if ($LASTEXITCODE -ne 0) { throw "cbp collect failed" }

# Rename .mingw.tar.gz to .windows.tar.gz if needed
if (Test-Path "${BASE_PROJ}.mingw.tar.gz") {
    Move-Item -Force "${BASE_PROJ}.mingw.tar.gz" "${BASE_PROJ}.windows.tar.gz"
}

# Remove the package from cache
vcpkg remove --recurse `
    --overlay-ports=ports `
    --overlay-triplets="$(cbp prefix triplets)" `
    --x-buildtrees-root=vcpkg/buildtrees `
    --downloads-root=vcpkg/downloads `
    --x-install-root=vcpkg/installed `
    --x-packages-root=vcpkg/packages `
    "${BASE_PROJ}:${TRIPLET}"

# Move archive to the binaries directory
Move-Item -Force "${BASE_PROJ}.${OS_TYPE}.tar.gz" binaries/
