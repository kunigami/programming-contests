import sys

lines = sys.stdin.read().splitlines()
M = 25

def index(pad):
    m = {}
    for i, row in enumerate(pad):
        for j, v in enumerate(row):
            m[v] = (i, j)
    return m

pad1 = index(['789', '456', '123', '*0A'])
pad2 = index(['*^A', '<v>'])

dirs = {
    'v': (1, 0),
    '>': (0, 1),
    '^': (-1, 0),
    '<': (0, -1)
}

def search_for_symbol(sc, ec, symbol, excluded_state) -> list[str]:
    delta = dirs[symbol]
    new_sc = (sc[0] + delta[0], sc[1] + delta[1])
    paths = search_paths_rec(new_sc, ec, excluded_state)
    return [symbol + p for p in paths]

def search_paths_rec(sc, ec, excluded_state) -> list[str]:
    if sc == excluded_state:
        return []

    if sc == ec:
        return ['A']

    symbols = []
    if sc[0] < ec[0]:
        symbols.append('v')
    if sc[1] < ec[1]:
        symbols.append('>')
    if sc[0] > ec[0]:
        symbols.append('^')
    if sc[1] > ec[1]:
        symbols.append('<')
    
    new_paths: list[str] = []
    for symbol in symbols:
        new_paths += search_for_symbol(sc, ec, symbol, excluded_state)
    return new_paths

memo = {}

def get_paths(s, e, lvl):
    if s + e in memo:
        return memo[s + e]

    pad = pad1 if lvl == 0 else pad2
    paths = search_paths_rec(pad[s], pad[e], excluded_state=pad['*'])
    memo[s + e] = paths
    return memo[s + e]

memoc = {}

def compute(a, b, lvl) -> int:
    key = f"{a}{b}{lvl}"
    if key in memoc:
        return memoc[key]

    best_cost = 10**1000
    for path in get_paths(a, b, lvl): 
        if lvl == M:
            cost = len(path)
        else:
            path = 'A' + path
            cost = 0
            for i in range(len(path) - 1):
                cost += compute(path[i], path[i + 1], lvl + 1)

        best_cost = min(best_cost, cost)
    
    memoc[key] = best_cost
    return best_cost

t = 0
for l in lines:
    l = 'A' + l
    c = 0
    for i in range(len(l) - 1):
        c += compute(l[i], l[i + 1], 0)
    t += (c * int(l[1:-1]))
    
print(t)
