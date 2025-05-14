import sys
from lib import Idx, Mat 

b = Mat([line for line in sys.stdin.read().strip().split("\n")])
e = Mat.new(b.h(), b.w(), set)
v = Mat.new(b.w(), b.h(), False)

def populate(idx):
    if v[idx]:
        return
    v[idx] = True

    for i, nidx in enumerate(idx.adj4()):
        if (not b.within(nidx)) or b[nidx] != b[idx]:
            e[idx].add(i)
        else:
            populate(nidx)


def traverse(b, v, idx):
    if v[idx]:
        return 0, 0
    v[idx] = True
    perimeter = 0
    area = 1
    covered = set()
    for di, dj in [(-1, 0), (0, -1)]:
        nidx = idx + Idx(di, dj)
        if b.within(nidx) and b[nidx] == b[idx]:
            covered = covered | e[nidx]

    for nidx in idx.adj4():        
        if b.within(nidx) and b[nidx] == b[idx]:
            p, a = traverse(b, v, nidx)
            perimeter += p
            area += a
    perimeter += len(e[idx] - covered)

    return perimeter, area

for idx in b.indices():
    if not v[idx]:
        populate(idx)

v.fill(False)
t = 0
for idx in b.indices():
    if v[idx]:
        continue
    p, a = traverse(b, v, idx)
    t += p * a

print(t)
