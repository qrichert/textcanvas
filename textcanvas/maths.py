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


class Interpolation:
    @staticmethod
    def lerp(a: float, b: float, t: float) -> float:
        """Linear.

        Find value given time.

        Examples:
            >>> Interpolation.lerp(10.0, 20.0, 0.5)
            15.0

        Args:
            - `a` - Start
            - `b` - End
            - `t` - Time [0; 1]
        """
        return (1.0 - t) * a + t * b

    @staticmethod
    def rlerp(a: float, b: float, v: float) -> float:
        """Reverse Linear.

        Find time given value.

        Examples:
            >>> Interpolation.rlerp(10.0, 20.0, 15.0)
            0.5

        Args:
            - `a` - Start
            - `b` - End
            - `v` - Value [start; end]
        """
        return (v - a) / (b - a)

    @staticmethod
    def ease_in_quad(a: float, b: float, t: float) -> float:
        """Ease In Quad (^2) - Start slow, accelerate.

        Examples:
            >>> Interpolation.ease_in_quad(0.0, 100.0, 0.00)
            0.0
            >>> Interpolation.ease_in_quad(0.0, 100.0, 0.25)
            6.25
            >>> Interpolation.ease_in_quad(0.0, 100.0, 0.50)
            25.0
            >>> Interpolation.ease_in_quad(0.0, 100.0, 0.75)
            56.25
            >>> Interpolation.ease_in_quad(0.0, 100.0, 1.00)
            100.0

        Args:
            - `a` - Start
            - `b` - End
            - `t` - Time [0; 1]
        """
        t = t * t
        return Interpolation.lerp(a, b, t)

    @staticmethod
    def ease_out_quad(a: float, b: float, t: float) -> float:
        """Ease Out Quad (^2) - Start fast, decelerate.

        Examples:
            >>> Interpolation.ease_out_quad(0.0, 100.0, 0.00)
            0.0
            >>> Interpolation.ease_out_quad(0.0, 100.0, 0.25)
            43.75
            >>> Interpolation.ease_out_quad(0.0, 100.0, 0.50)
            75.0
            >>> Interpolation.ease_out_quad(0.0, 100.0, 0.75)
            93.75
            >>> Interpolation.ease_out_quad(0.0, 100.0, 1.00)
            100.0

        Args:
            - `a` - Start
            - `b` - End
            - `t` - Time [0; 1]
        """
        t = 1.0 - (1.0 - t) * (1.0 - t)
        return Interpolation.lerp(a, b, t)

    @staticmethod
    def ease_in_out_quad(a: float, b: float, t: float) -> float:
        """Ease In-Out Quad (^2) - Start slow, accelerate, end slow.

        Examples:
            >>> Interpolation.ease_in_out_quad(0.0, 100.0, 0.00)
            0.0
            >>> Interpolation.ease_in_out_quad(0.0, 100.0, 0.25)
            12.5
            >>> Interpolation.ease_in_out_quad(0.0, 100.0, 0.50)
            50.0
            >>> Interpolation.ease_in_out_quad(0.0, 100.0, 0.75)
            87.5
            >>> Interpolation.ease_in_out_quad(0.0, 100.0, 1.00)
            100.0

        Args:
            - `a` - Start
            - `b` - End
            - `t` - Time [0; 1]
        """
        if t < 0.5:
            t = 2.0 * t * t
        else:
            t = 1.0 - (-2.0 * t + 2.0) ** 2 / 2.0
        return Interpolation.lerp(a, b, t)

    @staticmethod
    def smoothstep(a: float, b: float, t: float) -> float:
        """Smoothstep (Smooth ease in-out).

        Examples:
            >>> Interpolation.smoothstep(0.0, 100.0, 0.00)
            0.0
            >>> Interpolation.smoothstep(0.0, 100.0, 0.25)
            15.625
            >>> Interpolation.smoothstep(0.0, 100.0, 0.50)
            50.0
            >>> Interpolation.smoothstep(0.0, 100.0, 0.75)
            84.375
            >>> Interpolation.smoothstep(0.0, 100.0, 1.00)
            100.0

        Args:
            - `a` - Start
            - `b` - End
            - `t` - Time [0; 1]
        """
        t = t * t * (3.0 - 2.0 * t)
        return Interpolation.lerp(a, b, t)

    @staticmethod
    def catmull_rom(
        p0: Vec2D,
        p1: Vec2D,
        p2: Vec2D,
        p3: Vec2D,
        t: float,
        alpha: float,
    ) -> Vec2D:
        """Catmull-Rom.

        ```text
              P1              - P3
              -  *          -
            -     *      -
        P0 -        *  *
                       P2
        ```

        Examples:
            >>> p0 = Vec2D(0.0, 0.25)
            >>> p1 = Vec2D(0.33, 0.85)
            >>> p2 = Vec2D(0.67, 0.15)
            >>> p3 = Vec2D(1.0, 0.75)
            >>> Interpolation.catmull_rom(p0, p1, p2, p3, 0.00, 0.5) # doctest: +SKIP
            p1
            >>> Interpolation.catmull_rom(p0, p1, p2, p3, 0.25, 0.5) # doctest: +SKIP
            Vec2D(0.416, 0.740)
            >>> Interpolation.catmull_rom(p0, p1, p2, p3, 0.50, 0.5) # doctest: +SKIP
            Vec2D(0.5, 0.5)
            >>> Interpolation.catmull_rom(p0, p1, p2, p3, 0.75, 0.5) # doctest: +SKIP
            Vec2D(0.584, 0.260)
            >>> Interpolation.catmull_rom(p0, p1, p2, p3, 1.00, 0.5) # doctest: +SKIP
            p2

        Args:
            - `p0` - Control point 1
            - `p1` - Spline start
            - `p2` - Spline end
            - `p3` - Control point 2
            - `t` - Time [0; 1]
            - `alpha` - 0.5 = centripetal, 0 = uniform, 1 = chordal
        """

        def get_t(t: float, alpha: float, p0: Vec2D, p1: Vec2D) -> float:
            d = p1 - p0
            a = d.dot_product(d)
            b = a ** (alpha * 0.5)
            return b + t

        t0 = 0.0
        t1 = get_t(t0, alpha, p0, p1)
        t2 = get_t(t1, alpha, p1, p2)
        t3 = get_t(t2, alpha, p2, p3)
        t = Interpolation.lerp(t1, t2, t)

        a1 = ((t1 - t) / (t1 - t0) * p0) + ((t - t0) / (t1 - t0) * p1)
        a2 = ((t2 - t) / (t2 - t1) * p1) + ((t - t1) / (t2 - t1) * p2)
        a3 = ((t3 - t) / (t3 - t2) * p2) + ((t - t2) / (t3 - t2) * p3)
        b1 = ((t2 - t) / (t2 - t0) * a1) + ((t - t0) / (t2 - t0) * a2)
        b2 = ((t3 - t) / (t3 - t1) * a2) + ((t - t1) / (t3 - t1) * a3)

        return ((t2 - t) / (t2 - t1) * b1) + ((t - t1) / (t2 - t1) * b2)
