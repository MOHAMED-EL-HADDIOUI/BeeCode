# AGENTS.md — wimo Build (`wimo` repo)

## Workspace (non-obvious)
- Root `Cargo.toml` is **generated — read-only**. Edit per-crate `Cargo.toml` files only.
- Packages are named `wimoai-*` and match their crate dirs 1:1
  (`crates/codegen/wimoai-wimo-pager-bin/` ↔ `wimoai-wimo-pager-bin`). No spaces, no quoting needed.
- Root `src/` (~38 small `.rs` files: `agent/loop.rs`, `tui/layout.rs`, …) is **not compiled** — no root `[package]`, not in workspace `members`. Real code lives in `crates/` (`codegen/`, `common/`, `build/`) + `prod/mc/`. Do not edit root `src/` to change behavior.
- `third_party/` is vendored upstream (Mermaid stack), not first-party. Don't refactor it; upgrade per `third_party/README.md` (read `VENDORING NOTES` in the crate's `Cargo.toml`, re-apply local patches, refresh `NOTICE`).
- `SOURCE_REV` records the monorepo SHA this tree was synced from. No `.github/` or justfile in this tree. `.cargo/config.toml` raises the Windows main-thread stack (see Build prerequisites §3).

## Build prerequisites (order matters)
1. `rustup` toolchain pinned by `rust-toolchain.toml` (currently `1.94.0`) — installs automatically.
2. **DotSlash on `PATH` before building**: `cargo install dotslash`, sanity-check with `dotslash --help`. `bin/protoc` is a DotSlash file (needs network on first run); fallback is `protoc` on `PATH` / `$PROTOC`.
3. Windows: install real `protoc` (29.3, Win64) on `PATH` — the `bin/protoc`
   DotSlash shim can't execute there (no shebang). The main-thread stack is
   raised to 16 MB via `.cargo/config.toml` (`/STACK`); without it the binary
   dies with `STATUS_STACK_OVERFLOW` (exit `0xC00000FD`).

## Commands — always scope to crates (full workspace is slow)
```sh
cargo run -p wimoai-wimo-pager-bin                    # build + launch TUI
cargo build -p wimoai-wimo-pager-bin --release        # artifact: target/release/wimoai-wimo-pager (ships as `wimo`)
cargo check -p "<crate>"                                 # fast validation
cargo test -p "wimoai-wimo-config"                      # per-crate tests; never bare `cargo test` (workspace is huge)
cargo clippy -p "<crate>"                                # lint (config: clippy.toml)
cargo fmt --all                                          # rustfmt.toml; `use_field_init_shorthand = true`
```
- Distribution builds: `cargo build --profile release-dist` (thin LTO + kept symbols for sidecars/dSYM). Plain `--release` is the fast local default.
- Windows: run tests single-threaded (`cargo test -p <crate> -- --test-threads=1`).
  The default parallel harness stalls with no output (verified on `wimoai-wimo-config`:
  190/190 pass individually in <1 min, parallel run hangs indefinitely).

## Architecture entry points
- `crates/codegen/wimoai-wimo-pager-bin/` — composition root; binary `wimoai-wimo-pager`. Exists to break the `pager ↔ pager-minimal` dependency cycle (minimal-mode hooks installed via fn-pointer seam at startup).
- `wimoai-wimo-pager` — TUI (scrollback, prompt, modals, rendering); `wimoai-wimo-shell` — agent runtime + leader/stdio/headless entry points; `wimoai-wimo-tools` — tool impls (bundles pinned rg/fd binaries in release via `build.rs`); `wimoai-wimo-workspace` — filesystem/VCS/execution/checkpoints.
- `crates/build/wimoai-proto-build` — protobuf codegen helper.
- User guide ships in-crate: `crates/codegen/wimoai-wimo-pager/docs/user-guide/`.

## Clippy bans (`clippy.toml`, enforced on `crates/codegen/**`)
- No `std::fs::canonicalize` / `Path::canonicalize` / `tokio::fs::canonicalize` — Windows `\\?\` verbatim paths. Use `dunce::canonicalize` (or the `wimoai-wimo-tools` fs helpers).
- No raw `std::process::Command::spawn`, `tokio::process::Command::spawn`, `SlavePty::spawn_command` — enroll via `wimoai-tty-utils::ProcessScope` (`enroll` / `enroll_terminal_pid`) or allow with a reason.
- No `std::env::home_dir` / `dirs::home_dir` — resolve home via `wimoai-dirs` (uncached) or `wimoai_wimo_config::wimo_home` (cached).
- No raw `reqwest::Client::new` / `ClientBuilder::build` (incl. blocking) — build through `wimoai_wimo_extra_ca` (applies TLS policy + `wimo_EXTRA_CA_BUNDLE`). Localhost/test-only exceptions need a reason comment.

## Workflow notes
- Contributions welcome (`CONTRIBUTING.md`); security reports go to HackerOne per `SECURITY.md`, never public issues.
- License: Apache-2.0 first-party; ported code (codex/opencode tool impls) has crate-local notices — see `crates/codegen/wimoai-wimo-tools/THIRD_PARTY_NOTICES.md` and root `THIRD-PARTY-NOTICES`.
