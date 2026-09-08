<div align="center">

<img src="assets/logo/wimo.svg" alt="WIMO logo" width="120">

# WIMO

**Your open-source AI developer workstation** — a full-screen TUI that
understands your codebase, edits files, runs shell commands, searches the
web, and manages long-running tasks.

Interactively · Headlessly (scripting / CI) · Embedded via ACP

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.94-orange.svg)](rust-toolchain.toml)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux-lightgrey.svg)](#building-from-source)

[Quickstart](#quickstart) · [Build from source](#building-from-source) ·
[Development](#development) · [Docs](#documentation) · [Contact](#contact)

</div>

---

## Quickstart

```sh
curl -fsSL https://x.ai/cli/install.sh | bash   # macOS / Linux / Git Bash
irm https://x.ai/cli/install.ps1 | iex          # Windows PowerShell
wimo --version
```

On first launch your browser opens to authenticate.

## Building from source

Requirements: pinned Rust toolchain (`rust-toolchain.toml`, auto-installed by
`rustup`) + [DotSlash](https://dotslash-cli.com) on `PATH` **before** building
(proto codegen via `bin/protoc` needs it; falls back to `protoc` on `PATH`).

```sh
cargo install dotslash
dotslash --help                                  # sanity check
cargo run -p "wimoai-wimo-pager-bin"            # build + launch the TUI
cargo build -p "wimoai-wimo-pager-bin" --release # binary: target/release/wimoai-wimo-pager (ships as `wimo`)
```

> macOS and Linux are supported. Windows builds are best-effort.

## Development

```sh
cargo check -p "<crate>"            # fast validation — always scope to one crate
cargo test -p "wimoai-wimo-config" # per-crate tests (never bare `cargo test`)
cargo clippy -p "<crate>"           # lint rules: clippy.toml
cargo fmt --all                     # format
```

> Root `Cargo.toml` is generated — edit per-crate `Cargo.toml` files only.
> Package names contain a space, so quote `-p` args. See [AGENTS.md](AGENTS.md).

## Repository layout

| Path | Contents |
|------|----------|
| `crates/codegen/wimoai-wimo-pager-bin/` | Composition root — builds the `wimo` binary |
| `crates/codegen/wimoai-wimo-pager/` | The TUI: scrollback, prompt, modals, rendering |
| `crates/codegen/wimoai-wimo-shell/` | Agent runtime + headless / ACP entry points |
| `crates/codegen/wimoai-wimo-tools/` | Tool implementations (terminal, edit, search, …) |
| `crates/codegen/wimoai-wimo-workspace/` | Filesystem, VCS, execution, checkpoints |
| `crates/common/`, `crates/build/`, `prod/mc/` | Shared leaf crates |
| `third_party/` | Vendored upstream sources (don't refactor) |

## Documentation

Full user guide ships in-crate:
[`crates/codegen/wimoai-wimo-pager/docs/user-guide/`](crates/codegen/wimoai-wimo-pager/docs/user-guide/)
— setup, shortcuts, slash commands, config, theming, MCP, skills, plugins,
hooks, headless mode, sandboxing.

## Contact

Maintained by **Mohamed El Haddioui**.

[![LinkedIn](https://img.shields.io/badge/LinkedIn-Connect-0A66C2?logo=linkedin)](https://www.linkedin.com/in/mohamed-el-haddioui-ba8ba8170/)
[![Portfolio](https://img.shields.io/badge/Portfolio-Visit-38BDF8?logo=googlechrome)](https://mohamedelhaddioui.netlify.app/)
[![Email](https://img.shields.io/badge/Email-Contact-EA4335?logo=gmail)](mailto:mohamedelhaddioui99@gmail.com)

## License

Apache-2.0 — see [LICENSE](LICENSE). Third-party attributions:
[THIRD-PARTY-NOTICES](THIRD-PARTY-NOTICES). Security reports:
[SECURITY.md](SECURITY.md) (HackerOne, never public issues).
