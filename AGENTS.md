# AGENTS.md — beecode Build (`beecode` repo)

## Workspace (non-obvious)
- Root `Cargo.toml` is **generated — read-only**. Edit per-crate `Cargo.toml` files only.
- Packages are named `beecode-*` and match their crate dirs 1:1
  (`crates/codegen/beecode-pager-bin/` ↔ `beecode-pager-bin`). No spaces, no quoting needed.
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
cargo run -p beecode-pager-bin                    # build + launch TUI
cargo build -p beecode-pager-bin --release        # artifacts: target/release/bee + target/release/beecode + target/release/beecode-pager
cargo check -p "<crate>"                                 # fast validation
cargo test -p "beecode-config"                      # per-crate tests; never bare `cargo test` (workspace is huge)
cargo clippy -p "<crate>"                                # lint (config: clippy.toml)
cargo fmt --all                                          # rustfmt.toml; `use_field_init_shorthand = true`
```
- Distribution builds: `cargo build --profile release-dist` (thin LTO + kept symbols for sidecars/dSYM). Plain `--release` is the fast local default.
- Windows: run tests single-threaded (`cargo test -p <crate> -- --test-threads=1`).
  The default parallel harness stalls with no output (verified on `beecode-config`:
  190/190 pass individually in <1 min, parallel run hangs indefinitely).
- Windows env failures (pre-existing, unrelated to the provider rework):
  `beecode-config` `managed_text::*` (temp-file locking) and any test
  using the `test_counting_provider` shell-out (needs a POSIX shell) fail.

## Architecture entry points
- `crates/codegen/beecode-pager-bin/` — composition root; binaries `bee` + `beecode-pager`. Exists to break the `pager ↔ pager-minimal` dependency cycle (minimal-mode hooks installed via fn-pointer seam at startup).
- `beecode-pager` — TUI (scrollback, prompt, modals, rendering); `beecode-shell` — agent runtime + leader/stdio/headless entry points; `beecode-tools` — tool impls (bundles pinned rg/fd binaries in release via `build.rs`); `beecode-workspace` — filesystem/VCS/execution/checkpoints.
- `crates/build/beecode-proto-build` — protobuf codegen helper.
- User guide ships in-crate: `crates/codegen/beecode-pager/docs/user-guide/`.

## Providers & auth (differs from upstream)
- LLM config comes from project `.beecode/settings.json` (walk-up from CWD,
  nearest wins), merged into the user tier above `~/.beecode/config.toml`
  and below `BEECODE_CONFIG` overlays /
  requirements. Same shape as
  `config.toml` in JSON: `[models] default` + `[model.<name>]` (OpenAI
  `responses` / `chat_completions`, Anthropic `messages` backends).
- Auth is provider API keys only (`[model.*]` `api_key`/`env_key`,
  `beecode_API_KEY` fallback). Session login (`beecode.com`, `oidc`,
  `cached_token`), `beecode login/logout` CLI + `/login`/`/logout` slash
  commands are removed — only `beecode.api_key` is ever advertised.
  The OAuth/device-code machinery underneath is dormant, not deleted.

## Clippy bans (`clippy.toml`, enforced on `crates/codegen/**`)
- No `std::fs::canonicalize` / `Path::canonicalize` / `tokio::fs::canonicalize` — Windows `\\?\` verbatim paths. Use `dunce::canonicalize` (or the `beecode-tools` fs helpers).
- No raw `std::process::Command::spawn`, `tokio::process::Command::spawn`, `SlavePty::spawn_command` — enroll via `beecode-tty-utils::ProcessScope` (`enroll` / `enroll_terminal_pid`) or allow with a reason.
- No `std::env::home_dir` / `dirs::home_dir` — resolve home via `beecode-dirs` (uncached) or `beecode_config::beecode_home` (cached).
- No raw `reqwest::Client::new` / `ClientBuilder::build` (incl. blocking) — build through `beecode_extra_ca` (applies TLS policy + `beecode_EXTRA_CA_BUNDLE`). Localhost/test-only exceptions need a reason comment.

## Workflow notes
- Contributions welcome (`CONTRIBUTING.md`); security reports go to HackerOne per `SECURITY.md`, never public issues.
- License: Apache-2.0 first-party; ported code (codex/opencode tool impls) has crate-local notices — see `crates/codegen/beecode-tools/THIRD_PARTY_NOTICES.md` and root `THIRD-PARTY-NOTICES`.
