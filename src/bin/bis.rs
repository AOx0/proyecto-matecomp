use proyecto_matecomp::MAX_ITER;

fn main() {
    let solver = BM::new(
        &|x| x.powf(5.) + 20. * x.powf(2.) + x + 1.5,
        (-3, 3),
    );
    //.with_iter(MAX_ITER)
    //.with_precision(0.000000);

    for (i, s) in solver.into_iter().enumerate() {
        println!("{}: {:.60}", i, s);
    }
}

struct BM<'a> {
    f: &'a dyn Fn(f64) -> f64,
    a: f64,
    b: f64,
    p: Option<f64>,
    iter: Option<usize>,
}

impl<'a> BM<'a> {
    fn new(f: &'a dyn Fn(f64) -> f64, (a, b): (f64, f64)) -> Self {
        Self {
            f,
            a,
            b,
            p: None,
            iter: None,
        }
    }

    fn with_iter(mut self, iter: usize) -> Self {
        self.iter = Some(iter);
        self
    }

    fn with_precision(mut self, p: f64) -> Self {
        self.p = Some(p);
        self
    }
}

impl<'a> Iterator for BM<'a> {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        (
            self.a != (self.a + self.b) / 2. &&
            (matches!(self.iter, Some(x) if x > 0) || self.iter.is_none()) &&
            (matches!(self.p, Some(p) if (self.b - self.a).abs()/2. > p ) || self.p.is_none())
        ).then(|| {
            if self.iter.is_some() {
                *self.iter.as_mut().unwrap() -= 1;
            }
            
            let m = (self.a + self.b) / 2.;

            if (self.f)(m).is_sign_negative() != (self.f)(self.a).is_sign_negative() {
                self.b = m;
            } else {
                self.a = m;
            }

            m
        })
    }
}
