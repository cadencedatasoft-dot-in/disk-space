# disk-space
Locate and delete empty folders and duplicate files on your Mac, Windows or Linux machines. In a non intrusive and destructive way.

Locate and delete empty folders and duplicate files using non-intrusive find-duplicate 0.1.0
Cadence Data Soft Pvt. Ltd <debbi@cadencedatasoft.in>

USAGE:
    find-empty-folders.exe [FLAGS] [OPTIONS] [dirs]...

FLAGS:
    -i, --dispdup      Display duplicate files on concole.
    -d, --dupfiles     Generate batch file to delete duplicate files.
    -e, --emptydirs    Generate batch file to delete empty folders.
    -h, --help         Prints help information
    -t, --timeit       Print timing info.
    -V, --version      Prints version information

OPTIONS:
        --dup_minsize <dup_minsize>    Specify min file size above which duplicates should be
                                       searched for, in MB. [default: 5]

ARGS:
    <dirs>...

