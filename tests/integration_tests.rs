use std::fs;
use std::process::Command;

#[test]
fn test_rust_mysqldump() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("rust_mysqldump")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Script did not run successfully");
}

#[test]
fn test_dump_files_created() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("rust_mysqldump")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Script did not run successfully");

    let db_folder = "exports";
    assert!(fs::read_dir(db_folder).is_ok(), "Dump files not found");
}

#[test]
fn test_output_contains_expected_text() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("rust_mysqldump")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Script did not run successfully");

    let output_str = String::from_utf8(output.stdout).unwrap();
    assert!(
        output_str.contains("Successfully dumped database"),
        "Expected text not found in the output"
    );
}
