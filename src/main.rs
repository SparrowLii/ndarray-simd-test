use ndarray::Array;
use std::time;
use ndarray_rand::RandomExt;
use ndarray_rand::rand::distributions::Uniform;

fn main() {
    // f64
    let a = Array::random((500, 500), Uniform::new(0., 2.));
    let b = Array::random((500, 500), Uniform::new(0., 2.));
    let start = time::SystemTime::now();
    let c_simd = &a * &b;
    let end = time::SystemTime::now();
    println!("simd f64 {:?}",end.duration_since(start).unwrap());

    let start = time::SystemTime::now();
    let c = a * b;
    let end = time::SystemTime::now();
    println!("normal f64 {:?}",end.duration_since(start).unwrap());
    assert_eq!(c_simd, c);

    // i32
    let a = Array::random((500, 500), Uniform::new(0, 255));
    let b = Array::random((500, 500), Uniform::new(0, 255));
    let start = time::SystemTime::now();
    let c_simd = &a * &b;
    let end = time::SystemTime::now();
    println!("simd i32 {:?}",end.duration_since(start).unwrap());

    let start = time::SystemTime::now();
    let c = a * b;
    let end = time::SystemTime::now();
    println!("normal i32 {:?}",end.duration_since(start).unwrap());
    assert_eq!(c_simd, c);

}
