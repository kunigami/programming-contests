import sys
from collections import defaultdict

input_data = [r.strip() for r in sys.stdin.readlines()]
coords_by_freq = defaultdict(list)

n = len(input_data)
m = len(input_data[0])
for i, row in enumerate(input_data):
    for j, cell in enumerate(row):
        if cell != ".":
            coords_by_freq[cell].append((i, j))


def is_within_bounds(i, j):
    return i >= 0 and j >= 0 and i < n and j < m


ans = set()
for coords in coords_by_freq.values():
    for i1, j1 in coords:
        for i2, j2 in coords:
            if (i1, j1) == (i2, j2):
                continue
            di = i2 - i1
            dj = j2 - j1
            ian = i1 - di
            jan = j1 - dj
            if is_within_bounds(ian, jan):
                ans.add((ian, jan))

print(len(ans))
