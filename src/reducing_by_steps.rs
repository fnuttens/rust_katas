use std::cmp;

fn som(x: i64, y: i64) -> i64 {
    x + y
}
fn maxi(x: i64, y: i64) -> i64 {
    cmp::max(x, y)
}
fn mini(x: i64, y: i64) -> i64 {
    cmp::min(x, y)
}
fn gcdi(m: i64, n: i64) -> i64 {
    if n == 0 {
        return m.abs();
    } else {
        let r = m % n;
        return gcdi(n, r).abs();
    }
}
fn lcmu(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return 0;
    }
    let lcmu = (a * b) / gcdi(a, b);
    lcmu.abs()
}

// first parameter: dots have to be replaced by function of two variables
fn oper_array(fct: fn(i64, i64) -> i64, a: &[i64], init: i64) -> Vec<i64> {
    let mut result = Vec::new();
    let mut acc = init;

    for x in a {
        let y = fct(acc, *x);
        result.push(y);
        acc = y;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing_som(a: &[i64], exp: &Vec<i64>) -> () {
        assert_eq!(&oper_array(som, a, 0), exp);
    }
    fn testing_lcmu(a: &[i64], exp: &Vec<i64>) -> () {
        assert_eq!(&oper_array(lcmu, a, a[0]), exp);
    }
    fn testing_gcdi(a: &[i64], exp: &Vec<i64>) -> () {
        assert_eq!(&oper_array(gcdi, a, a[0]), exp);
    }
    fn testing_maxi(a: &[i64], exp: &Vec<i64>) -> () {
        assert_eq!(&oper_array(maxi, a, a[0]), exp);
    }

    #[test]
    fn basics_som() {
        testing_som(&[18, 69, -90, -78, 65, 40], &vec![18, 87, -3, -81, -16, 24]);
    }
    #[test]
    fn basics_lcmu() {
        testing_lcmu(
            &[18, 69, -90, -78, 65, 40],
            &vec![18, 414, 2070, 26910, 26910, 107640],
        );
    }
    #[test]
    fn basics_maxi() {
        testing_maxi(&[18, 69, -90, -78, 65, 40], &vec![18, 69, 69, 69, 69, 69]);
    }
    #[test]
    fn basics_gcdi() {
        testing_gcdi(&[18, 69, -90, -78, 65, 40], &vec![18, 3, 3, 3, 1, 1]);
    }
}
