#[cfg(test)]
mod mony_tests {
    #[test]
    fn it_works() {
        let five: Doller = Doller::new(5);
        five.times(2);
        assert_eq!(10, five.amount);
    }
}
