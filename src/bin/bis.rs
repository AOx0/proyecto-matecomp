use std::f64::consts::E;

use proyecto_matecomp::MAX_ITER;

fn main() {
    let mut p = 10.;
    let solver = BM::new(&|x| E.powf(3. * x) - 4., (0., 1.)).with_iter(MAX_ITER);

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
        matches!(self.iter, Some(x) if x > 0)
            .then(|| {
                *self.iter.as_mut().unwrap() -= 1;
                let m = (self.a + self.b) / 2.;
                let ins = [self.a, m, self.b];
                let pos = ins
                    .iter()
                    .enumerate()
                    .find_map(|(i, &v)| ((self.f)(v) >= 0. && i > 0).then_some(i));

                pos.and_then(|i| {
                    self.a = ins[i - 1];
                    self.b = ins[i];
                    Some(m)
                })
            })
            .flatten()
    }
}
