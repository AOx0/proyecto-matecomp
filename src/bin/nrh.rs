type F = fraction::BigFraction;

fn main() {
    /* Muy muy caro, muy preciso. Solo son pruebas */
    let p = F::from(2);
    let sr = SR::new(
        &|x: F| {
            (x.clone() * x.clone() * x.clone() * x.clone() * x.clone())
                + F::new(20usize, 1usize) * (x.clone() * x.clone())
                + x.clone()
                + 1.5
        },
        &|x| {
            F::new(5usize, 1usize) * (x.clone() * x.clone() * x.clone() * x.clone())
                + F::new(40usize, 1usize) * x.clone()
                + 1
        },
        p.clone(),
    )
    .with_iter(15);

    const P: usize = 200;

    let mut p = format!("{:.P$}", p);
    for (i, s) in sr.into_iter().enumerate() {
        let res = format!("{:.P$}", s);
        if res == p {
            println!("Equal sol iter {i}, val:\n{}", res);
            break;
        }
        println!("{res}");
        p = res;
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
