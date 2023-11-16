use proyecto_matecomp::MAX_ITER;

fn main() {
    let (sol, mut p) = (1., 10.);
    let sr = MS::new(&|x| x.powf(5.) + 20. * x.powf(2.) + x + 1.5, sol, p).with_iter(MAX_ITER);

    for (i, s) in sr.into_iter().enumerate() {
        if s == p {
            println!("Equal sol iter {i}, val:\n{:.60}", p);
            break;
        }
        println!("{}: {:.60}", i, s);
        p = s;
    }
}

struct MS<'a> {
    f: &'a dyn Fn(f64) -> f64,
    solp1: f64,
    solm1: f64,
    iter: Option<usize>,
}

impl<'a> MS<'a> {
    fn new(f: &'a dyn Fn(f64) -> f64, solp1: f64, solm1: f64) -> Self {
        Self {
            f,
            solp1,
            solm1,
            iter: None,
        }
    }

    #[must_use]
    fn with_iter(mut self, iter: usize) -> Self {
        self.iter = Some(iter);
        self
    }
}

impl<'a> Iterator for MS<'a> {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        matches!(self.iter, Some(x) if x > 0).then(|| {
            *self.iter.as_mut().unwrap() -= 1;
            let a = self.solp1;
            self.solp1 = self.solp1
                - (self.solp1 - self.solm1) / ((self.f)(self.solp1) - (self.f)(self.solm1))
                    * (self.f)(self.solp1);
            self.solm1 = a;
            self.solp1
        })
    }
}
