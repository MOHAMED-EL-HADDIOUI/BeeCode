//! Version / release info — REAL version tracking (Section 41/36 of l.txt)
//! wimoai is open source (opensource). Anyone can contribute.

pub const VERSION: &str = "1.0.0-wimo";
pub const RELEASE_TARGETS: &[&str] = &[
    "linux-x86_64",
    "linux-arm64",
    "macos-x86_64",
    "macos-arm64",
    "windows-x86_64",
];

pub fn version_string() -> String {
    format!("wimo {} (wimoai open source)", VERSION)
}
