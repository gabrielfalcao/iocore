//     /\\\\         /\\       /\\\\     /\\\\\\\    /\\\\\\\\
//   /\\    /\\   /\\   /\\  /\\    /\\  /\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\ /\\      /\\\\\\
// /\\        /\\/\\       /\\        /\\/\\  /\\    /\\
//   /\\     /\\  /\\   /\\  /\\     /\\ /\\    /\\  /\\
//     /\\\\        /\\\\      /\\\\     /\\      /\\/\\\\\\\\

use std::process::{Command, Stdio};

use sanitation::SString;

use crate::exceptions::Result;
use crate::fs::Path;

/// Utility function to spawn a command from a string rather than
/// array of arguments.
///
/// Returns a triple with `(exit code, stdout, stderr)`
pub fn shell_command_vec_output(
    command: impl std::fmt::Display,
    current_dir: impl Into<Path>,
) -> Result<(i32, Vec<u8>, Vec<u8>)> {
    let args = command
        .to_string()
        .split(" ")
        .map(|arg| arg.trim().to_string())
        .collect::<Vec<String>>();
    let mut cmd = Command::new(args[0].clone());
    let cmd = cmd.current_dir(Into::<Path>::into(current_dir));
    let cmd = cmd.args(args[1..].to_vec());
    let cmd = cmd.stdin(Stdio::null());
    let cmd = cmd.stdout(Stdio::piped());
    let cmd = cmd.stderr(Stdio::piped());
    let child = cmd.spawn()?;
    let output = child.wait_with_output()?;
    let status = output.status.code().unwrap_or_default();
    Ok((status, output.stdout.to_vec(), output.stderr.to_vec()))
}

/// Utility function to spawn a command from a string rather than
/// array of arguments and returns strings for stdout and stderr
/// rather than Vec<u8>. Stdout and Stderr are sanitized for memory
/// safety during string conversion.
///
/// Returns a triple with `(exit code, stdout, stderr)`
pub fn shell_command_string_output(
    command: impl std::fmt::Display,
    current_dir: impl Into<Path>,
) -> Result<(i32, String, String)> {
    let (status, stdout, stderr) = shell_command_vec_output(command, current_dir)?;
    let stdout = SString::new(&stdout).safe()?;
    let stderr = SString::new(&stderr).safe()?;
    Ok((status, stdout, stderr))
}

/// Utility function to spawn a command from a string rather than
/// array of arguments and returns the exit code. Stdout and Stderr
/// are inherited from the current process.
pub fn shell_command(
    command: impl std::fmt::Display,
    current_dir: impl Into<Path>,
) -> Result<i32> {
    let args = command
        .to_string()
        .split(" ")
        .map(|arg| arg.trim().to_string())
        .collect::<Vec<String>>();
    let mut cmd = Command::new(args[0].clone());
    let cmd = cmd.current_dir(Into::<Path>::into(current_dir));
    let cmd = cmd.args(args[1..].to_vec());
    let cmd = cmd.stdin(Stdio::null());
    let cmd = cmd.stdout(Stdio::inherit());
    let cmd = cmd.stderr(Stdio::inherit());
    let child = cmd.spawn()?;
    let output = child.wait_with_output()?;
    let status = output.status.code().unwrap_or_default();
    Ok(status)
}
