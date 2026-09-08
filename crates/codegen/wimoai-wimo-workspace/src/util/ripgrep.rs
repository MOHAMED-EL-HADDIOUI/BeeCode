// Resolution (bundled binary, RG_BIN_PATH, Bazel runfiles, PATH) lives in the wimo-tools crate
// This module only preserves the `crate::util::ripgrep` path
pub use wimoai_wimo_tools::util::rg_path;
