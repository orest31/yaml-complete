# Completions

## Adding completion to bashrc

```sh
_test_completion() {
    COMPREPLY=($(/usr/bin/yaml-complete ${COMP_WORDS[@]}))
}

complete -F _test_completion test
```
