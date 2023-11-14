// use fuzzy_match::*;

type F = fraction::BigFraction;

fn main() {
    /* Muy muy caro, muy preciso. Solo son pruebas */
    // let p = F::from(1);
    // let sr = SR::new(
    //     &|x: F| {
    //         (x.clone() * x.clone() * x.clone() * x.clone() * x.clone())
    //             + F::new(20usize, 1usize) * (x.clone() * x.clone())
    //             + x.clone()
    //             + 1.5
    //     },
    //     &|x| {
    //         F::new(5usize, 1usize) * (x.clone() * x.clone() * x.clone() * x.clone())
    //             + F::new(40usize, 1usize) * x.clone()
    //             + 1
    //     },
    //     p.clone(),
    // )
    // .with_iter(15);

    // const P: usize = 200;

    // let mut p = format!("{:.P$}", p);
    // for (i, s) in sr.into_iter().enumerate() {
    //     let res = format!("{:.P$}", s);
    //     if res == p {
    //         println!("Equal sol iter {i}, val:\n{}", res);
    //         break;
    //     }
    //     println!("{res}");
    //     p = res;
    // }

    /* Usando floats comunes */
    const MAX_ITER: usize = 3500;

    println!("NEWTON-RAPHSON:");
    let mut p = 10.;
    let sr = SRF64::new(
        &|x| x.powf(5.) + 20. * x.powf(2.) + 0.5 + x + 1.,
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

    /* Usando floats comunes */
    println!("SECANTE:");
    let (sol, mut p) = (1., 10.);
    let sr = MS::new(&|x| x.powf(5.) + 20. * x.powf(2.) + 0.5 + x + 1., sol, p).with_iter(MAX_ITER);
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

struct SR<'a> {
    f: &'a dyn Fn(F) -> F,
    fp: &'a dyn Fn(F) -> F,
    sol: F,
    iter: Option<usize>,
}

impl<'a> SR<'a> {
    #[must_use]
    fn new(f: &'a dyn Fn(F) -> F, fp: &'a dyn Fn(F) -> F, guess: F) -> Self {
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
    type Item = F;
    fn next(&mut self) -> Option<Self::Item> {
        matches!(self.iter, Some(x) if x > 0).then(|| {
            *self.iter.as_mut().unwrap() -= 1;
            self.sol =
                self.sol.clone() - ((self.f)(self.sol.clone()) / (self.fp)(self.sol.clone()));
            self.sol.clone()
        })
    }
}
struct SRF64<'a> {
    f: &'a dyn Fn(f64) -> f64,
    fp: &'a dyn Fn(f64) -> f64,
    sol: f64,
    iter: Option<usize>,
}

impl<'a> SRF64<'a> {
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

impl<'a> Iterator for SRF64<'a> {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        matches!(self.iter, Some(x) if x > 0).then(|| {
            *self.iter.as_mut().unwrap() -= 1;
            self.sol = self.sol - ((self.f)(self.sol) / (self.fp)(self.sol));
            self.sol
        })
    }
}
