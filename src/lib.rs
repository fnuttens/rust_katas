mod weight_for_weight {
    use std::cmp::Ordering;

    fn order_weight(s: &str) -> String {
        let mut map: Vec<(&str, u32)> = s
            .trim()
            .split(' ')
            .map(|weight| {
                let w = weight.chars().fold(0, |acc, x| {
                    if let Some(digit) = x.to_digit(10) {
                        return acc + digit;
                    };
                    acc
                });
                (weight, w)
            })
            .collect();

        map.sort_by(|a, b| {
            let cmp_w = a.1.cmp(&b.1);
            if cmp_w == Ordering::Equal {
                return a.0.cmp(b.0);
            }
            cmp_w
        });

        let mut result = String::from("");
        for weight in map {
            result.push_str(weight.0);
            result.push(' ');
        }
        result.pop();
        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn testing(s: &str, exp: &str) -> () {
            assert_eq!(order_weight(s), exp)
        }

        #[test]
        fn basics_order_weight() {
            testing("103 123 4444 99 2000", "2000 103 123 4444 99");
            testing(
                "2000 10003 1234000 44444444 9999 11 11 22 123",
                "11 11 2000 10003 22 123 1234000 44444444 9999",
            );
        }
    }
}

mod camel_case_method {
    use std::iter::FromIterator;

    fn camel_case(str: &str) -> String {
        str.split(' ')
            .map(|word| {
                let mut chars = word.chars();
                if let Some(mut first_char) = chars.next() {
                    first_char.make_ascii_uppercase();
                    let mut result = String::from_iter(chars);
                    result.insert(0, first_char);
                    return result;
                }
                String::from_iter(chars)
            })
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn sample_test() {
            assert_eq!(camel_case("test case"), "TestCase");
            assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
            assert_eq!(camel_case("say hello "), "SayHello");
            assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
            assert_eq!(camel_case(""), "");
        }
    }
}

mod prize_draw {
    use std::cmp::Ordering;

    const UPPERCASE_OFFSET: i32 = 64;
    const LOWERCASE_OFFSET: i32 = 96;

    fn rank(st: &str, we: Vec<i32>, n: usize) -> &str {
        if st.is_empty() {
            return "No participants";
        }

        let names: Vec<&str> = st.split(',').collect();

        if n > names.len() {
            return "Not enough participants";
        }

        let mut rankings: Vec<(i32, &str)> = names
            .iter()
            .map(|name| get_som(name))
            .zip(we.iter())
            .map(|(som, weight)| som * weight)
            .zip(names.clone().into_iter())
            .collect();

        println!("Sorting...");
        rankings.sort_by(|(lscore, lname), (rscore, rname)| {
            if lscore.cmp(rscore) == Ordering::Equal {
                return lname.cmp(rname);
            }
            rscore.cmp(lscore)
        });

        rankings.get(n - 1).unwrap().1
    }

    fn get_som(name: &str) -> i32 {
        let sum = name.chars().fold(0, |acc, c| {
            let offset = if c.is_ascii_uppercase() {
                UPPERCASE_OFFSET
            } else {
                LOWERCASE_OFFSET
            };

            acc + (c as i32 - offset)
        });
        sum + name.len() as i32
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        fn testing(st: &str, we: Vec<i32>, n: usize, exp: &str) -> () {
            assert_eq!(rank(st, we, n), exp)
        }

        #[test]
        fn basics_rank() {
            testing(
                "Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin",
                vec![4, 2, 1, 4, 3, 1, 2],
                4,
                "Benjamin",
            );
            testing(
                "Elijah,Chloe,Elizabeth,Matthew,Natalie,Jayden",
                vec![1, 3, 5, 5, 3, 6],
                2,
                "Matthew",
            );
            testing(
                "Aubrey,Olivai,Abigail,Chloe,Andrew,Elizabeth",
                vec![3, 1, 4, 4, 3, 2],
                4,
                "Abigail",
            );
            testing("Lagon,Lily", vec![1, 5], 2, "Lagon");
        }
    }
}

mod directions_reduction {
    #[derive(Clone, PartialEq, Debug)]
    enum Direction {
        NORTH,
        SOUTH,
        EAST,
        WEST,
    }

    use Direction::*;

    fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
        let mut v = arr.to_vec();
        let mut i = 0;
        loop {
            if is_scan_complete(i, v.len()) {
                break;
            }

            let target = match v[i] {
                NORTH => SOUTH,
                SOUTH => NORTH,
                EAST => WEST,
                WEST => EAST,
            };

            if v[i + 1] == target {
                // Remove opposite directions
                v.remove(i + 1);
                v.remove(i);
                // Reset index
                i = 0;
            } else {
                i += 1;
            }
        }
        v
    }

    fn is_scan_complete(i: usize, len: usize) -> bool {
        // Scan is complete if :
        //  - array is empty OR
        //  - array contains 1 direction OR
        //  - current index points to array's last element
        len == 0 || len == 1 || i == len - 1
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn returns_expected() {
            use Direction::*;
            let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH, WEST];
            assert_eq!(dir_reduc(&a), [WEST]);
            let a = [NORTH, WEST, SOUTH, EAST];
            assert_eq!(dir_reduc(&a), [NORTH, WEST, SOUTH, EAST]);
        }
    }
}

mod bankers_plan {
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
}

mod catalog {
    extern crate regex;

    use regex::Regex;

    fn catalog(s: &str, article: &str) -> String {
        let entries: Vec<&str> = s.split("\n\n").collect();

        let mut products = Vec::new();
        for e in entries {
            products.push(Product::from(e));
        }

        let matching_prods: Vec<String> = products
            .into_iter()
            .filter(move |p| p.name.contains(article))
            .map(|p| format!("{} > prx: ${} qty: {}", p.name, p.price, p.quantity))
            .collect();

        let mut result = matching_prods.join("\n");
        if result == "" {
            result = String::from("Nothing");
        }
        result
    }

    #[derive(Debug)]
    struct Product {
        name: String,
        price: String,
        quantity: String,
    }


    impl From<&str> for Product {
        fn from(s: &str) -> Self {
            let r = Regex::new("^<prod><name>(.*)</name><prx>(.*)</prx><qty>(.*)</qty></prod>$")
                .unwrap();
            let caps = r.captures(s).unwrap();
            Product {
                name: caps.get(1).unwrap().as_str().to_string(),
                price: caps.get(2).unwrap().as_str().to_string(),
                quantity: caps.get(3).unwrap().as_str().to_string(),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn s() -> String {
            let a: Vec<&str> = vec![
                "<prod><name>drill</name><prx>99</prx><qty>5</qty></prod>",
                "<prod><name>hammer</name><prx>10</prx><qty>50</qty></prod>",
                "<prod><name>screwdriver</name><prx>5</prx><qty>51</qty></prod>",
                "<prod><name>table saw</name><prx>1099.99</prx><qty>5</qty></prod>",
                "<prod><name>saw</name><prx>9</prx><qty>10</qty></prod>",
                "<prod><name>chair</name><prx>100</prx><qty>20</qty></prod>",
                "<prod><name>fan</name><prx>50</prx><qty>8</qty></prod>",
                "<prod><name>wire</name><prx>10.8</prx><qty>15</qty></prod>",
                "<prod><name>battery</name><prx>150</prx><qty>12</qty></prod>",
                "<prod><name>pallet</name><prx>10</prx><qty>50</qty></prod>",
                "<prod><name>wheel</name><prx>8.80</prx><qty>32</qty></prod>",
                "<prod><name>extractor</name><prx>105</prx><qty>17</qty></prod>",
                "<prod><name>bumper</name><prx>150</prx><qty>3</qty></prod>",
                "<prod><name>ladder</name><prx>112</prx><qty>12</qty></prod>",
                "<prod><name>hoist</name><prx>13.80</prx><qty>32</qty></prod>",
                "<prod><name>platform</name><prx>65</prx><qty>21</qty></prod>",
                "<prod><name>car wheel</name><prx>505</prx><qty>7</qty></prod>",
                "<prod><name>bicycle wheel</name><prx>150</prx><qty>11</qty></prod>",
                "<prod><name>big hammer</name><prx>18</prx><qty>12</qty></prod>",
                "<prod><name>saw for metal</name><prx>13.80</prx><qty>32</qty></prod>",
                "<prod><name>wood pallet</name><prx>65</prx><qty>21</qty></prod>",
                "<prod><name>circular fan</name><prx>80</prx><qty>8</qty></prod>",
                "<prod><name>exhaust fan</name><prx>62</prx><qty>8</qty></prod>",
                "<prod><name>window fan</name><prx>62</prx><qty>8</qty></prod>",
            ];
            return a.join("\n\n");
        }

        fn dotest(s: &str, article: &str, exp: &str) -> () {
            println!("s:{:?}", s);
            println!("article:{:?}", article);
            let ans = catalog(s, article);
            println!("actual: {:?}", ans);
            println!("expect: {:?}", exp);
            println!("{}", ans == exp);
            assert_eq!(ans, exp);
            println!("{}", "-");
        }

        #[test]
        fn basic_tests() {
            let s = &s();
            dotest(s, "ladder", "ladder > prx: $112 qty: 12");
            dotest(s, "ladder", "ladder > prx: $112 qty: 12");
        }
    }

}

mod two_to_one {
    fn longest(a1: &str, a2: &str) -> String {
        let a1_copy = a1.clone();
        let owned_a1: String = a1_copy.to_owned();
        let concat = owned_a1 + a2;
        let concat_iter = concat.chars();
        let mut concat: Vec<char> = concat_iter.collect();
        concat.sort();
        remove_duplicate_chars(concat.into_iter())
    }

    fn remove_duplicate_chars(s: std::vec::IntoIter<char>) -> String {
        let mut last = '0';
        let s = s.filter(move |c| {
            if *c == last {
                return false;
            } else {
                last = c.clone();
            }

            true
        });

        s.collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn testing(s1: &str, s2: &str, exp: &str) -> () {
            println!("s1:{:?} s2:{:?}", s1, s2);
            println!("{:?} {:?}", longest(s1, s2), exp);
            println!("{}", longest(s1, s2) == exp);
            assert_eq!(&longest(s1, s2), exp)
        }

        #[test]
        fn basic_tests() {
            testing("aretheyhere", "yestheyarehere", "aehrsty");
            testing(
                "loopingisfunbutdangerous",
                "lessdangerousthancoding",
                "abcdefghilnoprstu",
            );
        }
    }
}

mod how_green_is_my_valley {
    fn make_valley(arr: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut left_wing: Vec<i32> = Vec::new();
        let mut right_wing: Vec<i32> = Vec::new();

        let mut bottom = None;

        let mut copy = arr.clone();
        copy.sort();

        // Odd
        if arr.len() % 2 != 0 {
            // Remove odd bottom from the array
            bottom = Some(copy[0]);
            copy.remove(0);
        }

        for _ in 0..arr.len() / 2 {
            let next_tuple = dbg!(get_next_tuple(&mut copy));
            left_wing.push(next_tuple.0);
            right_wing.insert(0, next_tuple.1);
        }

        result.append(&mut left_wing);
        match bottom {
            Some(b) => result.push(b),
            None => (),
        }
        result.append(&mut right_wing);
        result
    }

    fn get_next_tuple(arr: &mut Vec<i32>) -> (i32, i32) {
        let len = arr.len();
        let next_tuple = (arr[len - 1], arr[len - 2]);
        arr.pop();
        arr.pop();
        next_tuple
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn dotest(arr: Vec<i32>, exp: Vec<i32>) -> () {
            println!("arr: {:?}", arr);
            let ans = make_valley(arr);
            println!("actual:\n{:?}", ans);
            println!("expect:\n{:?}", exp);
            println!("{}", ans == exp);
            assert_eq!(ans, exp);
            println!("{}", "-");
        }

        #[test]
        fn basic_tests() {
            dotest(
                vec![17, 17, 15, 14, 8, 7, 7, 5, 4, 4, 1],
                vec![17, 15, 8, 7, 4, 1, 4, 5, 7, 14, 17],
            );
            dotest(vec![20, 7, 6, 2], vec![20, 6, 2, 7]);
        }
    }

}

mod reducing_by_steps {
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
}

mod bouncing_balls {
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
}

mod multiply {
    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn returns_expected() {
            assert_eq!(multiply(3, 5), 15)
        }
    }
}
