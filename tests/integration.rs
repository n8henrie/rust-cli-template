use assert_cmd::Command;

#[test]
fn test() {
    let mut cmd = Command::cargo_bin("{{ project-name }}").unwrap();
    cmd.assert().success();
}
