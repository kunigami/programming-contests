is_hard = True
import sys
from heapq import heappush, heapreplace

def read_input():
    lines = sys.stdin.readlines()
    return [
        [int(x) for x in line.rstrip().split(',')] for line in lines
    ]

def area(p1, p2):
    l1 = abs(p1[0] - p2[0]) + 1
    l2 = abs(p1[1] - p2[1]) + 1
    return l1 * l2

def solve_easy():
    pts = read_input()
    L = 0
    for i in range(len(pts)):
        for j in range(i + 1, len(pts)):
            L = max(L, area(l1, l2))

    return L

def interval(p1, p2, dim):
    return min(p1[dim], p2[dim]), max(p1[dim], p2[dim])

def edges(pts):
    n = len(pts)
    return ((pts[i], pts[(i + 1) % n]) for i in range(n))

def count_cross(p, pts):
    cross = 0
    for u, v in edges(pts):
        # only looking for horizontal segments
        if u[1] != v[1] or u[1] < p[1]:
            continue
        l, r = interval(u, v, 0)
        if p[0] >= l and p[0] <= r:
            cross += 1

    return cross

# assume s1, s2 = [a, b] with a < b
def overlap(s1, s2):
    return max(s1[0], s2[0]) < min(s1[1], s2[1])


# checks if any edge cuts through
# the rectangle of pi, pj
def cuts(pi, pj, pts):
    b, t = interval(pi, pj, 1)
    l, r = interval(pi, pj, 0)
    for u, v in edges(pts):
        if u[1] == v[1]: # horizontal
            el, er = interval(u, v, 0)
            if u[1] > b and u[1] < t and overlap((el, er), (l, r)):
                return True

        else: # vertical
            eb, et = interval(u, v, 1)
            if u[0] > l and u[0] < r and overlap((eb, et), (b, t)):
                return True
    return False

# whether p inside polygon pts
def is_inside(p, pts):
    return count_cross(p, pts) % 2 == 1

def solve_hard():
    '''
    O(n^3)
    '''
    pts = read_input()
    L = 0
    for i in range(len(pts)):
        pi = pts[i]
        for j in range(i + 1, len(pts)):
            pj = pts[j]
            A = area(pi, pj)
            if A <= L:
                # prune to avoid O(n)
                continue

            # 1. no other edge must 'cut' the rectangle
            if cuts(pi, pj, pts):
                continue

            # 2. the center of rectangle must lie inside
            #    the polygon

            # shift by (0.1, 0.1) to avoid corner cases
            mid = ((pi[0] + pj[0])/2 + 0.1, (pi[1] + pj[1])/2 + 0.1)
            if is_inside(mid, pts):
                L = A

    return L


print(solve_hard() if is_hard else solve_easy())
