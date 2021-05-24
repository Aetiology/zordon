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
struct MulValLitEndUnsignTest<'a> {
    pub unsigned_16: MulByteVal<'a, u16, LitEnd>,
    pub unsigned_32: MulByteVal<'a, u32, LitEnd>,
    pub unsigned_64: MulByteVal<'a, u64, LitEnd>,
    pub unsigned_128: MulByteVal<'a, u128, LitEnd>,
}

#[allow(dead_code)]
#[derive(MutViewNew)]
struct MulValBigEndUnsignTest<'a> {
    pub unsigned_16: MulByteVal<'a, u16, BigEnd>,
    pub unsigned_32: MulByteVal<'a, u32, BigEnd>,
    pub unsigned_64: MulByteVal<'a, u64, BigEnd>,
    pub unsigned_128: MulByteVal<'a, u128, BigEnd>,
}

#[test]
fn mulval_val() {
    const SIMPLEVAL_TESTDATA: [u8; 30] = [
        2, 3, 4, 5, 6, 7, 8, 9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
        0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
    ];

    let mut buf = SIMPLEVAL_TESTDATA.to_vec();
    let (t, _) = MulValLitEndUnsignTest::new(&mut buf);

    assert_eq_hex!(t.unsigned_16.val(), 0x302);
    assert_eq_hex!(t.unsigned_32.val(), 0x7060504);
    assert_eq_hex!(t.unsigned_64.val(), 0xF0E0D0C0B0A0908);
    assert_eq_hex!(t.unsigned_128.val(), 0x1F1E1D1C_1B1A1918_17161514_13121110);

    let mut buf = SIMPLEVAL_TESTDATA.to_vec();
    let (t, _) = MulValBigEndUnsignTest::new(&mut buf);

    assert_eq_hex!(t.unsigned_16.val(), 0x0203);
    assert_eq_hex!(t.unsigned_32.val(), 0x04050607);
    assert_eq_hex!(t.unsigned_64.val(), 0x08090A0B_0C0D0E0F);
    assert_eq_hex!(t.unsigned_128.val(), 0x10111213_14151617_18191A1B_1C1D1E1F);
}

#[test]
fn mulval_set() {
    let mut buf = vec![0; 30];
    let (mut t, _) = MulValLitEndUnsignTest::new(&mut buf);

    t.unsigned_16.set(0x1112);
    t.unsigned_32.set(0xD0E0F10);
    t.unsigned_64.set(0x5060708_090A0B0C);
    t.unsigned_128.set(0x0D0E0F10_11121314_15161718_191A1B1C);

    assert_eq_hex!(buf[0..2], [0x12, 0x11]);
    assert_eq_hex!(buf[2..6], [0x10, 0xF, 0xE, 0xD]);
    assert_eq_hex!(buf[6..14], [0xC, 0xB, 0xA, 0x9, 0x8, 0x7, 0x6, 0x5]);
    assert_eq_hex!(
        buf[14..30],
        [
            0x1C, 0x1B, 0x1A, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10, 0xF, 0xE,
            0xD
        ]
    );

    let mut buf = vec![0; 30];
    let (mut t, _) = MulValBigEndUnsignTest::new(&mut buf);

    t.unsigned_16.set(0x1112);
    t.unsigned_32.set(0xD0E0F10);
    t.unsigned_64.set(0x5060708_090A0B0C);
    t.unsigned_128.set(0x0D0E0F10_11121314_15161718_191A1B1C);

    /*
    assert_eq_hex!(buf[0..2], [0x11, 0x12]);
    assert_eq_hex!(buf[2..6], [0xD, 0xE, 0xF, 0x10]);
    assert_eq_hex!(buf[6..14], [0xC, 0xB, 0xA, 0x9, 0x8, 0x7, 0x6, 0x5]);
    assert_eq_hex!(
        buf[14..30],
        [
            0x1C, 0x1B, 0x1A, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10, 0xF, 0xE,
            0xD
        ]
    );
    */
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
            let mut buf = vec![2; 30];
            let (mut t, _) = MulValLitEndUnsignTest::new(&mut buf);

            t.unsigned_16 $oper 2;
            t.unsigned_32 $oper 2;
            t.unsigned_64 $oper 2;

            assert_eq_hex!(buf[0..2], $result[0..2]);
            assert_eq_hex!(buf[2..6], $result[2..6]);
            assert_eq_hex!(buf[6..14], $result[6..14]);
        }
    };
}

impl_simpleval_assign_test!(simple_val_addassign, +=, [4, 2, 4, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2]);
impl_simpleval_assign_test!(simple_val_subassign, -=, [0, 2, 0, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2]);
impl_simpleval_assign_test!(simple_val_mulassign, *=, [4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4]);
impl_simpleval_assign_test!(simple_val_divassign, /=, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
