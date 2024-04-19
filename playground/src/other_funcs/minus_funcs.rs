pub fn subtract_10(num: u32) -> u32 {
    num - 10
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn subtract_10_test() {
        let x: u32 = 100;
        let y: u32 = subtract_10(x);
        println!("y is from test: {y}");
        assert_eq!(y, 90);
    }
}
