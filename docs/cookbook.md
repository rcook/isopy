# Cookbook

## "What packages are available for download?"

```bash
isopy available
```

## "What packages have I downloaded locally?"

```bash
isopy downloaded
```

## "I'd like to set up my current project to use Python 3.11.1"

```bash
# Creates an .isopy.yaml configuration file
isopy add python:3.11.1

# Installs the package into an environment associated with this directory
isopy install-project

# Start a shell in this environment
isopy shell
```
