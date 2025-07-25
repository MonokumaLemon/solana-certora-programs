#[macro_export]
macro_rules! foreach_assert {
    ($arr:expr, $cmp:tt, $rhs:expr) => {{
        let bound: NativeInt = NativeInt::from($rhs as u64);
        for i in 0..$arr.len() {
            let elem: NativeInt = NativeInt::from(u64::from($arr[i]));
            clog!(elem);
            clog!(bound);
            cvlr_assert!(elem $cmp bound);
        }
    }};
}
