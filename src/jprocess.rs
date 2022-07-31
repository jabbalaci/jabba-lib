//! # process
//!
//! Working with external commands.
//!
//! - call an external command and get its exit code, stdout, and stderr
//! - call an external command (and see its output on the stdout)

use shlex;

use std::process;

/// Stores process information: exit code, stdout, stderr.
#[allow(dead_code)]
#[derive(Debug)]
pub struct ProcStat {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

impl ProcStat {
    /// Returns a copy of the output.
    ///
    /// Use it if you want to avoid value moving.
    pub fn output(&self) -> String {
        self.stdout.clone()
    }

    /// Trims the trailing whitespaces from the output.
    pub fn trimmed_output(&self) -> String {
        self.stdout.trim_end().to_string()
    }
}

/// Executes an external command and gets its exit code, stdout and stderr.
///
/// The three values are returned in a `ProcStat` structure.
///
/// The command must be a simple command with some optional arguments.
/// Pipes, redirections are not allowed.
///
/// # Examples
///
/// ```
/// let commands = vec![
///     r#"python -c "print('Hello Rust!')""#,
///     "python3 --version",
///     "date",
/// ];
///
/// for cmd in commands.iter() {
///     let stat = jabba_lib::jprocess::get_exitcode_stdout_stderr(cmd).unwrap();
///     println!("{:?}", stat);
/// }
///
/// let date = jabba_lib::jprocess::get_exitcode_stdout_stderr("rustc --version")
///     .unwrap()
///     .trimmed_output(); // no trailing whitespaces
/// println!("{:?}", date);
/// ```
///
/// # Sample Output
///
/// ```text
/// ProcStat { exit_code: 0, stdout: "Hello Rust!\n", stderr: "" }
/// ProcStat { exit_code: 0, stdout: "Python 3.10.5\n", stderr: "" }
/// ProcStat { exit_code: 0, stdout: "2022. júl. 31., vasárnap, 12:29:23 CEST\n", stderr: "" }
/// "rustc 1.62.1 (e092d0b6b 2022-07-16)"
/// ```
pub fn get_exitcode_stdout_stderr(cmd: &str) -> Option<ProcStat> {
    let parts = shlex::split(cmd).unwrap_or_else(|| panic!("cannot parse command {:?}", cmd));
    let head = &parts[0];
    let tail = &parts[1..];

    let mut p = process::Command::new(head);
    p.args(tail);
    let p = p
        .output()
        .unwrap_or_else(|_| panic!("failed to execute {:?}", cmd));

    let result = ProcStat {
        exit_code: p.status.code()?,
        stdout: String::from_utf8_lossy(&p.stdout).to_string(),
        stderr: String::from_utf8_lossy(&p.stderr).to_string(),
    };

    Some(result)
}

/// Executes an external command.
///
/// The command's output goes to stdout (i.e., not captured).
/// Similar to Python's `os.system()`.
///
/// The command must be a simple command with some optional arguments.
/// Pipes, redirections are not allowed.
///
/// # Examples
///
/// ```
/// let cmd = "ls -al";
/// jabba_lib::jprocess::exec_cmd(cmd);
/// ```
///
/// # Sample Output
///
/// ```text
/// total 60
/// drwxr-xr-x 8 jabba jabba 4096 júl   31 07.06 .
/// drwxr-xr-x 6 jabba jabba 4096 júl   30 13.44 ..
/// -rw-r--r-- 1 jabba jabba 5303 júl   31 11.57 Cargo.lock
/// -rw-r--r-- 1 jabba jabba  646 júl   31 11.57 Cargo.toml
/// drwxr-xr-x 2 jabba jabba 4096 júl   31 11.56 examples
/// drwxr-xr-x 8 jabba jabba 4096 júl   31 16.32 .git
/// -rw-r--r-- 1 jabba jabba   34 júl   18 09.20 .gitignore
/// -rw-r--r-- 1 jabba jabba 1067 júl    9 20.40 LICENSE
/// -rw-r--r-- 1 jabba jabba  133 júl   18 10.44 Makefile
/// -rw-r--r-- 1 jabba jabba 2828 júl   30 14.40 README.md
/// drwxr-xr-x 2 jabba jabba 4096 júl   31 11.53 src
/// drwxr-xr-x 4 jabba jabba 4096 júl   31 12.26 target
/// drwxr-xr-x 2 jabba jabba 4096 júl   12 20.21 .vscode
/// ```
pub fn exec_cmd(cmd: &str) {
    let parts = shlex::split(cmd).unwrap();
    let head = &parts[0];
    let tail = &parts[1..];

    let mut p = process::Command::new(head);
    p.args(tail);
    let mut child = p.spawn().expect("command failed to start");
    child.wait().expect("command wasn't running");
}

// ==========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_exitcode_stdout_stderr_exit_code_test() {
        use which::which;

        let cmd = "rustc";
        let result = which(cmd);
        if result.is_err() {
            return;
        }
        // else, if "rustc" is available (should be...)
        let cmd = "rustc --version";
        let stat = get_exitcode_stdout_stderr(cmd).unwrap();
        assert_eq!(stat.exit_code, 0);
    }

    #[test]
    fn get_exitcode_stdout_stderr_stdout_test() {
        use which::which;

        let cmd = "rustc";
        let result = which(cmd);
        if result.is_err() {
            return;
        }
        // else, if "rustc" is available (should be...)
        let cmd = "rustc --version";
        let stat = get_exitcode_stdout_stderr(cmd).unwrap();
        assert!(stat.stdout.starts_with("rustc"));
    }

    #[test]
    fn get_exitcode_stdout_stderr_stderr_test() {
        use which::which;

        let cmd = "rustc";
        let result = which(cmd);
        if result.is_err() {
            return;
        }
        // else, if "rustc" is available (should be...)
        let cmd = "rustc --nothing20220731"; // this option doesn't exist
        let stat = get_exitcode_stdout_stderr(cmd).unwrap();
        assert!(stat.exit_code != 0);
        assert!(stat.stderr.len() > 0);
    }

    #[test]
    fn trimmed_output_test() {
        use which::which;

        let cmd = "rustc";
        let result = which(cmd);
        if result.is_err() {
            return;
        }
        // else, if "rustc" is available (should be...)
        let cmd = "rustc --version";
        let stat = get_exitcode_stdout_stderr(cmd).unwrap();
        assert!(stat.output() == stat.stdout.clone());
        assert!(stat.trimmed_output().len() < stat.output().len());
    }

    #[test]
    fn exec_cmd_test() {
        let cmd = "date";
        exec_cmd(cmd);
    }
}
