# isopy

[![CI](https://github.com/rcook/isopy/actions/workflows/ci.yaml/badge.svg)][ci-workflow]
[![Release](https://github.com/rcook/isopy/actions/workflows/release.yaml/badge.svg)][release-workflow]
[![Latest release](https://img.shields.io/github/v/tag/rcook/isopy)][latest-release]

[GitHub Pages documentation][github-pages] including [usage][usage]

Isolated Python environment tool to download and manage Python builds
downloaded from [Python Standalone Builds][python-build-standalone-releases]

Released under [MIT License](LICENSE)

## Installation

See [GitHub Pages documentation][github-pages] for installation and usage
instructions.

## Development

isopy is written in [Rust][rust], built using [Cargo][cargo-book] installed via [rustup]
[rustup] and distributed via [GitHub][releases]. It's based on an original
[reference implementation in Python](py).

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

## Similar to

This tool is similar in philosophy to the following projects:

* [pyenv][pyenv]
* [pyenv-win][pyenv-win]
* [rbenv][rbenv]
* [ruby-build][ruby-build]

[cargo-book]: https://doc.rust-lang.org/cargo/
[ci-workflow]: https://github.com/rcook/isopy/actions/workflows/ci.yaml
[github-pages]: https://rcook.github.io/isopy/
[latest-release]: https://github.com/rcook/isopy/releases
[pyenv]: https://github.com/pyenv/pyenv
[pyenv-win]: https://github.com/pyenv-win/pyenv-win
[python-build-standalone-releases]: https://github.com/astral-sh/python-build-standalone/releases
[rbenv]: https://github.com/rbenv/rbenv
[release-workflow]: https://github.com/rcook/isopy/actions/workflows/release.yaml
[releases]: https://github.com/rcook/isopyrs/releases
[ruby-build]: https://github.com/rbenv/ruby-build
[rust]: https://www.rust-lang.org/
[rustup]: https://rustup.rs/
[usage]: https://rcook.github.io/isopy/usage
