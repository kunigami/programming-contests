import re
import sys


input_data = sys.stdin.read()

print(input_data)


def solve(s):
    pattern = r"(mul\((\d+),(\d+)\)|do\(\)|don\'t\(\))"
    matches = re.findall(pattern, s)
    t = 0
    enabled = True
    for match in matches:
        c, l, r = match
        if c == "don't()":
            enabled = False
        elif c == "do()":
            enabled = True
        elif enabled:
            t += int(l) * int(r)
        print(match)
    return t


print(solve(input_data))
