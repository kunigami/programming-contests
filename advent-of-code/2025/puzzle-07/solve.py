is_hard = True
import sys


def get_start(b):
    for i in range(len(b)):
        for j in range(len(b[0])):
            if b[i][j] == 'S':
                return i, j

def solve():
    b = sys.stdin.readlines()
    si, sj = get_start(b)

    rs = [0]*len(b[0])
    rs[sj] = 1
    t = 0
    for i in range(si + 1, len(b)):
        new_rs = [0]*len(rs)
        for j in range(len(b[i])):
            if rs[j] > 0:
                if b[i][j] == '^':
                    new_rs[j - 1] += rs[j]
                    new_rs[j + 1] += rs[j]
                    new_rs[j] = 0
                    t += 1
                else:
                    new_rs[j] += rs[j]

        rs = new_rs

    return sum(rs) if is_hard else t


print(solve())
