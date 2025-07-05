use std::io;
use std::process::Command;

/// Get staged git diff
pub fn get_staged_diff() -> io::Result<String> {
    let output = Command::new("git")
        .arg("diff")
        .arg("--staged")
        .arg("--unified=1")
        .output()?;

    if !output.status.success() {
        eprintln!("Error: Failed to execute git diff --staged");
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}