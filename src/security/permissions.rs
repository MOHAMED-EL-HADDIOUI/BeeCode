//! Security / Permissions — REAL classification (Section 22 of l.txt)
//! beecode is open source (opensource). Anyone can contribute.

pub enum ActionClass {
    READ,
    WRITE,
    DELETE,
    EXECUTE,
    NETWORK,
    GIT_WRITE,
    PACKAGE_INSTALL,
    SYSTEM_MODIFY,
}

pub enum Policy {
    ALLOW,
    ASK,
    DENY,
}

pub fn check_permission(action: ActionClass, policy: Policy) -> bool {
    match policy {
        Policy::ALLOW => true,
        Policy::DENY => false,
        Policy::ASK => {
            // In production: ask user; here: allow by default for framework
            true
        }
    }
}

pub fn classify_command(cmd: &str) -> ActionClass {
    if cmd.contains("rm") || cmd.contains("delete") {
        ActionClass::DELETE
    } else if cmd.contains("write") || cmd.contains("edit") {
        ActionClass::WRITE
    } else if cmd.contains("git commit") || cmd.contains("git push") {
        ActionClass::GIT_WRITE
    } else if cmd.contains("npm install") || cmd.contains("pip install") {
        ActionClass::PACKAGE_INSTALL
    } else if cmd.contains("sudo") || cmd.contains("chmod") {
        ActionClass::SYSTEM_MODIFY
    } else {
        ActionClass::READ
    }
}
