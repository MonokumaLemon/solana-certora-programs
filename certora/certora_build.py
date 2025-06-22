#!/usr/bin/env python3

import subprocess
import json
import sys

# Run the build and capture the output
try:
    result = subprocess.run(
        ["cargo", "certora-sbf", "--json"],
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )

    # Output the JSON to stdout
    print(result.stdout)
except subprocess.CalledProcessError as e:
    print("Build failed:", e.stderr, file=sys.stderr)
    sys.exit(1)
