import sys
from collections import defaultdict

from lib import Mat, StopSearch, none_throws

# parameters
is_example = True
is_hard = True

# derived parameters
diff = 100
if is_hard:
    L = 20
else: 
    L = 2
    
show_hist = False
if is_example:
    show_hist = True
    if is_hard:
        diff = 50
    else:
        diff = 1


b = Mat.from_str(sys.stdin.read())
print(b)

s = none_throws(b.find("S"))
e = none_throws(b.find("E"))

f = defaultdict(int)

def process(b, idx, c):
    if idx == e:
        raise StopSearch()
    return [] if b[idx] == "#" else idx.adj4()


cmax = none_throws(b.bfs(s, process)).dist
ub = cmax - diff
print("cmax=", cmax)

by_cost1 = defaultdict(list)
by_cost2 = defaultdict(list)


def process1(b, idx, c):
    if c + e.l1_dist(idx) > ub:
        raise StopSearch()
    if b[idx] == "#":
        return []
    by_cost1[c].append(idx)
    return idx.adj4()

b.bfs(s, process1)

def process2(b, idx, c):
    if c + s.l1_dist(idx) > ub:
        raise StopSearch()
    if b[idx] == "#":
        return []
    by_cost2[c].append(idx)
    return idx.adj4()

b.bfs(e, process2)

by_cost1 = sorted(by_cost1.items(), key=lambda x: x[0])
by_cost2 = sorted(by_cost2.items(), key=lambda x: x[0])

t = 0
for c1, idxs1 in by_cost1:
    for c2, idxs2 in by_cost2:
        if c1 + c2 > ub:
            break

        for idx1 in idxs1:
            for idx2 in idxs2:
                if idx1 == idx2:
                    continue

                c12 = idx1.l1_dist(idx2)
                if c12 > L:
                    continue

                c = c1 + c12 + c2
                if c > ub:
                    continue
                if show_hist:
                    f[cmax - c] += 1
                t += 1

if show_hist:
    f = [(p[1], p[0]) for p in sorted(f.items(), key=lambda x: x[0])]
    print(f)
print(t)
