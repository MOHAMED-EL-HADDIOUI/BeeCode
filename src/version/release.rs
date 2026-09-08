//! Version / release info — REAL version tracking (Section 41/36 of l.txt)
//! wimo ai is open source (opensource). Anyone can contribute.

pub const VERSION: &str = "1.0.0-wimo";
pub const RELEASE_TARGETS: &[&str] = &[
    "linux-x86_64",
    "linux-arm64",
    "macos-x86_64",
    "macos-arm64",
    "windows-x86_64",
];

pub fn version_string() -> String {
    format!("wimo {} (wimo ai open source)", VERSION)
}
