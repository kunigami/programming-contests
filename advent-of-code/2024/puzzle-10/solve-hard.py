import sys

a = sys.stdin.read().strip()
b = [[int(c) for c in line] for line in a.split("\n")]

DELTA = [(0, 1), (1, 0), (0, -1), (-1, 0)]


def dfs(b, v, i, j):
    if v[i][j] != -1:
        return v[i][j]
    n, m = len(b), len(b[0])
    paths = 0
    val = b[i][j]
    if val == 9:
        return 1

    for di, dj in DELTA:
        ni, nj = i + di, j + dj
        if not (0 <= ni < n and 0 <= nj < m):
            continue
        nval = b[ni][nj]
        if nval == val + 1:
            paths += dfs(b, v, ni, nj)
    v[i][j] = paths
    return paths


cnt = 0
n, m = len(b), len(b[0])
for i in range(len(b)):
    row = b[i]
    for j in range(len(row)):
        if b[i][j] == 0:
            v = [[-1] * m for _ in range(n)]
            score = dfs(b, v, i, j)
            print(score, i, j)
            cnt += score

print(cnt)
