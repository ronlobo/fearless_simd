//! Sinewave generation example, as benchmark.

#![feature(test)]

extern crate test;

extern crate fearless_simd;

use fearless_simd::{count, GeneratorF32, SimdF32, SimdFnF32};

use test::Bencher;

struct Sin9Fn;
impl SimdFnF32 for Sin9Fn {
    #[inline]
    fn call<S: SimdF32>(&mut self, x: S) -> S {
        let c0 =   6.28308759;
        let c1 = -41.33318707;
        let c2 =  81.39900205;
        let c3 = -74.66884436;
        let c4 =  33.15324345;

        let a = (x - x.round()).abs() - 0.25;
        let a2 = a * a;
        ((((a2 * c4 + c3) * a2 + c2) * a2 + c1) * a2 + c0) * a
    }
}

fn gen_sinewave(freq: f32, obuf: &mut [f32]) {
    count(0.25, freq).map(Sin9Fn).collect(obuf);
}

fn gen_sin_scalar(freq: f32, obuf: &mut [f32]) {
    let delta_phase = 2.0 * ::std::f32::consts::PI * freq;
    let mut phase = 0.0f32;
    for out in obuf {
        *out = phase.sin();
        phase += delta_phase;
    }
}

#[bench]
fn sinewave(b: &mut Bencher) {
    let mut obuf = [0.0; 64];
    b.iter(|| gen_sinewave(test::black_box(0.1), &mut obuf));
}

#[bench]
fn sin_scalar(b: &mut Bencher) {
    let mut obuf = [0.0; 64];
    b.iter(|| gen_sin_scalar(test::black_box(0.1), &mut obuf));
}
