use std::env;
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::services::shell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalHost {
    WindowsTerminal,
    Unsupported,
}

pub fn detect() -> TerminalHost {
    if env::var_os("WT_SESSION").is_some() {
        TerminalHost::WindowsTerminal
    } else {
        TerminalHost::Unsupported
    }
}

impl TerminalHost {
    pub fn open_tab(&self, line: &str, cwd: &Path) -> io::Result<()> {
        match self {
            TerminalHost::WindowsTerminal => {
                Command::new("wt")
                    .args(wt_args(line, cwd, &shell::program()))
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()?;
                Ok(())
            }
            TerminalHost::Unsupported => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "new tab not supported in this terminal",
            )),
        }
    }
}

pub fn wt_args(line: &str, cwd: &Path, shell: &OsStr) -> Vec<OsString> {
    vec![
        OsString::from("-w"),
        "0".into(),
        "new-tab".into(),
        "-d".into(),
        cwd.as_os_str().to_owned(),
        shell.to_owned(),
        "-NoExit".into(),
        "-Command".into(),
        line.replace(';', "\\;").into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wt_args_escapes_semicolons() {
        let dir = Path::new("C:\\work");

        let args = wt_args("git add . ; git status", dir, OsStr::new("powershell"));

        assert_eq!(
            args.last(),
            Some(&OsString::from("git add . \\; git status"))
        );
    }

    #[test]
    fn wt_args_uses_the_given_shell() {
        let args = wt_args("dir", Path::new("C:\\work"), OsStr::new("powershell"));

        assert!(args.contains(&OsString::from("powershell")));
        assert!(!args.contains(&OsString::from("pwsh")));
    }

    #[test]
    fn wt_args_keeps_the_working_directory() {
        let args = wt_args("dir", Path::new("C:\\work"), OsStr::new("pwsh"));

        let d = args
            .iter()
            .position(|a| a == "-d")
            .expect("-d flag is present");
        assert_eq!(args[d + 1], OsString::from("C:\\work"));
    }
}
