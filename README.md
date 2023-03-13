# isopy

Isolated Python environment tool to download and manage Python builds
downloaded from [Python Standalone Builds][python-build-standalone]

Released under [MIT License](LICENSE)

## Installation

TBD: Install prebuilt binaries: eventually we'll build self-contained
downloads using [PyInstaller][pyinstaller] or similar. For now, these
instructions will assume that you're a developer, so please follow the
bootstrapping instructions below.

## Bootstrapping development environment

```bash
# Report version of system-wide Python interpreter
python3 --version

# Clone source code and bootstrap
cd /path/to/workspace
git clone git@github.com:rcook/isopy.git
cd isopy
./bootstrap

# Examine the contents of the isopy wrapper script
which isopy
cat -n ~/.local/bin/isopy
     1	#!/bin/bash
     2	set -euo pipefail
     3	project_dir=$HOME/src/isopy
     4	PATH=$HOME/.isopy/envs/isopy/cpython-3.11.1+20230116/bin:$PATH \
     5	  PYTHONPATH=$project_dir \
     6	  python3 $project_dir/isopy_bin/main.py "$@"

# Now you should have an "isopy" script available on PATH
isopy --help

# Configure this project to use the named environment "isopy"
isopy use isopy

# Run a local isopy shell
isopy shell

# In isopy shell, report version of isopy-installed Python interpreter
$ python3 --version
```

`$` indicates that the command is being run in the child shell.

### Making an isopy environment available on shell startup

Note that the `PATH=$HOME/.isopy/envs/isopy/cpython-3.11.1+20230116/bin:$PATH`
in the output shown above is the critical bit. If you want to use an
environment globally (e.g. from your `.bashrc` add a line to modify your
`PATH` environment variable in a similar way), e.g.

```bash
export PATH=$HOME/.isopy/envs/isopy/cpython-3.11.1+20230116/bin:$PATH
```

You may, instead, be able to call out to the `shell` command instead at
the end of your `.bashrc` file:

```bash
isopy shell -e isopy
```

_Note that this has not been tested and may or may not work propertly
since the wrapper scripts use [`exec`][man-exec]._

### What does this all do?

* Intended to minimally impact your system
* Tool downloads and manages standalone Python builds into a hidden
directory (`$HOME/.isopy`) in your home directory
* Allows fully isolated Python sessions simply by putting the right
directory on the system `PATH`
* Site packages are installed in the isolated Python directory
completely separate from any system Python installations you might have

And that's it. It makes no permanent alterations to your system. It does
not modify anything outside its own project directory except for
creating the `isopy` wrapper script on your search `PATH`, which you
don't actually need unless you want to call the tool using `isopy` from
outside the project directory.

## Open development shell

This makes use of the `shell` subcommand and tells isopy to open a
child shell (Bash or similar or Linux or macOS, PowerShell on Windows)
using the environment named `isopy`:

```bash
cd /path/to/workspace/isopy
isopy shell -e isopy
$ python3 -m pip install --upgrade pip
$ python3 -m pip install -r requirements.txt
```

If you would like a dedicated environment for your Python project, do
the following:

```bash
cd /path/to/workspace
git init my-python-project
cd my-python-project
isopy new 3.11.1
isopy init
isopy shell
$ python3 -m pip install --upgrade pip
$ python3 -m pip install -r requirements.txt
```

From now on, you'll just need to run `isopy shell` to run in the
appropriate environment:

```bash
cd /path/to/workspace/my-python-project
$ isopy shell
```

## Build self-contained executable

```bash
cd /path/to/workspace/isopy
isopy exec -e isopy pyinstaller isopy.spec
```

## Usage

### `debug` subcommand

Dumps out useful runtime troubleshooting information.

### `available` subcommand

Lists versions of Python available for download from
[python-build-standalone][python-build-standalone] optionally filtered
by Python version and/or tag. The release index is cached locally since
this is fairly expensive to download. Use `--refresh` to force download
of the index.

### `download` subcommand

Downloads a Python distribution from
[python-build-standalone][python-build-standalone] optionally filtered
by Python version and/or tag without extracting it to an environment.

### `downloaded` subcommand

Lists locally downloaded Python distributions.

### `list` subcommand

Lists environments.

### `new` subcommand

TBD

### `init` subcommand

TBD

### `use` subcommand

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

[man-exec]: https://linuxcommand.org/lc3_man_pages/exech.html
[pyinstaller]: https://pyinstaller.org/
[python-build-standalone]: https://github.com/indygreg/python-build-standalone/releases
