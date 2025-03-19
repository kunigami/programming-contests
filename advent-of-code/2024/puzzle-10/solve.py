import sys

a = sys.stdin.read().strip()
b = [[int(c) for c in line] for line in a.split("\n")]

DELTA = [(0, 1), (1, 0), (0, -1), (-1, 0)]


def bfs(b, i0, j0):
    n, m = len(b), len(b[0])
    v = [[False] * m for _ in range(n)]
    queue = [(i0, j0)]
    v[i0][j0] = True
    cnt = 0
    while queue:
        i, j = queue.pop()
        val = b[i][j]
        if val == 9:
            cnt += 1
        for di, dj in DELTA:
            ni, nj = i + di, j + dj
            if not (0 <= ni < n and 0 <= nj < m):
                continue
            if v[ni][nj]:
                continue
            nval = b[ni][nj]
            if nval == val + 1:
                v[ni][nj] = True
                queue.append((ni, nj))
    return cnt


cnt = 0
for i in range(len(b)):
    row = b[i]
    for j in range(len(row)):
        if b[i][j] == 0:
            cnt += bfs(b, i, j)

print(cnt)
