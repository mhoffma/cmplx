#![allow(unused)]
use fixed::prelude::*;
use fixed::types::I1F15;
use num::Complex;
use std::fmt;
use std::ops;

#[derive(Debug, Clone, PartialEq)]
pub struct Sc16(Complex<I1F15>);
impl Sc16 {
    pub fn new(r: f64, i: f64) -> Sc16 {
        Sc16 {
            0: Complex::new(I1F15::from_num(r), I1F15::from_num(i)),
        }
    }
    pub fn exp(&self) -> Sc16 {
        let e = Complex::new(f64::from_fixed(self.0.re), f64::from_fixed(self.0.im)).exp();
        Sc16::new(e.re, e.im)
    }
}
impl fmt::Display for Sc16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}+{}j", &self.0.re, &self.0.im)
    }
}
impl ops::Add for Sc16 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            0: Complex::new(
                self.0.re.saturating_add(other.0.re),
                self.0.im.saturating_add(other.0.im),
            ),
        }
    }
}
impl ops::Sub for Sc16 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            0: Complex::new(
                self.0.re.saturating_sub(other.0.re),
                self.0.im.saturating_sub(other.0.im),
            ),
        }
    }
}
impl ops::Mul for Sc16 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let a = self.0.re;
        let b = self.0.im;
        let c = other.0.re;
        let d = other.0.im;

        Self {
            0: Complex::new(
                a.wide_mul(c)
                    .saturating_sub(b.wide_mul(d))
                    .to_fixed::<I1F15>(),
                a.wide_mul(d)
                    .saturating_add(b.wide_mul(c))
                    .to_fixed::<I1F15>(),
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused)]
    use std::dbg;
    #[test]
    fn t0() {
        let a = Sc16::new(0.5, 0.25);
        let b = Sc16::new(0.2, 0.125);
        let d = a + b;
        assert_eq!(format!("{}", d), "0.7+0.375j");
    }
    #[test]
    fn t1() {
        let a = Sc16::new(0.5, 0.25);
        let b = Sc16::new(0.7, -0.625);
        let d = a + b;
        assert_eq!(format!("{}", d), "0.99997+-0.375j");
    }
    #[test]
    fn t2() {
        let a = Sc16::new(0.5, 0.25);
        let b = Sc16::new(0.2, 0.125);
        dbg!(&a);
        dbg!(&b);
        let d = a - b;
        dbg!(&d);
        assert_eq!(format!("{}", d), "0.3+0.125j");
    }
    #[test]
    fn t3() {
        let a = Sc16::new(0.5, 0.25);
        let b = Sc16::new(0.625, 0.125);
        let ar = Complex::new(0.5, 0.25);
        let br = Complex::new(0.625, 0.125);
        dbg!(&a);
        dbg!(&b);
        let d = a * b;
        let dr = ar * br;
        dbg!(&d);
        dbg!(&dr);
        assert_eq!(format!("{}", d), format!("{}+{}j", dr.re, dr.im));
    }

    use std::f64::consts::PI;
    #[allow(unused)]
    fn t4() {
        let theta = 0.5;
        let thetaf = 0.5 * 2.0 * PI;
        let a = Sc16::new(0.0, theta);
        let d = a.exp();
        let c = Sc16::new(thetaf.cos(), thetaf.sin());
        assert_eq!(format!("{}", d), format!("{}", c));
    }

    use ndarray::prelude::*;
    #[test]
    fn ndarray_tests() {
        let a = Array::from_iter((0..8).map(|_| Sc16::new(0.5, 0.0)));
        let b = Array::from_iter((0..8).map(|_| Sc16::new(0.0, 0.5)));
        let d = a + b;
        assert_eq!(d, Array::from_iter(0..8).map(|_| Sc16::new(0.5, 0.5)));
        let a = Array::from_iter((0..8).map(|_| Sc16::new(0.5, 0.0)));
        let b = Array::from_iter((0..8).map(|_| Sc16::new(0.0, 0.5)));
        let d = a - b;
        assert_eq!(d, Array::from_iter(0..8).map(|_| Sc16::new(0.5, -0.5)));
        let a = Array::from_iter((0..8).map(|_| Sc16::new(0.5, 0.0)));
        let b = Array::from_iter((0..8).map(|_| Sc16::new(0.0, 0.5)));
        let d = a * b;
        assert_eq!(d, Array::from_iter(0..8).map(|_| Sc16::new(0.0, 0.25)));
    }
}
