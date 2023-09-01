use crate::c_int;

pub const __NSIG: c_int = 64;
pub const __RESERVED_SIGRT: c_int = 2;
pub const __SIGRTMIN: c_int = 32;
pub const __SIGRTMAX: c_int = __NSIG;

pub const NSIG: c_int = __NSIG + 1;
pub const SIGRTMIN: c_int = __oumalibc_current_sigrtmin();
pub const SIGRTMAX: c_int = __oumalibc_current_sigrtmax();

#[no_mangle]
pub const extern "C" fn __oumalibc_current_sigrtmin() -> c_int {
  __SIGRTMIN + __RESERVED_SIGRT
}

#[no_mangle]
pub const extern "C" fn __oumalibc_current_sigrtmax() -> c_int {
  __SIGRTMAX
}

pub const SYS_SIGLIST: [&str; 32] = [
  "Unknown signal 0",
  "Hangup",
  "Interrupt",
  "Quit",
  "Illegal instruction",
  "Trace/breakpoint trap",
  "Aborted",
  "Bus error",
  "Floating point exception",
  "Killed",
  "User defined signal 1",
  "Segmentation fault",
  "User defined signal 2",
  "Broken pipe",
  "Alarm clock",
  "Terminated",
  "Stack fault",
  "Child exited",
  "Continued",
  "Stopped (signal)",
  "Stopped",
  "Stopped (tty input)",
  "Stopped (tty output)",
  "Urgent I/O condition",
  "CPU time limit exceeded",
  "File size limit exceeded",
  "Virtual timer expired",
  "Profiling timer expired",
  "Window changed",
  "I/O possible",
  "Power failure",
  "Bad system call"
];
