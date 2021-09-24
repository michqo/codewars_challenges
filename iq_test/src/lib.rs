fn iq_test(numbers: &str) -> u64 {
    let numbers_vec: Vec<u64> = numbers
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    let mut even_nums: Vec<u64> = Vec::new();
    let mut odd_nums: Vec<u64> = Vec::new();

    for (i, n) in numbers_vec.into_iter().enumerate() {
        if n % 2 == 0 {
            even_nums.push(i as u64);
        } else {
            odd_nums.push(i as u64);
        }
    }

    if even_nums.len() == 1 {
        return even_nums[0] + 1;
    } else {
        return odd_nums[0] + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(iq_test("2 4 7 8 10"), 3);
        assert_eq!(iq_test("1 2 2"), 1);
    }
}
