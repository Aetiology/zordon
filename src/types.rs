use crate::MutViewNew;
#[allow(unused_imports)]
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::rc::Rc;
#[allow(unused_attributes)]
#[macro_use]
#[allow(unused_imports)]
#[macro_use]
use assert_hex::assert_eq_hex;
///###
#[derive(Debug, PartialEq)]
pub struct SimpleVal<'a, T>
where
    Self: ModSimpleVal<'a, T>,
{
    val: &'a mut [u8],
    _marker: std::marker::PhantomData<T>,
}
pub trait ModSimpleVal<'a, T> {
    fn val(&self) -> T;
    fn set(&mut self, v: T);
}

impl<'a, T> SimpleVal<'a, T>
where
    Self: ModSimpleVal<'a, T>,
{
    pub fn new(arr: &'a mut [u8]) -> (Self, &'a mut [u8]) {
        let (val, leftover) = arr.split_at_mut(std::mem::size_of::<T>());

        (
            Self {
                val,
                _marker: std::marker::PhantomData::<T>,
            },
            leftover,
        )
    }
}

#[macro_export]
macro_rules! impl_modsimpleval {
    ($target:tt, $bytesized_type:tt, $read:ident, $write:ident) => {
        impl<'a> ModSimpleVal<'a, $bytesized_type> for SimpleVal<'a, $bytesized_type> {
            fn val(&self) -> $bytesized_type {
                self.val[0] as $bytesized_type
            }

            fn set(&mut self, v: $bytesized_type) {
                self.val[0] = v as u8
            }
        }
    };

    ($target:tt, $type:tt, $read:ident, $write:ident, $endian:ident) => {
        impl<'a> ModSimpleVal<'a, $type> for $target<'a, $type> {
            fn val(&self) -> $type {
                $endian::$read(self.val)
            }

            fn set(&mut self, v: $type) {
                $endian::$write(self.val, v)
            }
        }
    };
}

impl_modsimpleval!(SimpleVal, u8, read_u8, write_u8);
impl_modsimpleval!(SimpleVal, u16, read_u16, write_u16, LittleEndian);
impl_modsimpleval!(SimpleVal, u32, read_u32, write_u32, LittleEndian);
impl_modsimpleval!(SimpleVal, u64, read_u64, write_u64, LittleEndian);
impl_modsimpleval!(SimpleVal, i8, read_i8, write_i8);
impl_modsimpleval!(SimpleVal, i16, read_i16, write_i16, LittleEndian);
impl_modsimpleval!(SimpleVal, i32, read_i32, write_i32, LittleEndian);
impl_modsimpleval!(SimpleVal, i64, read_i64, write_i64, LittleEndian);

#[macro_export]
macro_rules! impl_oper_assign_overload {
    ($oper_name:ident, $bound:ident, $fname:ident, $oper:tt, $gen:tt) => {
        impl<'a, $gen> $oper_name<$gen> for SimpleVal<'a, $gen>
        where
            SimpleVal<'a, $gen>: ModSimpleVal<'a, $gen>,
            $gen: $bound + $bound<Output = $gen>,
        {
            fn $fname(&mut self, rhs: $gen) {
                self.set(self.val() $oper rhs)
            }
        }
    };
}

impl_oper_assign_overload!(AddAssign, Add, add_assign, +, T);
impl_oper_assign_overload!(SubAssign, Sub, sub_assign, -, T);
impl_oper_assign_overload!(MulAssign, Mul, mul_assign, *, T);
impl_oper_assign_overload!(DivAssign, Div, div_assign, /, T);

#[derive(Debug, PartialEq)]
pub struct ArrayVal<'a, T> {
    buf: Rc<RefCell<&'a mut [u8]>>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T> ArrayVal<'a, T> {
    pub fn new(arr: &'a mut [u8]) -> (Self, &'a mut [u8]) {
        let (val, leftover) = arr.split_at_mut(std::mem::size_of::<T>());

        (
            Self {
                buf: Rc::new(RefCell::new(val)),
                _marker: std::marker::PhantomData::<T>,
            },
            leftover,
        )
    }
}

impl<'a, const L: usize> ArrayVal<'a, [u8; L]> {
    pub fn as_mut_ref(&self) -> RefMut<&'a mut [u8]> {
        self.buf.borrow_mut()
    }

    pub fn as_ref(&self) -> Ref<&'a mut [u8]> {
        self.buf.borrow()
    }

    pub fn rc_clone(&self) -> Rc<RefCell<&'a mut [u8]>> {
        self.buf.clone()
    }

    pub fn set(&mut self, src: &[u8]) {
        let mut dst = self.buf.borrow_mut();

        for i in 0..L {
            dst[i] = src[i]
        }
    }
}
// Tests
#[allow(dead_code)]
#[derive(MutViewNew)]
struct SimpleValTest<'a> {
    pub unsigned_8: SimpleVal<'a, u8>,
    pub unsigned_16: SimpleVal<'a, u16>,
    pub unsigned_32: SimpleVal<'a, u32>,
    pub unsigned_64: SimpleVal<'a, u64>,
    pub unsigned_arr: ArrayVal<'a, [u8; 4]>,
}

#[test]
fn simpleval_val() {
    const SIMPLEVAL_TESTDATA: [u8; 0x13] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF, 0x10, 0x11, 0x12, 0x13,
    ];

    let mut buf = SIMPLEVAL_TESTDATA.to_vec();
    let (t, _): (SimpleValTest, _) = SimpleValTest::new(&mut buf);

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
    let (mut t, _) = SimpleValTest::new(&mut buf);

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
            let (mut t, _) = SimpleValTest::new(&mut buf);

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
