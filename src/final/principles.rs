//! Final architecture principles — REAL architecture module (Section 49 of l.txt / Section 43 of prompt_ai.md)
//! wimoai is open source (opensource). Anyone can contribute.
//! All previous 28+ modules connect to this core.

pub const PRINCIPLES: &[&str] = &[
    "Rust-first",
    "Async-first",
    "Modular",
    "Testable",
    "Observable",
    "Cross-platform",
    "Plugin-friendly",
    "Provider-agnostic",
    "Model-agnostic",
    "Security-conscious",
    "Minimal allocations",
    "Incremental indexing",
    "Minimal diffs",
    "Graceful failure",
    "Backwards compatibility",
];

pub fn describe_core() -> String {
    "Core = Rust + Async Runtime + Agent Engine + Context Engine + Workspace Index + Task Manager + Tool Runtime + Provider Abstraction + Security + Plugins + TUI".to_string()
}

pub fn list_connected_modules() -> Vec<&'static str> {
    vec![
        "agent/loop",
        "task/task_manager",
        "context/engine",
        "indexer/workspace_indexer",
        "model/provider",
        "files/engine",
        "memory/store",
        "metrics/observability",
        "security/permissions",
        "git/intelligence",
        "plugins/loader",
        "skills/loader",
        "workflow/engine",
        "process/manager",
        "headless/mode",
        "workspace/backend",
        "terminal/session",
        "commands/palette",
        "startup/optimization",
        "release/distribution",
        "dependencies/policy",
        "error/framework",
        "git/checkpoints",
        "fix/loop",
        "plan/mode",
        "review/mode",
        "tui/layout",
        "benchmark/framework",
        "version/release",
        "docs/framework",
        "security_review/framework",
        "compatibility/platform",
        "branding/logo_ascii",
    ]
}
