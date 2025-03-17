# Perl

Build on a CentOS 7 VM

```bash
curl https://raw.githubusercontent.com/tristanisham/zvm/master/install.sh | bash
source $HOME/.bashrc
zvm install 0.13.0

curl -LO https://github.com/wang-q/cbp/releases/latest/download/cbp.linux
chmod +x cbp.linux
./cbp.linux init --dev

source ~/.bashrc

cbp install zlib bzip2 libxcrypt

curl -L https://cpanmin.us | perl - --sudo App::cpanminus

cpanm --sudo IPC::Run3 Devel::PatchPerl Path::Tiny

cd ~/Scripts/cbp

curl -o sources/perl-5.34.3.tar.gz -L https://www.cpan.org/src/5.0/perl-5.34.3.tar.gz

perl scripts/relocatable.pl ~/perl sources/perl-5.34.3.tar.gz --jobs 8

ldd ~/perl/bin/perl
        # linux-vdso.so.1 =>  (0x00007ffdce7bc000)
        # libm.so.6 => /lib64/libm.so.6 (0x00007f662d69c000)
        # libpthread.so.0 => /lib64/libpthread.so.0 (0x00007f662d480000)
        # libc.so.6 => /lib64/libc.so.6 (0x00007f662d0b2000)
        # libdl.so.2 => /lib64/libdl.so.2 (0x00007f662ceae000)
        # /lib64/ld-linux-x86-64.so.2 (0x00007f662d99e000)

cbp tar -o binaries/perl5.34.linux.tar.gz ~/perl/

rm -fr ~/perl/

```
