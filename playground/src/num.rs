#[cfg(test)]
mod num_test {
    use num_traits::ToPrimitive;

    #[test]
    fn num_convert() {
        assert!(f64::MAX > i64::MAX as f64);
        assert_eq!(i64::MAX, 9223372036854775807);
        assert!(i64::MAX.to_i32().is_none());

        assert_eq!(f64::MAX, 1.7976931348623157e308);
        assert!(f64::MAX.to_i32().is_none());
        assert!(f64::MAX.to_i64().is_none());

        assert_eq!(12.01.to_i64(), Some(12));
        assert_eq!(12.01.to_i32(), Some(12));

        assert_eq!(true as i64, 1);
        assert_eq!(false as i64, 0);
    }
}
