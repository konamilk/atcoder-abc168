use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::f64::consts::PI;

fn main() {
    // let source = AutoSource::from("0");
    input! {
        // from source,
        a: f64,
        b: f64,
        h: f64,
        m: f64
    }
    ;

    let alpha = 2. * PI * m / 60.;
    let beta = 2. * PI * (h * 60. + m) / 720.;

    let theta = alpha - beta;

    let ans = (a*a + b*b - 2.*a*b*theta.cos()).sqrt();

    println!("{}", ans)
}
