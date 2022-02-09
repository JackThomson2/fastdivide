#![feature(test)]

extern crate fastdivide;
extern crate test;

use fastdivide::{make_divider_array, DividerU64};
use test::Bencher;

#[bench]
fn bench_normal_divide(b: &mut Bencher) {
    let q: u64 = test::black_box(112u64);
    b.iter(|| {
        let n: u64 = test::black_box(152342341u64);
        n / q
    })
}

const CONST_ARRAY: [DividerU64; 1024] = make_divider_array::<1024>();

fn divide(item: u64, divider: usize) -> u64 {
    let divider = unsafe { CONST_ARRAY.get_unchecked(divider) };
    divider.divide(item)
}

#[bench]
fn bench_fast_divide(b: &mut Bencher) {
    let fast_divider = DividerU64::divide_by(test::black_box(112u64));
    b.iter(|| {
        let n: u64 = test::black_box(152342341u64);
        fast_divider.divide(n)
    })
}

#[bench]
fn bench_const_array_fast_divide(b: &mut Bencher) {
    b.iter(|| divide(test::black_box(152342341u64), test::black_box(112)))
}

#[bench]
fn bench_const_fast_divide(b: &mut Bencher) {
    const FAST_DIVIDER: DividerU64 = DividerU64::divide_by(112u64);
    b.iter(|| {
        let n: u64 = test::black_box(152342341u64);
        FAST_DIVIDER.divide(n)
    })
}
