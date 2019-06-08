fn fortune(f0: i32, p: f64, c0: i32, n: i32, i: f64) -> bool {
    if f0 < 0 {
        false
    } else if n == 0 {
        true
    } else {
        let interest = p / 100.0 * f0 as f64;
        let f_n = f0 + interest as i32 - c0;

        let inflation = (i / 100.0) * c0 as f64;
        let c_n = c0 + inflation as i32;
        fortune(f_n, p, c_n, n - 1, i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testequal(f0: i32, p: f64, c0: i32, n: i32, i: f64, exp: bool) -> () {
        assert_eq!(exp, fortune(f0, p, c0, n, i))
    }

    #[test]
    fn basics() {
        testequal(100000, 1.0, 2000, 15, 1.0, true);
        testequal(100000, 1.0, 9185, 12, 1.0, false);
        testequal(100000000, 1.0, 100000, 50, 1.0, true);
        testequal(100000000, 1.5, 10000000, 50, 1.0, false);
    }
}
