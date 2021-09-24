fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let mut p0c = p0 as f64;
    let pc = p as f64;
    let mut years = 0;
    loop {
        if p0c >= pc {
            println!("population: {}", p0c);
            return years;
        }
        years += 1;
        p0c += p0c * percent / 100.0;
        p0c += aug as f64;
    }
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) -> () {
        //println!("p0: {:?};", p0);
        //println!("percent: {:?};", percent);
        //println!("aug: {:?};", aug);
        //println!("p: {:?};", p);
        let ans = nb_year(p0, percent, aug, p);
        //println!("actual:\n{:?};", ans);
        //println!("expect:\n{:?};", exp);
        //println!("{};", ans == exp);
        assert_eq!(ans, exp);
        //println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        //dotest(1500, 5.0, 100, 5000, 15);
        //dotest(1500000, 2.5, 10000, 2000000, 10);
        dotest(165131, 0.2, 825, 1168431, 505);
    }
}
