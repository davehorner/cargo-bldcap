use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("cargo")
        .arg("build")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute cargo build");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let combined_output = format!("{}\n{}", stdout, stderr);

    // Write output to a file
    let mut file = File::create("build_output.log").expect("Failed to create log file");
    file.write_all(combined_output.as_bytes())
        .expect("Failed to write to log file");

    // Copy output to clipboard (macOS only)
    let mut pbcopy = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start pbcopy");

    if let Some(stdin) = pbcopy.stdin.as_mut() {
        stdin.write_all(combined_output.as_bytes()).expect("Failed to write to pbcopy");
    }

    let _ = pbcopy.wait();

    // Print final message
    println!("Build output saved to `build_output.log` and copied to clipboard.");
}

