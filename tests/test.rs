use std::process::*;

const CRATE: &str = env!("CARGO_MANIFEST_DIR");

#[test]
fn test() {
    let path = format!("{}/target/debug/foo", CRATE);
    let output = Command::new(path).env_clear().output().expect("failed");

    println!("status: {}", output.status);
    println!("stdout: {}", std::str::from_utf8(&output.stdout).unwrap());
    assert!(output.status.success());

    panic!();
}
