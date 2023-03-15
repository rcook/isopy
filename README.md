# isopy

[GitHub Pages documentation][github-pages]

Isolated Python environment tool to download and manage Python builds
downloaded from [Python Standalone Builds][python-build-standalone-releases]

Released under [MIT License](LICENSE)

## Installation

See [GitHub Pages documentation][github-pages] for installation instructions.

## Bootstrapping development environment

If you'd like to contribute to development of isopy, please follow the
developer bootstrapping instructions below.

```bash
# Report version of system-wide Python interpreter
python3 --version

# Clone source code and bootstrap
cd /path/to/workspace
git clone git@github.com:rcook/isopy.git
cd isopy

# Run the local (development) isopy
./isopy --help

# Configure this project to use the named environment "isopy"
./isopy use isopy

# Run a local isopy shell
./isopy shell

# In isopy shell, report version of isopy-installed Python interpreter
$ python3 --version
```

`$` indicates that the command is being run in the child shell.

## Making an isopy environment available on shell startup

Note that putting the appropriate Python directory at the head of your
`PATH` is all you really need to do:

Linux/macOS (bash or similar)

```bash
export PATH=$HOME/.isopy/envs/isopy/python/bin:$PATH
```

Windows (PowerShell)

```pwsh
$env:Path = ~\.isopy\envs\isopy\python + ';' + ~\.isopy\envs\isopy\python\Scripts + ';' + $env:Path
```

Windows (Command Script)

```pwsh
set PATH=%USERPROFILE%\.isopy\envs\isopy\python;%USERPROFILE%\.isopy\envs\isopy\python\Scripts;%PATH%
```

You can also call out to the `shell` command instead at the end of your
`.bashrc` or similar shell configuration file:

```bash
if [ "$ISOPY_ENV" == '' ]; then
     isopy shell -e isopy
fi
```

## What does this all do?

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

## Similar to

This tool is similar in philosophy to the following projects:

* [pyenv][pyenv]
* [pyenv-win][pyenv-win]
* [rbenv][rbenv]
* [ruby-build][ruby-build]

isopy is 99% Python clean-room implementation and doesn't borrow any code from
these projects.

[github-pages]: https://rcook.github.io/isopy/
[pyenv]: https://github.com/pyenv/pyenv
[pyenv-win]: https://github.com/pyenv-win/pyenv-win
[python-build-standalone-releases]: https://github.com/indygreg/python-build-standalone/releases
[rbenv]: https://github.com/rbenv/rbenv
[ruby-build]: https://github.com/rbenv/ruby-build
