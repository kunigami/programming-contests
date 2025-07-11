from collections import defaultdict
import sys

adj = defaultdict(set)
for l in sys.stdin.readlines():
    x, y = l.strip().split("-")
    adj[x].add(y)
    adj[y].add(x)

curr_tpls = [([x], adj[x]) for x in adj.keys()]
while len(curr_tpls) > 1:
    new_tpls = []
    for ls, adjs in curr_tpls:        
        for k in adjs:
            if k > ls[-1]:                
                new_tpls.append((ls + [k], adjs & adj[k]))

    curr_tpls = new_tpls

print(curr_tpls[0][0])
