use crate::x86_64::_mm512_mul_pd;
use crate::x86_64::_mm512_loadu_pd;
use crate::x86_64::_mm512_storeu_pd;
use std::arch::x86_64::{_mm512_mullo_epi32, _mm512_loadu_epi32, _mm512_storeu_epi32};

// c = a * b
#[inline]
pub unsafe fn mul_512_f64(a: *const f64, b: *const f64, c: *mut f64, len: usize) {
    let conut = &len / 8;
    let rem = &len % 8;
    let mut a = a;
    let mut b = b;
    let mut c = c;
    for _ in 0..conut {
        let lhs = _mm512_loadu_pd(a);
        let rhs = _mm512_loadu_pd(b);
        _mm512_storeu_pd(c, _mm512_mul_pd(lhs, rhs));
        a = a.offset(8);
        b = b.offset(8);
        c = c.offset(8);
    }
    for _ in 0..rem {
        let x = a.read();
        let y = b.read();
        *c = x * y;
        a = a.offset(1);
        b = b.offset(1);
        c = c.offset(1);
    }
}
#[inline]
pub unsafe fn mul_512_i32(a: *const i32, b: *const i32, c: *mut i32, len: usize) {
    let conut = &len / 16;
    let rem = &len % 16;
    let mut a = a;
    let mut b = b;
    let mut c = c;
    for _ in 0..conut {
        let lhs = _mm512_loadu_epi32(a);
        let rhs = _mm512_loadu_epi32(b);
        _mm512_storeu_epi32(c, _mm512_mullo_epi32(lhs, rhs));
        a = a.offset(16);
        b = b.offset(16);
        c = c.offset(16);
    }
    for _ in 0..rem {
        let x = a.read();
        let y = b.read();
        *c = x * y;
        a = a.offset(1);
        b = b.offset(1);
        c = c.offset(1);
    }
}