// Copyright 2018-2026 the Deno authors. MIT license.

use deno_core::op2;
use deno_core::v8;

/// Mirrors libuv's UV_ERRNO_MAP macro from deps/uv/include/uv.h.
/// Single source of truth for all UV error names and messages.
/// Invokes `$cb!(NAME, MESSAGE)` for each UV error.
macro_rules! uv_errno_map {
  ($cb:ident) => {
    $cb!(E2BIG, "argument list too long");
    $cb!(EACCES, "permission denied");
    $cb!(EADDRINUSE, "address already in use");
    $cb!(EADDRNOTAVAIL, "address not available");
    $cb!(EAFNOSUPPORT, "address family not supported");
    $cb!(EAGAIN, "resource temporarily unavailable");
    $cb!(EAI_ADDRFAMILY, "address family not supported");
    $cb!(EAI_AGAIN, "temporary failure");
    $cb!(EAI_BADFLAGS, "bad ai_flags value");
    $cb!(EAI_BADHINTS, "invalid value for hints");
    $cb!(EAI_CANCELED, "request canceled");
    $cb!(EAI_FAIL, "permanent failure");
    $cb!(EAI_FAMILY, "ai_family not supported");
    $cb!(EAI_MEMORY, "out of memory");
    $cb!(EAI_NODATA, "no address");
    $cb!(EAI_NONAME, "unknown node or service");
    $cb!(EAI_OVERFLOW, "argument buffer overflow");
    $cb!(EAI_PROTOCOL, "resolved protocol is unknown");
    $cb!(EAI_SERVICE, "service not available for socket type");
    $cb!(EAI_SOCKTYPE, "socket type not supported");
    $cb!(EALREADY, "connection already in progress");
    $cb!(EBADF, "bad file descriptor");
    $cb!(EBUSY, "resource busy or locked");
    $cb!(ECANCELED, "operation canceled");
    $cb!(ECHARSET, "invalid Unicode character");
    $cb!(ECONNABORTED, "software caused connection abort");
    $cb!(ECONNREFUSED, "connection refused");
    $cb!(ECONNRESET, "connection reset by peer");
    $cb!(EDESTADDRREQ, "destination address required");
    $cb!(EEXIST, "file already exists");
    $cb!(EFAULT, "bad address in system call argument");
    $cb!(EFBIG, "file too large");
    $cb!(EHOSTUNREACH, "host is unreachable");
    $cb!(EINTR, "interrupted system call");
    $cb!(EINVAL, "invalid argument");
    $cb!(EIO, "i/o error");
    $cb!(EISCONN, "socket is already connected");
    $cb!(EISDIR, "illegal operation on a directory");
    $cb!(ELOOP, "too many symbolic links encountered");
    $cb!(EMFILE, "too many open files");
    $cb!(EMSGSIZE, "message too long");
    $cb!(ENAMETOOLONG, "name too long");
    $cb!(ENETDOWN, "network is down");
    $cb!(ENETUNREACH, "network is unreachable");
    $cb!(ENFILE, "file table overflow");
    $cb!(ENOBUFS, "no buffer space available");
    $cb!(ENODEV, "no such device");
    $cb!(ENOENT, "no such file or directory");
    $cb!(ENOMEM, "not enough memory");
    $cb!(ENONET, "machine is not on the network");
    $cb!(ENOPROTOOPT, "protocol not available");
    $cb!(ENOSPC, "no space left on device");
    $cb!(ENOSYS, "function not implemented");
    $cb!(ENOTCONN, "socket is not connected");
    $cb!(ENOTDIR, "not a directory");
    $cb!(ENOTEMPTY, "directory not empty");
    $cb!(ENOTSOCK, "socket operation on non-socket");
    $cb!(ENOTSUP, "operation not supported on socket");
    $cb!(EOVERFLOW, "value too large for defined data type");
    $cb!(EPERM, "operation not permitted");
    $cb!(EPIPE, "broken pipe");
    $cb!(EPROTO, "protocol error");
    $cb!(EPROTONOSUPPORT, "protocol not supported");
    $cb!(EPROTOTYPE, "protocol wrong type for socket");
    $cb!(ERANGE, "result too large");
    $cb!(EROFS, "read-only file system");
    $cb!(ESHUTDOWN, "cannot send after transport endpoint shutdown");
    $cb!(ESPIPE, "invalid seek");
    $cb!(ESRCH, "no such process");
    $cb!(ETIMEDOUT, "connection timed out");
    $cb!(ETXTBSY, "text file is busy");
    $cb!(EXDEV, "cross-device link not permitted");
    $cb!(UNKNOWN, "unknown error");
    $cb!(EOF, "end of file");
    $cb!(ENXIO, "no such device or address");
    $cb!(EMLINK, "too many links");
    $cb!(EHOSTDOWN, "host is down");
    $cb!(EREMOTEIO, "remote I/O error");
    $cb!(ENOTTY, "inappropriate ioctl for device");
    $cb!(EFTYPE, "inappropriate file type or format");
    $cb!(EILSEQ, "illegal byte sequence");
    $cb!(ESOCKTNOSUPPORT, "socket type not supported");
    $cb!(ENODATA, "no data available");
    $cb!(EUNATCH, "protocol driver not attached");
    $cb!(ENOEXEC, "exec format error");
  };
}

// Per-platform value resolution macros.
// Each resolves a UV error name to its i32 numeric code, following the same
// logic as libuv's errno.h: use -(libc::NAME) when available, otherwise
// use libuv's hardcoded fallback value.

#[cfg(any(target_os = "linux", target_os = "android"))]
macro_rules! uv_error_code {
  (EAI_ADDRFAMILY) => {
    -3000
  };
  (EAI_AGAIN) => {
    -3001
  };
  (EAI_BADFLAGS) => {
    -3002
  };
  (EAI_BADHINTS) => {
    -3013
  };
  (EAI_CANCELED) => {
    -3003
  };
  (EAI_FAIL) => {
    -3004
  };
  (EAI_FAMILY) => {
    -3005
  };
  (EAI_MEMORY) => {
    -3006
  };
  (EAI_NODATA) => {
    -3007
  };
  (EAI_NONAME) => {
    -3008
  };
  (EAI_OVERFLOW) => {
    -3009
  };
  (EAI_PROTOCOL) => {
    -3014
  };
  (EAI_SERVICE) => {
    -3010
  };
  (EAI_SOCKTYPE) => {
    -3011
  };
  (ECHARSET) => {
    -4080
  };
  (UNKNOWN) => {
    -4094
  };
  (EOF) => {
    -4095
  };
  (EFTYPE) => {
    -4028
  };
  ($name:ident) => {
    -(libc::$name as i32)
  };
}

#[cfg(target_os = "macos")]
macro_rules! uv_error_code {
  (EAI_ADDRFAMILY) => {
    -3000
  };
  (EAI_AGAIN) => {
    -3001
  };
  (EAI_BADFLAGS) => {
    -3002
  };
  (EAI_BADHINTS) => {
    -3013
  };
  (EAI_CANCELED) => {
    -3003
  };
  (EAI_FAIL) => {
    -3004
  };
  (EAI_FAMILY) => {
    -3005
  };
  (EAI_MEMORY) => {
    -3006
  };
  (EAI_NODATA) => {
    -3007
  };
  (EAI_NONAME) => {
    -3008
  };
  (EAI_OVERFLOW) => {
    -3009
  };
  (EAI_PROTOCOL) => {
    -3014
  };
  (EAI_SERVICE) => {
    -3010
  };
  (EAI_SOCKTYPE) => {
    -3011
  };
  (ECHARSET) => {
    -4080
  };
  (UNKNOWN) => {
    -4094
  };
  (EOF) => {
    -4095
  };
  (ENONET) => {
    -4056
  };
  (EREMOTEIO) => {
    -4030
  };
  (EUNATCH) => {
    -4023
  };
  ($name:ident) => {
    -(libc::$name as i32)
  };
}

#[cfg(target_os = "freebsd")]
macro_rules! uv_error_code {
  (EAI_ADDRFAMILY) => {
    -3000
  };
  (EAI_AGAIN) => {
    -3001
  };
  (EAI_BADFLAGS) => {
    -3002
  };
  (EAI_BADHINTS) => {
    -3013
  };
  (EAI_CANCELED) => {
    -3003
  };
  (EAI_FAIL) => {
    -3004
  };
  (EAI_FAMILY) => {
    -3005
  };
  (EAI_MEMORY) => {
    -3006
  };
  (EAI_NODATA) => {
    -3007
  };
  (EAI_NONAME) => {
    -3008
  };
  (EAI_OVERFLOW) => {
    -3009
  };
  (EAI_PROTOCOL) => {
    -3014
  };
  (EAI_SERVICE) => {
    -3010
  };
  (EAI_SOCKTYPE) => {
    -3011
  };
  (ECHARSET) => {
    -4080
  };
  (UNKNOWN) => {
    -4094
  };
  (EOF) => {
    -4095
  };
  (ENONET) => {
    -4056
  };
  (EREMOTEIO) => {
    -4030
  };
  (EUNATCH) => {
    -4023
  };
  // FreeBSD: ENODATA only in C++ headers
  (ENODATA) => {
    -9919
  };
  ($name:ident) => {
    -(libc::$name as i32)
  };
}

#[cfg(target_os = "openbsd")]
macro_rules! uv_error_code {
  (EAI_ADDRFAMILY) => {
    -3000
  };
  (EAI_AGAIN) => {
    -3001
  };
  (EAI_BADFLAGS) => {
    -3002
  };
  (EAI_BADHINTS) => {
    -3013
  };
  (EAI_CANCELED) => {
    -3003
  };
  (EAI_FAIL) => {
    -3004
  };
  (EAI_FAMILY) => {
    -3005
  };
  (EAI_MEMORY) => {
    -3006
  };
  (EAI_NODATA) => {
    -3007
  };
  (EAI_NONAME) => {
    -3008
  };
  (EAI_OVERFLOW) => {
    -3009
  };
  (EAI_PROTOCOL) => {
    -3014
  };
  (EAI_SERVICE) => {
    -3010
  };
  (EAI_SOCKTYPE) => {
    -3011
  };
  (ECHARSET) => {
    -4080
  };
  (UNKNOWN) => {
    -4094
  };
  (EOF) => {
    -4095
  };
  (ENONET) => {
    -4056
  };
  (EREMOTEIO) => {
    -4030
  };
  (EUNATCH) => {
    -4023
  };
  (ENODATA) => {
    -4024
  };
  ($name:ident) => {
    -(libc::$name as i32)
  };
}

#[cfg(windows)]
macro_rules! uv_error_code {
  (E2BIG) => {
    -4093
  };
  (EACCES) => {
    -4092
  };
  (EADDRINUSE) => {
    -4091
  };
  (EADDRNOTAVAIL) => {
    -4090
  };
  (EAFNOSUPPORT) => {
    -4089
  };
  (EAGAIN) => {
    -4088
  };
  (EAI_ADDRFAMILY) => {
    -3000
  };
  (EAI_AGAIN) => {
    -3001
  };
  (EAI_BADFLAGS) => {
    -3002
  };
  (EAI_BADHINTS) => {
    -3013
  };
  (EAI_CANCELED) => {
    -3003
  };
  (EAI_FAIL) => {
    -3004
  };
  (EAI_FAMILY) => {
    -3005
  };
  (EAI_MEMORY) => {
    -3006
  };
  (EAI_NODATA) => {
    -3007
  };
  (EAI_NONAME) => {
    -3008
  };
  (EAI_OVERFLOW) => {
    -3009
  };
  (EAI_PROTOCOL) => {
    -3014
  };
  (EAI_SERVICE) => {
    -3010
  };
  (EAI_SOCKTYPE) => {
    -3011
  };
  (EALREADY) => {
    -4084
  };
  (EBADF) => {
    -4083
  };
  (EBUSY) => {
    -4082
  };
  (ECANCELED) => {
    -4081
  };
  (ECHARSET) => {
    -4080
  };
  (ECONNABORTED) => {
    -4079
  };
  (ECONNREFUSED) => {
    -4078
  };
  (ECONNRESET) => {
    -4077
  };
  (EDESTADDRREQ) => {
    -4076
  };
  (EEXIST) => {
    -4075
  };
  (EFAULT) => {
    -4074
  };
  (EFBIG) => {
    -4036
  };
  (EHOSTUNREACH) => {
    -4073
  };
  (EINTR) => {
    -4072
  };
  (EINVAL) => {
    -4071
  };
  (EIO) => {
    -4070
  };
  (EISCONN) => {
    -4069
  };
  (EISDIR) => {
    -4068
  };
  (ELOOP) => {
    -4067
  };
  (EMFILE) => {
    -4066
  };
  (EMSGSIZE) => {
    -4065
  };
  (ENAMETOOLONG) => {
    -4064
  };
  (ENETDOWN) => {
    -4063
  };
  (ENETUNREACH) => {
    -4062
  };
  (ENFILE) => {
    -4061
  };
  (ENOBUFS) => {
    -4060
  };
  (ENODEV) => {
    -4059
  };
  (ENOENT) => {
    -4058
  };
  (ENOMEM) => {
    -4057
  };
  (ENONET) => {
    -4056
  };
  (ENOPROTOOPT) => {
    -4035
  };
  (ENOSPC) => {
    -4055
  };
  (ENOSYS) => {
    -4054
  };
  (ENOTCONN) => {
    -4053
  };
  (ENOTDIR) => {
    -4052
  };
  (ENOTEMPTY) => {
    -4051
  };
  (ENOTSOCK) => {
    -4050
  };
  (ENOTSUP) => {
    -4049
  };
  (EOVERFLOW) => {
    -4026
  };
  (EPERM) => {
    -4048
  };
  (EPIPE) => {
    -4047
  };
  (EPROTO) => {
    -4046
  };
  (EPROTONOSUPPORT) => {
    -4045
  };
  (EPROTOTYPE) => {
    -4044
  };
  (ERANGE) => {
    -4034
  };
  (EROFS) => {
    -4043
  };
  (ESHUTDOWN) => {
    -4042
  };
  (ESPIPE) => {
    -4041
  };
  (ESRCH) => {
    -4040
  };
  (ETIMEDOUT) => {
    -4039
  };
  (ETXTBSY) => {
    -4038
  };
  (EXDEV) => {
    -4037
  };
  (UNKNOWN) => {
    -4094
  };
  (EOF) => {
    -4095
  };
  (ENXIO) => {
    -4033
  };
  (EMLINK) => {
    -4032
  };
  (EHOSTDOWN) => {
    -4031
  };
  (EREMOTEIO) => {
    -4030
  };
  (ENOTTY) => {
    -4029
  };
  (EFTYPE) => {
    -4028
  };
  (EILSEQ) => {
    -4027
  };
  (ESOCKTNOSUPPORT) => {
    -4025
  };
  (ENODATA) => {
    -4024
  };
  (EUNATCH) => {
    -4023
  };
  (ENOEXEC) => {
    -4022
  };
}

/// Returns the UV error map for the current platform as a v8::Map.
/// Keys are integer error codes, values are [name, message] arrays.
/// This mirrors Node's `process.binding('uv').getErrorMap()`.
#[op2]
pub fn op_node_uv_error_map<'s>(
  scope: &mut v8::PinScope<'s, '_>,
) -> v8::Local<'s, v8::Value> {
  let err_map = v8::Map::new(scope);

  macro_rules! insert_entry {
    ($name:ident, $msg:expr) => {
      let key = v8::Integer::new(scope, uv_error_code!($name)).into();
      let name_str = v8::String::new_external_onebyte_static(
        scope,
        stringify!($name).as_bytes(),
      )
      .unwrap()
      .into();
      let msg_str =
        v8::String::new_external_onebyte_static(scope, $msg.as_bytes())
          .unwrap()
          .into();
      let arr =
        v8::Array::new_with_elements(scope, &[name_str, msg_str]).into();
      err_map.set(scope, key, arr);
    };
  }

  uv_errno_map!(insert_entry);
  err_map.into()
}
