//! Shared utilities used by both `beecode-shell` and its downstream clients (e.g. `beecode-pager-render`).
//! This crate sits upstream of `beecode-shell` so it must never depend on it.

pub mod clipboard;
pub mod placeholder_images;
pub mod session;
pub mod stderr;
pub mod ui_config;
