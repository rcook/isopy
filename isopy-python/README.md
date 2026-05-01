# isopy-python

Python language plugin for the [`isopy`](../isopy) CLI.

Downloads and manages [python-build-standalone](https://github.com/astral-sh/python-build-standalone)
releases, parsing their versioning scheme (including prereleases and
build-date labels), verifying checksums, and installing into isolated
environments.

Exposes a single factory, `new_plugin(moniker)`, consumed by the `isopy`
binary.

## Status

Not intended for third-party consumption. The API is internal to the
`isopy` project and may break between patch releases without notice.

## License

MIT — see [LICENSE](../LICENSE).
