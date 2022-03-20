use cargo_snippet::snippet;

// only supports operations between ModInt x ModInt
#[snippet("mod_int")]
pub mod mod_int {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
    type InternalNum = i64;
    pub struct ModInt(pub InternalNum);

    impl Clone for ModInt {
        fn clone(&self) -> Self {
            Self(self.0)
        }
    }
    impl Copy for ModInt {}

    fn modulo() -> InternalNum {
        1_000_000_007 // 998_244_353
    }

    impl ModInt {
        pub fn new(x: InternalNum) -> Self {
            Self::internal_new(x)
        }

        fn internal_new(x: InternalNum) -> Self {
            if x < 0 {
                Self::internal_new(x + modulo())
            } else {
                Self(if x > modulo() { x % modulo() } else { x })
            }
        }

        pub fn pow(&self, mut e: u64) -> Self {
            let mut result = 1;
            let mut cur = self.0;
            let m = modulo();
            while e > 0 {
                if e % 2 == 1 {
                    result *= cur;
                    result %= m;
                }
                e /= 2;
                cur = (cur * cur) % m;
            }
            Self::internal_new(result)
        }

        pub fn inv(&self) -> Self {
            let m = modulo() as u64;
            self.pow(m - 2)
        }

        pub fn value(&self) -> InternalNum {
            self.0
        }
    }

    impl Add<ModInt> for ModInt {
        type Output = ModInt;
        fn add(self, rhs: ModInt) -> ModInt {
            ModInt::internal_new(self.value() + rhs.value() + modulo())
        }
    }

    impl AddAssign<ModInt> for ModInt {
        fn add_assign(&mut self, rhs: ModInt) {
            self.0 += rhs.value();
            if self.0 > modulo() {
                self.0 %= modulo();
            }
        }
    }

    impl Sub<ModInt> for ModInt {
        type Output = ModInt;
        fn sub(self, rhs: ModInt) -> ModInt {
            ModInt::internal_new(self.value() - rhs.value())
        }
    }

    impl SubAssign<ModInt> for ModInt {
        fn sub_assign(&mut self, rhs: ModInt) {
            self.0 -= rhs.value() - modulo();
            self.0 %= modulo();
        }
    }

    impl Mul<ModInt> for ModInt {
        type Output = ModInt;
        fn mul(self, rhs: ModInt) -> ModInt {
            ModInt::internal_new(self.value() * rhs.value())
        }
    }

    impl MulAssign<ModInt> for ModInt {
        fn mul_assign(&mut self, rhs: ModInt) {
            self.0 *= rhs.value();
            self.0 %= modulo();
        }
    }

    impl Div<ModInt> for ModInt {
        type Output = ModInt;
        fn div(self, rhs: ModInt) -> ModInt {
            self * rhs.inv()
        }
    }

    impl DivAssign<ModInt> for ModInt {
        fn div_assign(&mut self, rhs: ModInt) {
            self.0 *= rhs.inv().value();
            self.0 %= modulo();
        }
    }
}

// test on MOD = 1e9+7
#[cfg(test)]
mod tests {
    use super::mod_int::ModInt;

    #[test]
    fn add() {
        let x = ModInt(1_000_000_006);
        let y = ModInt(2);
        assert_eq!((x + y).value(), 1);
    }

    #[test]
    fn add_assign() {
        let mut x = ModInt(1_000_000_006);
        let y = ModInt(2);
        x += y;
        assert_eq!(x.value(), 1);
    }

    #[test]
    fn sub() {
        let x = ModInt(1);
        let y = ModInt(2);
        assert_eq!((x - y).value(), 1_000_000_006);
    }

    #[test]
    fn sub_assign() {
        let mut x = ModInt(1);
        let y = ModInt(2);
        x -= y;
        assert_eq!(x.value(), 1_000_000_006);
    }

    #[test]
    fn mul() {
        let x = ModInt(1_000_000);
        let y = ModInt(1_000_000);
        assert_eq!((x * y).value(), 999993007);
    }

    #[test]
    fn mul_assign() {
        let mut x = ModInt(1_000_000);
        let y = ModInt(1_000_000);
        x *= y;
        assert_eq!(x.value(), 999993007);
    }

    #[test]
    fn div() {
        let x = ModInt(1_000_000);
        let y = ModInt(1_000_000);
        assert_eq!((x / y).value(), 1);
    }

    #[test]
    fn div_assign() {
        let mut x = ModInt(1_000_000);
        let y = ModInt(1_000_000);
        x /= y;
        assert_eq!(x.value(), 1);
    }

    #[test]
    fn pow() {
        let x = ModInt(100);
        assert_eq!(x.pow(100).value(), 424090053);
    }
}
