use std::process::Command;

fn main() {
    // Automatically update README_src_outline.md by running the generate_outline.sh script
    if cfg!(target_os = "windows") {
        // On Windows, try to use Git Bash or WSL bash
        let bash_paths = [
            "C:\\Program Files\\Git\\bin\\bash.exe",
            "C:\\Program Files (x86)\\Git\\bin\\bash.exe",
            "bash.exe", // Try PATH
        ];

        let mut success = false;

        for bash_path in bash_paths {
            match Command::new(bash_path)
                .arg("-c")
                .arg("./scripts/generate_outline.sh > README_src_outline.md")
                .status()
            {
                Ok(s) if s.success() => {
                    success = true;
                    break;
                }
                Ok(_) => continue,  // Command executed but script failed
                Err(_) => continue, // Try next path
            }
        }

        if !success {
            eprintln!("cargo:warning=Could not generate README_src_outline.md (bash not found or script failed)");
        }
    } else {
        // On Unix-like systems, use sh
        match Command::new("sh").arg("-c").arg("./scripts/generate_outline.sh > README_src_outline.md").status() {
            Ok(s) if s.success() => {}
            Ok(s) => eprintln!("cargo:warning=generate_outline.sh failed with exit code {:?}", s.code()),
            Err(e) => eprintln!("cargo:warning=Failed to execute generate_outline.sh: {:?}", e),
        }
    }

    // Build script runs on every build to ensure README_src_outline.md is up-to-date
}
