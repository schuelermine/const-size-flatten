macro_rules! test_flatten_n_m {
    ($n: literal, $m: literal) => {
        ::paste::paste! {
            #[test]
            fn [<test_flatten_ $n _ $m>]() {
                let len = $n * $m;
                let mut iter = ::const_size_flatten::const_size_flatten((1u8..=$n).map(|n| [n; $m]));
                assert_eq!(iter.len(), len);
                let mut i = 0;
                loop {
                    if i == len { break }
                    iter.next();
                    i += 1;
                    assert_eq!(iter.len(), len - i)
                }
            }
        }
    };
}

macro_rules! test_flat_map_n_m {
    ($n: literal, $m: literal) => {
        ::paste::paste! {
            #[test]
            fn [<test_flat_map_ $n _ $m>]() {
                let len = $n * $m;
                let mut iter = ::const_size_flatten::const_size_flat_map(1u8..=$n, |n| [n; $m]);
                assert_eq!(iter.len(), len);
                let mut i = 0;
                loop {
                    if i == len { break }
                    iter.next();
                    i += 1;
                    assert_eq!(iter.len(), len - i)
                }
            }
        }
    };
}

macro_rules! test_flat_map_and_flatten {
    ($n: literal, $m: literal) => {
        test_flatten_n_m!($n, $m);
        test_flat_map_n_m!($n, $m);
    };
}

test_flat_map_and_flatten!(0, 0);
test_flat_map_and_flatten!(10, 0);
test_flat_map_and_flatten!(0, 10);
test_flat_map_and_flatten!(1, 1);
test_flat_map_and_flatten!(1, 10);
test_flat_map_and_flatten!(10, 1);
test_flat_map_and_flatten!(1, 0);
test_flat_map_and_flatten!(0, 1);
test_flat_map_and_flatten!(3, 5);
test_flat_map_and_flatten!(4, 4);
test_flat_map_and_flatten!(4, 10);
