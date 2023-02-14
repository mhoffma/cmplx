#![allow(unused)]
use ndarray::prelude::*;
use num::Complex;
use std::f64::consts::PI;

pub fn logb(n: usize) -> usize {
    (n as f32).log(2.0).ceil() as usize
}
pub fn brevidx(i: usize, n: usize) -> usize {
    if n > 0 {
        i.reverse_bits() >> (64 - n)
    } else {
        0
    }
}
pub fn brevidxs(n: usize) -> Vec<usize> {
    (0..n).map(|x| brevidx(x, logb(n))).collect::<Vec<_>>()
}
pub fn brev<T: Clone>(a: &[T]) -> Vec<T> {
    brevidxs(a.len()).iter().map(|i| a[*i].clone()).collect()
}

pub fn fft(y: &Array1<Complex<f64>>) -> Array1<Complex<f64>> {
    let mut x = y.clone();
    let n = x.len();
    let nstages = logb(n);
    for m in 0..nstages {
        let w = Array::from_iter((0..n / 2).map(|k| {
            Complex::new(
                0.0,
                -2.0 * PI * (brevidx(k, m) as f64) / (2.0f64.powi(m as i32 + 1)),
            )
        }))
        .mapv(|x| x.exp());

        let e = &x.clone().slice_move(s![..(n / 2)]);
        let o = &(x.clone().slice_move(s![(n / 2)..]) * w);
        x.slice_mut(s![0..;2]).assign(&(e + o));
        x.slice_mut(s![1..;2]).assign(&(e - o));
    }

    x.select(Axis(0), &brevidxs(n))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t0() {
        let mut a = Array::from_iter((0..8).map(|x| Complex::new(1.0, 0.0)));
        let o = fft(&a);
        let max = o
            .fold(0.0, |x, y| y.im.max(x))
            .max(o.fold(0.0, |x, y| y.re.max(x)));
        let min = o
            .fold(0.0, |x, y| y.im.min(x))
            .min(o.fold(0.0, |x, y| y.re.min(x)));
        let n = o.len();
        println!("{min}..{max} = {n}");
        assert_eq!(o[0].re, 8.0);
        assert_eq!(o[0].im, 0.0);
    }
    #[test]
    fn t1() {
        let mut a = Array::from_iter((0..8).map(|x| Complex::new(0.0, 0.0)));
        a[1].re = 1.0;
        let o = fft(&a);
        let max = o
            .fold(0.0, |x, y| y.im.max(x))
            .max(o.fold(0.0, |x, y| y.re.max(x)));
        let min = o
            .fold(0.0, |x, y| y.im.min(x))
            .min(o.fold(0.0, |x, y| y.re.min(x)));
        let n = o.len();
        let a = -2.0 * PI / 8.0;
        for i in 0..8 {
            let a = i as f64 * &a;
            let e = o[i];
            let re = a.cos();
            let im = a.sin();
            println!("{i} {e} == ({re},{im})");
            assert_eq!((o[i].re - a.cos()).abs() < 1.0e-4, true);
            assert_eq!((o[i].im - a.sin()).abs() < 1.0e-4, true);
        }
    }
}
