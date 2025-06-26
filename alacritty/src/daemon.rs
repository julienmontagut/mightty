use std::ffi::OsStr;
#[cfg(not(target_os = "macos"))]
use std::fs;
use std::io;
use std::process::{Command, Stdio};

use std::env;
use std::error::Error;
use std::os::unix::process::CommandExt;
use std::os::unix::io::RawFd;
use std::path::PathBuf;

use libc::pid_t;

#[cfg(target_os = "macos")]
use crate::macos;

/// Start a new process in the background.
pub fn spawn_daemon<I, S>(
    program: &str,
    args: I,
    master_fd: RawFd,
    shell_pid: u32,
) -> io::Result<()>
where
    I: IntoIterator<Item = S> + Copy,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(program);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    let working_directory = foreground_process_path(master_fd, shell_pid).ok();
    unsafe {
        command
            .pre_exec(move || {
                match libc::fork() {
                    -1 => return Err(io::Error::last_os_error()),
                    0 => (),
                    _ => libc::_exit(0),
                }

                // Copy foreground process' working directory, ignoring invalid paths.
                if let Some(working_directory) = working_directory.as_ref() {
                    let _ = env::set_current_dir(working_directory);
                }

                if libc::setsid() == -1 {
                    return Err(io::Error::last_os_error());
                }

                Ok(())
            })
            .spawn()?
            .wait()
            .map(|_| ())
    }
}

/// Get working directory of controlling process.
pub fn foreground_process_path(
    master_fd: RawFd,
    shell_pid: u32,
) -> Result<PathBuf, Box<dyn Error>> {
    let mut pid = unsafe { libc::tcgetpgrp(master_fd) };
    if pid < 0 {
        pid = shell_pid as pid_t;
    }

    #[cfg(not(any(target_os = "macos", target_os = "freebsd")))]
    let link_path = format!("/proc/{pid}/cwd");
    #[cfg(target_os = "freebsd")]
    let link_path = format!("/compat/linux/proc/{}/cwd", pid);

    #[cfg(not(target_os = "macos"))]
    let cwd = fs::read_link(link_path)?;

    #[cfg(target_os = "macos")]
    let cwd = macos::proc::cwd(pid)?;

    Ok(cwd)
}
