## `available` command

> **Work in progress**: Many of the commands have crappy names.
> If you can do better, please [file an issue][issues].

Lists Python packages available for download from
[Python Standalone Builds][python-build-standalone-releases]. The
packages are filtered down to the operating system and architecture of
the local machine.

The package index is cached locally since this is fairly expensive to
download and can hit GitHub rate-limiting issues if called too
frequently. isopy will check for updates on each

[issues]: https://github.com/rcook/isopy/issues
[python-build-standalone-releases]: https://github.com/indygreg/python-build-standalone/releases
