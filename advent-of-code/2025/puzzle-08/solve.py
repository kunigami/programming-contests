is_hard = True
import sys

def read_input():
    lines = sys.stdin.readlines()
    return [
        [int(x) for x in line.rstrip().split(',')] for line in lines
    ]

class UnionFind:
    def __init__(self, n):
        self.p = list(range(n))
        self.s = [1]*n

    def find(self, x):
        p = self.p
        if p[x] != x:
            p[x] = self.find(p[x])
        return p[x]

    def union(self, x, y):
        px, py = self.find(x), self.find(y)
        if px == py:
            return

        s, p = self.s, self.p
        if s[px] < s[py]:
            px, py = py, px

        s[px] += s[py]
        p[py] = p[px]
        return s[px]


def d2(p1, p2):
    df = (p1[i]-p2[i] for i in range(3))
    return sum(x**2 for x in df)

def get_sorted_distances(pts):
    h = []
    for i in range(len(pts)):
        for j in range(i + 1, len(pts)):
            h.append((d2(pts[i], pts[j]), i, j))

    return sorted(h)


def solve_easy():
    """
    O(n^2 log n), O(n^2) space

    can be optimized to O(n^2 log N) and O(N)
    by using a fixed size heap
    """
    pts = read_input()

    N = 1000
    h = get_sorted_distances(pts)
    h = h[:N]

    u = UnionFind(len(pts))
    for _, i, j in h:
        u.union(i, j)

    szs = []
    for i in range(len(pts)):
        if u.p[i] == i:
            szs.append(u.s[i])
    szs = sorted(szs, reverse=True)[:3]

    return szs[0]*szs[1]*szs[2]

def solve_hard():
    pts = read_input()

    h = get_sorted_distances(pts)

    u = UnionFind(len(pts))
    for _, i, j in h:
        sz = u.union(i, j)
        if sz == len(pts):
            return pts[i][0] * pts[j][0]

print(solve_hard() if is_hard else solve_easy())
