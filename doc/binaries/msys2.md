# MSYS2 building environment

## Development Environment

### MSYS2 Installer

1. Download [msys2 installer](https://github.com/msys2/msys2-installer/releases/download/2025-02-21/msys2-x86_64-20250221.exe) and install it.

2. Add a profile to Windows Terminal.

    ```json
    {
        "commandline": "C:/msys64/msys2_shell.cmd -defterm -here -no-start -clang64",
        "guid": "{17da3cac-b318-431e-8a3e-7fcdefe6d114}",
        "icon": "C:/msys64/clang64.ico",
        "name": "CLANG64 / MSYS2",
        "startingDirectory": "C:/msys64/home/%USERNAME%"
    }
    ```

### Setup

* Inside Windows

```powershell
iwr "https://github.com/wang-q/cbp/releases/latest/download/cbp.windows.exe" -OutFile cbp.windows.exe
.\cbp.windows.exe init

```

* CLANG64 / MSYS2

```bash
# Update the system packages
pacman -Syu
pacman -Sy --needed bash pacman pacman-mirrors msys2-runtime

# msys itself
pacman -S --needed --noconfirm base-devel
pacman -S --needed --noconfirm xz zip unzip
pacman -S --needed --noconfirm p7zip

# clang64
pacman -S mingw-w64-clang-x86_64-gcc

pacman -S mingw-w64-clang-x86_64-zig
pacman -S mingw-w64-clang-x86_64-jq

pacman -S mingw-w64-clang-x86_64-cmake
pacman -S mingw-w64-clang-x86_64-ninja
pacman -S mingw-w64-clang-x86_64-meson

# cbp
cd
curl -LO https://github.com/wang-q/cbp/releases/latest/download/cbp.windows.exe

cbp.windows.exe init

echo "export PATH=/c/Users/$USER/.cbp/bin:$PATH" >> ~/.bashrc
source ~/.bashrc

```

## Core Libraries

```bash
cd /c/Users/$USER/Scripts/cbp

bash scripts/zlib.sh -t
bash scripts/bzip2.sh -t
bash scripts/libdeflate.sh -t
bash scripts/xz.sh -t

cbp local zlib bzip2 libdeflate xz

```


## Other Libraries

```bash
# bash scripts/ncurses.sh

bash scripts/argtable.sh

```
## `Makefile`

```bash
bash scripts/pigz.sh
cbp local -l pigz

```
## `./configure`

```bash
bash scripts/TRF.sh
cbp local -l TRF

# ./stopwatch.h:23:10: fatal error: 'sys/times.h' file not found
# cbp local argtable
# bash scripts/clustalo.sh

```
## `cmake`

```bash
#  error: no matching function for call to 'max'
bash scripts/bifrost.sh

```
