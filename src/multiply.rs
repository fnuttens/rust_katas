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
