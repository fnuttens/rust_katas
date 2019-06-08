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
