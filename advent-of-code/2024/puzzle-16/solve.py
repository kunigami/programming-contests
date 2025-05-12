import sys
import heapq
from collections import defaultdict

class Board:
    def __init__(self, b):
        self._b = b
        
    def __getitem__(self, p):
        return self._b[p[0]][p[1]]

    def __setitem__(self, p, v):
        self._b[p[0]][p[1]] = v

    def find(self, c):
        for i in range(len(self._b)):
            for j in range(len(self._b[0])):
                if self._b[i][j] == c:
                    return i, j
        raise Exception("no")

    def is_valid(self, p):
        return (
            p[0] >= 0 and p[0] < len(self._b) and 
            p[1] >= 0 and p[1] < len(self._b[0]) and 
            self[p] != '#'
        )

    def print(self):
        for row in self._b:
            print("".join(row))
        print("\n")

b = Board([list(line.strip()) for line in sys.stdin.readlines()])

left = lambda p: (p[0], p[1], (p[2] + 3) % 4)
right = lambda p: (p[0], p[1], (p[2] + 1) % 4)

def next(p):
    di, dj = [(-1, 0), (0, 1), (1, 0), (0, -1)][p[2]]
    return (p[0] + di, p[1] + dj, p[2])

INF = 123123123
P = defaultdict(set) # parents
C = defaultdict(lambda: INF)

vis = set()
def dfs(p):
    t = 0
    if p and p not in vis:
        vis.add(p)
        if b[p] != 'O':
            t += 1
        b[p] = 'O'
        for p1 in P[p]:
            t += dfs(p1)
    return t

pq = []
i, j = b.find('S')
heapq.heappush(pq, (0, ((i, j, 1), None)))
while len(pq) > 0:
    c, (p1, p0) = heapq.heappop(pq)

    if p1 in C: # already visited
        if C[p1] == c:
            P[p1].add(p0)
    else:
        C[p1] = c
        P[p1].add(p0)

        if b[p1] == 'E':
            print('cost =', c)
            print('Os =', dfs(p1))
            b.print()
            break

        if (p2 := next(p1)) and b.is_valid(p2):
            heapq.heappush(pq, (c + 1, (p2, p1)))
        heapq.heappush(pq, (c + 1000, (right(p1), p1)))
        heapq.heappush(pq, (c + 1000, (left(p1), p1)))
