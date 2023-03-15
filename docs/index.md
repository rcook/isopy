# isopy

Manages multiple isolated versions of Python obtained from
[Python Standalone Builds][python-build-standalone]

## Install

### Linux and macOS (bash)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/rcook/isopy/main/setup/setup | bash -s -- --stdout
```

### Windows (PowerShell)

```pwsh
& ([ScriptBlock]::Create((iwr https://raw.githubusercontent.com/rcook/isopy/main/setup/setup.ps1)))
```

## Similar to

This tool is similar in philosophy to the following projects:

* [pyenv][pyenv]
* [pyenv-win][pyenv-win]
* [rbenv][rbenv]
* [ruby-build][ruby-build]

This is 99% Python clean-room implementation and doesn't borrow any code from
these projects.

## Developers

See [README](../README.md) for developer guide.

[pyenv]: https://github.com/pyenv/pyenv
[pyenv-win]: https://github.com/pyenv-win/pyenv-win
[python-build-standalone]: https://github.com/indygreg/python-build-standalone
[rbenv]: https://github.com/rbenv/rbenv
[ruby-build] https://github.com/rbenv/ruby-build
