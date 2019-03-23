#!/usr/bin/env bash

_g_completion() {
    COMPREPLY=($(/s/Dev/yaml-complete/target/debug/yaml-complete ${COMP_WORDS[@]}))
}

complete -F _g_completion g
