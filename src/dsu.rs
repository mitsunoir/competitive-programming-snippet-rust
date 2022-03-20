use cargo_snippet::snippet;

// for disjoint set union problems
#[snippet("dsu")]
pub mod dsu {
    type Node = usize;
    pub struct DSU {
        n: usize,
        parent: Vec<i32>,
    }

    impl DSU {
        pub fn new(n: usize) -> Self {
            DSU {
                n: n,
                parent: vec![-1; n],
            }
        }

        pub fn merge(&mut self, a: Node, b: Node) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            let mut a = self.root(a);
            let mut b = self.root(b);
            if a == b {
                return false;
            }
            if -self.parent[a] < -self.parent[b] {
                std::mem::swap(&mut a, &mut b);
            }
            self.parent[a] += self.parent[b];
            self.parent[b] = a as i32;
            true
        }

        pub fn root(&mut self, x: Node) -> Node {
            assert!(x < self.n);
            if self.parent[x] < 0 {
                return x;
            }
            self.parent[x] = self.root(self.parent[x] as usize) as i32;
            self.parent[x] as usize
        }

        pub fn same(&mut self, a: Node, b: Node) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            let a = self.root(a);
            let b = self.root(b);
            a == b
        }

        pub fn size(&mut self, x: Node) -> usize {
            assert!(x < self.n);
            let x = self.root(x);
            -self.parent[x] as usize
        }
    }
}

#[cfg(test)]
mod tests {
    use super::dsu::DSU;

    #[test]
    fn merge_same() {
        let mut dsu = DSU::new(5);
        dsu.merge(0, 1);
        dsu.merge(1, 2);
        dsu.merge(3, 4);
        assert!(dsu.same(0, 1));
        assert!(dsu.same(0, 2));
        assert!(!dsu.same(0, 3));
    }

    #[test]
    fn size() {
        let mut dsu = DSU::new(5);
        dsu.merge(0, 1);
        dsu.merge(1, 2);
        dsu.merge(3, 4);
        assert_eq!(dsu.size(0), 3);
        assert_eq!(dsu.size(1), 3);
        assert_eq!(dsu.size(2), 3);
        assert_eq!(dsu.size(3), 2);
        assert_eq!(dsu.size(4), 2);
    }
}
