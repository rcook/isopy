# Basic shell integration

Something like the following should do the trick:

```bash
function __temp_isopy {
  if [ -d "$HOME/.isopy" ]; then
    function cd_isopy {
      builtin cd "$@"
      isopy prompt
    }
    alias cd='cd_isopy'
    export PATH=$HOME/.isopy/bin:$PATH
  fi
}
__temp_isopy
unset -f __temp_isopy
```
