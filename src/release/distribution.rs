//! Release / Distribution framework — REAL release targets (Section 36 of l.txt)
//! beecode is open source (opensource). Anyone can contribute.

pub enum PlatformTarget {
    LinuxX86_64,
    LinuxARM64,
    MacOSX86_64,
    MacOSARM64,
    WindowsX86_64,
}

pub struct ReleaseInfo {
    pub version: String,
    pub targets: Vec<PlatformTarget>,
    pub checksums: Vec<String>,
    pub binary_path: String,
}

impl ReleaseInfo {
    pub fn new(version: String) -> Self {
        Self {
            version,
            targets: vec![
                PlatformTarget::LinuxX86_64,
                PlatformTarget::LinuxARM64,
                PlatformTarget::MacOSX86_64,
                PlatformTarget::MacOSARM64,
                PlatformTarget::WindowsX86_64,
            ],
            checksums: Vec::new(),
            binary_path: format!("beecode-{}", version),
        }
    }

    pub fn add_checksum(&mut self, checksum: String) {
        self.checksums.push(checksum);
    }
}
