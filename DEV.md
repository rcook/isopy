# Developer Guide

This document describes the conventions followed in the isopy Rust source tree. New code should match this style unless there is a clear reason to deviate.

## Toolchain

The project builds and runs on the **stable** toolchain (pinned in `rust-toolchain.toml`). Stable handles `cargo build`, `cargo check`, `cargo test`, and `cargo clippy`.

Formatting requires the **nightly** toolchain, because `rustfmt.toml` uses options (`group_imports`) that are still gated behind `unstable_features`. Stable `rustfmt` parses the file, prints a warning, and formats without those options — so it will silently let non-conforming code through. Always use nightly for formatting:

```bash
cargo +nightly fmt            # format in place
cargo +nightly fmt -- --check # verify, non-zero exit on drift
```

The `precheckin` script already runs `cargo +nightly fmt`; CI should do the same.

### One-time setup

```bash
rustup toolchain install nightly
rustup component add rustfmt --toolchain nightly
```

Only `rustfmt` from nightly is needed — nothing else uses it. The stable pin in `rust-toolchain.toml` still governs every other `cargo` subcommand.

## `use` statements

### Ordering

Within each file, `use` statements appear in three groups, separated by a single blank line, in this order:

1. `use std::…`
2. External crates (alphabetical by crate name)
3. `use crate::…` / `use super::…` — items from the current crate

Within each group, statements are alphabetical. This matches `rustfmt`'s `group_imports = "StdExternalCrate"`, which is enabled in `rustfmt.toml`.

Example (`isopy/src/download.rs`):

```rust
use std::fs::create_dir_all;
use std::path::Path;

use anyhow::{Result, anyhow};
use futures_util::StreamExt;
use isopy_lib::{
    DownloadAssetOptions, Extent, ProgressIndicator, ProgressIndicatorOptionsBuilder,
    error_for_github_rate_limit,
};
use log::info;
use reqwest::Client;
use reqwest::Url as ReqwestUrl;
use reqwest::header::{ACCEPT, USER_AGENT};
use tokio::fs::File as FSFile;
use tokio::io::AsyncWriteExt;
use url::Url;

use crate::constants::ISOPY_USER_AGENT;
```

### Granularity

- One `use` per path. Prefer two lines over one nested list when the parent paths differ:

  ```rust
  // Yes
  use std::fs::File;
  use std::path::Path;

  // No
  use std::{fs::File, path::Path};
  ```

- Group into braces **only** when pulling multiple items from the same immediate parent:

  ```rust
  use anyhow::{Result, anyhow};
  use std::path::{Path, PathBuf};
  ```

- Long brace lists wrap onto multiple lines with a trailing comma, indented four spaces:

  ```rust
  use isopy_lib::{
      DownloadAssetOptions, Extent, ProgressIndicator, ProgressIndicatorOptionsBuilder,
      error_for_github_rate_limit,
  };
  ```

`imports_granularity` is **not** enabled in `rustfmt.toml` — keep one-leaf-per-line for cleaner diffs.

### Leaf imports, unqualified use

Import at the leaf and reference unqualified at the call site:

```rust
// Yes
use std::collections::HashMap;
let m = HashMap::new();

// No
use std::collections;
let m = collections::HashMap::new();

// No
let m = std::collections::HashMap::new();
```

### Intra-crate paths

Use absolute `use crate::…` paths. `super::` is reserved for `use super::*;` inside `#[cfg(test)] mod tests`.

### Aliases (`as`)

Use `as` to resolve name collisions between crates. Current examples:

```rust
use reqwest::Url as ReqwestUrl;
use url::Url;

use std::result::Result as StdResult;
use anyhow::Result;

use tokio::fs::File as FSFile;
use std::fs::File;
```

When both `anyhow::Result` and the standard-library `Result` are needed in the same file, import `anyhow::Result` unaliased and import the standard one as `StdResult`. Do not define private type aliases for this.

### Glob imports

Forbidden outside two specific contexts:

- `use super::*;` at the top of a `#[cfg(test)] mod tests` block.
- `pub use entrypoint::*;` in a plugin crate's `lib.rs` for public re-export.

### Re-exports

Keep re-exports minimal. Each plugin crate exposes its public surface via a single `pub use entrypoint::*;` in `lib.rs`. Internal modules do not re-export.

### Macros

Do not use `#[macro_use] extern crate …`. Rely on `#[derive(...)]`, attribute macros (`#[rstest]`), and `use` for function-like macros where needed.

## Formatting config

`rustfmt.toml` at the repo root:

```toml
unstable_features = true
group_imports = "StdExternalCrate"
```

Both options require nightly rustfmt — see the [Toolchain](#toolchain) section above.
