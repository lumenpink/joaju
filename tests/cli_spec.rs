use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_joaju"))
        .arg("--help")
        .output()
        .expect("Failed to execute joaju --help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Scientific academic compiler"));
    assert!(stdout.contains("compile"));
    assert!(stdout.contains("render"));
    assert!(stdout.contains("preprocess"));
}

#[test]
fn test_cli_preprocess_inline() {
    let input = "# Test\n\n%% table\n%% caption=Exemplo\nA, B\n1, 2\n";
    let mut child = Command::new(env!("CARGO_BIN_EXE_joaju"))
        .arg("preprocess")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn joaju preprocess");

    {
        use std::io::Write;
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(input.as_bytes()).unwrap();
    }

    let output = child.wait_with_output().expect("Failed to wait on child");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("# Test"));
}

#[test]
fn test_cli_render_table() {
    let input = "%% caption=Exemplo\nA, B\n1, 2\n";
    let mut child = Command::new(env!("CARGO_BIN_EXE_joaju"))
        .args(["render", "--engine", "table"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn joaju render");

    {
        use std::io::Write;
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(input.as_bytes()).unwrap();
    }

    let output = child.wait_with_output().expect("Failed to wait on child");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("#table") || stdout.contains("table("));
}
