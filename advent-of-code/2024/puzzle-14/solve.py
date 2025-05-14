import re
import sys

a = sys.stdin.read().strip()
b = [line for line in a.split("\n")]
pattern = re.compile(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")

WIDTH = 101
HEIGHT = 103
# WIDTH = 11
# HEIGHT = 7
N = 100
HWIDTH = WIDTH // 2
HHEIGHT = HEIGHT // 2


def parse_line(line):
    match = pattern.match(line)
    assert match
    x0, y0, vx, vy = match.groups()
    return int(x0), int(y0), int(vx), int(vy)


rows = []
for line in b:
    rows.append(parse_line(line))

N = 100000
max_max_len = 0
for n in range(N):
    if n % 1000 == 0:
        print(n)
    B = []
    for i in range(HEIGHT):
        B.append(["."] * WIDTH)

    q = [[0, 0], [0, 0]]

    hist = [0] * HEIGHT

    for row in rows:

        x0, y0, vx, vy = row
        # print(x0, y0, vx, vy)
        x = (x0 + vx * n) % WIDTH
        if x < 0:
            x += WIDTH
        y = (y0 + vy * n) % HEIGHT
        if y < 0:
            y += HEIGHT

        B[y][x] = "#"

        if x == HWIDTH or y == HHEIGHT:
            continue

        qy = 0 if y < HHEIGHT else 1
        qx = 0 if x < HWIDTH else 1
        q[qy][qx] += 1

    max_len = 0
    for x in range(WIDTH):
        curr_len = 0
        for y in range(HEIGHT):
            if B[y][x] != "#":
                max_len = max(max_len, curr_len)
                curr_len = 0
            else:
                curr_len += 1

    if max_len > max_max_len:
        max_max_len = max_len
        print(n, max_max_len)
        print("=================================")
        for i in range(HEIGHT):
            l = "".join(B[i])
            print(l)

        import time

        time.sleep(3)


# print(q)
