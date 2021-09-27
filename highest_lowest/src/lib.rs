fn high_and_low(numbers: &str) -> String {
    let chunks: Vec<i32> = numbers.split(' ').map(|x| x.parse().unwrap()).collect();
    let mut highest: i32 = chunks[0];
    let mut lowest: i32 = chunks[0];

    for n in chunks.into_iter() {
        if n > highest {
            highest = n;
        } else if n < lowest {
            lowest = n;
        }
    }

    format!("{} {}", highest, lowest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
      assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
    }
}
