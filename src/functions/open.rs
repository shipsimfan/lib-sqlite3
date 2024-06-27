use crate::SQLite3;
use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use crate::{SQLITE_CORRUPT, SQLITE_OK};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "sqlite3")]
extern "C" {
    /// This routine opens an SQLite database file as specified by the `filename` argument. The
    /// `filename` argument is interpreted as UTF-8. A database connection handle usually returned
    /// in `db`, even if an error occurs. The only exception is that if SQLite is unable to
    /// allocate memory to hold the [`SQLite3`] object, a [`null_mut`] will be written into `db`
    /// instead of a pointer to the [`SQLite3`] object. If the database is opened (and/or created)
    /// successfully, then [`SQLITE_OK`] is returned. Otherwise an error code is returned. The
    /// [`sqlite3_errmsg`] or [`sqlite3_errmsg16`] routines can be used to obtain an English
    /// language description of the error following a failure of the [`sqlite3_open`] routine.
    ///
    /// The default encoding will be UTF-8 for databases created using [`sqlite3_open`].
    ///
    /// Whether or not an error occurs when it is opened, resources associated with the database
    /// connection handle should be released by passing it to [`sqlite3_close`] when it is no
    /// longer required.
    ///
    /// If the filename is ":memory:", then a private, temporary in-memory database is created for
    /// the connection. This in-memory database will vanish when the database connection is closed.
    /// Future versions of SQLite might make use of additional special filenames that begin with
    /// ":" character. It is recommended that when a database filename actually does begin with a
    /// ":" character you should prefix the filename with a pathname such as "./" to avoid
    /// ambiguity.
    ///
    /// If the filename is an empty string, then a private, temporary on-disk database will be
    /// created. This private database will be automatically deleted as soon as the database
    /// connection is closed.
    ///
    /// # URI Filenames
    /// If URI filename interpretation is enabled, and the filename argument begins with "file:",
    /// then the filename is interpreted as a URI. URI filename interpretation is enabled if the
    /// [`SQLITE_CONFIG_URI`] option has been enabled globally with the [`sqlite3_config`] method
    /// or by the [`SQLITE_USE_URI`] compile-time option. URI filename interpretation is turned off
    /// by default, but future releases of SQLite might enable URI filename interpretation by
    /// default.
    ///
    /// URI filenames are parsed according to RFC 3986. If the URI contains an authority, then it
    /// must be either an empty string or the string "localhost". If the authority is not an empty
    /// string or "localhost", an error is returned to the caller. The fragment component of a URI,
    /// if present, is ignored.
    ///
    /// SQLite uses the path component of the URI as the name of the disk file which contains the
    /// database. If the path begins with a '/' character, then it is interpreted as an absolute
    /// path. If the path does not begin with a '/' (meaning that the authority section is omitted
    /// from the URI) then the path is interpreted as a relative path. On windows, the first
    /// component of an absolute path is a drive specification (e.g. "C:").
    ///
    /// The query component of a URI may contain parameters that are interpreted either by SQLite
    /// itself, or by a custom VFS implementation. SQLite and its built-in VFSes interpret the
    /// following query parameters:
    ///  * "vfs" - The "vfs" parameter may be used to specify the name of a VFS object that
    ///            provides the operating system interface that should be used to access the
    ///            database file on disk. If this option is set to an empty string the default VFS
    ///            object is used. Specifying an unknown VFS is an error.
    ///  * "mode" - The "mode" parameter may be set to either "ro", "rw", "rwc", or "memory".
    ///             Attempting to set it to any other value is an error. If "ro" is specified, then
    ///             the database is opened for read-only access, just as if the
    ///             [`SQLITE_OPEN_READONLY`] flag had been set in the third argument to
    ///             [`sqlite3_open_v2`]. If the mode option is set to "rw", then the database is
    ///             opened for read-write (but not create) access, as if [`SQLITE_OPEN_READWRITE`]
    ///             (but not [`SQLITE_OPEN_CREATE`]) had been set. Value "rwc" is equivalent to
    ///             setting both [`SQLITE_OPEN_READWRITE`] and [`SQLITE_OPEN_CREATE`]. If the mode
    ///             option is set to "memory" then a pure in-memory database that never reads or
    ///             writes from disk is used.
    ///  * "cache" - The "cache" parameter may be set to either "shared" or "private". Setting it
    ///              to "shared" is equivalent to setting the [`SQLITE_OPEN_SHAREDCACHE`] bit in
    ///              the `flags` argument passed to [`sqlite3_open_v2`]. Setting the cache
    ///              parameter to "private" is equivalent to setting the
    ///              [`SQLITE_OPEN_PRIVATECACHE`] bit.
    ///  * "psow" - The "psow" parameter indicates whether or not the powersafe overwrite property
    ///             does or does not apply to the storage media on which the database file resides.
    ///  * "nolock" - The "nolock" parameter is a boolean query parameter which if set disables
    ///               file locking in rollback journal modes. This is useful for accessing a
    ///               database on a filesystem that does not support locking. Caution: Database
    ///               corruption might result if two or more processes write to the same database
    ///               and any one of those processes uses nolock=1.
    ///  * "immutable" - The "immutable" parameter is a boolean query parameter that indicates that
    ///                  the database file is stored on read-only media. When immutable is set,
    ///                  SQLite assumes that the database file cannot be changed, even by a process
    ///                  with higher privilege, and so the database is opened read-only and all
    ///                  locking and change detection is disabled. Caution: Setting the immutable
    ///                  property on a database file that does in fact change can result in
    ///                  incorrect query results and/or [`SQLITE_CORRUPT`] errors. See also:
    ///                  [`SQLITE_IOCAP_IMMUTABLE`].
    ///
    /// Specifying an unknown parameter in the query component of a URI is not an error. Future
    /// versions of SQLite might understand additional query parameters.
    ///
    /// URI hexadecimal escape sequences (%HH) are supported within the path and query components
    /// of a URI. A hexadecimal escape sequence consists of a percent sign - "%" - followed by
    /// exactly two hexadecimal digits specifying an octet value. Before the path or query
    /// components of a URI filename are interpreted, they are encoded using UTF-8 and all
    /// hexadecimal escape sequences replaced by a single byte containing the corresponding octet.
    /// If this process generates an invalid UTF-8 encoding, the results are undefined.
    pub fn sqlite3_open(filename: *const c_char, db: *mut *mut SQLite3) -> c_int;
}
