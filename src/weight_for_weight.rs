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
