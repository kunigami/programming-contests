is_hard = True
import sys


def read_input():
    return [l.strip() for l in sys.stdin.readlines()]


def solve_bank(b, N):
    max_i = 0
    s = ""
    for n in range(N):
        end = len(b) - N + n + 1
        for i in range(max_i, end):
            if b[i] > b[max_i]:
                max_i = i
        s += b[max_i]
        max_i += 1
    return int(s)

def get_mat():
    b = []
    for l in read_input():
        r = []
        for c in l:
            r.append(c)
        b.append(r)
    return b

def solve(b):
    n = len(b)
    m = len(b[0])
    sol = []
    for i in range(n):
        for j in range(m):
            if b[i][j] != '@':
                continue

            cnt = 0
            for di in range(-1, 2):
                for dj in range(-1, 2):
                    if di == dj and di == 0:
                        continue
                    pi = i + di
                    pj = j + dj
                    if pi < 0 or pi >= n or pj < 0 or pj >= m:
                        continue
                    if b[pi][pj] == '@':
                        cnt += 1
            if cnt < 4:
                sol.append((i, j))

    return sol


def solve_easy():
    b = get_mat()
    sol = solve(b)
    return len(sol)


def solve_hard():
    b = get_mat()
    t = 0
    while True:
        sol = solve(b)
        if len(sol) == 0:
            break
        t += len(sol)
        for (i, j) in sol:
            b[i][j] = '.'

    return t


print(solve_hard() if is_hard else solve_easy())
