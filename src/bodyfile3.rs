use duplicate::duplicate;

/// Quote from <https://wiki.sleuthkit.org/index.php?title=Body_file>:
/// 
/// > The body file is an intermediate file when creating a timeline of file 
/// > activity. It is a pipe ("|") delimited text file that contains one line 
/// > for each file (or other even type, such as a log or registry key). The 
/// > fls, ils, and mac-robber tools all output this data format. The mactime 
/// > tool reads this file and sorts the contents (therefore the format is 
/// > sometimes referred to as the "mactime format").
/// > 
/// > The body file format in TSK 3.0+ is different from the format used in TSK
/// > 1.X and 2.X.
/// > 
/// > The 3.X output has the following fields:
/// 
/// > ```ignore,no_run
/// > MD5|name|inode|mode_as_string|UID|GID|size|atime|mtime|ctime|crtime
/// > ```
/// 
/// > The times are reported in UNIX time format. Lines that start with '#' are 
/// > ignored and treated as comments. In mactime, many of theses fields are 
/// > optional. Its only requirement is that at least one of the time values is 
/// > non-zero. The non-time values are simply printed as is. Other tools that 
/// > read this file format may have different requirements.
///
/// This struct implements the bodyfile format generated by TSK 3.x
/// 
pub struct Bodyfile3Line {
    md5: String,
    name: String,
    inode: String,
    mode_as_string: String,
    uid: i64,
    gid: i64,
    size: i64,
    atime: i64,
    mtime: i64,
    ctime: i64,
    crtime: i64,
}

impl Bodyfile3Line {
    /// create a new empty bodyfile line
    /// 
    /// # Example
    /// ```
    /// use bodyfile::Bodyfile3Line;
    /// 
    /// let bf = Bodyfile3Line::new();
    /// assert_eq!(bf.get_md5(), "0");
    /// assert_eq!(bf.get_name(), "");
    /// assert_eq!(bf.get_inode(), "0");
    /// assert_eq!(bf.get_mode(), "");
    /// assert_eq!(bf.get_uid(), 0);
    /// assert_eq!(bf.get_gid(), 0);
    /// assert_eq!(bf.get_size(), 0);
    /// assert_eq!(bf.get_atime(), -1);
    /// assert_eq!(bf.get_mtime(), -1);
    /// assert_eq!(bf.get_ctime(), -1);
    /// assert_eq!(bf.get_crtime(), -1);
    /// ```
    pub fn new() -> Self {
        Self {
            md5: "0".to_owned(),
            name: "".to_owned(),
            inode: "0".to_owned(),
            mode_as_string: "".to_owned(),
            uid: 0,
            gid: 0,
            size: 0,
            atime: -1,
            mtime: -1,
            ctime: -1,
            crtime: -1
        }
    }

    pub fn from_values(
        md5: String,
        name: String,
        inode: String,
        mode_as_string: String,
        uid: i64,
        gid: i64,
        size: i64,
        atime: i64,
        mtime: i64,
        ctime: i64,
        crtime: i64) -> Self {
        Self {
            md5,
            name,
            inode,
            mode_as_string,
            uid,
            gid,
            size,
            atime,
            mtime,
            ctime,
            crtime
        }
    }

    #[duplicate(
        method_name attribute_name attribute_type;
        [with_md5]    [md5]            [String];
        [with_name]   [name]           [String];
        [with_inode]  [inode]          [String];
        [with_mode]   [mode_as_string] [String];
        [with_uid]    [uid]            [i64];
        [with_gid]    [gid]            [i64];
        [with_size]   [size]           [i64];
        [with_atime]  [atime]          [i64];
        [with_mtine]  [mtime]          [i64];
        [with_crtime] [crtime]         [i64];
    )]
    pub fn method_name(mut self, attribute_name: attribute_type) -> Self {
        self.attribute_name = attribute_name;
        self
    }

    #[duplicate(
        method_name   attribute_name;
        [get_md5]    [md5];
        [get_name]   [name];
        [get_inode]  [inode];
        [get_mode]   [mode_as_string];
    )]
    pub fn method_name(&self) -> &str {
        &self.attribute_name
    }
    
    #[duplicate(
        method_name   attribute_name   attribute_type;
        [get_uid]    [uid]            [i64];
        [get_gid]    [gid]            [i64];
        [get_size]   [size]           [i64];
        [get_atime]  [atime]          [i64];
        [get_mtime]  [mtime]          [i64];
        [get_ctime]  [ctime]          [i64];
        [get_crtime] [crtime]         [i64];
    )]
    pub fn method_name(&self) -> attribute_type {
        self.attribute_name
    }
}

impl ToString for Bodyfile3Line {
    fn to_string(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.md5,
            self.name,
            self.inode,
            self.mode_as_string,
            self.uid,
            self.gid,
            self.size,
            self.atime,
            self.mtime,
            self.ctime,
            self.crtime
        )
    }
}