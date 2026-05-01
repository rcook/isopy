# isopy-java

Java language plugin for the [`isopy`](../isopy) CLI.

Downloads and manages JDK releases from the
[Adoptium](https://adoptium.net) API, parsing `jdk-*` version strings
and Maven version ranges for dependency resolution.

Exposes a single factory, `new_plugin(moniker)`, consumed by the `isopy`
binary.

## Status

Not intended for third-party consumption. The API is internal to the
`isopy` project and may break between patch releases without notice.

## License

MIT — see [LICENSE](../LICENSE).
