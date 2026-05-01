# isopy

The `isopy` command-line tool — a Rust binary for downloading and managing
isolated Python, Java, and Go environments.

This crate contains the CLI entry point. The actual logic lives in:

- [`isopy-lib`](../isopy-lib) — shared plugin infrastructure
- [`isopy-python`](../isopy-python) — Python language plugin
- [`isopy-java`](../isopy-java) — Java language plugin
- [`isopy-go`](../isopy-go) — Go language plugin

## Installation

Install from GitHub releases or build from source with
`cargo install --path isopy`.

See the [project README](../README.md) and the
[GitHub Pages documentation](https://rcook.github.io/isopy/) for usage.

## License

MIT — see [LICENSE](../LICENSE).
