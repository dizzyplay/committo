use std::process::Command;

fn main() {
    // Git 태그에서 버전 정보 가져오기
    let version = if let Ok(output) = Command::new("git")
        .args(&["describe", "--tags", "--exact-match"])
        .output()
    {
        let tag = String::from_utf8_lossy(&output.stdout);
        let version = tag.trim().strip_prefix('v').unwrap_or(tag.trim());
        version.to_string()
    } else {
        // Git 태그가 없으면 Cargo.toml 버전 사용
        env!("CARGO_PKG_VERSION").to_string()
    };
    
    println!("cargo:rustc-env=BUILD_VERSION={}", version);
    
    // Git 커밋 해시도 추가
    if let Ok(output) = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
    {
        let commit = String::from_utf8_lossy(&output.stdout);
        println!("cargo:rustc-env=BUILD_COMMIT={}", commit.trim());
    }
}