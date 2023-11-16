use fraction::Signed;

type F = fraction::BigFraction;

struct BM {
    a: F,
    b: F,
    t: F,
    iter: Option<usize>,
}

impl BM {
    fn new(a: F, b: F) -> Self {
        Self {
            a,
            b,
            t: F::new(0u64, 1u64),
            iter: None,
        }
    }
    fn with_iter(mut self, iter: usize) -> Self {
        self.iter = Some(iter);
        self
    }
}

impl Iterator for BM {
    type Item = F;
    fn next(&mut self) -> Option<Self::Item> {
        matches!(self.iter, Some(x) if x > 0).then(|| {
            *self.iter.as_mut().unwrap() -= 1;
            self.t = (self.a.clone() + self.b.clone()) / F::new(2u64, 1u64);
            let ft = self.t.clone() * self.t.clone() - F::new(3u64, 1u64);
            if ft.clone() * (self.a.clone() * self.t.clone() - F::new(3u64, 1u64))
                < F::new(0u64, 1u64)
            {
                self.b = self.t.clone();
            } else {
                self.a = self.t.clone();
            }
            self.t.clone()
        })
    }
}

fn main() {
    let mut solver = BM::new(F::new(1u64, 1u64), F::new(2u64, 1u64)).with_iter(7);

    for (i, j) in solver.into_iter().enumerate() {
        println!("{}", j);
    }
}
