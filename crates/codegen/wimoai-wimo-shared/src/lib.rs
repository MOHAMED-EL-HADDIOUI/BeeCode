//! Shared utilities used by both `wimoai-wimo-shell` and its downstream clients (e.g. `wimoai-wimo-pager-render`).
//! This crate sits upstream of `wimoai-wimo-shell` so it must never depend on it.

pub mod clipboard;
pub mod placeholder_images;
pub mod session;
pub mod stderr;
pub mod ui_config;
