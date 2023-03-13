# isopy

Isolated Python environment tool to download and manage Python builds
downloaded from [Python Standalone Builds][python-build-standalone].

## Install

TBD: Install prebuilt binaries

## Set up development environment

```bash
cd /path/to/workspace
git clone git@github.com:rcook/isopy.git
cd isopy
./isopy-bootstrap
```

## Open development shell

```bash
cd /path/to/workspace/isopy
./isopy shell
python -m pip install --upgrade pip
python -m pip install -r requirements.txt
```

## Build self-contained executable

```bash
cd /path/to/workspace/isopy
./isopy exec pyinstaller isopy.spec
```

## Usage

### `debug` subcommand

Dumps out useful runtime troubleshooting information

### `available` subcommand

Lists versions of Python available for download from
[python-build-standalone][python-build-standalone] optionally filtered
by Python version and/or tag. The release index is cached locally since
this is fairly expensive to download. Use `--refresh` to force download
of the index.

### `download` subcommand

TBD

### `downloaded` subcommand

TBD

### `list` subcommand

TBD

### `new` subcommand

TBD

### `init` subcommand

TBD

### `create` subcommand

TBD

### `info` subcommand

TBD

### `shell` subcommand

TBD

### `exec` subcommand

TBD

### `wrap` subcommand

TBD

[python-build-standalone]: https://github.com/indygreg/python-build-standalone/releases