#[allow(unused_imports)]
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct SimpleVal<'a, T> {
    val: &'a mut [u8],
    _marker: std::marker::PhantomData<T>,
}

pub trait ModSimpleVal<'a, T> {
    fn val(&self) -> T;
    fn set(&mut self, v: T);
}

impl<'a, T> SimpleVal<'a, T> {
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
