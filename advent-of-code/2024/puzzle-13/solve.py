import re
import sys

input = sys.stdin.read().strip()
lines = [line for line in input.split("\n")]
pattern = re.compile(r"(\w+(?: \w+)?): X([+=-]\d+), Y([+=-]?\d+)")

t = 0


def parse_line(line):
    match = pattern.match(line)
    assert match
    _label, x_str, y_str = match.groups()
    x = int(x_str.replace("=", ""))
    y = int(y_str.replace("=", ""))
    return x, y


hard = False
l = (len(lines) + 1) // 4
for i in range(l):
    l1, l2, l3 = lines[i * 4 : i * 4 + 3]

    x1, y1 = parse_line(l1)
    x2, y2 = parse_line(l2)
    x, y = parse_line(l3)

    if hard:
        x += 10000000000000
        y += 10000000000000

    b1 = x1 * y - y1 * x
    b2 = x1 * y2 - y1 * x2
    print(b1, b2, b1 % b2)
    if b1 % b2 != 0:
        continue
    b = b1 // b2
    a1 = x - b * x2
    if a1 % x1 != 0:
        continue
    a = a1 // x1

    t += 3 * a + b

print(t)
