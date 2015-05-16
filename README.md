# cmded
Set of command line tools for getting and setting lines in files, written in Rust. Work in progress.

cmded-getline <-|filepath> <line number>[..<$|line number>]
    print specific line(s) from standard input or given file
Examples:
a)
    cat /etc/apt/sources.list | cmded-getline - 5
    Prints fifth line from file /etc/apt/sources.list.
b)
    cmded-getline /etc/apt/sources.list 5
    Does the same as a)
c)
    cmded-getline /etc/apt/sources.list 5..7
    Prints lines from 5th to 7th of given file.
d)
    cmded-getline /etc/apt/sources.list 5..$
    Prints lines from 5th to the end of the file.
e)
    cmded-getline /etc/apt/sources.list 1..$
    Prints whole file.

cmded-setline: TODO
