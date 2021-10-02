// Another solution: https://doc.rust-lang.org/std/primitive.slice.html#method.chunks

fn solution(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let chars_len = chars.len();
    let mut pairs: Vec<String> = Vec::new();

    for i in (0..chars_len).step_by(2) {
        // println!("i: {}", i);
        let mut pair = String::new();
        pair.push(chars[i]);
        if i + 1 < chars_len {
            pair.push(chars[i + 1]);
        } else {
            pair.push('_');
        }
        pairs.push(pair);
    }

    // println!("pairs: {:?}", pairs);

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}
