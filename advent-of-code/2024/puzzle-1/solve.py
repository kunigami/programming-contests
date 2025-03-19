import sys
from collections import Counter


def solve(l1, l2):
    l1 = sorted(l1)
    l2 = sorted(l2)
    print(l1, l2)
    s = 0
    for v1, v2 in zip(l1, l2):
        s += abs(v1 - v2)
    return s


def solve2(l1, l2):
    cnt_map = Counter(l2)
    sim_score = 0
    for v1 in l1:
        sim_score += cnt_map.get(v1, 0) * v1
    return sim_score


input_data = sys.stdin.readlines()
l1 = []
l2 = []
for row in input_data:
    [v1, v2] = row.strip().split("   ")
    l1.append(int(v1))
    l2.append(int(v2))

print(solve2(l1, l2))
