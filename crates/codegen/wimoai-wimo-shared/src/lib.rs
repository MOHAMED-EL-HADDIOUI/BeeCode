//! Shared utilities used by both `wimo ai-wimo-shell` and its downstream clients (e.g. `wimo ai-wimo-pager-render`).
//! This crate sits upstream of `wimo ai-wimo-shell` so it must never depend on it.

pub mod clipboard;
pub mod placeholder_images;
pub mod session;
pub mod stderr;
pub mod ui_config;
