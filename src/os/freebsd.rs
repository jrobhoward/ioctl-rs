use libc::{c_int,c_ulong};

// socket

pub const FIOSETOWN: c_ulong = 0x8004667c;
pub const SIOCSPGRP: c_ulong = 0x80047308;
pub const FIOGETOWN: c_ulong = 0x4004667b;
pub const SIOCGPGRP: c_ulong = 0x40047309;

// termios

pub const TIOCEXCL: c_ulong = 0x2000740d;
pub const TIOCNXCL: c_ulong = 0x2000740e;
pub const TIOCSCTTY: c_ulong = 0x20007461;
pub const TIOCGPGRP: c_ulong = 0x40047477;
pub const TIOCSPGRP: c_ulong = 0x80047476;
pub const TIOCOUTQ: c_ulong = 0x40047473;
pub const TIOCSTI: c_ulong = 0x80017472;
pub const TIOCGWINSZ: c_ulong = 0x40087468;
pub const TIOCSWINSZ: c_ulong = 0x80087467;
pub const TIOCMGET: c_ulong = 0x4004746a;
pub const TIOCMBIS: c_ulong = 0x8004746c;
pub const TIOCMBIC: c_ulong = 0x8004746b;
pub const TIOCMSET: c_ulong = 0x8004746d;
pub const FIONREAD: c_ulong = 0x4004667f;
pub const TIOCCONS: c_ulong = 0x80047462;
pub const TIOCPKT: c_ulong = 0x80047470;
pub const FIONBIO: c_ulong = 0x8004667e;
pub const TIOCNOTTY: c_ulong = 0x20007471;
pub const TIOCSETD: c_ulong = 0x8004741b;
pub const TIOCGETD: c_ulong = 0x4004741a;
pub const FIONCLEX: c_ulong = 0x20006602;
pub const FIOCLEX: c_ulong = 0x20006601;
pub const FIOASYNC: c_ulong = 0x8004667d;

// sockios

pub const SIOCGIFCONF: c_ulong = 0xc0106924;
pub const SIOCGIFFLAGS: c_ulong = 0xc0206911;
pub const SIOCSIFFLAGS: c_ulong = 0x80206910;
pub const SIOCGIFADDR: c_ulong = 0xc0206921;
pub const SIOCSIFADDR: c_ulong = 0x8020690c;
pub const SIOCGIFDSTADDR: c_ulong = 0xc0206922;
pub const SIOCSIFDSTADDR: c_ulong = 0x8020690e;
pub const SIOCGIFBRDADDR: c_ulong = 0xc0206923;
pub const SIOCSIFBRDADDR: c_ulong = 0x80206913;
pub const SIOCGIFNETMASK: c_ulong = 0xc0206925;
pub const SIOCSIFNETMASK: c_ulong = 0x80206916;
pub const SIOCGIFMETRIC: c_ulong = 0xc0206917;
pub const SIOCSIFMETRIC: c_ulong = 0x80206918;
pub const SIOCGIFMTU: c_ulong = 0xc0206933;
pub const SIOCSIFMTU: c_ulong = 0x80206934;
pub const SIOCADDMULTI: c_ulong = 0x80206931;
pub const SIOCDELMULTI: c_ulong = 0x80206932;

// modem control lines

pub const TIOCM_LE: c_int = 0x00000001;
pub const TIOCM_DTR: c_int = 0x00000002;
pub const TIOCM_RTS: c_int = 0x00000004;
pub const TIOCM_ST: c_int = 0x00000008;
pub const TIOCM_SR: c_int = 0x00000010;
pub const TIOCM_CTS: c_int = 0x00000020;
pub const TIOCM_CAR: c_int = 0x00000040;
pub const TIOCM_CD: c_int = 0x00000040;
pub const TIOCM_RNG: c_int = 0x00000080;
pub const TIOCM_RI: c_int = 0x00000080;
pub const TIOCM_DSR: c_int = 0x00000100;

// auditpipe interface

pub const AUDITPIPE_PRESELECT_MODE_TRAIL: c_int = 0x00000001;
pub const AUDITPIPE_PRESELECT_MODE_LOCAL: c_int = 0x00000002;
pub const AUDITPIPE_GET_QLEN: c_ulong = 0x40044101;
pub const AUDITPIPE_GET_QLIMIT: c_ulong = 0x40044102;
pub const AUDITPIPE_SET_QLIMIT: c_ulong = 0x80044103;
pub const AUDITPIPE_GET_QLIMIT_MIN: c_ulong = 0x40044104;
pub const AUDITPIPE_GET_QLIMIT_MAX: c_ulong = 0x40044105;
pub const AUDITPIPE_GET_PRESELECT_FLAGS: c_ulong = 0x40084106;
pub const AUDITPIPE_SET_PRESELECT_FLAGS: c_ulong = 0x80084107;
pub const AUDITPIPE_GET_PRESELECT_NAFLAGS: c_ulong = 0x40084108;
pub const AUDITPIPE_SET_PRESELECT_NAFLAGS: c_ulong = 0x80084109;
pub const AUDITPIPE_GET_PRESELECT_AUID: c_ulong = 0x400c410a;
pub const AUDITPIPE_SET_PRESELECT_AUID: c_ulong = 0x800c410b;
pub const AUDITPIPE_DELETE_PRESELECT_AUID: c_ulong = 0x8004410c;
pub const AUDITPIPE_FLUSH_PRESELECT_AUID: c_ulong = 0x2000410d;
pub const AUDITPIPE_GET_PRESELECT_MODE: c_ulong = 0x4004410e;
pub const AUDITPIPE_SET_PRESELECT_MODE: c_ulong = 0x8004410f;
pub const AUDITPIPE_FLUSH: c_ulong = 0x20004110;
pub const AUDITPIPE_GET_MAXAUDITDATA: c_ulong = 0x40044111;
pub const AUDITPIPE_GET_INSERTS: c_ulong = 0x40084164;
pub const AUDITPIPE_GET_READS: c_ulong = 0x40084165;
pub const AUDITPIPE_GET_DROPS: c_ulong = 0x40084166;
pub const AUDITPIPE_GET_TRUNCATES: c_ulong = 0x40084167;


// auditpipe, /etc/security/audit_class
pub const ACLASS_INVALID_CLASS: c_ulong = 0x00000000;
pub const ACLASS_FILE_READ: c_ulong = 0x00000001;
pub const ACLASS_FILE_WRITE: c_ulong = 0x00000002;
pub const ACLASS_FILE_ATTR_ACCESS: c_ulong = 0x00000004;
pub const ACLASS_FILE_ADDR_MODIFY: c_ulong = 0x00000008;
pub const ACLASS_FILE_CREATE: c_ulong = 0x00000010;
pub const ACLASS_FILE_DELETE: c_ulong = 0x00000020;
pub const ACLASS_FILE_CLOSE: c_ulong = 0x00000040;
pub const ACLASS_PROCESS: c_ulong = 0x00000080;
pub const ACLASS_NETWORK: c_ulong = 0x00000100;
pub const ACLASS_IPC: c_ulong = 0x00000200;
pub const ACLASS_NON_ATTRIB: c_ulong = 0x00000400;
pub const ACLASS_ADMINISTRATIVE: c_ulong = 0x00000800;
pub const ACLASS_LOGIN_LOGOUT: c_ulong = 0x00001000;
pub const ACLASS_AUTH: c_ulong = 0x00002000;
pub const ACLASS_APPLICATUON: c_ulong = 0x00004000;
pub const ACLASS_IOCTL: c_ulong = 0x20000000;
pub const ACLASS_EXEC: c_ulong = 0x40000000;
pub const ACLASS_MISC: c_ulong = 0x80000000;
pub const ACLASS_ALL: c_ulong = 0xffffffff;


extern "C" {
    pub fn ioctl(fildes: c_int, request: c_ulong, ...) -> c_int;
}
