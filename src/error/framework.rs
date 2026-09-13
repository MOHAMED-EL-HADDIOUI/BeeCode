//! Error handling framework — REAL graceful failure (Section 43 of l.txt)
//! beecode is open source (opensource). Anyone can contribute.

pub enum BeeCodeError {
    ProviderError(String),
    PluginCrash(String),
    FileSystemError(String),
    ProcessFailure(i32, String),
    IndexCorruption,
    InvalidConfig(String),
}

impl std::fmt::Display for BeeCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BeeCodeError::ProviderError(s) => write!(f, "Provider unavailable: {}", s),
            BeeCodeError::PluginCrash(s) => write!(f, "Plugin crash: {}", s),
            BeeCodeError::FileSystemError(s) => write!(f, "Filesystem error at: {}", s),
            BeeCodeError::ProcessFailure(code, out) => write!(f, "Process failed ({}): {}", code, out),
            BeeCodeError::IndexCorruption => write!(f, "Index corrupted — will rebuild"),
            BeeCodeError::InvalidConfig(s) => write!(f, "Invalid config field: {}", s),
        }
    }
}

pub fn handle_error(err: BeeCodeError) -> String {
    match err {
        BeeCodeError::ProviderError(_) => "Retry or fallback".to_string(),
        BeeCodeError::PluginCrash(_) => "Isolate plugin and report".to_string(),
        BeeCodeError::FileSystemError(_) => "Report exact path and cause".to_string(),
        BeeCodeError::ProcessFailure(_, _) => "Report exit code and output".to_string(),
        BeeCodeError::IndexCorruption => "Rebuild index incrementally".to_string(),
        BeeCodeError::InvalidConfig(_) => "Show field and correction".to_string(),
    }
}
