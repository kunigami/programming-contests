import sys

b = [[int(c) for c in line.strip()] for line in sys.stdin.readlines()]

DELTA = [(0, 1), (1, 0), (0, -1), (-1, 0)]


def dfs(b, v, i, j):
    if v[i][j] != -1:
        return v[i][j]

    if b[i][j] == 9:
        return 1

    n, m = len(b), len(b[0])
    paths = 0
    for di, dj in DELTA:
        ni, nj = i + di, j + dj
        if 0 <= ni < n and 0 <= nj < m and b[ni][nj] == b[i][j] + 1:
            paths += dfs(b, v, ni, nj)
    v[i][j] = paths
    return paths

cnt = 0
n, m = len(b), len(b[0])
for i in range(n):
    for j in range(m):
        if b[i][j] == 0:
            v = [[-1] * m for _ in range(n)]
            score = dfs(b, v, i, j)
            cnt += score

print(cnt)
