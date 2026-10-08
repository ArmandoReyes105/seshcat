use std::env;
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

/// Looks up `exe` in the given PATH-style variable, returning the first match.
fn find_in_path(exe: &str, path_var: &OsStr) -> Option<PathBuf> {
    env::split_paths(path_var)
        .map(|dir| dir.join(exe))
        .find(|candidate| candidate.is_file())
}

/// Shell program used to run user commands: `pwsh` when installed, else
/// Windows PowerShell.
#[cfg(windows)]
pub fn program() -> OsString {
    let has_pwsh =
        env::var_os("PATH").is_some_and(|path| find_in_path("pwsh.exe", &path).is_some());

    if has_pwsh { "pwsh" } else { "powershell" }.into()
}

/// Shell program used to run user commands: `$SHELL`, falling back to `sh`.
#[cfg(unix)]
pub fn program() -> OsString {
    env::var_os("SHELL").unwrap_or_else(|| "sh".into())
}

#[cfg(windows)]
const COMMAND_ARGS: [&str; 2] = ["-NoLogo", "-Command"];

#[cfg(unix)]
const COMMAND_ARGS: [&str; 1] = ["-c"];

pub fn shell_command(line: &str, cwd: &Path) -> Command {
    let mut cmd = Command::new(program());
    cmd.args(COMMAND_ARGS).arg(line).current_dir(cwd);
    cmd
}

pub fn run_blocking(line: &str, cwd: &Path) -> io::Result<ExitStatus> {
    shell_command(line, cwd).status()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn find_in_path_finds_an_executable_in_a_listed_dir() -> Result<(), Box<dyn std::error::Error>>
    {
        let empty = TempDir::new()?;
        let bin = TempDir::new()?;
        let exe = bin.path().join("tool.exe");
        fs::write(&exe, "")?;
        let path_var = env::join_paths([empty.path(), bin.path()])?;

        assert_eq!(find_in_path("tool.exe", &path_var), Some(exe));

        Ok(())
    }

    #[test]
    fn find_in_path_returns_none_when_missing() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let path_var = env::join_paths([dir.path()])?;

        assert_eq!(find_in_path("tool.exe", &path_var), None);

        Ok(())
    }
}
