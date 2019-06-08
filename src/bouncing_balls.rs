fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    // Check conditions
    if h <= 0.0 || bounce <= 0.0 || bounce >= 1.0 || window >= h {
        return -1;
    }

    let mut count = 0;

    // Falling
    if window < h {
        count += 1;
    }

    // Bouncing
    let next_h = h * bounce;

    if window < next_h {
        count += 1;
        return count + bouncing_ball(next_h, bounce, window);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testequal(h: f64, bounce: f64, window: f64, exp: i32) -> () {
        assert_eq!(bouncing_ball(h, bounce, window), exp)
    }

    #[test]
    fn tests_bouncing_ball() {
        testequal(3.0, 0.66, 1.5, 3);
        testequal(30.0, 0.66, 1.5, 15);
        testequal(40.0, 0.4, 10.0, 3);
        testequal(10.0, 0.6, 10.0, -1);
    }
}
