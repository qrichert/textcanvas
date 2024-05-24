import doctest
import unittest

import textcanvas.maths
from textcanvas.maths import Vec2D


def load_tests(
    loader: unittest.TestLoader, tests: unittest.TestSuite, ignore: str
) -> unittest.TestSuite:
    """Add module doctests."""
    tests.addTests(doctest.DocTestSuite(textcanvas.maths))
    return tests


class TestVec2D(unittest.TestCase):
    def test_new(self) -> None:
        v = Vec2D(3.0, 6.0)

        self.assertAlmostEqual(v.x, 3.0)
        self.assertAlmostEqual(v.y, 6.0)

    def test_from_segment(self) -> None:
        v = Vec2D.from_segment(9.0, 2.0, 5.0, 7.0)

        self.assertEqual(v, Vec2D(-4.0, 5.0))

    def test_to_int(self) -> None:
        v = Vec2D(3.0, 6.0)

        (x, y) = v.to_int()

        self.assertEqual(x, 3)
        self.assertEqual(y, 6)

    def test_zero(self) -> None:
        self.assertEqual(Vec2D.zero(), Vec2D(0.0, 0.0))

    def test_one(self) -> None:
        self.assertEqual(Vec2D.one(), Vec2D(1.0, 1.0))

    def test_default(self) -> None:
        self.assertEqual(Vec2D(), Vec2D(0.0, 0.0))

    def test_vec_eq(self) -> None:
        self.assertEqual(Vec2D(), Vec2D())
        self.assertEqual(Vec2D(42, 108), Vec2D(42, 108))

    def test_vec_not_eq(self) -> None:
        self.assertNotEqual(Vec2D.zero(), Vec2D.one())
        self.assertNotEqual(Vec2D(42, 108), Vec2D(108, 42))
        self.assertNotEqual(Vec2D.one(), 1)
        self.assertNotEqual(Vec2D.one(), None)

    def test_vec_add(self) -> None:
        u = Vec2D(1.0, 0.0)
        v = Vec2D(2.0, 3.0)

        self.assertEqual(u + v, Vec2D(3.0, 3.0))
        self.assertEqual(v + v, Vec2D(4.0, 6.0))

    def test_vec_add_assign(self) -> None:
        u = Vec2D(1.0, 0.0)
        v = Vec2D(2.0, 3.0)

        u += v
        v += v

        self.assertEqual(u, Vec2D(3.0, 3.0))
        self.assertEqual(v, Vec2D(4.0, 6.0))

    def test_vec_subtract(self) -> None:
        u = Vec2D(1.0, 0.0)
        v = Vec2D(2.0, 3.0)

        self.assertEqual(u - v, Vec2D(-1.0, -3.0))
        self.assertEqual(v - u, Vec2D(1.0, 3.0))
        self.assertEqual(v - v, Vec2D(0.0, 0.0))

    def test_vec_subtract_assign(self) -> None:
        u = Vec2D(1.0, 0.0)
        v = Vec2D(2.0, 3.0)
        w = Vec2D(2.0, 3.0)

        u -= v
        v -= Vec2D(1.0, 0.0)
        w -= w

        self.assertEqual(u, Vec2D(-1.0, -3.0))
        self.assertEqual(v, Vec2D(1.0, 3.0))
        self.assertEqual(w, Vec2D(0.0, 0.0))

    def test_vec_negative(self) -> None:
        v = Vec2D(6.0, 9.0)

        self.assertEqual(-v, Vec2D(-6.0, -9.0))

    def test_vec_multiply(self) -> None:
        u = Vec2D(1.0, 0.0)
        v = Vec2D(2.0, 3.0)

        self.assertEqual(u * v, Vec2D(2.0, 0.0))
        self.assertEqual(v * v, Vec2D(4.0, 9.0))

    def test_vec_multiply_by_scalar(self) -> None:
        v = Vec2D(2.0, 3.0)

        self.assertEqual(v * 3.0, Vec2D(6.0, 9.0))
        self.assertEqual(v * 3, Vec2D(6.0, 9.0))

    def test_vec_multiply_scalar_by_vec(self) -> None:
        v = Vec2D(2.0, 3.0)

        self.assertEqual(3.0 * v, Vec2D(6.0, 9.0))
        self.assertEqual(3 * v, Vec2D(6.0, 9.0))

    def test_vec_multiply_assign(self) -> None:
        u = Vec2D(1.0, 0.0)
        v = Vec2D(2.0, 3.0)

        u *= v
        v *= v

        self.assertEqual(u, Vec2D(2.0, 0.0))
        self.assertEqual(v, Vec2D(4.0, 9.0))

    def test_vec_multiply_by_scalar_assign(self) -> None:
        u = Vec2D(2.0, 3.0)
        v = Vec2D(2.0, 3.0)

        u *= 3.0
        v *= 3

        self.assertEqual(u, Vec2D(6.0, 9.0))
        self.assertEqual(v, Vec2D(6.0, 9.0))

    def test_vec_divide(self) -> None:
        u = Vec2D(1.0, 0.0)
        v = Vec2D(2.0, 3.0)

        self.assertEqual(u / v, Vec2D(0.5, 0.0))
        self.assertEqual(v / v, Vec2D(1.0, 1.0))

    def test_vec_divide_by_scalar(self) -> None:
        v = Vec2D(6.0, 9.0)

        self.assertEqual(v / 3.0, Vec2D(2.0, 3.0))
        self.assertEqual(v / 3, Vec2D(2.0, 3.0))

    def test_vec_divide_assign(self) -> None:
        u = Vec2D(1.0, 0.0)
        v = Vec2D(2.0, 3.0)

        u /= v
        v /= v

        self.assertEqual(u, Vec2D(0.5, 0.0))
        self.assertEqual(v, Vec2D(1.0, 1.0))

    def test_vec_divide_by_scalar_assign(self) -> None:
        u = Vec2D(6.0, 9.0)
        v = Vec2D(6.0, 9.0)

        u /= 3.0
        v /= 3

        self.assertEqual(u, Vec2D(2.0, 3.0))
        self.assertEqual(v, Vec2D(2.0, 3.0))

    def test_sum(self) -> None:
        vectors = [
            Vec2D(1.0, 0.0),
            Vec2D(2.0, 3.0),
            Vec2D(-1.0, -0.5),
        ]

        sum = Vec2D.sum(vectors)

        self.assertEqual(sum, Vec2D(2.0, 2.5))

    def test_mean(self) -> None:
        vectors = [
            Vec2D(5.0, -9.5),
            Vec2D(2.0, 1.0),
            Vec2D(-1.0, -0.5),
        ]

        mean = Vec2D.mean(vectors)

        self.assertEqual(mean, Vec2D(2.0, -3.0))

    def test_magnitude(self) -> None:
        v = Vec2D(3.0, 4.0)

        self.assertAlmostEqual(v.magnitude(), 5.0)

    def test_normalize(self) -> None:
        v = Vec2D(3.0, 4.0)

        self.assertEqual(v.normalize(), Vec2D(0.6, 0.8))

    def test_normal(self) -> None:
        v = Vec2D(3.0, 4.0)

        self.assertEqual(v.normal(), Vec2D(4.0, -3.0))

    def test_dot_product(self) -> None:
        u = Vec2D(1.0, 0.0)
        v = Vec2D(-1.0, 0.0)
        w = Vec2D(0.0, 1.0)
        x = Vec2D(0.5, 0.5)
        y = Vec2D(-0.5, -0.5)

        self.assertAlmostEqual(u.dot_product(u), 1.0)
        self.assertAlmostEqual(u.dot_product(v), -1.0)
        self.assertAlmostEqual(u.dot_product(w), 0.0)
        self.assertAlmostEqual(u.dot_product(x), 0.5)
        self.assertAlmostEqual(u.dot_product(y), -0.5)

    def test_projection_onto(self) -> None:
        u = Vec2D(1.0, 0.0)
        v = Vec2D(-1.0, 0.0)
        w = Vec2D(0.0, 1.0)
        x = Vec2D(0.5, 0.5)
        y = Vec2D(-0.5, -0.5)
        z = Vec2D(2.0, 0.0)

        self.assertAlmostEqual(u.projection_onto(u), 1.0)
        self.assertAlmostEqual(v.projection_onto(u), -1.0)
        self.assertAlmostEqual(w.projection_onto(u), 0.0)
        self.assertAlmostEqual(x.projection_onto(u), 0.5)
        self.assertAlmostEqual(y.projection_onto(u), -0.5)
        self.assertAlmostEqual(z.projection_onto(u), 2.0)


if __name__ == "__main__":
    unittest.main()
