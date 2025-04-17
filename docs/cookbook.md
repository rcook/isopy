# Cookbook

## "What packages are available for download?"

```bash
isopy packages --remote
```

## "What packages have I downloaded locally?"

```bash
isopy packages --local
```

## "I'd like to set up my current project to use Python 3.11.1"

```bash
# Creates an .isopy.yaml configuration file in current directory
isopy project python:3.11.1

# Installs the package into an environment associated with this directory
isopy init --download

# Starts a shell in this environment
isopy sh
```
