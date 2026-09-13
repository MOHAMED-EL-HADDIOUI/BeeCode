//! # beecode-hooks
//!
//! Runtime hook system for beecode: file-based discovery, command execution, and policy enforcement.
//!
//! ## Overview
//!
//! This crate provides a minimal hooks system for beecode. Hooks are discovered
//! from dedicated directories (`~/.beecode/hooks/` and `<git-worktree-root>/.beecode/hooks/`),
//! defined in JSON files (compatible settings format), and executed as child processes.
//!
//! ## Scope
//!
//! - Event types: `session_start`, `pre_tool_use`, `post_tool_use`, `user_prompt_submit`, `stop`/`subagent_stop`, `notification`, `session_end`
//! - Both command-backed and HTTP hooks
//! - `pre_tool_use` hooks can allow/ask/deny and rewrite tool input via `updatedInput`
//! - Prompt and stop gates can block
//! - Fail-open by default: a hook error never blocks
//!
//! ## Quick start
//!
//! ```rust,no_run
//! use std::path::Path;
//! use beecode_hooks::discovery::load_hooks;
//! use beecode_hooks::event::HookEventName;
//!
//! let (registry, errors) = load_hooks(
//!     Some(Path::new("/home/user/.beecode/hooks")),
//!     Some(Path::new("/project/.beecode/hooks")),
//! );
//!
//! for err in &errors {
//!     eprintln!("hook load warning: {err}");
//! }
//!
//! let pre_hooks = registry.hooks_for(HookEventName::PreToolUse);
//! println!("loaded {} pre_tool_use hooks", pre_hooks.len());
//! ```

pub mod config;
pub mod discovery;
pub mod dispatcher;
mod env_expand;
pub mod error;
pub mod event;
pub mod matcher;
pub mod result;
pub mod runner;
#[cfg(test)]
mod test_support;
pub mod trust;
