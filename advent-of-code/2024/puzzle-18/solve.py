import sys
from lib import Idx, Mat, StopSearch
from typing import Iterable

def parse(line):
    x, y = [int(v) for v in line.split(',')]
    return Idx(y, x)

pts = [parse(line) for line in sys.stdin.readlines()]

N = 71
M = 1024
b = Mat.new(N, N, '.')

for idx in pts[:M]:
    b[idx] = '#'

def process(b: Mat, idx: Idx, c: int) -> Iterable[Idx]:
    if idx == Idx(N - 1, N - 1):
        print('cost', c)
        raise StopSearch
    if b[idx] == '#':
        return []
    return idx.adj4()

b.bfs(Idx(0, 0), process)
