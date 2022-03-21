use cargo_snippet::snippet;

#[snippet("ord_util")]
pub mod ord_util {
    pub trait OrdUtil: PartialOrd + Sized {
        fn chmin(&mut self, mut rhs: Self) {
            if self > &mut rhs {
                *self = rhs;
            }
        }
        fn chmax(&mut self, mut rhs: Self) {
            if self < &mut rhs {
                *self = rhs;
            }
        }
    }
    impl<T: PartialOrd + Sized> OrdUtil for T {}
}

#[cfg(test)]
mod tests {
    use super::ord_util::OrdUtil;

    #[test]
    fn chmin() {
        let mut x: i64 = 5;
        x.chmin(3);
        assert_eq!(x, 3);
        x.chmin(5);
        assert_eq!(x, 3);
    }

    #[test]
    fn chmax() {
        let mut x: i64 = 1;
        x.chmax(4);
        assert_eq!(x, 4);
        x.chmax(1);
        assert_eq!(x, 4);
    }
}
