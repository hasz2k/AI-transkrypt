fn main() {
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("clang")
            .arg("--print-runtime-dir")
            .output()
        {
            if output.status.success() {
                let dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !dir.is_empty() {
                    println!("cargo:rustc-link-search=native={}", dir);
                    println!("cargo:rustc-link-lib=static=clang_rt.osx");
                }
            }
        }
    }

    tauri_build::build()
}
