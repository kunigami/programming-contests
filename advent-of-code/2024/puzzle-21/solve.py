import sys

ls = sys.stdin.read().splitlines()
M = 25 # 2 for easy

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

def search_for_symbol(sc, ec, symbol, excl) -> list[str]:
    delta = dirs[symbol]
    new_sc = (sc[0] + delta[0], sc[1] + delta[1])
    paths = search(new_sc, ec, excl)
    return [symbol + p for p in paths]

def search(sc, ec, excl) -> list[str]:
    if sc == excl:
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
        new_paths += search_for_symbol(sc, ec, symbol, excl)
    return new_paths

memo = {}

def get_paths(s, e, lvl):
    if s + e not in memo:
        pad = pad1 if lvl == 0 else pad2        
        memo[s + e] = search(pad[s], pad[e], excl=pad['*'])
    return memo[s + e]

memoc = {}

def path_cost(path, lvl = -1):
    p = 'A' + path
    return sum(pair_cost(p[i - 1], p[i], lvl + 1) for i in range(1, len(p)))


def pair_cost(a, b, l) -> int:
    key = f"{a}{b}{l}"
    if key not in memoc:
        memoc[key] = min(
            len(p) if l == M else path_cost(p, l) for p in get_paths(a, b, l)
        )        
    return memoc[key]

t = sum((path_cost(l) * int(l[1:-1])) for l in ls)
    
print(t)
