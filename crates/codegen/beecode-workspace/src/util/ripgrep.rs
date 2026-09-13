// Resolution (bundled binary, RG_BIN_PATH, Bazel runfiles, PATH) lives in the beecode-tools crate
// This module only preserves the `crate::util::ripgrep` path
pub use beecode_tools::util::rg_path;
