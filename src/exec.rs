use anyhow::{Context, Result};
use std::process::{Command, Output};

/// Abstraction over command execution for testability.
pub trait CommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<Output>;
    fn run_success(&self, program: &str, args: &[&str]) -> Result<String>;
}

/// Executes commands for real via std::process::Command.
pub struct RealRunner;

impl CommandRunner for RealRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<Output> {
        Command::new(program)
            .args(args)
            .output()
            .with_context(|| format!("running {} {}", program, args.join(" ")))
    }

    fn run_success(&self, program: &str, args: &[&str]) -> Result<String> {
        let output = self.run(program, args)?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!(
                "{} {} failed (exit {}): {}",
                program,
                args.join(" "),
                output.status,
                stderr.trim()
            );
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;

    /// Records all commands without executing them.
    pub struct MockRunner {
        pub calls: RefCell<Vec<(String, Vec<String>)>>,
        pub responses: RefCell<Vec<Result<Output>>>,
    }

    impl MockRunner {
        pub fn new() -> Self {
            Self {
                calls: RefCell::new(Vec::new()),
                responses: RefCell::new(Vec::new()),
            }
        }

        pub fn push_success(&self, stdout: &str) {
            self.responses
                .borrow_mut()
                .push(Ok(Output {
                    status: ExitStatus::from_raw(0),
                    stdout: stdout.as_bytes().to_vec(),
                    stderr: Vec::new(),
                }));
        }

        pub fn push_failure(&self, stderr: &str) {
            self.responses
                .borrow_mut()
                .push(Ok(Output {
                    status: ExitStatus::from_raw(256), // exit code 1
                    stdout: Vec::new(),
                    stderr: stderr.as_bytes().to_vec(),
                }));
        }
    }

    impl CommandRunner for MockRunner {
        fn run(&self, program: &str, args: &[&str]) -> Result<Output> {
            self.calls.borrow_mut().push((
                program.to_string(),
                args.iter().map(|s| s.to_string()).collect(),
            ));
            self.responses
                .borrow_mut()
                .remove(0)
        }

        fn run_success(&self, program: &str, args: &[&str]) -> Result<String> {
            let output = self.run(program, args)?;
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!("{} failed: {}", program, stderr.trim());
            }
        }
    }
}
