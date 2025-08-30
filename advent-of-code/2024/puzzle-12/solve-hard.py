import sys

from lib import Idx, Mat

b = Mat([line for line in sys.stdin.read().strip().split("\n")])


def is_neighbor(idx, nidx):
    return b.within(nidx) and b[idx] == b[nidx]


def find_borders():
    e = Mat.new(b.h(), b.w(), set)
    # Find sides of the square that form a border
    for idx in b.indices():
        for i, nidx in enumerate(idx.adj4()):
            if not is_neighbor(idx, nidx):
                e[idx].add(i)
    # Exclude borders accounted for in successors
    for idx in b.indices():
        for nidx in idx.adj([(0, 1), (1, 0)], filter=is_neighbor):
            e[idx] -= e[nidx]
    return e


def traverse(b, v, e, idx):
    if v[idx]:
        return 0, 0

    v[idx] = True
    perimeter = len(e[idx])
    area = 1
    for nidx in idx.adj4(filter=is_neighbor):
        p, a = traverse(b, v, e, nidx)
        perimeter += p
        area += a

    return perimeter, area


e = find_borders()

v = Mat.new(b.w(), b.h(), False)
t = 0
for idx in b.indices():
    p, a = traverse(b, v, e, idx)
    t += p * a

print(t)
