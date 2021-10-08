fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
        assert_eq!(sq_in_rect(lng, wdth), exp)
    }

    #[test]
    fn tests_sq_in_rect() {
        testing(5, 3, Some(vec![3, 2, 1, 1]));
        testing(3, 5, Some(vec![3, 2, 1, 1]));
        testing(5, 5, None);
    }
}
