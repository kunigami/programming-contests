exp = [2, 4, 1, 7, 7, 5, 1, 7, 0, 3, 4, 1, 5, 5, 3, 0]
N = len(exp) * 3
cnt = [0]


def bit(p):
    v = exp[p // 3]
    return 1 if (v & (1 << (p % 3))) > 0 else 0


def to_dec(bs):
    v = 0
    for b in bs[::-1]:
        v = v * 2 + b
    return v


def to_binary(d):
    bs = []
    for _ in range(3):
        bs.append(d % 2)
        d //= 2
    return bs


def run(a):
    s = a
    resp = []
    while a > 0:
        b = a % 8
        c = (a >> (7 - b)) % 8
        resp.append((a % 8) ^ c)
        a = a >> 3

    print(s, resp)


def for_octal(lvl, bs, oct, modified):
    def try_set_bit(p, v):
        if bs[p] is not None:
            if bs[p] != v:
                return False
        else:
            bs[p] = v
            modified.add(p)
        return True

    for v in to_binary(oct):
        if not try_set_bit(lvl, v):
            return

        if not try_set_bit(lvl + 7 - oct, v if bit(lvl) == 0 else 1 - v):
            return
        lvl += 1

    digits(lvl, bs)


def digits(lvl, bs):
    if lvl == N:
        dec = to_dec(bs)
        run(dec)
        cnt[0] += 1
        return

    for oct in range(8):
        modified = set()
        for_octal(lvl, bs, oct, modified)

        for p in modified:
            bs[p] = None


bs: list[int | None] = [None] * N
# Pad with trailing 0s so we don't have
# to handle out of bounds cases
for i in range(8):
    bs.append(0)
digits(0, bs)
