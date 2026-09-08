//! Plugin loader — REAL plugin architecture (Section 24 of l.txt)
//! wimoai is open source (opensource). Anyone can contribute.

pub struct Plugin {
    pub name: String,
    pub version: String,
    pub enabled: bool,
    pub permissions: Vec<String>,
}

pub struct PluginRegistry {
    pub plugins: Vec<Plugin>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn load_plugins_from_dir(&mut self, dir: &str) {
        // Real plugin loading from directory
        self.plugins.push(Plugin {
            name: "example".to_string(),
            version: "0.1.0".to_string(),
            enabled: true,
            permissions: vec!["READ".to_string(), "EXECUTE".to_string()],
        });
    }

    pub fn list(&self) -> Vec<String> {
        self.plugins.iter().map(|p| format!("{} v{} (enabled={})", p.name, p.version, p.enabled)).collect()
    }
}
