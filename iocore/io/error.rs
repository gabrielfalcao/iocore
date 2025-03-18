#[allow(unused)]
impl Code {
    #[rustfmt::skip]
    pub fn to_i32(self) -> i32 {
        match self {
            Code::EPERM                          => 0x40000001,	/* Operation not permitted */
            Code::ENOENT                         => 0x40000002,	/* No such file or directory */
            Code::ESRCH                          => 0x40000003,	/* No such process */
            Code::EINTR                          => 0x40000004,	/* Interrupted system call */
            Code::EIO                            => 0x40000005,	/* Input/output error */
            Code::ENXIO                          => 0x40000006,	/* No such device or address */
            Code::E2BIG                          => 0x40000007,	/* Argument list too long */
            Code::ENOEXEC                        => 0x40000008,	/* Exec format error */
            Code::EBADF                          => 0x40000009,	/* Bad file descriptor */
            Code::ECHILD                         => 0x4000000a,	/* No child processes */
            Code::EDEADLK                        => 0x4000000b,	/* Resource deadlock avoided */
            Code::ENOMEM                         => 0x4000000c,	/* Cannot allocate memory */
            Code::EACCES                         => 0x4000000d,	/* Permission denied */
            Code::EFAULT                         => 0x4000000e,	/* Bad address */
            Code::ENOTBLK                        => 0x4000000f,	/* Block device required */
            Code::EBUSY                          => 0x40000010,	/* Device or resource busy */
            Code::EEXIST                         => 0x40000011,	/* File exists */
            Code::EXDEV                          => 0x40000012,	/* Invalid cross-device link */
            Code::ENODEV                         => 0x40000013,	/* No such device */
            Code::ENOTDIR                        => 0x40000014,	/* Not a directory */
            Code::EISDIR                         => 0x40000015,	/* Is a directory */
            Code::EINVAL                         => 0x40000016,	/* Invalid argument */
            Code::EMFILE                         => 0x40000018,	/* Too many open files */
            Code::ENFILE                         => 0x40000017,	/* Too many open files in system */
            Code::ENOTTY                         => 0x40000019,	/* Inappropriate ioctl for device */
            Code::ETXTBSY                        => 0x4000001a,	/* Text file busy */
            Code::EFBIG                          => 0x4000001b,	/* File too large */
            Code::ENOSPC                         => 0x4000001c,	/* No space left on device */
            Code::ESPIPE                         => 0x4000001d,	/* Illegal seek */
            Code::EROFS                          => 0x4000001e,	/* Read-only file system */
            Code::EMLINK                         => 0x4000001f,	/* Too many links */
            Code::EPIPE                          => 0x40000020,	/* Broken pipe */
            Code::EDOM                           => 0x40000021,	/* Numerical argument out of domain */
            Code::ERANGE                         => 0x40000022,	/* Numerical result out of range */
            Code::EAGAIN                         => 0x40000023,	/* Resource temporarily unavailable */
            Code::EINPROGRESS                    => 0x40000024,	/* Operation now in progress */
            Code::EALREADY                       => 0x40000025,	/* Operation already in progress */
            Code::ENOTSOCK                       => 0x40000026,	/* Socket operation on non-socket */
            Code::EMSGSIZE                       => 0x40000028,	/* Message too long */
            Code::EPROTOTYPE                     => 0x40000029,	/* Protocol wrong type for socket */
            Code::ENOPROTOOPT                    => 0x4000002a,	/* Protocol not available */
            Code::EPROTONOSUPPORT                => 0x4000002b,	/* Protocol not supported */
            Code::ESOCKTNOSUPPORT                => 0x4000002c,	/* Socket type not supported */
            Code::EOPNOTSUPP                     => 0x4000002d,	/* Operation not supported */
            Code::EPFNOSUPPORT                   => 0x4000002e,	/* Protocol family not supported */
            Code::EAFNOSUPPORT                   => 0x4000002f,	/* Address family not supported by protocol */
            Code::EADDRINUSE                     => 0x40000030,	/* Address already in use */
            Code::EADDRNOTAVAIL                  => 0x40000031,	/* Cannot assign requested address */
            Code::ENETDOWN                       => 0x40000032,	/* Network is down */
            Code::ENETUNREACH                    => 0x40000033,	/* Network is unreachable */
            Code::ENETRESET                      => 0x40000034,	/* Network dropped connection on reset */
            Code::ECONNABORTED                   => 0x40000035,	/* Software caused connection abort */
            Code::ECONNRESET                     => 0x40000036,	/* Connection reset by peer */
            Code::ENOBUFS                        => 0x40000037,	/* No buffer space available */
            Code::EISCONN                        => 0x40000038,	/* Transport endpoint is already connected */
            Code::ENOTCONN                       => 0x40000039,	/* Transport endpoint is not connected */
            Code::EDESTADDRREQ                   => 0x40000027,	/* Destination address required */
            Code::ESHUTDOWN                      => 0x4000003a,	/* Cannot send after transport endpoint shutdown */
            Code::ETOOMANYREFS                   => 0x4000003b,	/* Too many references: cannot splice */
            Code::ETIMEDOUT                      => 0x4000003c,	/* Connection timed out */
            Code::ECONNREFUSED                   => 0x4000003d,	/* Connection refused */
            Code::ELOOP                          => 0x4000003e,	/* Too many levels of symbolic links */
            Code::ENAMETOOLONG                   => 0x4000003f,	/* File name too long */
            Code::EHOSTDOWN                      => 0x40000040,	/* Host is down */
            Code::EHOSTUNREACH                   => 0x40000041,	/* No route to host */
            Code::ENOTEMPTY                      => 0x40000042,	/* Directory not empty */
            Code::EPROCLIM                       => 0x40000043,	/* Too many processes */
            Code::EUSERS                         => 0x40000044,	/* Too many users */
            Code::EDQUOT                         => 0x40000045,	/* Disk quota exceeded */
            Code::ESTALE                         => 0x40000046,	/* Stale file handle */
            Code::EREMOTE                        => 0x40000047,	/* Object is remote */
            Code::EBADRPC                        => 0x40000048,	/* RPC struct is bad */
            Code::ERPCMISMATCH                   => 0x40000049,	/* RPC version wrong */
            Code::EPROGUNAVAIL                   => 0x4000004a,	/* RPC program not available */
            Code::EPROGMISMATCH                  => 0x4000004b,	/* RPC program version wrong */
            Code::EPROCUNAVAIL                   => 0x4000004c,	/* RPC bad procedure for program */
            Code::ENOLCK                         => 0x4000004d,	/* No locks available */
            Code::EFTYPE                         => 0x4000004f,	/* Inappropriate file type or format */
            Code::EAUTH                          => 0x40000050,	/* Authentication error */
            Code::ENEEDAUTH                      => 0x40000051,	/* Need authenticator */
            Code::ENOSYS                         => 0x4000004e,	/* Function not implemented */
            Code::ELIBEXEC                       => 0x40000053,	/* Cannot exec a shared library directly */
            Code::ENOTSUP                        => 0x40000076,	/* Not supported */
            Code::EILSEQ                         => 0x4000006a,	/* Invalid or incomplete multibyte or wide character */
            Code::EBACKGROUND                    => 0x40000064,	/* Inappropriate operation for background process */
            Code::EDIED                          => 0x40000065,	/* Translator died */
            Code::ED                             => 0x40000066,	/* ? */
            Code::EGREGIOUS                      => 0x40000067,	/* You really blew it this time */
            Code::EIEIO                          => 0x40000068,	/* Computer bought the farm */
            Code::EGRATUITOUS                    => 0x40000069,	/* Gratuitous error */
            Code::EBADMSG                        => 0x4000006b,	/* Bad message */
            Code::EIDRM                          => 0x4000006c,	/* Identifier removed */
            Code::EMULTIHOP                      => 0x4000006d,	/* Multihop attempted */
            Code::ENODATA                        => 0x4000006e,	/* No data available */
            Code::ENOLINK                        => 0x4000006f,	/* Link has been severed */
            Code::ENOMSG                         => 0x40000070,	/* No message of desired type */
            Code::ENOSR                          => 0x40000071,	/* Out of streams resources */
            Code::ENOSTR                         => 0x40000072,	/* Device not a stream */
            Code::EOVERFLOW                      => 0x40000073,	/* Value too large for defined data type */
            Code::EPROTO                         => 0x40000074,	/* Protocol error */
            Code::ETIME                          => 0x40000075,	/* Timer expired */
            Code::ECANCELED                      => 0x40000077,	/* Operation canceled */
            Code::EOWNERDEAD                     => 0x40000078,	/* Owner died */
            Code::ENOTRECOVERABLE                => 0x40000079,	/* State not recoverable */
            Code::EMACH_SEND_IN_PROGRESS         => 0x10000001,
            Code::EMACH_SEND_INVALID_DATA        => 0x10000002,
            Code::EMACH_SEND_INVALID_DEST        => 0x10000003,
            Code::EMACH_SEND_TIMED_OUT           => 0x10000004,
            Code::EMACH_SEND_WILL_NOTIFY         => 0x10000005,
            Code::EMACH_SEND_NOTIFY_IN_PROGRESS  => 0x10000006,
            Code::EMACH_SEND_INTERRUPTED         => 0x10000007,
            Code::EMACH_SEND_MSG_TOO_SMALL       => 0x10000008,
            Code::EMACH_SEND_INVALID_REPLY       => 0x10000009,
            Code::EMACH_SEND_INVALID_RIGHT       => 0x1000000a,
            Code::EMACH_SEND_INVALID_NOTIFY      => 0x1000000b,
            Code::EMACH_SEND_INVALID_MEMORY      => 0x1000000c,
            Code::EMACH_SEND_NO_BUFFER           => 0x1000000d,
            Code::EMACH_SEND_NO_NOTIFY           => 0x1000000e,
            Code::EMACH_SEND_INVALID_TYPE        => 0x1000000f,
            Code::EMACH_SEND_INVALID_HEADER      => 0x10000010,
            Code::EMACH_RCV_IN_PROGRESS          => 0x10004001,
            Code::EMACH_RCV_INVALID_NAME         => 0x10004002,
            Code::EMACH_RCV_TIMED_OUT            => 0x10004003,
            Code::EMACH_RCV_TOO_LARGE            => 0x10004004,
            Code::EMACH_RCV_INTERRUPTED          => 0x10004005,
            Code::EMACH_RCV_PORT_CHANGED         => 0x10004006,
            Code::EMACH_RCV_INVALID_NOTIFY       => 0x10004007,
            Code::EMACH_RCV_INVALID_DATA         => 0x10004008,
            Code::EMACH_RCV_PORT_DIED            => 0x10004009,
            Code::EMACH_RCV_IN_SET               => 0x1000400a,
            Code::EMACH_RCV_HEADER_ERROR         => 0x1000400b,
            Code::EMACH_RCV_BODY_ERROR           => 0x1000400c,
            Code::EKERN_INVALID_ADDRESS          => 1,
            Code::EKERN_PROTECTION_FAILURE       => 2,
            Code::EKERN_NO_SPACE                 => 3,
            Code::EKERN_INVALID_ARGUMENT         => 4,
            Code::EKERN_FAILURE                  => 5,
            Code::EKERN_RESOURCE_SHORTAGE        => 6,
            Code::EKERN_NOT_RECEIVER             => 7,
            Code::EKERN_NO_ACCESS                => 8,
            Code::EKERN_MEMORY_FAILURE           => 9,
            Code::EKERN_MEMORY_ERROR             => 10,
            Code::EKERN_NOT_IN_SET               => 12,
            Code::EKERN_NAME_EXISTS              => 13,
            Code::EKERN_ABORTED                  => 14,
            Code::EKERN_INVALID_NAME             => 15,
            Code::EKERN_INVALID_TASK             => 16,
            Code::EKERN_INVALID_RIGHT            => 17,
            Code::EKERN_INVALID_VALUE            => 18,
            Code::EKERN_UREFS_OVERFLOW           => 19,
            Code::EKERN_INVALID_CAPABILITY       => 20,
            Code::EKERN_RIGHT_EXISTS             => 21,
            Code::EKERN_INVALID_HOST             => 22,
            Code::EKERN_MEMORY_PRESENT           => 23,
            Code::EKERN_WRITE_PROTECTION_FAILURE => 24,
            Code::EKERN_TERMINATED               => 26,
            Code::EKERN_TIMEDOUT                 => 27,
            Code::EKERN_INTERRUPTED              => 28,
            Code::EMIG_TYPE_ERROR                => -300,	/* client type check failure */
            Code::EMIG_REPLY_MISMATCH            => -301,	/* wrong reply message ID */
            Code::EMIG_REMOTE_ERROR              => -302,	/* server detected error */
            Code::EMIG_BAD_ID                    => -303,	/* bad request message ID */
            Code::EMIG_BAD_ARGUMENTS             => -304,	/* server type check failure */
            Code::EMIG_NO_REPLY                  => -305,	/* no reply should be sent */
            Code::EMIG_EXCEPTION                 => -306,	/* server raised exception */
            Code::EMIG_ARRAY_TOO_LARGE           => -307,	/* array not large enough */
            Code::EMIG_SERVER_DIED               => -308,	/* server died */
            Code::EMIG_DESTROY_REQUEST           => -309,	/* destroy request with no reply */
            Code::ED_IO_ERROR                    => 2500,	/* hardware IO error */
            Code::ED_WOULD_BLOCK                 => 2501,	/* would block, but D_NOWAIT set */
            Code::ED_NO_SUCH_DEVICE              => 2502,	/* no such device */
            Code::ED_ALREADY_OPEN                => 2503,	/* exclusive-use device already open */
            Code::ED_DEVICE_DOWN                 => 2504,	/* device has been shut down */
            Code::ED_INVALID_OPERATION           => 2505,	/* bad operation for device */
            Code::ED_INVALID_RECNUM              => 2506,	/* invalid record (block) number */
            Code::ED_INVALID_SIZE                => 2507,	/* invalid IO size */
            Code::ED_NO_MEMORY                   => 2508,	/* memory allocation failure */
            Code::ED_READ_ONLY                   => 2509,	/* device cannot be written to */
        }
    }

    pub fn to_u64(self) -> u64 {
        let n = self.to_i32();
        (if n < 0 { n * -1 } else { n }) as u64
    }

    pub fn to_u8(self) -> u8 {
        (self.to_u64() & !0x40000000 & !0x10000000) as u8
    }

    pub fn desc(self) -> &'static str {
        match self {
            Code::EPERM => "Operation not permitted",
            Code::ENOENT => "No such file or directory",
            Code::ESRCH => "No such process",
            Code::EINTR => "Interrupted system call",
            Code::EIO => "Input/output error",
            Code::ENXIO => "No such device or address",
            Code::E2BIG => "Argument list too long",
            Code::ENOEXEC => "Exec format error",
            Code::EBADF => "Bad file descriptor",
            Code::ECHILD => "No child processes",
            Code::EDEADLK => "Resource deadlock avoided",
            Code::ENOMEM => "Cannot allocate memory",
            Code::EACCES => "Permission denied",
            Code::EFAULT => "Bad address",
            Code::ENOTBLK => "Block device required",
            Code::EBUSY => "Device or resource busy",
            Code::EEXIST => "File exists",
            Code::EXDEV => "Invalid cross-device link",
            Code::ENODEV => "No such device",
            Code::ENOTDIR => "Not a directory",
            Code::EISDIR => "Is a directory",
            Code::EINVAL => "Invalid argument",
            Code::EMFILE => "Too many open files",
            Code::ENFILE => "Too many open files in system",
            Code::ENOTTY => "Inappropriate ioctl for device",
            Code::ETXTBSY => "Text file busy",
            Code::EFBIG => "File too large",
            Code::ENOSPC => "No space left on device",
            Code::ESPIPE => "Illegal seek",
            Code::EROFS => "Read-only file system",
            Code::EMLINK => "Too many links",
            Code::EPIPE => "Broken pipe",
            Code::EDOM => "Numerical argument out of domain",
            Code::ERANGE => "Numerical result out of range",
            Code::EAGAIN => "Resource temporarily unavailable",
            Code::EINPROGRESS => "Operation now in progress",
            Code::EALREADY => "Operation already in progress",
            Code::ENOTSOCK => "Socket operation on non-socket",
            Code::EMSGSIZE => "Message too long",
            Code::EPROTOTYPE => "Protocol wrong type for socket",
            Code::ENOPROTOOPT => "Protocol not available",
            Code::EPROTONOSUPPORT => "Protocol not supported",
            Code::ESOCKTNOSUPPORT => "Socket type not supported",
            Code::EOPNOTSUPP => "Operation not supported",
            Code::EPFNOSUPPORT => "Protocol family not supported",
            Code::EAFNOSUPPORT => "Address family not supported by protocol",
            Code::EADDRINUSE => "Address already in use",
            Code::EADDRNOTAVAIL => "Cannot assign requested address",
            Code::ENETDOWN => "Network is down",
            Code::ENETUNREACH => "Network is unreachable",
            Code::ENETRESET => "Network dropped connection on reset",
            Code::ECONNABORTED => "Software caused connection abort",
            Code::ECONNRESET => "Connection reset by peer",
            Code::ENOBUFS => "No buffer space available",
            Code::EISCONN => "Transport endpoint is already connected",
            Code::ENOTCONN => "Transport endpoint is not connected",
            Code::EDESTADDRREQ => "Destination address required",
            Code::ESHUTDOWN => "Cannot send after transport endpoint shutdown",
            Code::ETOOMANYREFS => "Too many references: cannot splice",
            Code::ETIMEDOUT => "Connection timed out",
            Code::ECONNREFUSED => "Connection refused",
            Code::ELOOP => "Too many levels of symbolic links",
            Code::ENAMETOOLONG => "File name too long",
            Code::EHOSTDOWN => "Host is down",
            Code::EHOSTUNREACH => "No route to host",
            Code::ENOTEMPTY => "Directory not empty",
            Code::EPROCLIM => "Too many processes",
            Code::EUSERS => "Too many users",
            Code::EDQUOT => "Disk quota exceeded",
            Code::ESTALE => "Stale file handle",
            Code::EREMOTE => "Object is remote",
            Code::EBADRPC => "RPC struct is bad",
            Code::ERPCMISMATCH => "RPC version wrong",
            Code::EPROGUNAVAIL => "RPC program not available",
            Code::EPROGMISMATCH => "RPC program version wrong",
            Code::EPROCUNAVAIL => "RPC bad procedure for program",
            Code::ENOLCK => "No locks available",
            Code::EFTYPE => "Inappropriate file type or format",
            Code::EAUTH => "Authentication error",
            Code::ENEEDAUTH => "Need authenticator",
            Code::ENOSYS => "Function not implemented",
            Code::ELIBEXEC => "Cannot exec a shared library directly",
            Code::ENOTSUP => "Not supported",
            Code::EILSEQ => "Invalid or incomplete multibyte or wide character",
            Code::EBACKGROUND => "Inappropriate operation for background process",
            Code::EDIED => "Translator died",
            Code::ED => "?",
            Code::EGREGIOUS => "You really blew it this time",
            Code::EIEIO => "Computer bought the farm",
            Code::EGRATUITOUS => "Gratuitous error",
            Code::EBADMSG => "Bad message",
            Code::EIDRM => "Identifier removed",
            Code::EMULTIHOP => "Multihop attempted",
            Code::ENODATA => "No data available",
            Code::ENOLINK => "Link has been severed",
            Code::ENOMSG => "No message of desired type",
            Code::ENOSR => "Out of streams resources",
            Code::ENOSTR => "Device not a stream",
            Code::EOVERFLOW => "Value too large for defined data type",
            Code::EPROTO => "Protocol error",
            Code::ETIME => "Timer expired",
            Code::ECANCELED => "Operation canceled",
            Code::EOWNERDEAD => "Owner died",
            Code::ENOTRECOVERABLE => "State not recoverable",
            Code::EMACH_SEND_IN_PROGRESS => "emach send in progress",
            Code::EMACH_SEND_INVALID_DATA => "emach send invalid data",
            Code::EMACH_SEND_INVALID_DEST => "emach send invalid dest",
            Code::EMACH_SEND_TIMED_OUT => "emach send timed out",
            Code::EMACH_SEND_WILL_NOTIFY => "emach send will notify",
            Code::EMACH_SEND_NOTIFY_IN_PROGRESS => "emach send notify in progress",
            Code::EMACH_SEND_INTERRUPTED => "emach send interrupted",
            Code::EMACH_SEND_MSG_TOO_SMALL => "emach send msg too small",
            Code::EMACH_SEND_INVALID_REPLY => "emach send invalid reply",
            Code::EMACH_SEND_INVALID_RIGHT => "emach send invalid right",
            Code::EMACH_SEND_INVALID_NOTIFY => "emach send invalid notify",
            Code::EMACH_SEND_INVALID_MEMORY => "emach send invalid memory",
            Code::EMACH_SEND_NO_BUFFER => "emach send no buffer",
            Code::EMACH_SEND_NO_NOTIFY => "emach send no notify",
            Code::EMACH_SEND_INVALID_TYPE => "emach send invalid type",
            Code::EMACH_SEND_INVALID_HEADER => "emach send invalid header",
            Code::EMACH_RCV_IN_PROGRESS => "emach rcv in progress",
            Code::EMACH_RCV_INVALID_NAME => "emach rcv invalid name",
            Code::EMACH_RCV_TIMED_OUT => "emach rcv timed out",
            Code::EMACH_RCV_TOO_LARGE => "emach rcv too large",
            Code::EMACH_RCV_INTERRUPTED => "emach rcv interrupted",
            Code::EMACH_RCV_PORT_CHANGED => "emach rcv port changed",
            Code::EMACH_RCV_INVALID_NOTIFY => "emach rcv invalid notify",
            Code::EMACH_RCV_INVALID_DATA => "emach rcv invalid data",
            Code::EMACH_RCV_PORT_DIED => "emach rcv port died",
            Code::EMACH_RCV_IN_SET => "emach rcv in set",
            Code::EMACH_RCV_HEADER_ERROR => "emach rcv header error",
            Code::EMACH_RCV_BODY_ERROR => "emach rcv body error",
            Code::EKERN_INVALID_ADDRESS => "ekern invalid address",
            Code::EKERN_PROTECTION_FAILURE => "ekern protection failure",
            Code::EKERN_NO_SPACE => "ekern no space",
            Code::EKERN_INVALID_ARGUMENT => "ekern invalid argument",
            Code::EKERN_FAILURE => "ekern failure",
            Code::EKERN_RESOURCE_SHORTAGE => "ekern resource shortage",
            Code::EKERN_NOT_RECEIVER => "ekern not receiver",
            Code::EKERN_NO_ACCESS => "ekern no access",
            Code::EKERN_MEMORY_FAILURE => "ekern memory failure",
            Code::EKERN_MEMORY_ERROR => "ekern memory error",
            Code::EKERN_NOT_IN_SET => "ekern not in set",
            Code::EKERN_NAME_EXISTS => "ekern name exists",
            Code::EKERN_ABORTED => "ekern aborted",
            Code::EKERN_INVALID_NAME => "ekern invalid name",
            Code::EKERN_INVALID_TASK => "ekern invalid task",
            Code::EKERN_INVALID_RIGHT => "ekern invalid right",
            Code::EKERN_INVALID_VALUE => "ekern invalid value",
            Code::EKERN_UREFS_OVERFLOW => "ekern urefs overflow",
            Code::EKERN_INVALID_CAPABILITY => "ekern invalid capability",
            Code::EKERN_RIGHT_EXISTS => "ekern right exists",
            Code::EKERN_INVALID_HOST => "ekern invalid host",
            Code::EKERN_MEMORY_PRESENT => "ekern memory present",
            Code::EKERN_WRITE_PROTECTION_FAILURE => "ekern write protection failure",
            Code::EKERN_TERMINATED => "ekern terminated",
            Code::EKERN_TIMEDOUT => "ekern timedout",
            Code::EKERN_INTERRUPTED => "ekern interrupted",
            Code::EMIG_TYPE_ERROR => "client type check failure",
            Code::EMIG_REPLY_MISMATCH => "wrong reply message ID",
            Code::EMIG_REMOTE_ERROR => "server detected error",
            Code::EMIG_BAD_ID => "bad request message ID",
            Code::EMIG_BAD_ARGUMENTS => "server type check failure",
            Code::EMIG_NO_REPLY => "no reply should be sent",
            Code::EMIG_EXCEPTION => "server raised exception",
            Code::EMIG_ARRAY_TOO_LARGE => "array not large enough",
            Code::EMIG_SERVER_DIED => "server died",
            Code::EMIG_DESTROY_REQUEST => "destroy request with no reply",
            Code::ED_IO_ERROR => "hardware IO error",
            Code::ED_WOULD_BLOCK => "would block, but D_NOWAIT set",
            Code::ED_NO_SUCH_DEVICE => "no such device",
            Code::ED_ALREADY_OPEN => "exclusive-use device already open",
            Code::ED_DEVICE_DOWN => "device has been shut down",
            Code::ED_INVALID_OPERATION => "bad operation for device",
            Code::ED_INVALID_RECNUM => "invalid record (block) number",
            Code::ED_INVALID_SIZE => "invalid IO size",
            Code::ED_NO_MEMORY => "memory allocation failure",
            Code::ED_READ_ONLY => "device cannot be written to",
        }
    }

    pub fn from_i32(code: i32) -> Option<Code> {
        Some(match code {
            0x40000001 => Code::EPERM,
            0x40000002 => Code::ENOENT,
            0x40000003 => Code::ESRCH,
            0x40000004 => Code::EINTR,
            0x40000005 => Code::EIO,
            0x40000006 => Code::ENXIO,
            0x40000007 => Code::E2BIG,
            0x40000008 => Code::ENOEXEC,
            0x40000009 => Code::EBADF,
            0x4000000A => Code::ECHILD,
            0x4000000B => Code::EDEADLK,
            0x4000000C => Code::ENOMEM,
            0x4000000D => Code::EACCES,
            0x4000000E => Code::EFAULT,
            0x4000000F => Code::ENOTBLK,
            0x40000010 => Code::EBUSY,
            0x40000011 => Code::EEXIST,
            0x40000012 => Code::EXDEV,
            0x40000013 => Code::ENODEV,
            0x40000014 => Code::ENOTDIR,
            0x40000015 => Code::EISDIR,
            0x40000016 => Code::EINVAL,
            0x40000018 => Code::EMFILE,
            0x40000017 => Code::ENFILE,
            0x40000019 => Code::ENOTTY,
            0x4000001A => Code::ETXTBSY,
            0x4000001B => Code::EFBIG,
            0x4000001C => Code::ENOSPC,
            0x4000001D => Code::ESPIPE,
            0x4000001E => Code::EROFS,
            0x4000001F => Code::EMLINK,
            0x40000020 => Code::EPIPE,
            0x40000021 => Code::EDOM,
            0x40000022 => Code::ERANGE,
            0x40000023 => Code::EAGAIN,
            0x40000024 => Code::EINPROGRESS,
            0x40000025 => Code::EALREADY,
            0x40000026 => Code::ENOTSOCK,
            0x40000028 => Code::EMSGSIZE,
            0x40000029 => Code::EPROTOTYPE,
            0x4000002A => Code::ENOPROTOOPT,
            0x4000002B => Code::EPROTONOSUPPORT,
            0x4000002C => Code::ESOCKTNOSUPPORT,
            0x4000002D => Code::EOPNOTSUPP,
            0x4000002E => Code::EPFNOSUPPORT,
            0x4000002F => Code::EAFNOSUPPORT,
            0x40000030 => Code::EADDRINUSE,
            0x40000031 => Code::EADDRNOTAVAIL,
            0x40000032 => Code::ENETDOWN,
            0x40000033 => Code::ENETUNREACH,
            0x40000034 => Code::ENETRESET,
            0x40000035 => Code::ECONNABORTED,
            0x40000036 => Code::ECONNRESET,
            0x40000037 => Code::ENOBUFS,
            0x40000038 => Code::EISCONN,
            0x40000039 => Code::ENOTCONN,
            0x40000027 => Code::EDESTADDRREQ,
            0x4000003A => Code::ESHUTDOWN,
            0x4000003B => Code::ETOOMANYREFS,
            0x4000003C => Code::ETIMEDOUT,
            0x4000003D => Code::ECONNREFUSED,
            0x4000003E => Code::ELOOP,
            0x4000003F => Code::ENAMETOOLONG,
            0x40000040 => Code::EHOSTDOWN,
            0x40000041 => Code::EHOSTUNREACH,
            0x40000042 => Code::ENOTEMPTY,
            0x40000043 => Code::EPROCLIM,
            0x40000044 => Code::EUSERS,
            0x40000045 => Code::EDQUOT,
            0x40000046 => Code::ESTALE,
            0x40000047 => Code::EREMOTE,
            0x40000048 => Code::EBADRPC,
            0x40000049 => Code::ERPCMISMATCH,
            0x4000004A => Code::EPROGUNAVAIL,
            0x4000004B => Code::EPROGMISMATCH,
            0x4000004C => Code::EPROCUNAVAIL,
            0x4000004D => Code::ENOLCK,
            0x4000004F => Code::EFTYPE,
            0x40000050 => Code::EAUTH,
            0x40000051 => Code::ENEEDAUTH,
            0x4000004E => Code::ENOSYS,
            0x40000053 => Code::ELIBEXEC,
            0x40000076 => Code::ENOTSUP,
            0x4000006A => Code::EILSEQ,
            0x40000064 => Code::EBACKGROUND,
            0x40000065 => Code::EDIED,
            0x40000066 => Code::ED,
            0x40000067 => Code::EGREGIOUS,
            0x40000068 => Code::EIEIO,
            0x40000069 => Code::EGRATUITOUS,
            0x4000006B => Code::EBADMSG,
            0x4000006C => Code::EIDRM,
            0x4000006D => Code::EMULTIHOP,
            0x4000006E => Code::ENODATA,
            0x4000006F => Code::ENOLINK,
            0x40000070 => Code::ENOMSG,
            0x40000071 => Code::ENOSR,
            0x40000072 => Code::ENOSTR,
            0x40000073 => Code::EOVERFLOW,
            0x40000074 => Code::EPROTO,
            0x40000075 => Code::ETIME,
            0x40000077 => Code::ECANCELED,
            0x40000078 => Code::EOWNERDEAD,
            0x40000079 => Code::ENOTRECOVERABLE,
            0x10000001 => Code::EMACH_SEND_IN_PROGRESS,
            0x10000002 => Code::EMACH_SEND_INVALID_DATA,
            0x10000003 => Code::EMACH_SEND_INVALID_DEST,
            0x10000004 => Code::EMACH_SEND_TIMED_OUT,
            0x10000005 => Code::EMACH_SEND_WILL_NOTIFY,
            0x10000006 => Code::EMACH_SEND_NOTIFY_IN_PROGRESS,
            0x10000007 => Code::EMACH_SEND_INTERRUPTED,
            0x10000008 => Code::EMACH_SEND_MSG_TOO_SMALL,
            0x10000009 => Code::EMACH_SEND_INVALID_REPLY,
            0x1000000A => Code::EMACH_SEND_INVALID_RIGHT,
            0x1000000B => Code::EMACH_SEND_INVALID_NOTIFY,
            0x1000000C => Code::EMACH_SEND_INVALID_MEMORY,
            0x1000000D => Code::EMACH_SEND_NO_BUFFER,
            0x1000000E => Code::EMACH_SEND_NO_NOTIFY,
            0x1000000F => Code::EMACH_SEND_INVALID_TYPE,
            0x10000010 => Code::EMACH_SEND_INVALID_HEADER,
            0x10004001 => Code::EMACH_RCV_IN_PROGRESS,
            0x10004002 => Code::EMACH_RCV_INVALID_NAME,
            0x10004003 => Code::EMACH_RCV_TIMED_OUT,
            0x10004004 => Code::EMACH_RCV_TOO_LARGE,
            0x10004005 => Code::EMACH_RCV_INTERRUPTED,
            0x10004006 => Code::EMACH_RCV_PORT_CHANGED,
            0x10004007 => Code::EMACH_RCV_INVALID_NOTIFY,
            0x10004008 => Code::EMACH_RCV_INVALID_DATA,
            0x10004009 => Code::EMACH_RCV_PORT_DIED,
            0x1000400A => Code::EMACH_RCV_IN_SET,
            0x1000400B => Code::EMACH_RCV_HEADER_ERROR,
            0x1000400C => Code::EMACH_RCV_BODY_ERROR,
            1 => Code::EKERN_INVALID_ADDRESS,
            2 => Code::EKERN_PROTECTION_FAILURE,
            3 => Code::EKERN_NO_SPACE,
            4 => Code::EKERN_INVALID_ARGUMENT,
            5 => Code::EKERN_FAILURE,
            6 => Code::EKERN_RESOURCE_SHORTAGE,
            7 => Code::EKERN_NOT_RECEIVER,
            8 => Code::EKERN_NO_ACCESS,
            9 => Code::EKERN_MEMORY_FAILURE,
            10 => Code::EKERN_MEMORY_ERROR,
            12 => Code::EKERN_NOT_IN_SET,
            13 => Code::EKERN_NAME_EXISTS,
            14 => Code::EKERN_ABORTED,
            15 => Code::EKERN_INVALID_NAME,
            16 => Code::EKERN_INVALID_TASK,
            17 => Code::EKERN_INVALID_RIGHT,
            18 => Code::EKERN_INVALID_VALUE,
            19 => Code::EKERN_UREFS_OVERFLOW,
            20 => Code::EKERN_INVALID_CAPABILITY,
            21 => Code::EKERN_RIGHT_EXISTS,
            22 => Code::EKERN_INVALID_HOST,
            23 => Code::EKERN_MEMORY_PRESENT,
            24 => Code::EKERN_WRITE_PROTECTION_FAILURE,
            26 => Code::EKERN_TERMINATED,
            27 => Code::EKERN_TIMEDOUT,
            28 => Code::EKERN_INTERRUPTED,
            -300 => Code::EMIG_TYPE_ERROR,
            -301 => Code::EMIG_REPLY_MISMATCH,
            -302 => Code::EMIG_REMOTE_ERROR,
            -303 => Code::EMIG_BAD_ID,
            -304 => Code::EMIG_BAD_ARGUMENTS,
            -305 => Code::EMIG_NO_REPLY,
            -306 => Code::EMIG_EXCEPTION,
            -307 => Code::EMIG_ARRAY_TOO_LARGE,
            -308 => Code::EMIG_SERVER_DIED,
            -309 => Code::EMIG_DESTROY_REQUEST,
            2500 => Code::ED_IO_ERROR,
            2501 => Code::ED_WOULD_BLOCK,
            2502 => Code::ED_NO_SUCH_DEVICE,
            2503 => Code::ED_ALREADY_OPEN,
            2504 => Code::ED_DEVICE_DOWN,
            2505 => Code::ED_INVALID_OPERATION,
            2506 => Code::ED_INVALID_RECNUM,
            2507 => Code::ED_INVALID_SIZE,
            2508 => Code::ED_NO_MEMORY,
            2509 => Code::ED_READ_ONLY,
            _ => return None,
        })
    }

    pub fn from_error_kind(kind: std::io::ErrorKind) -> Option<Code> {
        Some(match kind {
            std::io::ErrorKind::AddrInUse => Code::EADDRINUSE,
            std::io::ErrorKind::AddrNotAvailable => Code::EADDRNOTAVAIL,
            std::io::ErrorKind::AlreadyExists => Code::EEXIST,
            std::io::ErrorKind::ArgumentListTooLong => Code::E2BIG,
            std::io::ErrorKind::BrokenPipe => Code::EPIPE,
            std::io::ErrorKind::ConnectionAborted => Code::ECONNABORTED,
            std::io::ErrorKind::ConnectionRefused => Code::ECONNREFUSED,
            std::io::ErrorKind::ConnectionReset => Code::ENETRESET,
            std::io::ErrorKind::CrossesDevices => Code::EXDEV,
            std::io::ErrorKind::Deadlock => Code::EDEADLK,
            std::io::ErrorKind::DirectoryNotEmpty => Code::ENOTEMPTY,
            std::io::ErrorKind::ExecutableFileBusy => Code::EBUSY, // ETXTBSY
            std::io::ErrorKind::FileTooLarge => Code::EFBIG,
            std::io::ErrorKind::FilesystemLoop => Code::ELOOP,
            std::io::ErrorKind::QuotaExceeded => Code::EDQUOT,
            std::io::ErrorKind::HostUnreachable => Code::EHOSTUNREACH,
            std::io::ErrorKind::InProgress => Code::EINPROGRESS,
            std::io::ErrorKind::Interrupted => Code::EINTR,
            std::io::ErrorKind::InvalidData => Code::EINVAL,
            std::io::ErrorKind::InvalidFilename => Code::EMSGSIZE,
            std::io::ErrorKind::InvalidInput => Code::EINVAL,
            std::io::ErrorKind::IsADirectory => Code::EISDIR,
            std::io::ErrorKind::NetworkDown => Code::ENETDOWN,
            std::io::ErrorKind::NetworkUnreachable => Code::ENETUNREACH,
            std::io::ErrorKind::NotADirectory => Code::ENOTDIR,
            std::io::ErrorKind::NotConnected => Code::ENOTCONN,
            std::io::ErrorKind::NotFound => Code::ENOENT,
            std::io::ErrorKind::NotSeekable => Code::ESPIPE,
            std::io::ErrorKind::OutOfMemory => Code::ENOMEM,
            std::io::ErrorKind::PermissionDenied => Code::EACCES,
            std::io::ErrorKind::ReadOnlyFilesystem => Code::EROFS,
            std::io::ErrorKind::ResourceBusy => Code::EBUSY,
            std::io::ErrorKind::StaleNetworkFileHandle => Code::ENETUNREACH,
            std::io::ErrorKind::StorageFull => Code::EDQUOT,
            std::io::ErrorKind::TimedOut => Code::ETIMEDOUT,
            std::io::ErrorKind::TooManyLinks => Code::EMLINK,
            std::io::ErrorKind::UnexpectedEof => Code::ED_INVALID_SIZE,
            std::io::ErrorKind::Unsupported => Code::EOPNOTSUPP,
            std::io::ErrorKind::WouldBlock => Code::ED_WOULD_BLOCK,
            std::io::ErrorKind::WriteZero => Code::ED_INVALID_SIZE,
            _ => return None,
        })
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Code {
            EPERM                          ,	/* Operation not permitted */
            ENOENT                         ,	/* No such file or directory */
            ESRCH                          ,	/* No such process */
            EINTR                          ,	/* Interrupted system call */
            EIO                            ,	/* Input/output error */
            ENXIO                          ,	/* No such device or address */
            E2BIG                          ,	/* Argument list too long */
            ENOEXEC                        ,	/* Exec format error */
            EBADF                          ,	/* Bad file descriptor */
            ECHILD                         ,	/* No child processes */
            EDEADLK                        ,	/* Resource deadlock avoided */
            ENOMEM                         ,	/* Cannot allocate memory */
            EACCES                         ,	/* Permission denied */
            EFAULT                         ,	/* Bad address */
            ENOTBLK                        ,	/* Block device required */
            EBUSY                          ,	/* Device or resource busy */
            EEXIST                         ,	/* File exists */
            EXDEV                          ,	/* Invalid cross-device link */
            ENODEV                         ,	/* No such device */
            ENOTDIR                        ,	/* Not a directory */
            EISDIR                         ,	/* Is a directory */
            EINVAL                         ,	/* Invalid argument */
            EMFILE                         ,	/* Too many open files */
            ENFILE                         ,	/* Too many open files in system */
            ENOTTY                         ,	/* Inappropriate ioctl for device */
            ETXTBSY                        ,	/* Text file busy */
            EFBIG                          ,	/* File too large */
            ENOSPC                         ,	/* No space left on device */
            ESPIPE                         ,	/* Illegal seek */
            EROFS                          ,	/* Read-only file system */
            EMLINK                         ,	/* Too many links */
            EPIPE                          ,	/* Broken pipe */
            EDOM                           ,	/* Numerical argument out of domain */
            ERANGE                         ,	/* Numerical result out of range */
            EAGAIN                         ,	/* Resource temporarily unavailable */
            EINPROGRESS                    ,	/* Operation now in progress */
            EALREADY                       ,	/* Operation already in progress */
            ENOTSOCK                       ,	/* Socket operation on non-socket */
            EMSGSIZE                       ,	/* Message too long */
            EPROTOTYPE                     ,	/* Protocol wrong type for socket */
            ENOPROTOOPT                    ,	/* Protocol not available */
            EPROTONOSUPPORT                ,	/* Protocol not supported */
            ESOCKTNOSUPPORT                ,	/* Socket type not supported */
            EOPNOTSUPP                     ,	/* Operation not supported */
            EPFNOSUPPORT                   ,	/* Protocol family not supported */
            EAFNOSUPPORT                   ,	/* Address family not supported by protocol */
            EADDRINUSE                     ,	/* Address already in use */
            EADDRNOTAVAIL                  ,	/* Cannot assign requested address */
            ENETDOWN                       ,	/* Network is down */
            ENETUNREACH                    ,	/* Network is unreachable */
            ENETRESET                      ,	/* Network dropped connection on reset */
            ECONNABORTED                   ,	/* Software caused connection abort */
            ECONNRESET                     ,	/* Connection reset by peer */
            ENOBUFS                        ,	/* No buffer space available */
            EISCONN                        ,	/* Transport endpoint is already connected */
            ENOTCONN                       ,	/* Transport endpoint is not connected */
            EDESTADDRREQ                   ,	/* Destination address required */
            ESHUTDOWN                      ,	/* Cannot send after transport endpoint shutdown */
            ETOOMANYREFS                   ,	/* Too many references: cannot splice */
            ETIMEDOUT                      ,	/* Connection timed out */
            ECONNREFUSED                   ,	/* Connection refused */
            ELOOP                          ,	/* Too many levels of symbolic links */
            ENAMETOOLONG                   ,	/* File name too long */
            EHOSTDOWN                      ,	/* Host is down */
            EHOSTUNREACH                   ,	/* No route to host */
            ENOTEMPTY                      ,	/* Directory not empty */
            EPROCLIM                       ,	/* Too many processes */
            EUSERS                         ,	/* Too many users */
            EDQUOT                         ,	/* Disk quota exceeded */
            ESTALE                         ,	/* Stale file handle */
            EREMOTE                        ,	/* Object is remote */
            EBADRPC                        ,	/* RPC struct is bad */
            ERPCMISMATCH                   ,	/* RPC version wrong */
            EPROGUNAVAIL                   ,	/* RPC program not available */
            EPROGMISMATCH                  ,	/* RPC program version wrong */
            EPROCUNAVAIL                   ,	/* RPC bad procedure for program */
            ENOLCK                         ,	/* No locks available */
            EFTYPE                         ,	/* Inappropriate file type or format */
            EAUTH                          ,	/* Authentication error */
            ENEEDAUTH                      ,	/* Need authenticator */
            ENOSYS                         ,	/* Function not implemented */
            ELIBEXEC                       ,	/* Cannot exec a shared library directly */
            ENOTSUP                        ,	/* Not supported */
            EILSEQ                         ,	/* Invalid or incomplete multibyte or wide character */
            EBACKGROUND                    ,	/* Inappropriate operation for background process */
            EDIED                          ,	/* Translator died */
            ED                             ,	/* ? */
            EGREGIOUS                      ,	/* You really blew it this time */
            EIEIO                          ,	/* Computer bought the farm */
            EGRATUITOUS                    ,	/* Gratuitous error */
            EBADMSG                        ,	/* Bad message */
            EIDRM                          ,	/* Identifier removed */
            EMULTIHOP                      ,	/* Multihop attempted */
            ENODATA                        ,	/* No data available */
            ENOLINK                        ,	/* Link has been severed */
            ENOMSG                         ,	/* No message of desired type */
            ENOSR                          ,	/* Out of streams resources */
            ENOSTR                         ,	/* Device not a stream */
            EOVERFLOW                      ,	/* Value too large for defined data type */
            EPROTO                         ,	/* Protocol error */
            ETIME                          ,	/* Timer expired */
            ECANCELED                      ,	/* Operation canceled */
            EOWNERDEAD                     ,	/* Owner died */
            ENOTRECOVERABLE                ,	/* State not recoverable */
            EMACH_SEND_IN_PROGRESS         ,
            EMACH_SEND_INVALID_DATA        ,
            EMACH_SEND_INVALID_DEST        ,
            EMACH_SEND_TIMED_OUT           ,
            EMACH_SEND_WILL_NOTIFY         ,
            EMACH_SEND_NOTIFY_IN_PROGRESS  ,
            EMACH_SEND_INTERRUPTED         ,
            EMACH_SEND_MSG_TOO_SMALL       ,
            EMACH_SEND_INVALID_REPLY       ,
            EMACH_SEND_INVALID_RIGHT       ,
            EMACH_SEND_INVALID_NOTIFY      ,
            EMACH_SEND_INVALID_MEMORY      ,
            EMACH_SEND_NO_BUFFER           ,
            EMACH_SEND_NO_NOTIFY           ,
            EMACH_SEND_INVALID_TYPE        ,
            EMACH_SEND_INVALID_HEADER      ,
            EMACH_RCV_IN_PROGRESS          ,
            EMACH_RCV_INVALID_NAME         ,
            EMACH_RCV_TIMED_OUT            ,
            EMACH_RCV_TOO_LARGE            ,
            EMACH_RCV_INTERRUPTED          ,
            EMACH_RCV_PORT_CHANGED         ,
            EMACH_RCV_INVALID_NOTIFY       ,
            EMACH_RCV_INVALID_DATA         ,
            EMACH_RCV_PORT_DIED            ,
            EMACH_RCV_IN_SET               ,
            EMACH_RCV_HEADER_ERROR         ,
            EMACH_RCV_BODY_ERROR           ,
            EKERN_INVALID_ADDRESS          ,
            EKERN_PROTECTION_FAILURE       ,
            EKERN_NO_SPACE                 ,
            EKERN_INVALID_ARGUMENT         ,
            EKERN_FAILURE                  ,
            EKERN_RESOURCE_SHORTAGE        ,
            EKERN_NOT_RECEIVER             ,
            EKERN_NO_ACCESS                ,
            EKERN_MEMORY_FAILURE           ,
            EKERN_MEMORY_ERROR             ,
            EKERN_NOT_IN_SET               ,
            EKERN_NAME_EXISTS              ,
            EKERN_ABORTED                  ,
            EKERN_INVALID_NAME             ,
            EKERN_INVALID_TASK             ,
            EKERN_INVALID_RIGHT            ,
            EKERN_INVALID_VALUE            ,
            EKERN_UREFS_OVERFLOW           ,
            EKERN_INVALID_CAPABILITY       ,
            EKERN_RIGHT_EXISTS             ,
            EKERN_INVALID_HOST             ,
            EKERN_MEMORY_PRESENT           ,
            EKERN_WRITE_PROTECTION_FAILURE ,
            EKERN_TERMINATED               ,
            EKERN_TIMEDOUT                 ,
            EKERN_INTERRUPTED              ,
            EMIG_TYPE_ERROR                ,	/* client type check failure */
            EMIG_REPLY_MISMATCH            ,	/* wrong reply message ID */
            EMIG_REMOTE_ERROR              ,	/* server detected error */
            EMIG_BAD_ID                    ,	/* bad request message ID */
            EMIG_BAD_ARGUMENTS             ,	/* server type check failure */
            EMIG_NO_REPLY                  ,	/* no reply should be sent */
            EMIG_EXCEPTION                 ,	/* server raised exception */
            EMIG_ARRAY_TOO_LARGE           ,	/* array not large enough */
            EMIG_SERVER_DIED               ,	/* server died */
            EMIG_DESTROY_REQUEST           ,	/* destroy request with no reply */
            ED_IO_ERROR                    ,	/* hardware IO error */
            ED_WOULD_BLOCK                 ,	/* would block, but D_NOWAIT set */
            ED_NO_SUCH_DEVICE              ,	/* no such device */
            ED_ALREADY_OPEN                ,	/* exclusive-use device already open */
            ED_DEVICE_DOWN                 ,	/* device has been shut down */
            ED_INVALID_OPERATION           ,	/* bad operation for device */
            ED_INVALID_RECNUM              ,	/* invalid record (block) number */
            ED_INVALID_SIZE                ,	/* invalid IO size */
            ED_NO_MEMORY                   ,	/* memory allocation failure */
            ED_READ_ONLY                   ,	/* device cannot be written to */
}
