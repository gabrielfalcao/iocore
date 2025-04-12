use iocore::{shell_command, shell_command_string_output, shell_command_vec_output, shell_command_stdout, Path};
use sanitation::SString;

#[test]
fn test_shell_command_vec_output() {
    let (exit_code, out, err) = shell_command_vec_output("echo 'out'", ".").unwrap();

    assert_eq!(SString::new(&out).unchecked_safe(), "'out'\n");
    assert_eq!(exit_code, 0);
    assert_eq!(SString::new(&err).unchecked_safe(), "");

    let (exit_code, out, err) = shell_command_vec_output("dd if=/dev/null of=/", ".").unwrap();

    assert_eq!(exit_code, 1);
    assert_eq!(SString::new(&err).unchecked_safe().len() > 0, true);
    assert_eq!(SString::new(&out).unchecked_safe(), "");
}

#[test]
fn test_shell_command_string_output() {
    let (exit_code, out, err) = shell_command_string_output("echo -n out", ".").unwrap();

    assert_eq!(out, "out");
    assert_eq!(exit_code, 0);
    assert_eq!(err, "");

    let (exit_code, out, err) = shell_command_string_output("dd if=/dev/null of=/", ".").unwrap();

    assert_eq!(exit_code, 1);
    assert_eq!(err.len() > 0, true);
    assert_eq!(out, "");
}

#[test]
fn test_shell_command() {
    let exit_code = shell_command("test 4 -eq 2", ".").unwrap();

    assert_eq!(exit_code, 1);
}


#[test]
fn test_shell_command_stdout() {
    let stdout = shell_command_stdout("mktemp -qd", ".").unwrap();
    assert_eq!(Path::raw(stdout.trim()).exists(), true);
    assert_eq!(Path::raw(stdout.trim()).is_directory(), true);
}
