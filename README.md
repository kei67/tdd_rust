# TDD reading repository
* https://www.amazon.co.jp/テスト駆動開発-Kent-Beck/dp/4274217884
の第一章の写経です。

## 1章 仮実装
省略
## 2章 明白な実装
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
    fn test_multiplication {
        let mut five: Doller = Doller::new(5);
        let product: Doller = five.times(2);
        assert_eq!(10, product.amount);
        let product: Doller = five.times(3);
        assert_eq!(15, product.amount);
    }
}
```

## 3章　三角測量
```rust
#[derive(Debug)]
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

impl PartialEq for Doller {
    fn eq(&self, other: &Doller) -> bool {
        self.amount == other.amount
    } 
}


#[cfg(test)]
mod mony_tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let mut five: Doller = Doller::new(5);
        let product: Doller = five.times(2);
        assert_eq!(10, product.amount);
        let product: Doller = five.times(3);
        assert_eq!(15, product.amount);
    }

    fn test_equality() {
        // assert!(Doller::new(5).equals(Doller::new(5)));
        assert_eq!(Doller::new(5),Doller::new(5)); // operator overload
    }
}

```
## 4章 意図を図るテスト
productを消した。
``` rust
    fn test_multiplication() {
        let five: Doller = Doller::new(5);
        assert_eq!(Doller::new(10), five.times(2));
        assert_eq!(Doller::new(15), five.times(3));
    }
```


## 5章 原則をあえて破るとき
Dollerを**コピー**してFrancを作った。
```rust 
#[derive(Debug)]
struct Franc{
    amount:i32
}

impl Franc {
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
```
以下のテストを追加
```rust
    #[test]
    fn test_franc_multiplication() {
        let five: Franc = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    
    }
```