use std;
use std::fmt;

#[derive(Clone, Copy, Debug)]
enum Errno {
    OK = 0,
    EPERM = 1,
    ENOENT = 2,
    ESRCH = 3,
    EINTR = 4,
    EIO = 5,
    ENXIO = 6,
    E2BIG = 7,
    ENOEXEC = 8,
    EBADF = 9,
    ECHILD = 10,
    EAGAIN = 11,
    ENOMEM = 12,
    EACCES = 13,
    EFAULT = 14,
    ENOTBLK = 15,
    EBUSY = 16,
    EEXIST = 17,
    EXDEV = 18,
    ENODEV = 19,
    ENOTDIR = 20,
    EISDIR = 21,
    EINVAL = 22,
    ENFILE = 23,
    EMFILE = 24,
    ENOTTY = 25,
    ETXTBSY = 26,
    EFBIG = 27,
    ENOSPC = 28,
    ESPIPE = 29,
    EROFS = 30,
    EMLINK = 31,
    EPIPE = 32,
    EDOM = 33,
    ERANGE = 34,
    EDEADLK = 35,
    ENAMETOOLONG = 36,
    ENOLCK = 37,
}

impl Errno {
    fn as_retval(&self) -> i32 {
        *self as i32
    }
}

impl Default for Errno {
    fn default() -> Errno {
        Errno::OK
    }
}

impl fmt::Display for Errno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})",
            *self as i32,
            match *self {
                Errno::OK => "Ok",
                Errno::EPERM =>  "Operation not permitted",
                Errno::ENOENT => "No such file or directory",
                Errno::ESRCH => "No such process",
                Errno::EINTR => "Interrupted system call",
                Errno::EIO => "I/O error",
                Errno::ENXIO => "No such device or address",
                Errno::E2BIG => "Argument list too long",
                Errno::ENOEXEC => "Exec format error",
                Errno::EBADF => "Bad file number",
                Errno::ECHILD => "No child processes",
                Errno::EAGAIN => "Try again",
                Errno::ENOMEM => "Out of memory",
                Errno::EACCES => "Permission denied",
                Errno::EFAULT => "Bad address",
                Errno::ENOTBLK => "Block device required",
                Errno::EBUSY => "Device or resource busy",
                Errno::EEXIST => "File exists",
                Errno::EXDEV => "Cross-device link",
                Errno::ENODEV => "No such device",
                Errno::ENOTDIR => "Not a directory",
                Errno::EISDIR => "Is a directory",
                Errno::EINVAL => "Invalid argument",
                Errno::ENFILE => "File table overflow",
                Errno::EMFILE => "Too many open files",
                Errno::ENOTTY => "Not a typewriter",
                Errno::ETXTBSY => "Text file busy",
                Errno::EFBIG => "File too large",
                Errno::ENOSPC => "No space left on device",
                Errno::ESPIPE => "Illegal seek",
                Errno::EROFS => "Read-only file system",
                Errno::EMLINK => "Too many links",
                Errno::EPIPE => "Broken pipe",
                Errno::EDOM => "Math argument out of domain of func",
                Errno::ERANGE => "Math result not representable",
                Errno::EDEADLK => "Resource deadlock would occur",
                Errno::ENAMETOOLONG => "File name too long",
                Errno::ENOLCK => "No record locks available",
                _ => "Unknown",
            })
    }
}
