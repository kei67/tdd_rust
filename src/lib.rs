#![allow(dead_code)]


use std::ops::Mul;

#[derive(Debug)]
struct Money {
    amount: i32,
}

#[derive(Debug)]
struct Doller {
    money: Money,
}

#[derive(Debug)]
struct Franc {
    money: Money,
}
trait MoneyTrait {
    fn new(amount: i32) -> Self;
    fn amount(&self) -> i32;
    fn times(&self, multiplier: i32) -> Self;
}

impl MoneyTrait for Doller {
    fn new(amount: i32) -> Self {
        Doller {
            money: Money { amount: amount },
        }
    }
    fn amount(&self) -> i32 {
        self.money.amount
    }
    fn times(&self, multipler: i32) -> Self {
        Doller {
            money: Money {
                amount: self.amount() * multipler,
            },
        }
    }
}

impl MoneyTrait for Franc {
    fn new(amount: i32) -> Self {
        Franc {
            money: Money { amount: amount },
        }
    }
    fn amount(&self) -> i32 {
        self.money.amount
    }
    fn times(&self, multipler: i32) -> Self {
        Franc {
            money: Money {
                amount: self.amount() * multipler,
            },
        }
    }
}

impl PartialEq for Doller {
    fn eq(&self, other: &Self) -> bool {
        self.amount() == other.amount()
    }
}

impl PartialEq for Franc {
    fn eq(&self, other: &Self) -> bool {
        self.amount() == other.amount()
    }
}

impl Mul<i32> for Doller {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Self::new(self.amount() * rhs)
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
        assert_eq!(Doller::new(5), Doller::new(5)); // operator overload
        assert_ne!(Doller::new(5), Doller::new(6)); // operator overload
        assert_eq!(Franc::new(5), Franc::new(5)); // operator overload
        assert_ne!(Franc::new(5), Franc::new(6)); // operator overload
                                                  // assert_ne!(Doller::new(5),Franc::new(5));
    }

    #[test]
    fn test_franc_multiplication() {
        let five: Franc = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }
}
