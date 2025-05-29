import sys
from lib import Idx, Mat, StopSearch
from typing import Iterable

def parse(line):
    x, y = [int(v) for v in line.split(',')]
    return Idx(y, x)

pts = [parse(line) for line in sys.stdin.readlines()]

N = 71

def process(b: Mat, idx: Idx, c: int) -> Iterable[Idx]:
    if idx == Idx(N - 1, N - 1):
        raise StopSearch
    if b[idx] == '#':
        return []
    return idx.adj4()

def solve(m):
    b = Mat.new(N, N, '.')
    for idx in pts[:m]:
        b[idx] = '#'

    return b.bfs(Idx(0, 0), process) is not None
    
def bsearch():
    lo = 0
    hi = len(pts)
    while lo < hi:
        mid = (lo + hi) // 2
        if solve(mid):
            lo = mid + 1
        else:
            hi = mid
    return lo

m = bsearch()
print(pts[m-1])
