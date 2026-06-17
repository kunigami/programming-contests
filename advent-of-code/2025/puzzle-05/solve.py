is_hard = True
import sys


def read_input():
    lines = sys.stdin.readlines()

    ranges = []
    i = 0
    while i < len(lines):
        li = lines[i].rstrip("\n")
        i += 1
        if li == '':
            break
        l, r = [int(p) for p in li.split('-')]
        ranges.append((l, r))

    index = []
    while i < len(lines):
        li = lines[i].rstrip("\n")
        i += 1
        if li == '':
            break
        index.append(int(li))

    return ranges, index


def get_mat():
    b = []
    for l in read_input():
        r = []
        for c in l:
            r.append(c)
        b.append(r)
    return b


def in_interval(range, index):
    l, r = range
    return index >= l and index <= r

def solve_easy():
    ranges, indices = read_input()
    cnt = 0
    for idx in indices:
        for range in ranges:
            if in_interval(range, idx):
                cnt += 1
                break
    return cnt

def solve_hard():
    ranges, _ = read_input()
    ranges.sort(key=lambda x: x[0])

    t = 0
    i = 1
    curr = ranges[0]
    while i < len(ranges):
        if curr[1] >= ranges[i][0]:
            curr = curr[0], max(curr[1], ranges[i][1])
        else: # close interval
            t += curr[1] - curr[0] + 1
            curr = ranges[i]
        i += 1

    t += curr[1] - curr[0] + 1
    return t



print(solve_hard() if is_hard else solve_easy())
