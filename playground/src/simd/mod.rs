mod bitvec;

#[cfg(test)]
mod simd_test {
    use std::simd::f32x4;
    extern crate test;
    use test::Bencher;

    // https://rust-lang.github.io/packed_simd/perf-guide/target-feature/rustflags.html
    // https://github.com/rust-lang/portable-simd/blob/master/beginners-guide.md#terms
    //
    // Lane: A single element position within a vector is called a lane.
    //
    // Bit Widths: When talking about SIMD, the bit widths used are the bit size of the vectors involved, not the individual elements.
    // So "128-bit SIMD" has 128-bit vectors, and that might be f32x4, i32x4, i16x8, or other variations. While 128-bit SIMD is the most common, there's also 64-bit, 256-bit, and even 512-bit on the newest CPUs.
    #[test]
    fn simd_basic_test() {
        let a = f32x4::splat(10.0);
        let b = f32x4::from_array([10.0, 20.0, 30.0, 40.0]);
        println!("{:?}", a + b);
    }

    #[bench]
    #[ignore]
    fn test_raw_add(b: &mut Bencher) {
        let a1 = 10.0;
        let a2 = vec![10.0, 20.0, 30.0, 40.0];
        b.iter(|| a2.iter().map(|x| x + a1).collect::<Vec<f32>>());
    }

    #[bench]
    #[ignore]
    fn test_simd_add(b: &mut Bencher) {
        let a1 = f32x4::splat(10.0);
        let a2 = f32x4::from_array([10.0, 20.0, 30.0, 40.0]);
        b.iter(|| {
            let res = a1 + a2;
            let _ = res.as_array();
        });
    }
}
