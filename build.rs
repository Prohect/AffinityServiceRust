use std::process::Command;

fn main() {
    // Automatically update src_outline.md by running the generate_outline.sh script
    let status = Command::new("sh")
        .arg("-c")
        .arg("./scripts/generate_outline.sh > src_outline.md")
        .status()
        .expect("Failed to execute generate_outline.sh");

    if !status.success() {
        panic!("generate_outline.sh failed with exit code {:?}", status.code());
    }

    // Build script runs on every build to ensure src_outline.md is up-to-date
}
