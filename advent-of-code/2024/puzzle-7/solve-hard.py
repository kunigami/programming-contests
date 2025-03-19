import sys

input_data = sys.stdin.readlines()

def remove_suffix(a, b):
    a = str(a)
    b = str(b)
    if a != b and a.endswith(b):
        return int(a[: -len(b)])
    return None


def solve(r, ns, i):
    if r < 0:
        return False
    if i == 0:
        return r == ns[0]
    if r % ns[i] == 0:
        if solve(r // ns[i], ns, i - 1):
            return True
    r2 = remove_suffix(r, ns[i])
    if r2 is not None:
        if solve(r2, ns, i - 1):
            return True
    return solve(r - ns[i], ns, i - 1)

total = 0
for row in input_data:
    r, ns = row.split(":")
    r = int(r)
    ns = [int(n) for n in ns.split(" ") if n.strip() != ""]
    if solve(r, ns, len(ns) - 1):
        total += r

print(total)    
