use proyecto_matecomp::MAX_ITER;

fn main() {
    let mut p = 10.;
    let solver =
        BM::new(&|x| x.powf(5.) + 20. * x.powf(2.) + x + 1.5, (-3., -2.)).with_iter(MAX_ITER);

    for (i, s) in solver.into_iter().enumerate() {
        if s == p {
            println!("Equal sol iter {i}, val:\n{:.60}", p);
            break;
        }
        println!("{}: {:.60}", i, s);
        p = s;
    }
}

struct BM<'a> {
    f: &'a dyn Fn(f64) -> f64,
    a: f64,
    b: f64,
    iter: Option<usize>,
}

impl<'a> BM<'a> {
    fn new(f: &'a dyn Fn(f64) -> f64, (a, b): (f64, f64)) -> Self {
        Self {
            f,
            a,
            b,
            iter: None,
        }
    }

    fn with_iter(mut self, iter: usize) -> Self {
        self.iter = Some(iter);
        self
    }
}

impl<'a> Iterator for BM<'a> {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        matches!(self.iter, Some(x) if x > 0).then(|| {
            *self.iter.as_mut().unwrap() -= 1;
            let m = (self.a + self.b) / 2.;

            if (self.f)(m) >= 0. {
                self.b = m;
            } else {
                self.a = m;
            }

            m
        })
    }
}
