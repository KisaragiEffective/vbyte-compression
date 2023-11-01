use super::*;

macro_rules! single_test {
        ($n:ident, $i:expr) => {
            #[test]
            fn $n() {
                let i = $i;
                let c = compress(i);
                let (d, rest) = decompress(&c).unwrap();
                assert_eq!(i, d);
                assert!(rest.is_empty(), "no rest should be remaining.");
            }
        };
    }

#[test]
fn simple1() {
    let i = 36;
    let c = compress(i);
    let (d, rest) = decompress(&c).unwrap();
    assert_eq!(i, d);
    assert!(rest.is_empty(), "no rest should be remaining.");
}

single_test!(simple0, 0);
single_test!(simple2, 2);
single_test!(simple3, 3);
single_test!(simple32, 32);
single_test!(simple64, 64);
single_test!(simple65, 65);
single_test!(simple127, 127);
single_test!(simple128, 128);
single_test!(simple244, 244);
single_test!(simple12341234, 12341234);

#[test]
fn test_set() {
    let ints = [
        2134123213213u64,
        2313,
        3,
        3213,
        21321,
        3213,
        213,
        213,
        5435,
        5654,
        6,
        5437,
        567,
        3465241345,
        677,
        90,
        98765,
        4,
        324567897654321,
        3456,
        7754,
        32,
        4567,
        432,
        56789654321,
        4,
        5678906543,
        256,
        7895432,
        56789654,
        3256,
        78543,
    ];

    for i in ints {
        let c = compress(i);
        let (d, rest) = decompress(&c).unwrap();

        assert_eq!(d, i);
        assert_eq!(rest, Vec::new());
    }

    // now test if rest is parsed correctly

    for i in ints {
        let mut c = compress(i);
        c.push(1);
        c.push(2);
        c.push(3);
        c.push(4);
        let (d, rest) = decompress(&c).unwrap();

        assert_eq!(d, i);
        assert_eq!(rest, vec![1, 2, 3, 4]);
    }
}

#[test]
fn list_compression() {
    let list = (0..100000).map(|n| n * 13).collect::<Vec<u64>>();
    let c = compress_list(&list);
    let d = decompress_list(&c).unwrap();

    assert_eq!(d, list);
}

#[test]
fn compression_fuzzing() {
    let list = (0..100000u64).map(|n| n * 13);
    for elem in list {
        let c = compress(elem);
        let (d, rest) = decompress(&c).unwrap();

        assert_eq!(d, elem);
        assert!(rest.is_empty(), "no data should be remaining.");
    }
}