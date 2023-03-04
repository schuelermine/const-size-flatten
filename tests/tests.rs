use std::hint::black_box;

use const_size_flatten::ConstSizeIteratorExtension;
use paste::paste;

fn test_flatten<const N: u8, const M: usize, const B: bool>() {
    let mut len = N as usize * M;
    let mut iter = (1u8..=N).map(|n| [n; M]).const_size_flatten();
    while len != 0 {
        match B {
            true => black_box(iter.next_back()),
            false => black_box(iter.next()),
        };
        len -= 1;
        assert_eq!(len, iter.len())
    }
    assert_eq!(
        match B {
            true => black_box(iter.next_back()),
            false => black_box(iter.next()),
        },
        None
    )
}

fn test_flat_map<const N: u8, const M: usize, const B: bool>() {
    let mut len = N as usize * M;
    let mut iter = (1u8..=N).const_size_flat_map(|n| [n; M]);
    assert_eq!(len, iter.len());
    while len != 0 {
        match B {
            true => black_box(iter.next_back()),
            false => black_box(iter.next()),
        };
        len -= 1;
        assert_eq!(len, iter.len());
    }
    assert_eq!(
        match B {
            true => black_box(iter.next_back()),
            false => black_box(iter.next()),
        },
        None
    )
}

macro_rules! test_n_m {
    ($n: literal, $m: literal) => {
        paste! {
            #[test]
            fn [<test_flatten_ $n _ $m>]() {
                test_flatten::<$n, $m, false>();
            }

            #[test]
            fn [<test_flat_map_ $n _ $m>]() {
                test_flat_map::<$n, $m, false>();
            }

            #[test]
            fn [<test_flatten_back_ $n _ $m>]() {
                test_flatten::<$n, $m, true>();
            }

            #[test]
            fn [<test_flat_map_back_ $n _ $m>]() {
                test_flat_map::<$n, $m, true>();
            }
        }
    };
}

macro_rules! test {
    ($n: literal, $m: literal) => {
        test_n_m!($n, $m);
        test_n_m!($m, $n);
    };
    ($n: literal) => {
        test_n_m!($n, $n);
    };
}

test!(0);
test!(1);
test!(4);
test!(10);
test!(1, 0);
test!(10, 0);
test!(4, 0);
test!(3, 7);
test!(11, 5);
test!(3, 9);
