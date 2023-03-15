## `available` command

> **Work in progress**: Many of the commands have crappy names, if you
have any better ideas, [file an issue][issues]

Lists Python packages available for download from
[Python Standalone Builds][python-build-standalone-releases]. The
packages are filtered down to the operating system and architecture of
the local machine. The packages can be furthered filtered by Python
version and/or build tag.

The package index is cached locally since this is fairly expensive to
download and can hit GitHub rate-limiting issues if called too
frequently. Use `--refresh` to force redownload of the index.

| Argument           | Description           |
| ------------------ | --------------------- |
| `--tag`            | Build tag             |
| `--python-version` | Python version        |
| `--[no-]refresh`   | Refresh package index |

Global options `--log-level` and `--cache-dir` also apply.

[issues]: https://github.com/rcook/isopy/issues
[python-build-standalone-releases]: https://github.com/indygreg/python-build-standalone/releases
