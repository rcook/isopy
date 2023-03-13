# isopy

Isolated Python environment tool to download and manage Python builds
downloaded from [Python Standalone Builds][python-build-standalone].

## Install

TBD: Install prebuilt binaries

## Bootstrap development environment

```bash
# Report version of system-wide Python interpreter
python3 --version

# Clone source code and bootstrap
cd /path/to/workspace
git clone git@github.com:rcook/isopy.git
cd isopy
./bootstrap

# Examine the contents of the isopy wrapper script
cat -n ~/.local/bin/isopy
     1	#!/bin/bash
     2	set -euo pipefail
     3	project_dir=$HOME/src/isopy
     4	PATH=$HOME/.isopy/envs/isopy/cpython-3.11.1+20230116/bin:$PATH \
     5	  PYTHONPATH=$project_dir \
     6	  python3 $project_dir/isopy_bin/main.py "$@"

# Now you should have an "isopy" script available on PATH
isopy --help

# Run a local isopy shell
isopy shell -e isopy

# In isopy shell, report version of isopy-installed Python interpreter
$ python3 --version
```

Note that the `PATH=$HOME/.isopy/envs/isopy/cpython-3.11.1+20230116/bin:$PATH`
is the critical bit. If you want to use an environment globally (e.g.
from your `.bashrc` add a line to modify your `PATH` environment
variable in a similar way), e.g.

```bash
export PATH=$HOME/.isopy/envs/isopy/cpython-3.11.1+20230116/bin:$PATH
```

## Open development shell

```bash
cd /path/to/workspace/isopy
isopy shell -e isopy
python -m pip install --upgrade pip
python -m pip install -r requirements.txt
```

## Build self-contained executable

```bash
cd /path/to/workspace/isopy
isopy exec -e isopy pyinstaller isopy.spec
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