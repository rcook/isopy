# Basic shell integration

## Enable tab completion (bash)

Add something like the following to your shell startup script:

```bash
function __temp_isopy {
  if [ -d "$HOME/.isopy" ]; then
    export PATH=$HOME/.isopy/bin:$PATH
    if command -v isopy &> /dev/null; then
      local isopy_completions_file=$(mktemp)
      isopy completions --shell bash > $isopy_completions_file
      source $isopy_completions_file
      rm $isopy_completions_file
    fi
  fi
}
__temp_isopy
unset -f __temp_isopy
```

Similar mechanisms exist to add tab completion to other shells.

## Add info to prompt (bash)

```bash
export PS1="\$(isopy prompt --after ' ')\$ "
```
