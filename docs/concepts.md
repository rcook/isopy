# Concepts

> **Work in progress**: This document is out of date! It uses old terminology!

isopy manages isolated Python environments. This document
describes the core concepts underlying its operation.

## Repository

isopy comes with one preconfigured repository, namely
[Python Standalone Builds][python-standalone-builds]. This is a source
of asset metadata (index) and the assets themselves. Alternative
repositories will eventually be configurable to allow local mirroring
or caching of downloads from the Python Standalone Builds as well as
for air-gapped use of the tool.

## Index

The index is release and asset information downloaded from the
[Python Standalone Builds][python-standalone-builds] project on GitHub
or another repository.

## Named environment

A named environment is an isolated Python runtime environment
(interpreter plus site packages) that is not tied to a specific project
or directory. It has a name and you can use it anywhere.

## Project environment

A project environment has no name and is tied to a particular directory
on your file system so that you can open a shell or run Python scripts
in that environment from anywhere in that directory tree. These
environments are intended to be associated with a single directory tree
which would typically be a single Python project. Like named environments,
they are an isolated Python runtime environment consisting of an
interpreter and site packages. These are typically marked by the presence
of an `.python-version.yaml` file.

## Uses

Think of these as symlinks for environments. With these you can
associate an environment with a given directory without the need for an
intrusive `.python-version.yaml` file.

[python-standalone-builds]: https://github.com/indygreg/python-build-standalone/releases
