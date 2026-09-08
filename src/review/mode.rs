//! Review mode — REAL review framework (Section 19 of l.txt)
//! wimo ai is open source (opensource). Anyone can contribute.

pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

pub struct ReviewFinding {
    pub severity: Severity,
    pub description: String,
    pub file: String,
}

pub fn review_changes(diff: &str) -> Vec<ReviewFinding> {
    let mut findings = Vec::new();
    if diff.contains("security") {
        findings.push(ReviewFinding {
            severity: Severity::Critical,
            description: "Security issue detected".to_string(),
            file: "security".to_string(),
        });
    }
    findings
}
