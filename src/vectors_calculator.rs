const EPSILON: f64 = 0.000001;

fn are_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[derive(Copy, Clone)]
struct Vector {
    i: f64,
    j: f64,
    k: f64,
}

impl Vector {
    fn new(i: f64, j: f64, k: f64) -> Vector {
        Vector { i, j, k }
    }
    fn get_i() -> Vector {
        Vector {
            i: 1.0,
            j: 0.0,
            k: 0.0,
        }
    }
    fn get_j() -> Vector {
        Vector {
            i: 0.0,
            j: 1.0,
            k: 0.0,
        }
    }
    fn get_k() -> Vector {
        Vector {
            i: 0.0,
            j: 0.0,
            k: 1.0,
        }
    }
    fn get_magnitude(&self) -> f64 {
        (self.i.powi(2) + self.j.powi(2) + self.k.powi(2)).sqrt()
    }
    fn add(&self, other: Vector) -> Vector {
        Vector {
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
    fn multiply_by_scalar(&self, scalar: f64) -> Vector {
        Vector {
            i: self.i * scalar,
            j: self.j * scalar,
            k: self.k * scalar,
        }
    }
    fn dot(&self, other: Vector) -> f64 {
        self.i * other.i + self.j * other.j + self.k * other.k
    }
    fn cross(&self, other: Vector) -> Vector {
        Vector {
            i: self.j * other.k - self.k * other.j,
            j: self.k * other.i - self.i * other.k,
            k: self.i * other.j - self.j * other.i,
        }
    }
    fn is_parallel_to(&self, other: Vector) -> bool {
        if self.get_magnitude() == 0.0 || other.get_magnitude() == 0.0 {
            return false;
        }
        let mcross = self.cross(other).get_magnitude();
        are_equal(mcross, 0.0)
    }
    fn is_perpendicular_to(&self, other: Vector) -> bool {
        if self.get_magnitude() == 0.0 || other.get_magnitude() == 0.0 {
            return false;
        }
        let dot = self.dot(other);
        are_equal(dot, 0.0)
    }
    fn normalize(&self) -> Vector {
        let m = self.get_magnitude();
        Vector {
            i: self.i / m,
            j: self.j / m,
            k: self.k / m,
        }
    }
    fn is_normalized(&self) -> bool {
        let m = self.get_magnitude();
        are_equal(m, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_test() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(
            1.0, v.i,
            "Value of first argument passed into struct constructor should be assigned to \"i\""
        );
        assert_eq!(
            2.0, v.j,
            "Value of second argument passed into struct constructor should be assigned to \"j\""
        );
        assert_eq!(
            3.0, v.k,
            "Value of third argument passed into struct constructor should be assigned to \"k\""
        );

        let v = Vector::new(-4.0 / 3.0, 40.0 / 27.0, 68.0 / 69.0);
        assert!(
            are_equal(v.i, -4.0 / 3.0),
            "\"i\" of Vector should equal -4 / 3"
        );
        assert!(
            are_equal(v.j, 40.0 / 27.0),
            "\"j\" of Vector should equal 40 / 27"
        );
        assert!(
            are_equal(v.k, 68.0 / 69.0),
            "\"k\" of Vector should equal 68 / 69"
        );
    }

    #[test]
    fn get_magnitude_test() {
        let v = Vector::new(6.0, 10.0, -3.0);
        assert!(are_equal(v.get_magnitude(), (145 as f64).sqrt()));
    }

    #[test]
    fn struct_methods_test() {
        let i = Vector::get_i();
        let j = Vector::get_j();
        let k = Vector::get_k();
        assert_eq!(1.0, i.i);
        assert_eq!(0.0, i.j);
        assert_eq!(0.0, i.k);
        assert_eq!(0.0, j.i);
        assert_eq!(1.0, j.j);
        assert_eq!(0.0, j.k);
        assert_eq!(0.0, k.i);
        assert_eq!(0.0, k.j);
        assert_eq!(1.0, k.k);
    }

    #[test]
    fn add_test() {
        let v = Vector::new(3.0, 7.0 / 2.0, -3.0 / 2.0);
        let s: Vector = v.add(Vector::new(-27.0, 3.0, 4.0));
        assert!(are_equal(s.i, -24.0));
        assert!(are_equal(s.j, 13.0 / 2.0));
        assert!(are_equal(s.k, 5.0 / 2.0));
    }

    #[test]
    fn multiply_test() {
        let v = Vector::new(1.0 / 3.0, 177.0 / 27.0, -99.0);
        let e = v.multiply_by_scalar(-3.0 / 7.0);
        assert!(are_equal(e.i, -1.0 / 7.0));
        assert!(are_equal(e.j, -59.0 / 21.0));
        assert!(are_equal(e.k, 297.0 / 7.0));
    }

    #[test]
    fn dot_test() {
        let v = Vector::new(-99.0 / 71.0, 22.0 / 23.0, 45.0);
        assert!(are_equal(v.dot(Vector::new(-5.0, 4.0, 7.0)), 325.7979179));
    }

    #[test]
    fn cross_test() {
        let a = Vector::new(2.0, 1.0, 3.0);
        let b = Vector::new(4.0, 6.0, 5.0);
        let a_cross_b = a.cross(b);
        let b_cross_a = b.cross(a);
        assert!(are_equal(a_cross_b.i, -13.0));
        assert!(are_equal(a_cross_b.j, 2.0));
        assert!(are_equal(a_cross_b.k, 8.0));
        assert!(are_equal(b_cross_a.i, 13.0));
        assert!(are_equal(b_cross_a.j, -2.0));
        assert!(are_equal(b_cross_a.k, -8.0));
    }

    #[test]
    fn parallel_test() {
        let a = Vector::new(1045.0 / 23.0, -666.0 / 37.0, 15.0);
        let b = Vector::new(161.3385037, -59124.0 / 925.0, 9854.0 / 185.0);
        assert!(a.is_parallel_to(b));
        assert!(b.is_parallel_to(a));
        let c = Vector::new(-3.0, 0.0, 5.0);
        let d = Vector::new(-12.0, 1.0, 20.0);
        assert!(!c.is_parallel_to(d));
        assert!(!d.is_parallel_to(c));
    }

    #[test]
    fn perpendicular_test() {
        let a = Vector::new(3.0, 4.0, 7.0);
        let b = Vector::new(1.0 / 3.0, 2.0, -9.0 / 7.0);
        assert!(a.is_perpendicular_to(b));
        assert!(b.is_perpendicular_to(a));
        let c = Vector::new(1.0, 3.0, 5.0);
        let d = Vector::new(-2.0, -7.0, 4.4);
        assert!(!c.is_perpendicular_to(d));
        assert!(!d.is_perpendicular_to(c));
    }

    #[test]
    fn normalize_test() {
        let v = Vector::new(-1.0, -1.0, 1.0);
        let u = v.normalize();
        assert!(are_equal(u.get_magnitude(), 1.0));
        assert!(are_equal(u.i, -1.0 / (3.0 as f64).sqrt()));
        assert!(are_equal(u.j, -1.0 / (3.0 as f64).sqrt()));
        assert!(are_equal(u.k, 1.0 / (3.0 as f64).sqrt()));
    }

    #[test]
    fn is_normalized_test() {
        let a = Vector::new(-1.0 / (2.0 as f64).sqrt(), 0.0, 1.0 / (2.0 as f64).sqrt());
        let b = Vector::new(1.0, 1.0, 1.0);
        assert!(a.is_normalized());
        assert!(!b.is_normalized());
    }
}
