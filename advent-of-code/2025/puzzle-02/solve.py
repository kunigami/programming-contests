is_hard = True
import sys


def read_input():
    inp = sys.stdin.read()
    ranges = []
    max_val = 0
    for c in inp.split(","):
        l, r = c.split("-")
        ranges.append((int(l), int(r)))
        max_val = max(max_val, int(r))
    print("max val = ", max_val)
    return ranges


def precompute_dup(rep, covered, max_len=10):

    n = max_len // rep
    n10 = 10**n

    memo = [0] * n10
    cnt = 0
    for i in range(n10):
        s = str(i)
        ss = s * rep
        if ss not in covered:
            cnt += int(ss)
            covered.add(ss)
        memo[i] = cnt

    return memo


def chunk(v, l):
    assert len(v) % l == 0
    return [int(v[i : i + l]) for i in range(0, len(v), l)]


def is_above(chunks):
    first = chunks[0]
    for i in range(1, len(chunks)):
        if chunks[i] > first:
            return True
        if chunks[i] < first:
            return False
    return True


def count_dup(rep, memo, v):
    l = len(v)
    if l // rep == 0:
        return 0
    if l % rep != 0:
        index = int("9" * (l // rep))
    else:
        sz = l // rep
        chunks = chunk(v, sz)
        index = chunks[0]
        # exclude current dupe
        if not is_above(chunks):
            index -= 1
    return memo[index]


def solve(reps):
    cs = read_input()
    covered = set()

    t = 0
    for rep in reps:
        memo = precompute_dup(rep, covered)
        for l, r in cs:
            lc = count_dup(rep, memo, str(l - 1))
            rc = count_dup(rep, memo, str(r))
            t += rc - lc
            print(l, r, lc, rc, rc - lc)

    return t


def solve_easy():
    reps = [2]
    return solve(reps)


def solve_hard():
    reps = [2, 3, 5, 7]
    return solve(reps)


print(solve_hard() if is_hard else solve_easy())
