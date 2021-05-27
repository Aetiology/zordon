#[allow(unused_imports)]
use crate::{types::*, MutView};
#[allow(unused_attributes)]
#[macro_use]
#[allow(unused_imports)]
#[macro_use]
use assert_hex::assert_eq_hex;

#[cfg(test)]
#[derive(MutView)]
struct LitEndUnsignTest<'a> {
    pub unsigned_16: MulByteView<'a, u16, LitEnd>,
    pub unsigned_32: MulByteView<'a, u32, LitEnd>,
    pub unsigned_64: MulByteView<'a, u64, LitEnd>,
    pub unsigned_128: MulByteView<'a, u128, LitEnd>,
}

#[derive(MutView)]
struct BigEndUnsignTest<'a> {
    pub unsigned_16: MulByteView<'a, u16, BigEnd>,
    pub unsigned_32: MulByteView<'a, u32, BigEnd>,
    pub unsigned_64: MulByteView<'a, u64, BigEnd>,
    pub unsigned_128: MulByteView<'a, u128, BigEnd>,
}
#[derive(MutView)]
struct LitEndSignTest<'a> {
    pub signed_16: MulByteView<'a, i16, LitEnd>,
    pub signed_32: MulByteView<'a, i32, LitEnd>,
    pub signed_64: MulByteView<'a, i64, LitEnd>,
    pub signed_128: MulByteView<'a, i128, LitEnd>,
}

#[derive(MutView)]
struct BigEndSignTest<'a> {
    pub signed_16: MulByteView<'a, i16, BigEnd>,
    pub signed_32: MulByteView<'a, i32, BigEnd>,
    pub signed_64: MulByteView<'a, i64, BigEnd>,
    pub signed_128: MulByteView<'a, i128, BigEnd>,
}

const U8_RESULT: u8 = 0x01;
const U16_BE_RESULT: u16 = 0x0203;
const U32_BE_RESULT: u32 = 0x04050607;
const U64_BE_RESULT: u64 = 0x08090A0B_0C0D0E0F;
const U128_BE_RESULT: u128 = 0x10111213_14151617_18191A1B_1C1D1E1F;

const I8_RESULT: i8 = 0x01;
const I16_BE_RESULT: i16 = 0x0203;
const I32_BE_RESULT: i32 = 0x04050607;
const I64_BE_RESULT: i64 = 0x08090A0B_0C0D0E0F;
const I128_BE_RESULT: i128 = 0x10111213_14151617_18191A1B_1C1D1E1F;

#[test]
fn byteval_val() {
    let mut buf = vec![U8_RESULT];

    let (b, _): (ByteView<u8>, &mut [u8]) = ByteView::mut_view(&mut buf);
    assert_eq!(b.val(), U8_RESULT);

    let (b, _): (ByteView<i8>, &mut [u8]) = ByteView::mut_view(&mut buf);
    assert_eq!(b.val(), I8_RESULT);
}

#[test]
fn byteval_set() {
    let mut buf = vec![0];

    let (mut b, _): (ByteView<u8>, &mut [u8]) = ByteView::mut_view(&mut buf);
    b.set(U8_RESULT);
    assert_eq!(buf[0], U8_RESULT);

    let (mut b, _): (ByteView<i8>, &mut [u8]) = ByteView::mut_view(&mut buf);
    b.set(I8_RESULT);
    assert_eq!(buf[0], I8_RESULT as u8);
}

#[test]
fn mulbyteval_val() {
    const MULBYTEVAL_TESTDATA: [u8; 30] = [
        2, 3, 4, 5, 6, 7, 8, 9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
        0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
    ];

    let mut buf = MULBYTEVAL_TESTDATA.to_vec();
    let (t, _) = LitEndUnsignTest::mut_view(&mut buf);

    assert_eq_hex!(t.unsigned_16.val(), U16_BE_RESULT.swap_bytes());
    assert_eq_hex!(t.unsigned_32.val(), U32_BE_RESULT.swap_bytes());
    assert_eq_hex!(t.unsigned_64.val(), U64_BE_RESULT.swap_bytes());
    assert_eq_hex!(t.unsigned_128.val(), U128_BE_RESULT.swap_bytes());

    let mut buf = MULBYTEVAL_TESTDATA.to_vec();
    let (t, _) = BigEndUnsignTest::mut_view(&mut buf);

    assert_eq_hex!(t.unsigned_16.val(), U16_BE_RESULT);
    assert_eq_hex!(t.unsigned_32.val(), U32_BE_RESULT);
    assert_eq_hex!(t.unsigned_64.val(), U64_BE_RESULT);
    assert_eq_hex!(t.unsigned_128.val(), U128_BE_RESULT);

    let mut buf = MULBYTEVAL_TESTDATA.to_vec();
    let (t, _) = LitEndSignTest::mut_view(&mut buf);

    assert_eq_hex!(t.signed_16.val(), I16_BE_RESULT.swap_bytes());
    assert_eq_hex!(t.signed_32.val(), I32_BE_RESULT.swap_bytes());
    assert_eq_hex!(t.signed_64.val(), I64_BE_RESULT.swap_bytes());
    assert_eq_hex!(t.signed_128.val(), I128_BE_RESULT.swap_bytes());

    let mut buf = MULBYTEVAL_TESTDATA.to_vec();
    let (t, _) = BigEndSignTest::mut_view(&mut buf);

    assert_eq_hex!(t.signed_16.val(), I16_BE_RESULT);
    assert_eq_hex!(t.signed_32.val(), I32_BE_RESULT);
    assert_eq_hex!(t.signed_64.val(), I64_BE_RESULT);
    assert_eq_hex!(t.signed_128.val(), I128_BE_RESULT);
}

#[test]
fn mulbyteval_set() {
    let mut buf = vec![0; 30];
    let (mut t, _) = LitEndUnsignTest::mut_view(&mut buf);

    t.unsigned_16.set(U16_BE_RESULT);
    t.unsigned_32.set(U32_BE_RESULT);
    t.unsigned_64.set(U64_BE_RESULT);
    t.unsigned_128.set(U128_BE_RESULT);

    assert_eq_hex!(buf[0..2], U16_BE_RESULT.to_le_bytes());
    assert_eq_hex!(buf[2..6], U32_BE_RESULT.to_le_bytes());
    assert_eq_hex!(buf[6..14], U64_BE_RESULT.to_le_bytes());
    assert_eq_hex!(buf[14..30], U128_BE_RESULT.to_le_bytes());

    let mut buf = vec![0; 30];
    let (mut t, _) = BigEndUnsignTest::mut_view(&mut buf);

    t.unsigned_16.set(U16_BE_RESULT);
    t.unsigned_32.set(U32_BE_RESULT);
    t.unsigned_64.set(U64_BE_RESULT);
    t.unsigned_128.set(U128_BE_RESULT);

    assert_eq_hex!(buf[0..2], U16_BE_RESULT.to_be_bytes());
    assert_eq_hex!(buf[2..6], U32_BE_RESULT.to_be_bytes());
    assert_eq_hex!(buf[6..14], U64_BE_RESULT.to_be_bytes());
    assert_eq_hex!(buf[14..30], U128_BE_RESULT.to_be_bytes());

    let mut buf = vec![0; 30];
    let (mut t, _) = LitEndSignTest::mut_view(&mut buf);

    t.signed_16.set(I16_BE_RESULT);
    t.signed_32.set(I32_BE_RESULT);
    t.signed_64.set(I64_BE_RESULT);
    t.signed_128.set(I128_BE_RESULT);

    assert_eq_hex!(buf[0..2], I16_BE_RESULT.to_le_bytes());
    assert_eq_hex!(buf[2..6], I32_BE_RESULT.to_le_bytes());
    assert_eq_hex!(buf[6..14], I64_BE_RESULT.to_le_bytes());
    assert_eq_hex!(buf[14..30], I128_BE_RESULT.to_le_bytes());
}

#[test]
fn arrayval_deref() {
    let arr = [0x1, 0x2, 0x3, 0x4];
    let mut buf = arr.clone();
    let (t, _): (ArrayView<[u8; 4]>, _) = ArrayView::mut_view(&mut buf);

    assert_eq_hex!(*t.as_ref(), arr);
}

#[test]
fn arrayval_deref_mut() {
    let mut buf = [0];
    let (t, _): (ArrayView<[u8; 1]>, _) = ArrayView::mut_view(&mut buf);

    t.as_mut_ref()[0] = 0xA;

    assert_eq_hex!(*t.as_ref(), [0xA]);
}

#[test]
fn arrayval_set() {
    let mut buf = vec![0; 4];
    let (mut t, _): (ArrayView<[u8; 4]>, &mut [u8]) = ArrayView::mut_view(&mut buf);

    let new_data = [0x4, 0x3, 0x2, 0x1];
    t.set(&new_data);
    assert_eq_hex!(buf[0..4], new_data);
}

//TODO: Test all cases
#[macro_export]
macro_rules! impl_mulbyteval_assign_test {
    ($fname:ident, $oper:tt, $result:tt) => {
        #[test]
        fn $fname() {
            let mut buf = vec![2; 30];

            let (mut t, _) = LitEndUnsignTest::mut_view(&mut buf);

            t.unsigned_16 $oper 2;
            t.unsigned_32 $oper 2;
            t.unsigned_64 $oper 2;

            assert_eq_hex!(buf[0..2], $result[0..2]);
            assert_eq_hex!(buf[2..6], $result[2..6]);
            assert_eq_hex!(buf[6..14], $result[6..14]);
        }
    };
}

impl_mulbyteval_assign_test!(mulbyte_val_addassign, +=, [4, 2, 4, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2]);
impl_mulbyteval_assign_test!(mulbyte_val_subassign, -=, [0, 2, 0, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2]);
impl_mulbyteval_assign_test!(mulbyte_val_mulassign, *=, [4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4]);
impl_mulbyteval_assign_test!(mulbyte_val_divassign, /=, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
