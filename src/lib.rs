pub struct BM<'a> {
    f: &'a dyn Fn(f64) -> f64,
    a: f64,
    b: f64,
    p: Option<f64>,
}

impl<'a> BM<'a> {
    pub fn new(f: &'a dyn Fn(f64) -> f64, (a, b): (f64, f64)) -> Self {
        Self { f, a, b, p: None }
    }

    fn with_precision(mut self, p: f64) -> Self {
        self.p = Some(p);
        self
    }
}

impl<'a> Iterator for BM<'a> {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        let m = (self.a + self.b) / 2.;
        (self.b != m && self.a != m && (matches!(self.p, Some(p) if m > p ) || self.p.is_none())).then(|| {
            if (self.f)(m).is_sign_negative() != (self.f)(self.a).is_sign_negative() {
                self.b = m;
            } else {
                self.a = m;
            }

            m
        })
    }
}

pub struct SR<'a> {
    f: &'a dyn Fn(f64) -> f64,
    fp: &'a dyn Fn(f64) -> f64,
    sol: f64,
    p: Option<f64>,
}

impl<'a> SR<'a> {
    #[must_use]
    pub fn new(f: &'a dyn Fn(f64) -> f64, fp: &'a dyn Fn(f64) -> f64, guess: f64) -> Self {
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


pub struct MS<'a> {
    f: &'a dyn Fn(f64) -> f64,
    xn: f64,
    xpn: f64,
    p: Option<f64>,
}

impl<'a> MS<'a> {
    pub fn new(f: &'a dyn Fn(f64) -> f64, solp1: f64, solm1: f64) -> Self {
        Self {
            f,
            xn: solp1,
            xpn: solm1,
            p: None,
        }
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
        (self.xn != self.xpn
            && (matches!(self.p, Some(p) if (self.xn - self.xpn).abs()/2. > p )
                || self.p.is_none()))
        .then(|| {
            let a = self.xn;
            self.xn = self.xn
                - (self.xn - self.xpn) / ((self.f)(self.xn) - (self.f)(self.xpn))
                    * (self.f)(self.xn);
            self.xpn = a;
            self.xn
        })
    }
}
