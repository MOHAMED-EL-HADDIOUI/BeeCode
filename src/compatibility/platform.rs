//! Compatibility framework — REAL cross-platform (Section 11/44 of l.txt)
//! wimo ai is open source (opensource). Anyone can contribute.

pub enum Platform {
    Linux,
    MacOS,
    Windows,
    WSL,
}

pub fn detect_platform() -> Platform {
    #[cfg(target_os = "linux")]
    { Platform::Linux }
    #[cfg(target_os = "macos")]
    { Platform::MacOS }
    #[cfg(target_os = "windows")]
    { Platform::Windows }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    { Platform::Linux }
}

pub fn path_separator() -> &'static str {
    if cfg!(target_os = "windows") { "\\" } else { "/" }
}
