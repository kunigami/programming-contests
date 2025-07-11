from collections import defaultdict
import sys

lines = sys.stdin.read().splitlines()
lines = [l.strip().split("-") for l in lines]

pairs = set()
adj = defaultdict(set)
for l in sys.stdin.readlines():
    x, y = l.strip().split("-")
    adj[x].add(y)
    adj[y].add(x)
    pairs.add((x, y))


triplets = set()
ls = []
for k, v in adj.items():
    ls.append(len(v))
    if not k.startswith("t"):
        continue
    for i in range(len(v)):
        for j in range(i + 1, len(v)):
            x, y = v[i], v[j]
            if x < y:
                y, x = x, y
            if (x, y) in pairs:
                sorted_t = tuple(sorted([x, y, k]))
                triplets.add(sorted_t)

sorted_ls = sorted(ls, reverse=True)
print(len(adj))
