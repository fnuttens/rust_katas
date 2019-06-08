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
