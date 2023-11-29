use proyecto_matecomp::MAX_ITER;

fn main() {
    let sr = MS::new(&|x| x.powf(5.) + 20. * x.powf(2.) + x + 1.5, 10.0, 11.).with_iter(MAX_ITER);

    for (i, s) in sr.into_iter().enumerate() {
        println!("{}: {:.60}", i, s);
    }
}

struct MS<'a> {
    f: &'a dyn Fn(f64) -> f64,
    xn: f64,
    xpn: f64,
    iter: Option<usize>,
    p: Option<f64>,
}

impl<'a> MS<'a> {
    fn new(f: &'a dyn Fn(f64) -> f64, solp1: f64, solm1: f64) -> Self {
        Self {
            f,
            xn: solp1,
            xpn: solm1,
            iter: None,
            p: None,
        }
    }

    #[must_use]
    fn with_iter(mut self, iter: usize) -> Self {
        self.iter = Some(iter);
        self
    }

    #[must_use]
    fn with_precision(mut self, p: f64) -> Self {
        self.p = Some(p);
        self
    }
}

impl<'a> Iterator for MS<'a> {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        (
            self.xn != self.xpn &&
            (matches!(self.iter, Some(x) if x > 0) || self.iter.is_none()) &&
            (matches!(self.p, Some(p) if (self.xn - self.xpn).abs()/2. > p ) || self.p.is_none())
        ).then(|| {
            *self.iter.as_mut().unwrap() -= 1;
            let a = self.xn;
            self.xn = self.xn
                - (self.xn - self.xpn) / ((self.f)(self.xn) - (self.f)(self.xpn))
                    * (self.f)(self.xn);
            self.xpn = a;
            self.xn
        })
    }
}
