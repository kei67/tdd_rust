## TDD reading repository
* https://www.amazon.co.jp/テスト駆動開発-Kent-Beck/dp/4274217884
の第一章の写経です。

* 1章 仮実装
* 2章 明白な実装
```rust
struct Doller{
    amount:i32
}

impl Doller {
    fn new(amount:i32) -> Self {
        Doller{amount:amount}
    }
    fn times(&mut self, multipler:i32) -> Self{
        Doller {amount:self.amount*multipler}
    }
}


#[cfg(test)]
mod mony_tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut five: Doller = Doller::new(5);
        let product: Doller = five.times(2);
        assert_eq!(10, product.amount);
        let product: Doller = five.times(3);
        assert_eq!(15, product.amount);
        
    }
}
```

