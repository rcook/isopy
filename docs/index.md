# isopy

Manages multiple isolated versions of Python obtained from
[Python Standalone Builds][python-build-standalone-releases]

## Linux and macOS (bash)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/rcook/isopy/main/setup/setup | bash
```

## Windows (PowerShell)

```pwsh
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned
& ([ScriptBlock]::Create((iwr -useb https://raw.githubusercontent.com/rcook/isopy/main/setup/setup.ps1)))
```

_[Usage](usage.md) \| [Concepts](concepts.md) \| [Cookbook](cookbook.md) \| [Developers][readme]_

[issues]: https://github.com/rcook/isopy/issues
[python-build-standalone-releases]: https://github.com/indygreg/python-build-standalone/releases
[readme]: https://github.com/rcook/isopy/blob/main/README.md
