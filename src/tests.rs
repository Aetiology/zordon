#[allow(unused_imports)]
use crate::{types::*, MutViewNew};
#[allow(unused_attributes)]
#[macro_use]
#[allow(unused_imports)]
#[macro_use]
use assert_hex::assert_eq_hex;

#[cfg(test)]
#[allow(dead_code)]
#[derive(MutViewNew)]
struct ValTest<'a> {
    pub unsigned_8: ByteVal<'a, u8>,
    pub unsigned_16: MulByteVal<'a, u16>,
    pub unsigned_32: MulByteVal<'a, u32>,
    pub unsigned_64: MulByteVal<'a, u64>,
    pub unsigned_arr: ArrayVal<'a, [u8; 4]>,
}

#[test]
fn simpleval_val() {
    const SIMPLEVAL_TESTDATA: [u8; 0x13] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF, 0x10, 0x11, 0x12, 0x13,
    ];

    let mut buf = SIMPLEVAL_TESTDATA.to_vec();
    let (t, _): (ValTest, _) = ValTest::new(&mut buf);

    assert_eq_hex!(t.unsigned_8.val(), 01);
    assert_eq_hex!(t.unsigned_16.val(), 0x302);
    assert_eq_hex!(t.unsigned_32.val(), 0x7060504);
    assert_eq_hex!(t.unsigned_64.val(), 0xF0E0D0C0B0A0908);
}

#[test]
fn arrayval_deref() {
    let arr = [0x1, 0x2, 0x3, 0x4];
    let mut buf = arr.clone();
    let (t, _): (ArrayVal<[u8; 4]>, _) = ArrayVal::new(&mut buf);

    assert_eq_hex!(*t.as_ref(), arr);
}

#[test]
fn arrayval_deref_mut() {
    let mut buf = [0];
    let (t, _): (ArrayVal<[u8; 1]>, _) = ArrayVal::new(&mut buf);

    t.as_mut_ref()[0] = 0xA;

    assert_eq_hex!(*t.as_ref(), [0xA]);
}

#[test]
fn simpleval_set() {
    let mut buf = vec![0; 19];
    let (mut t, _) = ValTest::new(&mut buf);

    t.unsigned_8.set(0x13);
    t.unsigned_16.set(0x1112);
    t.unsigned_32.set(0xD0E0F10);
    t.unsigned_64.set(0x5060708090A0B0C);

    assert_eq_hex!(buf[0], 0x13);
    assert_eq_hex!(buf[1..3], [0x12, 0x11]);
    assert_eq_hex!(buf[3..7], [0x10, 0xF, 0xE, 0xD]);
    assert_eq_hex!(buf[7..15], [0xC, 0xB, 0xA, 0x9, 0x8, 0x7, 0x6, 0x5]);
}

#[test]
fn arrayval_set() {
    let mut buf = vec![0; 4];
    let (mut t, _): (ArrayVal<[u8; 4]>, &mut [u8]) = ArrayVal::new(&mut buf);

    let new_data = [0x4, 0x3, 0x2, 0x1];

    t.set(&new_data);

    assert_eq_hex!(buf[0..4], new_data);
}

#[macro_export]
macro_rules! impl_simpleval_assign_test {
    ($fname:ident, $oper:tt, $result:tt) => {
        #[test]
        fn $fname() {
            let mut buf = vec![2; 19];
            let (mut t, _) = ValTest::new(&mut buf);

            t.unsigned_8 $oper 2;
            t.unsigned_16 $oper 2;
            t.unsigned_32 $oper 2;
            t.unsigned_64 $oper 2;

            assert_eq_hex!(buf[0], $result[0]);
            assert_eq_hex!(buf[1..3], $result[1..3]);
            assert_eq_hex!(buf[3..7], $result[3..7]);
            assert_eq_hex!(buf[7..15], $result[7..15]);
        }
    };
}

impl_simpleval_assign_test!(simple_val_addassign, +=, [4, 4, 2, 4, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2]);
impl_simpleval_assign_test!(simple_val_subassign, -=, [0, 0, 2, 0, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2]);
impl_simpleval_assign_test!(simple_val_mulassign, *=, [4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4]);
impl_simpleval_assign_test!(simple_val_divassign, /=, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
