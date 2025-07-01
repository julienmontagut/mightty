use std::ffi::OsStr;
use std::fs;
use std::io;
use std::process::{Command, Stdio};

use std::env;
use std::error::Error;
use std::os::unix::io::RawFd;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;

use libc::pid_t;

use crate::config::defaults;
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

    let cwd = if defaults::USES_PROC_FS {
        let link_path = defaults::proc_cwd_path(pid);
        fs::read_link(link_path)?
    } else {
        #[cfg(target_os = "macos")]
        {
            macos::proc::cwd(pid)?
        }
        #[cfg(not(target_os = "macos"))]
        {
            return Err("Platform not supported for process working directory detection".into());
        }
    };

    Ok(cwd)
}
