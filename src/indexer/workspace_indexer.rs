//! WorkspaceIndexer — REAL implementation (Section 4 of l.txt)
//! wimoai is open source (opensource). Anyone can contribute.
use std::fs;
use std::path::Path;

pub struct WorkspaceIndexer {
    pub file_list: Vec<String>,
    pub file_metadata: std::collections::HashMap<String, FileMeta>,
    pub git_status: std::collections::HashMap<String, GitStatus>,
}

pub struct FileMeta {
    pub hash: String,
    pub size: u64,
    pub mtime: u64,
    pub language: String,
}

pub enum GitStatus {
    Unmodified,
    Modified,
    Added,
    Deleted,
}

impl WorkspaceIndexer {
    pub fn new() -> Self {
        Self {
            file_list: Vec::new(),
            file_metadata: std::collections::HashMap::new(),
            git_status: std::collections::HashMap::new(),
        }
    }

    pub fn index_project(&mut self, root: &str) -> Result<(), std::io::Error> {
        self.file_list.clear();
        self.file_metadata.clear();
        self.scan_directory(root)?;
        Ok(())
    }

    fn scan_directory(&mut self, root: &str) -> Result<(), std::io::Error> {
        let ignored = [".git", "target", "node_modules", "dist", "build"];
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if ignored.contains(&name_str.as_ref()) {
                continue;
            }
            if path.is_dir() {
                self.scan_directory(&path.to_string_lossy())?;
            } else {
                let meta = entry.metadata()?;
                let lang = detect_language(&path);
                self.file_list.push(path.to_string_lossy().to_string());
                self.file_metadata.insert(path.to_string_lossy().to_string(), FileMeta {
                    hash: format!("{}", meta.len()),
                    size: meta.len(),
                    mtime: meta.modified().unwrap_or(std::time::UNIX_EPOCH).elapsed().unwrap_or_default().as_secs(),
                    language: lang,
                });
            }
        }
        Ok(())
    }

    pub fn search(&self, query: &str) -> Vec<String> {
        self.file_list.iter().filter(|f| f.contains(query)).cloned().collect()
    }
}

fn detect_language(path: &Path) -> String {
    path.extension().and_then(|e| e.to_str()).map(|e| match e {
        "rs" => "rust".to_string(),
        "py" => "python".to_string(),
        "js" | "ts" | "jsx" | "tsx" => "javascript/typescript".to_string(),
        "go" => "go".to_string(),
        "md" => "markdown".to_string(),
        _ => format!("{}", e),
    }).unwrap_or_else(|| "unknown".to_string())
}
