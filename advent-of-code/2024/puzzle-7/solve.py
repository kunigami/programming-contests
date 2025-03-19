import sys

input_data = sys.stdin.readlines()

def solve(r, ns, i):
    if i == 0:
        return r == ns[0]
    if r % ns[i] == 0:
        if solve(r // ns[i], ns, i - 1):
            return True
    return solve(r - ns[i], ns, i - 1)

total = 0
for row in input_data:
    r, ns = row.split(":")
    r = int(r)
    ns = [int(n) for n in ns.split(" ") if n.strip() != ""]
    print(r, ns)

    ok = solve(r, ns, len(ns) - 1)
    if ok:
        total += r

print(total)    
