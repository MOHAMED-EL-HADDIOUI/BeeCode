//! Session synthesis and in-process e2e harness for the load-perf and fork bench tests.
//!
//! This module lives in `wimo ai-wimo-shell` (feature `test-support`) rather than `wimo ai-wimo-test-support`.
//! Synthesis drives the real `JsonlStorageAdapter`, so the reverse dependency would be circular.

pub mod e2e;
pub mod synth;
