#!/usr/bin/env perl
use strict;
use warnings FATAL => 'all';
use utf8;
use Getopt::Long qw();
use 5.10.1;
use Config;
use Path::Tiny qw();
use File::Temp qw(tempfile tempdir);
use File::Basename 'dirname';
use Devel::PatchPerl;
use IPC::Run3 qw();
use version;
use POSIX qw();

=head1 NAME

relocatable.pl - building perl with relocatable settings

=head1 SYNOPSIS

    $ relocatable.pl PREFIX TARBALL [OPTIONS]

    Arguments:
      PREFIX          install prefix, this is just for installation.
                     After installation, you can move perl wherever you want!
      TARBALL        use local tar.gz

    Options:
      --perl_version  install perl version (optional, will be parsed from tarball)
      --jobs         parallel build, default: 1
      --help, -h     show this help message

    Examples:
    $ relocatable.pl ~/perl perl-5.18.2.tar.gz
    $ relocatable.pl ~/perl perl-5.18.2.tar.gz --perl_version=5.18.2 --jobs=4

=head1 AUTHOR

Shoichi Kaji

=cut

#-----------------------------
# GetOptions
#-----------------------------
my $opt_prefix = shift or do {
    warn "Error: PREFIX argument is required\n";
    warn "Example: relocatable.pl ~/perl5 perl-5.36.1.tar.gz\n";
    Getopt::Long::HelpMessage(1);
};

my $opt_tarball = shift or do {
    warn "Error: TARBALL argument is required\n";
    warn "Example: relocatable.pl ~/perl5 perl-5.36.1.tar.gz\n";
    Getopt::Long::HelpMessage(1);
};

Getopt::Long::GetOptions(
    "perl_version=s" => \( my $opt_perl_version ),
    "jobs=i"         => \( my $opt_jobs ),
    "help|h"         => sub { Getopt::Long::HelpMessage(0) },
) or Getopt::Long::HelpMessage(1);

{
    $opt_prefix
      or do {
        warn "Error: --prefix option is required\n";
        warn "Example: relocatable.pl --prefix ~/perl5\n";
        Getopt::Long::HelpMessage(1);
      };

    $opt_tarball
      or do {
        warn "Error: --tarball option is required\n";
        warn "Usage: relocatable.pl --tarball <path/to/perl-source.tar.gz>\n";
        warn "Example: relocatable.pl --tarball perl-5.36.1.tar.gz\n";
        Getopt::Long::HelpMessage(1);
      };

    # Try to parse perl version from tarball name if not specified
    if ( !$opt_perl_version && $opt_tarball =~ /5\.(\d+)\.(\d+)/ ) {
        $opt_perl_version = "5.$1.$2";
    }

    $opt_perl_version
      or do {
        warn "Error: --perl_version option is required\n";
        warn "Usage: relocatable.pl --perl_version <version>\n";
        warn "Example: relocatable.pl --perl_version 5.36.1\n";
        Getopt::Long::HelpMessage(1);
      };

    if ( !-d $opt_prefix ) {
        mkdir $opt_prefix
          or die "Error: Failed to create directory $opt_prefix: $!\n";
    }
    elsif ( !-w $opt_prefix ) {
        die "Error: No write permission for directory $opt_prefix\n";
    }
}

#-----------------------------
# Build
#-----------------------------
perl_build( $opt_prefix, $opt_perl_version, $opt_tarball );
force_symlink( $opt_prefix, $opt_perl_version );

#-----------------------------
# Patch config
#-----------------------------
{
    my $config_heavy = `$opt_prefix/bin/perldoc -lm Config_heavy.pl`;
    die "failed to exec $opt_prefix/bin/perldoc -lm Config_heavy.pl\n"
      if $? != 0;
    chomp $config_heavy;
    patch_config_heavy($config_heavy);

    my $config_pm = `$opt_prefix/bin/perldoc -lm Config`;
    die "failed to exec $opt_prefix/bin/perldoc -lm Config\n" if $? != 0;
    chomp $config_pm;
    patch_config_pm($config_pm);
}

say "---> successfully build perl $opt_perl_version to $opt_prefix";
system "$opt_prefix/bin/perl -V";

exit;

sub run {
    my ( $cmd, $log ) = @_;
    warn "---> Executing: @$cmd\n";

    # Convert Path::Tiny object to string for IPC::Run3
    my $logfile = "$log";
    IPC::Run3::run3 $cmd, undef, $logfile, $logfile, { append_stdout => 1 };

    if ( $? != 0 ) {
        print Path::Tiny->path($logfile)->slurp_utf8;
        die "Error: Command failed with status $?\n";
    }
    return 1;
}

sub perl_build {
    my ( $prefix, $perl_version, $tarball ) = @_;

    my $current_dir = Path::Tiny->cwd;
    my $now         = time;
    my $tempdir     = Path::Tiny->tempdir(
        TEMPLATE => "perl-build-$now.$$-XXXXXX",
        CLEANUP  => 1
    );
    my $logfile = Path::Tiny->tempfile(
        TEMPLATE => "perl-build-$now.$$.log-XXXXXX",
        CLEANUP  => 0
    );

    my %tar_option =
      ( "tar.gz" => "xzf", "tar.bz2" => "xjf", "tar.xz" => "xJf" );
    say "---> use $tarball";
    my ($suffix) = $tarball =~ /\.(tar\.(?:gz|bz2|xz))$/;
    run [ "tar", $tar_option{$suffix}, $tarball, "-C", $tempdir ], $logfile;

    my @Configure = ( "./Configure", "-des", );

    # Basic configuration
    push @Configure,
      (
        "-Dprefix=$prefix", "-Duserelocatableinc",
        "-Duselargefiles",  "-Dusethreads",
        "-Dman1dir=none",   "-Dman3dir=none",
        "-UDEBUGGING",
      );

    # Compiler configuration
    push @Configure, qq(-Dcc=zig-cc);

    my $cbp_include = `cbp prefix include`; chomp $cbp_include;
    push @Configure, qq(-Accflags=-I${cbp_include});
    push @Configure, qq(-Accflags=-Wno-macro-redefined);
    push @Configure, qq(-Accflags=-Wno-compound-token-split-by-macro);

    my $cbp_lib = `cbp prefix lib`; chomp $cbp_lib;
    push @Configure, qq(-Aldflags=-L${cbp_lib});
    my @libpth = ($cbp_lib);
    push @Configure, "-Dlibpth=@libpth";

    if ( $^O eq "linux" ) {

        # ubuntu 18.04 does not have xlocale.h
        # we can safely remove xlocale.h because locale.h reads it
        # see https://github.com/agracio/electron-edge-js/issues/16
        push @Configure, "-Ui_xlocale";

        # RHEL8, Fedora28, CentOS8 does not have libnsl.so.1 by default
        # remove -lnsl
        push @Configure, "-Dlibs=-lpthread -ldl -lm -lcrypt -lutil -lc";

        # RHEL9 removes libcrypt.so.1 by default; link libcrypt.a statically
        # manually define d_crypt here
        push @Configure, "-Dd_crypt";

        # math.h in debian does not define _LIB_VERSION
        push @Configure, "-Ud_libm_lib_version";

        # my $arch   = (POSIX::uname)[4];
        # my @libpth = (
        #     "/lib",                     "/lib/$arch-linux-gnu",
        #     "/lib64",                   "/usr/lib",
        #     "/usr/lib/$arch-linux-gnu", "/usr/lib64",
        #     "/usr/local/lib",           "/usr/local/lib64",
        # );
        # push @Configure, "-Dlibpth=@libpth";
    }

    chdir "$tempdir/perl-$perl_version" or die;
    my $devel = "Devel::PatchPerl";
    say "---> patching by $devel " . $devel->VERSION;
    $devel->patch_source( $perl_version, "." );
    if ( $^O eq "darwin" ) {
        {
            say "---> patching, do not add macosx_version_min";
            my $file = "hints/darwin.sh";
            open my $in,  "<", $file       or die;
            open my $out, ">", "$file.tmp" or die;
            while ( my $line = <$in> ) {
                print {$out} $line;
                if ( $line =~ m{^ \s* add_macosx_version_min \s* \( }x ) {
                    print {$out} "  return\n";
                }
            }
            close $in;
            close $out;
            rename "$file.tmp", $file or die;
        }
    }
    say "---> building perl $perl_version, see $logfile for progress";
    run \@Configure, $logfile;
    my @option = $opt_jobs ? ("--jobs=$opt_jobs") : ();
    run [ "make", @option, "install" ], $logfile;
    chdir $current_dir;
}

sub patch_config_heavy {
    my $config_heavy = shift;
    my @relocatable  = do {
        open my $fh, "<", $config_heavy or die "open $config_heavy: $!\n";
        my @relocatable;
        while (<$fh>) {
            if (/^([^=]+)=['"].*\.\.\./) {
                push @relocatable, $1;
            }
        }
        @relocatable;
    };

    open my $in, "<", $config_heavy or die "open $config_heavy: $!\n";
    my ( $out, $tmpname ) = tempfile UNLINK => 0, DIR => dirname($config_heavy);
    my $mode = ( stat $config_heavy )[2];
    chmod $mode, $tmpname;

    my %fix = (

        # XXX initialinstalllocation
        installbin    => '.../../bin',
        installprefix => '.../..',
        perlpath      => '.../perl',
        startperl     => '#!.../perl',
    );
    push @relocatable, sort keys %fix;
    @relocatable = uniq(@relocatable);

    my $fix_line1 = 'foreach my $what';
    my $fix_line2 = 's/^($what=)';

    while (<$in>) {
        if ( /^([a-zA-Z0-9_]+)=/ && $fix{$1} ) {
            say {$out} "$1='$fix{$1}'";
        }
        elsif (/^\Q$fix_line1\E/) {
            say {$out} 'foreach my $what (qw(' . "@relocatable" . ')) {';
        }
        elsif (/^(\s+)\Q$fix_line2\E/) {
            say {$out} $1,
q{s/^($what=)(['"])(#!)?(.*?)\2/$1 . $2 . ($3 || "") . relocate_inc($4) . $2/me;};
        }
        else {
            print {$out} $_;
        }
    }
    close $_ for $in, $out;
    rename $tmpname, $config_heavy or die "rename $tmpname $config_heavy: $!\n";
}

sub patch_config_pm {
    my $config_pm = shift;
    open my $in, "<", $config_pm or die "open $config_pm: $!\n";
    my ( $out, $tmpname ) = tempfile UNLINK => 0, DIR => dirname($config_pm);
    my $mode = ( stat $config_pm )[2];
    chmod $mode, $tmpname;

    my %fix = (
        'while ($libdir =~ m!^\.\./!) {',
        => 'while ($libdir =~ m!^\.\.(?:/|$)!) {',
        '$libdir = "$prefix/$libdir";',
        => '$libdir = $prefix . (length $libdir ? "/$libdir" : "");',
    );
    my $fix = join "|", map { quotemeta $_ } sort keys %fix;
    while (<$in>) {
        if (/^(\s*)($fix)$/) {
            say {$out} "$1$fix{$2}";
        }
        elsif (/^(\s+)scriptdir\s*=>/) {
            say {$out} "$1scriptdir => relocate_inc('.../'),";
        }
        else {
            print {$out} $_;
        }
    }
    close $_ for $in, $out;
    rename $tmpname, $config_pm or die "rename $tmpname $config_pm: $!\n";
}

sub uniq {
    my @item = @_;
    my %seen;
    grep { !$seen{$_}++ } @item;
}

sub force_symlink {
    my ( $prefix, $perl_version ) = @_;

    # See utils.lst and installperl
    my %map = (
        perl    => "perl$perl_version",
        c2ph    => "pstruct",
        perlbug => "perlthanks",
    );
    my $cwd = Path::Tiny->cwd;
    chdir "$prefix/bin" or die "Failed to chdir $prefix/bin: $!";
    for my $file ( grep -f, values %map ) {
        unlink $file or die "Failed to unlink $file: $!";
    }
    for my $from ( sort grep -f, keys %map ) {
        my $target = $map{$from};
        symlink $from => $target
          or die "Failed to symlink $from => $target: $!";
    }
    chdir $cwd;
}
