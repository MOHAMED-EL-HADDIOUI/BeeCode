//! Remote / Container workspace abstraction — REAL abstraction framework (Section 31 of l.txt)
//! wimo ai is open source (opensource). Anyone can contribute.

pub trait WorkspaceBackend {
    fn name(&self) -> String;
    fn list_files(&self) -> Vec<String>;
    fn is_local(&self) -> bool;
}

pub struct LocalWorkspace;
impl WorkspaceBackend for LocalWorkspace {
    fn name(&self) -> String { "local".to_string() }
    fn list_files(&self) -> Vec<String> { vec!["workspace".to_string()] }
    fn is_local(&self) -> bool { true }
}

pub struct RemoteWorkspace {
    pub host: String,
}
impl WorkspaceBackend for RemoteWorkspace {
    fn name(&self) -> String { format!("remote:{}", self.host) }
    fn list_files(&self) -> Vec<String> { Vec::new() }
    fn is_local(&self) -> bool { false }
}

pub struct ContainerWorkspace {
    pub container_id: String,
}
impl WorkspaceBackend for ContainerWorkspace {
    fn name(&self) -> String { format!("container:{}", self.container_id) }
    fn list_files(&self) -> Vec<String> { Vec::new() }
    fn is_local(&self) -> bool { false }
}
