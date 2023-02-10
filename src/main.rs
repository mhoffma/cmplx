use core::ops::Add;
use core::ops::Deref;
use fixed;
use num;
use std::dbg;

type Q1_15 = fixed::FixedI16<fixed::types::extra::U15>;
type CQ1_15 = num::Complex<Q1_15>;

#[derive(Debug)]
struct Sc16(CQ1_15);

impl Add for Sc16 {
    type Output = Sc16;
    fn add(self, other: Sc16) -> Sc16 {
        Sc16(CQ1_15::new(self.0.re + other.0.re, self.0.im + other.0.im))
    }
}

impl Deref for Sc16 {
    type Target = CQ1_15;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let c = Q1_15::lit("0.2");
    let a = Sc16(CQ1_15::new(c, c));
    let b = Sc16(CQ1_15::new(Q1_15::lit("0.1"), Q1_15::lit("-0.1")));
    dbg!(a);
    let d = a + b;
    println!("{:?}", d);
}
