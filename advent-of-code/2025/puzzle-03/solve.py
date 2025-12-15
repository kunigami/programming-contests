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


def solve(N):
    return sum(solve_bank(b, N) for b in read_input())


def solve_easy():
    return solve(N=2)


def solve_hard():
    return solve(N=12)


print(solve_hard() if is_hard else solve_easy())
