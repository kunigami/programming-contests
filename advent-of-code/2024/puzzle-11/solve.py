import sys
from collections import Counter, defaultdict

lst = [int(i) for i in sys.stdin.read().split()]
h = Counter(lst)

def transform(v):
    if v == 0:
        return [1]
    vs = str(v)
    n = len(vs)
    if n % 2 == 0:
        return [int(vs[: n // 2]), int(vs[n // 2 :])]
    return [v * 2024]


def blink(h):
    results = defaultdict(int)
    for k, v in h.items():
        k2s = transform(k)
        for k2 in k2s:
            results[k2] += v
    return results


for i in range(25):
    h = blink(h)

print(sum(h.values()))
