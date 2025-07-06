use std::process::Command;

fn main() {
    // Git 태그에서 버전 정보 가져오기 (가장 최근 태그 사용)
    let version = if let Ok(output) = Command::new("git")
        .args(&["describe", "--tags", "--abbrev=0"])
        .output()
    {
        if output.status.success() {
            let tag = String::from_utf8_lossy(&output.stdout);
            let version = tag.trim().strip_prefix('v').unwrap_or(tag.trim());
            version.to_string()
        } else {
            env!("CARGO_PKG_VERSION").to_string()
        }
    } else {
        // Git이 없거나 태그가 없으면 Cargo.toml 버전 사용
        env!("CARGO_PKG_VERSION").to_string()
    };
    
    println!("cargo:rustc-env=BUILD_VERSION={}", version);
    println!("cargo:rerun-if-changed=.git/refs/tags/");
}