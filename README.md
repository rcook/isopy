# isopy

Isolated Python environment tool

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
