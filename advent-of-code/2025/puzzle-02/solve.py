is_hard = True
import sys


def read_input():
    ranges = []
    max_val = 0
    for c in sys.stdin.read().split(","):
        l, r = c.split("-")
        ranges.append((int(l), int(r)))
        max_val = max(max_val, int(r))
    print("max val = ", max_val)
    return ranges


def precompute_dup(rep, covered, max_len=10):
    n = max_len // rep
    memo = [0] * (10**n)
    cnt = 0
    for i in range(len(memo)):
        ss = str(i) * rep
        if ss not in covered:
            cnt += int(ss)
            covered.add(ss)
        memo[i] = cnt

    return memo


def chunk(v, l):
    assert len(v) % l == 0
    return [int(v[i : i + l]) for i in range(0, len(v), l)]


# determine if the number chunked is after
# the one repeated n-times. examples:
# 23-23-45 is after 23-23-23
# 23-21-45 is not.
def is_after(chunks):
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
        if not is_after(chunks):
            index -= 1
    return memo[index]


def solve(reps):
    cs = read_input()
    # need to make sure a number is not covered by
    # more than one rep. an example is 222222 which
    # is covered by 2 (222-222) and 3 (22-22-22)
    covered = set()

    t = 0
    for rep in reps:
        memo = precompute_dup(rep, covered)
        for l, r in cs:
            lc = count_dup(rep, memo, str(l - 1))
            rc = count_dup(rep, memo, str(r))
            t += rc - lc

    return t


def solve_easy():
    reps = [2]
    return solve(reps)


def solve_hard():
    reps = [2, 3, 5, 7]
    return solve(reps)


print(solve_hard() if is_hard else solve_easy())
