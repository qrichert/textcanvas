import math
from typing import Self


class Vec2D:
    def __init__(self, x: float = 0.0, y: float = 0.0) -> None:
        self.x = x
        self.y = y

    @classmethod
    def from_segment(cls, x1: float, y1: float, x2: float, y2: float) -> Self:
        return cls(x2 - x1, y2 - y1)

    def to_int(self) -> tuple[int, int]:
        return int(self.x), int(self.y)

    @classmethod
    def zero(cls) -> Self:
        return cls(0.0, 0.0)

    @classmethod
    def one(cls) -> Self:
        return cls(1.0, 1.0)

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Vec2D):
            return False
        return self.x == other.x and self.y == other.y

    def __add__(self, other: Self) -> Self:
        return self.__class__(self.x + other.x, self.y + other.y)

    def __iadd__(self, other: Self) -> Self:
        self.x += other.x
        self.y += other.y
        return self

    def __sub__(self, other: Self) -> Self:
        return self.__class__(self.x - other.x, self.y - other.y)

    def __isub__(self, other: Self) -> Self:
        self.x -= other.x
        self.y -= other.y
        return self

    def __neg__(self) -> Self:
        return self * -1

    def __mul__(self, other: Self | float) -> Self:
        if isinstance(other, Vec2D):
            return self.__class__(self.x * other.x, self.y * other.y)
        return self.__class__(self.x * other, self.y * other)

    def __rmul__(self, other: Self | float) -> Self:
        return self * other

    def __imul__(self, other: Self | float) -> Self:
        if isinstance(other, Vec2D):
            self.x *= other.x
            self.y *= other.y
        else:
            self.x *= other
            self.y *= other
        return self

    def __truediv__(self, other: Self | float) -> Self:
        if isinstance(other, Vec2D):
            return self.__class__(self.x / other.x, self.y / other.y)
        return self.__class__(self.x / other, self.y / other)

    def __itruediv__(self, other: Self | float) -> Self:
        if isinstance(other, Vec2D):
            self.x /= other.x
            self.y /= other.y
        else:
            self.x /= other
            self.y /= other
        return self

    @classmethod
    def sum(cls, vectors: list[Self]) -> Self:
        acc: Self = cls.zero()
        for vec in vectors:
            acc += vec
        return acc

    @classmethod
    def mean(cls, vectors: list[Self]) -> Self:
        sum: Self = cls.sum(vectors)
        return sum / len(vectors)

    def magnitude(self) -> float:
        return math.sqrt(self.x**2 + self.y**2)

    def normalize(self) -> Self:
        length = self.magnitude()
        return self / length

    def normal(self) -> Self:
        x, y = self.x, self.y
        return self.__class__(y, -x)

    def dot_product(self, other: Self) -> float:
        return self.x * other.x + self.y * other.y

    def projection_onto(self, b: Self) -> float:
        # (a⋅b)/∥b∥^2
        dot_product = self.dot_product(b)
        # magnitude(b) ** 2 involves a square root, canceled by "** 2".
        # It is more efficient to do it manually and avoid the sqrt().
        squared_magnitude_of_b = b.x * b.x + b.y * b.y
        return dot_product / squared_magnitude_of_b
