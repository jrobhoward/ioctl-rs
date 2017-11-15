extern crate libc;

use std::io;
use std::mem;
use std::os::unix::io::RawFd;

use libc::c_int;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use os::linux::*;

#[cfg(target_os = "macos")]
pub use os::macos::*;

#[cfg(target_os = "freebsd")]
pub use os::freebsd::*;

#[cfg(target_os = "openbsd")]
pub use os::openbsd::*;

mod os;


/// Put the terminal in exclusive mode.
pub fn tiocexcl(fd: RawFd) -> io::Result<()> {
    match unsafe { ioctl(fd, TIOCEXCL) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Disable exclusive mode.
pub fn tiocnxcl(fd: RawFd) -> io::Result<()> {
    match unsafe { ioctl(fd, TIOCNXCL) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Get the status of modem bits.
pub fn tiocmget(fd: RawFd) -> io::Result<c_int> {
    let mut bits: c_int = unsafe { mem::uninitialized() };

    match unsafe { ioctl(fd, TIOCMGET, &mut bits) } {
        0 => Ok(bits),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Set the status of modem bits.
pub fn tiocmset(fd: RawFd, bits: c_int) -> io::Result<()> {
    match unsafe { ioctl(fd, TIOCMSET, &bits) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Set the indicated modem bits.
pub fn tiocmbis(fd: RawFd, bits: c_int) -> io::Result<()> {
    match unsafe { ioctl(fd, TIOCMBIS, &bits) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Clear the indicated modem bits.
pub fn tiocmbic(fd: RawFd, bits: c_int) -> io::Result<()> {
    match unsafe { ioctl(fd, TIOCMBIC, &bits) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Query the current number of records in auditpipe
pub fn ap_get_len(fd: RawFd) -> io::Result<u32> {
    let result: u32 = 0;
    match unsafe { ioctl(fd, AUDITPIPE_GET_QLEN, &result) } {
        0 => Ok((result)),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Query the current max number of records that may be queued for reading in auditpipe
pub fn ap_get_qlimit(fd: RawFd) -> io::Result<u32> {
    let result: u32 = 0;
    match unsafe { ioctl(fd, AUDITPIPE_GET_QLIMIT, &result) } {
        0 => Ok((result)),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Set the current max number of records that may be queued for reading in auditpipe
pub fn ap_set_qlimit(fd: RawFd, new_limit: u32) -> io::Result<()> {
    match unsafe { ioctl(fd, AUDITPIPE_SET_QLIMIT, &new_limit) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Query the highest possible max number of records that may be queued for reading in auditpipe
pub fn ap_get_qlimit_max(fd: RawFd) -> io::Result<u32> {
    let result: u32 = 0;
    match unsafe { ioctl(fd, AUDITPIPE_GET_QLIMIT_MAX, &result) } {
        0 => Ok((result)),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Query the lowest possible max number of records that may be queued for reading in auditpipe
pub fn ap_get_qlimit_min(fd: RawFd) -> io::Result<u32> {
    let result: u32 = 0;
    match unsafe { ioctl(fd, AUDITPIPE_GET_QLIMIT_MIN, &result) } {
        0 => Ok((result)),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Flush all outstanding records in auditpipe
pub fn ap_flush(fd: RawFd) -> io::Result<()> {
    match unsafe { ioctl(fd, AUDITPIPE_FLUSH) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Query the max size of an individual
pub fn ap_get_maxauditdata_size(fd: RawFd) -> io::Result<u32> {
    let result: u32 = 0;
    match unsafe { ioctl(fd, AUDITPIPE_GET_MAXAUDITDATA, &result) } {
        0 => Ok((result)),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Get auditpipe drop count
pub fn ap_get_inserts(fd: RawFd) -> io::Result<(u64)> {
    let result: u64 = 0;
    match unsafe { ioctl(fd, AUDITPIPE_GET_INSERTS, &result) } {
        0 => Ok((result)),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Get auditpipe truncate count
pub fn ap_get_reads(fd: RawFd) -> io::Result<(u64)> {
    let result: u64 = 0;
    match unsafe { ioctl(fd, AUDITPIPE_GET_READS, &result) } {
        0 => Ok((result)),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Get auditpipe drop count
pub fn ap_get_drops(fd: RawFd) -> io::Result<(u64)> {
    let result: u64 = 0;
    match unsafe { ioctl(fd, AUDITPIPE_GET_DROPS, &result) } {
        0 => Ok((result)),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Get auditpipe truncate count
pub fn ap_get_truncates(fd: RawFd) -> io::Result<(u64)> {
    let result: u64 = 0;
    match unsafe { ioctl(fd, AUDITPIPE_GET_TRUNCATES, &result) } {
        0 => Ok((result)),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Set the auditpipe preselect mode
pub fn ap_set_preselect_mode(fd: RawFd) -> io::Result<()> {
    let preselect_mode: i32 = AUDITPIPE_PRESELECT_MODE_LOCAL;
    match unsafe { ioctl(fd, AUDITPIPE_SET_PRESELECT_MODE, &preselect_mode) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Set the auditpipe preselect flags
pub fn ap_set_preselect_flags(fd: RawFd, mask: u32) -> io::Result<()> {
    match unsafe { ioctl(fd, AUDITPIPE_SET_PRESELECT_FLAGS, &mask) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error()),
    }
}

/// Set the nonattr auditpipe preselect flags
pub fn ap_set_preselect_flags_na(fd: RawFd, mask: u32) -> io::Result<()> {
    match unsafe { ioctl(fd, AUDITPIPE_SET_PRESELECT_NAFLAGS, &mask) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error()),
    }
}
