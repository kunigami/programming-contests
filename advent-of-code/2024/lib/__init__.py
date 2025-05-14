class Idx: 
    def __init__(self, i, j):
        self._idx = (i, j)
    
    def __getitem__(self, p):
        return self._idx[p]

    def __add__(self, o):
        return Idx(self._idx[0] + o._idx[0], self._idx[1] + o._idx[1])

    def adj4(self):
        deltas = [(-1, 0), (0, 1), (1, 0), (0, -1)]
        for i, j in deltas:
            yield self + Idx(i, j)


class Mat:
    def __init__(self, arr):
        self._m = arr 

    def h(self):
        return len(self._m)

    def w(self):
        return len(self._m[0])

    def indices(self):
        for i in range(self.h()):
            for j in range(self.w()):
                yield Idx(i, j)

    def within(self, idx):
        i, j = idx
        return i >= 0 and i < self.h() and j >= 0 and j < self.w()
            
    def __getitem__(self, idx):
        i, j = idx
        return self._m[i][j]

    def __setitem__(self, idx, v):
        i, j = idx
        self._m[i][j] = v

    def fill(self, v):
        for i in range(self.h()):
            for j in range(self.w()):
                self._m[i][j] = v
        return self
    
    def clone(self):
        print(self._m)
        m = []
        for i in range(self.h()):
            m.append([])
            for j in range(self.w()):
                m[-1].append(self._m[i][j])
        return Mat(m)

    @staticmethod
    def new(h, w, value):
        m = []
        for _ in range(h):
            m.append([])
            for _ in range(w):
                if callable(value):
                    v = value()
                else:
                    v = value                
                m[-1].append(v)
        return Mat(m)
 
    @staticmethod
    def from_str(s):
        return Mat([list(line.strip()) for line in s.strip().split("\n")])
 