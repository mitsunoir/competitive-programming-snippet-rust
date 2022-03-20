#[allow(unused_attributes)]
#[cargo_snippet::snippet("template")]

pub mod template {
    use std::io;
    use std::io::Read;
    #[allow(dead_code)]
    fn main() -> io::Result<()> {
        let r = io::stdin();
        let mut sc = Scanner::new(r.lock());
        let _n: usize = sc.read();
        Ok(())
    }
    struct Scanner<R> {
        r: R,
    }
    impl<R: io::Read> Scanner<R> {
        fn new(r: R) -> Scanner<R> {
            Scanner { r: r }
        }
        #[allow(dead_code)]
        fn read<T: std::str::FromStr>(&mut self) -> T {
            let bytes = self
                .r
                .by_ref()
                .bytes()
                .map(|b| b.unwrap())
                .skip_while(|&b| b == b' ' || b == b'\r' || b == b'\n')
                .take_while(|&b| b != b' ' && b != b'\r' && b != b'\n')
                .collect::<Vec<_>>();
            std::str::from_utf8(&bytes)
                .expect("convert bytes into &str failed")
                .parse()
                .ok()
                .expect("parse failed")
        }
    }
}
