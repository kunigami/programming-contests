import sys
from lib import Mat 

b = Mat.from_str(sys.stdin.read())
v = b.clone().fill(False)

def traverse(b, v, idx):
    if v[idx]:
        return 0, 0
    v[idx] = True
    perimeter = 0
    area = 1
    for nidx in idx.adj4():
        if b.within(nidx) and b[nidx] == b[idx]:
            p, a = traverse(b, v, nidx)
            perimeter += p
            area += a
        else:
            perimeter += 1
    return perimeter, area

t = 0
for idx in b.indices():
    if not v[idx]:
        p, a = traverse(b, v, idx)
        t += p * a

print(t)
