const EXE: &str = env!("CARGO_BIN_EXE_hc-runner");

#[test]
fn test() {
    let result = process::Command::new(EXE).args([]).output().unwrap();
    assert!(result.status.success());
}
