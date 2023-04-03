# Cookbook

## "What versions of Python are availabe?"

```bash
isopy available
```

## "I'd like to set up my current project to use Python 3.11.1"

```bash
cd /path/to/project

# Create an .isopy.yaml configuration file
isopy new 3.11.1 -t 20230116

# Create the environment based on this configuration
isopy init

# Start a shell
isopy shell
```

## "Show version of Python in my environment"

```bash
cd /path/to/project
isopy exec python3 --version
```
