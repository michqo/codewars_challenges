#[allow(dead_code)]
fn solution(s: &str) -> String {
    let mut new_s = String::new();

    for c in s.chars() {
        if c.is_ascii_uppercase() {
            new_s.push(' ');
        }
        new_s.push(c);
    }

    new_s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}
