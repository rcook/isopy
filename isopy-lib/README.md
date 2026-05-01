# isopy-lib

Shared library for the [`isopy`](../isopy) CLI and its language plugins.

Provides the plugin trait, package-manager context, archive unpacking
(`.tar.gz`, `.tar.zst`, `.zip`), checksum handling, GitHub pagination
helpers, and shared version/tag primitives used by
[`isopy-python`](../isopy-python), [`isopy-java`](../isopy-java), and
[`isopy-go`](../isopy-go).

## Status

Not intended for third-party consumption. The API is internal to the
`isopy` project and may break between patch releases without notice.

## License

MIT — see [LICENSE](../LICENSE).
