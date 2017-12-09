#!/usr/bin/env python3

import json
import subprocess
import sys
from pathlib import Path
from itertools import zip_longest


def run_test(bin, answers):
    path = Path("./target/release") / bin
    print("Testing {}... ".format(path), end="")
    try:
        output = subprocess.check_output((str(path),)) \
                        .decode("utf-8").splitlines()
    except subprocess.CalledProcessError as e:
        print("Failed")
        print("Program returned {}".format(e.returncode))
        sys.exit(1)
    for result, answer in zip_longest(output, answers):
        if result != answer:
            print("Failed")
            print("Expected {} received {}".format(answer, result))
            print(output)
            sys.exit(1)
    print("OK")

def main(answers):
    with open(answers, "r") as handle:
        answers = json.load(handle)
    for bin, lines in answers.items():
        run_test(bin, lines)


if __name__ == "__main__":
    main(sys.argv[1])
