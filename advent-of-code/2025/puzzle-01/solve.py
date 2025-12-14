import sys


is_hard = False


def get_rows():
    rows = []
    for line in sys.stdin.readlines():
        d = int(line[1:])
        if line.startswith("L"):
            d = -d
        rows.append(d)
    return rows


def solve_easy():
    p = 50
    t = 0
    for d in get_rows():
        p = (p + d) % 100
        if p == 0:
            t += 1
    return t


def solve_hard():
    p = 50
    t = 0
    for d in get_rows():
        assert p >= 0 and p < 100

        if d < 0:
            d = -d
            t += d // 100
            d = d % 100

            if p < d:
                if p > 0:  # cross 0 to the left
                    t += 1
                p += 100

            p = p - d
            if p == 0:
                t += 1
        else:
            p = p + d
            t += p // 100
            p = p % 100
    return t


print(solve_hard() if is_hard else solve_easy())
