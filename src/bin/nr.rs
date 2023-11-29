use proyecto_matecomp::MAX_ITER;

fn main() {
    let sr = SR::new(
        &|x| x.powf(5.) + 20. * x.powf(2.) + x + 1.5,
        &|x| 5. * x.powf(4.) + 40. * x + 1.,
        10.,
    );

    for (i, s) in sr.into_iter().enumerate() {
        println!("{}: {:.60}", i, s);
    }
}

struct SR<'a> {
    f: &'a dyn Fn(f64) -> f64,
    fp: &'a dyn Fn(f64) -> f64,
    sol: f64,
    p: Option<f64>,
}

impl<'a> SR<'a> {
    #[must_use]
    fn new(f: &'a dyn Fn(f64) -> f64, fp: &'a dyn Fn(f64) -> f64, guess: f64) -> Self {
        Self {
            f,
            fp,
            sol: guess,
            p: None,
        }
    }

    #[must_use]
    fn with_precision(mut self, p: f64) -> Self {
        self.p = Some(p);
        self
    }
}

impl<'a> Iterator for SR<'a> {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        let s = self.sol - ((self.f)(self.sol) / (self.fp)(self.sol));
        (self.sol != s
            && (matches!(self.p, Some(p) if (s - self.sol).abs()/2. > p ) || self.p.is_none()))
        .then(|| {
            self.sol = s;
            self.sol
        })
    }
}
