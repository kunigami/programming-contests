from collections import defaultdict
import sys

adj = defaultdict(set)
for l in sys.stdin.readlines():
    x, y = l.strip().split("-")
    adj[x].add(y)
    adj[y].add(x)

curr = [([x], adj[x]) for x in adj.keys()]
while len(curr) > 1:
    new = []
    for ls, adjs in curr:        
        for k in adjs:
            if k > ls[-1]:                
                new.append((ls + [k], adjs & adj[k]))

    curr = new

print(curr[0][0])
