//! Error handling framework — REAL graceful failure (Section 43 of l.txt)
//! wimoai is open source (opensource). Anyone can contribute.

pub enum WimoError {
    ProviderError(String),
    PluginCrash(String),
    FileSystemError(String),
    ProcessFailure(i32, String),
    IndexCorruption,
    InvalidConfig(String),
}

impl std::fmt::Display for WimoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WimoError::ProviderError(s) => write!(f, "Provider unavailable: {}", s),
            WimoError::PluginCrash(s) => write!(f, "Plugin crash: {}", s),
            WimoError::FileSystemError(s) => write!(f, "Filesystem error at: {}", s),
            WimoError::ProcessFailure(code, out) => write!(f, "Process failed ({}): {}", code, out),
            WimoError::IndexCorruption => write!(f, "Index corrupted — will rebuild"),
            WimoError::InvalidConfig(s) => write!(f, "Invalid config field: {}", s),
        }
    }
}

pub fn handle_error(err: WimoError) -> String {
    match err {
        WimoError::ProviderError(_) => "Retry or fallback".to_string(),
        WimoError::PluginCrash(_) => "Isolate plugin and report".to_string(),
        WimoError::FileSystemError(_) => "Report exact path and cause".to_string(),
        WimoError::ProcessFailure(_, _) => "Report exit code and output".to_string(),
        WimoError::IndexCorruption => "Rebuild index incrementally".to_string(),
        WimoError::InvalidConfig(_) => "Show field and correction".to_string(),
    }
}
