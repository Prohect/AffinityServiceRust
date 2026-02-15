use std::process::Command;

fn main() {
    // Automatically update src_outline.md by running the generate_outline.sh script
    let status = if cfg!(target_os = "windows") {
        // On Windows, try to use Git Bash or WSL bash
        let bash_paths = vec![
            "C:\\Program Files\\Git\\bin\\bash.exe",
            "C:\\Program Files (x86)\\Git\\bin\\bash.exe",
            "bash.exe", // Try PATH
        ];

        let mut success = false;

        for bash_path in bash_paths {
            match Command::new(bash_path).arg("-c").arg("./scripts/generate_outline.sh > src_outline.md").status() {
                Ok(s) if s.success() => {
                    success = true;
                    break;
                }
                Ok(_) => {
                    // Command executed but script failed
                    continue;
                }
                Err(_) => {
                    // Try next path
                    continue;
                }
            }
        }

        if !success {
            // Fallback: create a minimal outline file
            eprintln!("Warning: Could not execute generate_outline.sh (bash not found)");
            eprintln!("Creating minimal src_outline.md");
            std::fs::write(
                "src_outline.md",
                "# Src Code Structure Outline\n\nBuild script could not generate outline (bash not found).\n",
            )
            .expect("Failed to write minimal src_outline.md");
        }

        Ok(())
    } else {
        // On Unix-like systems, use sh
        Command::new("sh").arg("-c").arg("./scripts/generate_outline.sh > src_outline.md").status().map(|s| {
            if !s.success() {
                panic!("generate_outline.sh failed with exit code {:?}", s.code());
            }
        })
    };

    if let Err(e) = status {
        panic!("Failed to execute generate_outline.sh: {:?}", e);
    }

    // Build script runs on every build to ensure src_outline.md is up-to-date
}
