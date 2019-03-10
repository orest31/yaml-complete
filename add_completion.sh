_test_completion() {
    COMPREPLY=($(/usr/bin/yaml-complete ${COMP_WORDS[@]}))
}

complete -F _test_completion test
