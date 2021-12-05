# disk-space
Locate and delete empty folders and duplicate files on your Mac, Windows, or Linux machines. In a non-intrusive and destructive way. 

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

Credits: Thanks to Andrew Gallant for sharing walkdir code https://github.com/BurntSushi/walkdir. It saved me a lot of time
    
For feature requests contact Cadence Data Soft Pvt. Ltd - debbi@cadencedatasoft.in
