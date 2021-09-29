fn get_count(string: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    string.chars().filter(|c| vowels.contains(&c)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_tests() {
        assert_eq!(get_count("o a kak ushakov lil vo kashu kakao"), 13);
    }
}
