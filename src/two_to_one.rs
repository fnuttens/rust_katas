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
