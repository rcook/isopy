## `packages` command

_Lists local and/or remote packages_

Lists locally cached packages or packages available for download from
sources such as [Python Standalone Builds][python-build-standalone-releases]
or [Adoptium][adoptium] filtered down to the operating system and
architecture of the local machine.

The package index is cached locally since this is can be fairly
expensive to download and can hit GitHub rate-limiting issues if called
too frequently. isopy will check for updates on each.

[adoptium]: https://adoptium.net/
[python-build-standalone-releases]: https://github.com/astral-sh/python-build-standalone/releases
