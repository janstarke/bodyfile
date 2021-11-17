# bodyfile
Parser and Generator for the famous `mactime` bodyfile format

# Documentation

Quote from <https://wiki.sleuthkit.org/index.php?title=Body_file>:

> The body file is an intermediate file when creating a timeline of file
> activity. It is a pipe ("|") delimited text file that contains one line
> for each file (or other even type, such as a log or registry key). The
> fls, ils, and mac-robber tools all output this data format. The mactime
> tool reads this file and sorts the contents (therefore the format is
> sometimes referred to as the "mactime format").
>
> The body file format in TSK 3.0+ is different from the format used in TSK
> 1.X and 2.X.
>
> The 3.X output has the following fields:

> ```ignore,no_run
> MD5|name|inode|mode_as_string|UID|GID|size|atime|mtime|ctime|crtime
> ```

> The times are reported in UNIX time format. Lines that start with '#' are
> ignored and treated as comments. In mactime, many of theses fields are
> optional. Its only requirement is that at least one of the time values is
> non-zero. The non-time values are simply printed as is. Other tools that
> read this file format may have different requirements.

This crate implements generation and parsing of bodyfile lines

## Example
```rust
use bodyfile::Bodyfile3Line;
use std::convert::TryFrom;

let str_line = "0|/Users/Administrator ($FILE_NAME)|93552-48-2|d/drwxrwxrwx|0|0|92|1577092511|1577092511|1577092511|-1";
let bf_line = Bodyfile3Line::try_from(str_line).unwrap();
assert_eq!(str_line, bf_line.to_string());
```

License: GPLv3
