fn is_vowel(c: char) -> bool {
    let lowercase_c = c.to_lowercase().to_string();
    return match lowercase_c.as_str() {
        "a" => true,
        "e" => true,
        "i" => true,
        "o" => true,
        "u" => true,
        _ => false
    }
}

fn disemvowel(s: &str) -> String {
    let s_chars: Vec<char> = s.chars().collect();
    let mut new_s = String::new();

    for c in s_chars.into_iter() {
        if !is_vowel(c) {
            new_s.push(c);
        }
    }

    new_s
}

#[cfg(test)]
mod tests {
    use super::disemvowel;

    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}
