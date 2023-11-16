use proyecto_matecomp::MAX_ITER;

fn main() {
    let mut p = 10.;
    let sr = SR::new(
        &|x| x.powf(5.) + 20. * x.powf(2.) + x + 1.5,
        &|x| 5. * x.powf(4.) + 40. * x + 1.,
        p,
    )
    .with_iter(MAX_ITER);

    for (i, s) in sr.into_iter().enumerate() {
        if s == p {
            println!("Equal sol iter {i}, val:\n{:.60}", p);
            break;
        }
        println!("{}: {:.60}", i, s);
        p = s;
    }
}

struct SR<'a> {
    f: &'a dyn Fn(f64) -> f64,
    fp: &'a dyn Fn(f64) -> f64,
    sol: f64,
    iter: Option<usize>,
}

impl<'a> SR<'a> {
    #[must_use]
    fn new(f: &'a dyn Fn(f64) -> f64, fp: &'a dyn Fn(f64) -> f64, guess: f64) -> Self {
        Self {
            f,
            fp,
            sol: guess,
            iter: None,
        }
    }

    #[must_use]
    fn with_iter(mut self, iter: usize) -> Self {
        self.iter = Some(iter);
        self
    }
}

impl<'a> Iterator for SR<'a> {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        matches!(self.iter, Some(x) if x > 0).then(|| {
            *self.iter.as_mut().unwrap() -= 1;
            self.sol = self.sol - ((self.f)(self.sol) / (self.fp)(self.sol));
            self.sol
        })
    }
}
