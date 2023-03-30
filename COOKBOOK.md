# Cookbook

## "What versions of Python are availabe?"

```bash
isopyrs available
```

## "I'd like to set up my current project to use Python 3.11.1"

```bash
cd /path/to/project

# Create an .isopy.yaml configuration file
isopyrs new 3.11.1 -t 20230116

# Create the environment based on this configuration
isopyrs init

# Start a shell
isopyrs shell
```
