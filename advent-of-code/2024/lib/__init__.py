from __future__ import annotations

from dataclasses import dataclass

from queue import Queue
from typing import Callable, Iterable, TypeVar


# Raise this exception when you want to stop search.
class StopSearch(Exception):
    pass


@dataclass
class BfsResult:
    idx: Idx
    dist: int
    visited: Mat


class Idx:
    def __init__(self, i, j):
        self._idx = (i, j)

    def __getitem__(self, p):
        return self._idx[p]

    def __add__(self, o):
        return Idx(self._idx[0] + o._idx[0], self._idx[1] + o._idx[1])

    def __eq__(self, o):
        return self._idx == o._idx

    def __lt__(self, o):
        return self._idx < o._idx

    def __str__(self):
        return f"({self._idx[0]}, {self._idx[1]})"

    def adj4(self, filter=None):
        yield from self.adj([(-1, 0), (0, 1), (1, 0), (0, -1)], filter)

    def adj(self, deltas, filter=None):
        for i, j in deltas:
            nidx = self + Idx(i, j)
            if filter and not filter(self, nidx):
                continue
            yield nidx

    def l1_dist(self, o):
        return abs(self._idx[0] - o._idx[0]) + abs(self._idx[1] - o._idx[1])


class Mat:
    def __init__(self, arr):
        self._m = arr

    def h(self):
        return len(self._m)

    def w(self):
        return len(self._m[0])

    def indices(self, is_reverse=False):
        if is_reverse:
            for i in reversed(range(self.h())):
                for j in reversed(range(self.w())):
                    yield Idx(i, j)
        else:
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

    def __str__(self):
        return "\n".join("".join(row) for row in self._m)

    def swap(self, idx1, idx2):
        self[idx1], self[idx2] = self[idx2], self[idx1]

    # returns index of first occurrence
    def find(self, v):
        for i in range(self.h()):
            for j in range(self.w()):
                if self._m[i][j] == v:
                    return Idx(i, j)

    def fill(self, v):
        for i in range(self.h()):
            for j in range(self.w()):
                self._m[i][j] = v
        return self

    def map(self, f):
        for idx in self.indices():
            self[idx] = f(self[idx], idx)
        return self

    def clone(self):
        m = []
        for i in range(self.h()):
            m.append([])
            for j in range(self.w()):
                m[-1].append(self._m[i][j])
        return Mat(m)

    def bfs(self, start: Idx, process: Callable[[Mat, Idx, int], Iterable[Idx]]):
        visited = Mat.new(self.h(), self.w(), False)
        q = Queue()
        q.put((start, 0))
        while not q.empty():
            idx, c = q.get()
            if visited[idx]:
                continue
            visited[idx] = True

            try:
                next_idxs = process(self, idx, c)
                for idx2 in next_idxs:
                    if self.within(idx2):
                        q.put((idx2, c + 1))
            except StopSearch:
                return BfsResult(idx, c, visited)
        return None

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


T = TypeVar("T")


def none_throws(x: T | None) -> T:
    if x is None:
        raise Exception("Expected value to be not None")
    return x
