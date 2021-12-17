# find empty folders

Locate and delete empty folders and duplicate files on your Mac, Windows, or Linux machines. In a non intrusivenon-intrusive and destructive way. Given the path, this utility generates an executable script to delete empty directories or duplicate files depending on set flag. The execution summary includes file category-based distribution of duplicates in the percentage format.

USAGE: find-empty-folders [FLAGS] [OPTIONS] [dirs]...

FLAGS: -i, --dispdup Display duplicate files on concole. -d, --dupfiles Generate batch file to delete duplicate files. -e, --emptydirs Generate batch file to delete empty folders. -h, --help Prints help information -t, --timeit Print timing info. -V, --version Prints version information

FLAGS: -i, --dispdup Display duplicate files on concole.

-d, --dupfiles     Generate batch file to delete duplicate files.

-e, --emptydirs    Generate batch file to delete empty folders.

-h, --help         Prints help information

-t, --timeit       Print timing info.

-V, --version      Prints version information
OPTIONS: --dup_minsize <dup_minsize> Specify min file size above which duplicates should be searched for, in MB. [default: 5]

ARGS: ...

Example:
find-empty-folders.exe --timeit --dupfiles --dup_minsize=25 d:\projects

Searching for duplicate files...
Total no. of files processes: 552636
Total no. of duplicate files: 33
Total amount of disk space can be reclaimed: 1244MB
To reclaimed the disk space run this generated script file "deleteduplicates.cmd"
. You may review the script before before running it
duration: 50.161057s
The distribution of duplicates looks like this:  exec_files: 18.182%, other_files: 81.818%

Credits: Thanks to Andrew Gallant for sharing walkdir code https://github.com/BurntSushi/walkdir. It saved me a lot of time

For feature requests contact Cadence Data Soft Pvt. Ltd - debbi@cadencedatasoft.in
