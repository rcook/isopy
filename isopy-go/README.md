# isopy-go

Go language plugin for the [`isopy`](../isopy) CLI.

Downloads and manages Go toolchain releases, parsing `go*` version
strings (including prerelease markers like `rc` and `beta`).

Exposes a single factory, `new_plugin(moniker)`, consumed by the `isopy`
binary.

## Status

Not intended for third-party consumption. The API is internal to the
`isopy` project and may break between patch releases without notice.

## License

MIT — see [LICENSE](../LICENSE).
