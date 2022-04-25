#![allow(dead_code)]

trait MoneyTrait {
    fn new(amount:i32) -> Self;
    fn times(&self, multiplier: i32) -> Self;
}

#[derive(Debug)]
struct Doller{
    amount:i32
}

impl MoneyTrait for Doller {
    fn new(amount:i32) -> Self {
        Doller{amount:amount}
    }
    fn times(&self, multipler:i32) -> Self{
        Doller {amount:self.amount*multipler}
    }
}

impl PartialEq for Doller {
    fn eq(&self, other: &Doller) -> bool {
        self.amount == other.amount
    } 
}

#[derive(Debug)]
struct Franc{
    amount:i32
}

impl MoneyTrait for Franc {
    fn new(amount:i32) -> Self {
        Franc{amount:amount}
    }
    fn times(&self, multipler:i32) -> Self{
        Franc {amount:self.amount*multipler}
    }
}

impl PartialEq for Franc {
    fn eq(&self, other: &Franc) -> bool {
        self.amount == other.amount
    } 
}


#[cfg(test)]
mod mony_tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let five: Doller = Doller::new(5);
        assert_eq!(Doller::new(10), five.times(2));
        assert_eq!(Doller::new(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        // assert!(Doller::new(5).equals(Doller::new(5)));
        assert_eq!(Doller::new(5),Doller::new(5)); // operator overload
        assert_ne!(Doller::new(5),Doller::new(6)); // operator overload
        assert_eq!(Franc::new(5),Franc::new(5)); // operator overload
        assert_ne!(Franc::new(5),Franc::new(6)); // operator overload
        // assert_ne!(Doller::new(5),Franc::new(5)); 
    }

    #[test]
    fn test_franc_multiplication() {
        let five: Franc = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }
}
