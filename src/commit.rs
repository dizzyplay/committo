use std::io::{self, Write};
use std::process::{Command, Stdio};

/// Execute git commit with the message piped to it
/// This automatically runs: echo "message" | git commit --edit -F - (with --edit)
/// or echo "message" | git commit -F - (without --edit)
pub fn execute_git_commit_with_pipe(message: &str, run_edit: bool) -> io::Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("commit");
    
    if run_edit {
        cmd.arg("--edit");
    }
    
    cmd.arg("-F")
       .arg("-") // Read from stdin
       .stdin(Stdio::piped());
    
    // In test environment, suppress output to avoid polluting test results
    if cfg!(test) {
        cmd.stdout(Stdio::null())
           .stderr(Stdio::null());
    } else {
        cmd.stdout(Stdio::inherit())
           .stderr(Stdio::inherit());
    }
    
    let mut child = cmd.spawn()?;
    
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(message.as_bytes())?;
        stdin.write_all(b"\n")?; // Add newline
    }
    
    let status = child.wait()?;
    
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("git commit failed with exit code: {}", status.code().unwrap_or(-1))
        ));
    }
    
    Ok(())
}

