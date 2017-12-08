#!/usr/bin/env bash

declare -a ANSWERS
readarray -t ANSWERS < answers.txt

i=1
for expected in "${ANSWERS[@]}"; do
    program="./target/release/day${i}"
    result="$($program)"
    if [[ "$expected" != "$result" ]]; then
       echo "$program failed, expected $expected got $result" >&2
       exit 1
    fi
    ((i++))
done

