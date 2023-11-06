use const_size_flatten::IteratorExtension;
use paste::paste;

fn test<const N: usize>() {
    let mut iter = [[(); N]; 3].into_iter().const_size_flatten();
    let mut consumed = 0;
    while if consumed % 8 == 7 {
        iter.next()
    } else {
        iter.next_back()
    }
    .is_some()
    {
        consumed += 1;
        assert_eq!(N * 3 - consumed, iter.size_hint().0);
    }
}

macro_rules! test_n {
    ($n: literal) => {
        paste! {
            #[test]
            fn [<test_ $n>]() {
                test::<$n>();
            }
        }
    };
}

test_n!(0);
test_n!(1);
test_n!(2);
test_n!(3);
test_n!(4);
test_n!(5);
test_n!(6);
test_n!(7);
test_n!(8);
test_n!(9);
test_n!(10);
test_n!(12);
