//! `zordon` types
//!
//! This module contains all of the custom types implemented by `zordon`.
//! These types come in the form of three
//! The new function for [`ByteView`], [`MulByteView`] and [`ArrayView`] take a
//! `&'a mut [u8]` as input and return

#[allow(unused_imports)]
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::rc::Rc;

pub trait ModByteView<'a, T> {
    /// Return a copy of the underlying value T
    fn val(&self) -> T;
    /// Set the underlying value to T
    fn set(&mut self, v: T);
}

pub trait ModMulByteView<'a, T, E> {
    /// Return a copy of the underlying value T
    fn val(&self) -> T;
    /// Set the underlying value to T
    fn set(&mut self, v: T);
}

/// Creates a little endian view when used as the E in the [`MulByteView`] data structure.
pub struct LitEnd;
/// Creates a big endian view when used as the E in the [`MulByteView`] data structure.
pub struct BigEnd;

#[derive(Debug, PartialEq)]
pub struct ByteView<'a, T> {
    val: &'a mut [u8],
    _type: std::marker::PhantomData<T>,
}

impl<'a, T> ByteView<'a, T> {
    /// Creates a tuple with a new [`ByteView`] type and the
    pub fn mut_view(arr: &'a mut [u8]) -> (Self, &'a mut [u8]) {
        let (val, leftover) = arr.split_at_mut(std::mem::size_of::<T>());

        (
            Self {
                val,
                _type: std::marker::PhantomData::<T>,
            },
            leftover,
        )
    }
}

#[macro_export]
macro_rules! impl_modbyteval {
    ($target:tt, $bytesized_type:tt) => {
        impl<'a> ModByteView<'a, $bytesized_type> for $target<'a, $bytesized_type> {
            fn val(&self) -> $bytesized_type {
                self.val[0] as $bytesized_type
            }

            fn set(&mut self, v: $bytesized_type) {
                self.val[0] = v as u8
            }
        }
    };
}

impl_modbyteval!(ByteView, u8);
impl_modbyteval!(ByteView, i8);

#[derive(Debug, PartialEq)]
pub struct MulByteView<'a, T, E> {
    val: &'a mut [u8],
    _type: std::marker::PhantomData<T>,
    _endian: std::marker::PhantomData<E>,
}

impl<'a, T, E> MulByteView<'a, T, E> {
    pub fn mut_view(arr: &'a mut [u8]) -> (Self, &'a mut [u8]) {
        let (val, leftover) = arr.split_at_mut(std::mem::size_of::<T>());

        (
            Self {
                val,
                _type: std::marker::PhantomData::<T>,
                _endian: std::marker::PhantomData::<E>,
            },
            leftover,
        )
    }
}

#[macro_export]
macro_rules! impl_modmulbyteval {
    ($target:tt, $type:tt, $endian:tt, $endianident:ident, $read:ident, $write:ident) => {
        impl<'a> ModMulByteView<'a, $type, $endian> for $target<'a, $type, $endian> {
            fn val(&self) -> $type {
                $endianident::$read(self.val)
            }

            fn set(&mut self, v: $type) {
                $endianident::$write(self.val, v)
            }
        }
    };
}

impl_modmulbyteval!(MulByteView, u16, LitEnd, LittleEndian, read_u16, write_u16);
impl_modmulbyteval!(MulByteView, u16, BigEnd, BigEndian, read_u16, write_u16);
impl_modmulbyteval!(MulByteView, u32, LitEnd, LittleEndian, read_u32, write_u32);
impl_modmulbyteval!(MulByteView, u32, BigEnd, BigEndian, read_u32, write_u32);
impl_modmulbyteval!(MulByteView, u64, LitEnd, LittleEndian, read_u64, write_u64);
impl_modmulbyteval!(MulByteView, u64, BigEnd, BigEndian, read_u64, write_u64);
impl_modmulbyteval!(
    MulByteView,
    u128,
    LitEnd,
    LittleEndian,
    read_u128,
    write_u128
);
impl_modmulbyteval!(MulByteView, u128, BigEnd, BigEndian, read_u128, write_u128);

impl_modmulbyteval!(MulByteView, i16, LitEnd, LittleEndian, read_i16, write_i16);
impl_modmulbyteval!(MulByteView, i16, BigEnd, BigEndian, read_i16, write_i16);
impl_modmulbyteval!(MulByteView, i32, LitEnd, LittleEndian, read_i32, write_i32);
impl_modmulbyteval!(MulByteView, i32, BigEnd, BigEndian, read_i32, write_i32);
impl_modmulbyteval!(MulByteView, i64, LitEnd, LittleEndian, read_i64, write_i64);
impl_modmulbyteval!(MulByteView, i64, BigEnd, BigEndian, read_i64, write_i64);
impl_modmulbyteval!(
    MulByteView,
    i128,
    LitEnd,
    LittleEndian,
    read_i128,
    write_i128
);
impl_modmulbyteval!(MulByteView, i128, BigEnd, BigEndian, read_i128, write_i128);

#[macro_export]
macro_rules! impl_oper_assign_overload {
    ($oper_name:ident, $bound:ident, $fname:ident, $oper:tt, $type:tt, $endian:tt) => {
        impl<'a, $type, $endian> $oper_name<$type> for MulByteView<'a, $type, $endian>
        where
            MulByteView<'a, $type, $endian>: ModMulByteView<'a, $type, $endian>,
            $type: $bound + $bound<Output = $type>,
        {
            fn $fname(&mut self, rhs: $type) {
                self.set(self.val() $oper rhs)
            }
        }

        impl<'a, $type> $oper_name<$type> for ByteView<'a, $type>
        where
            ByteView<'a, $type>: ModByteView<'a, $type>,
            $type: $bound + $bound<Output = $type>,
        {
            fn $fname(&mut self, rhs: $type) {
                self.set(self.val() $oper rhs)
            }
        }
    };
}

impl_oper_assign_overload!(AddAssign, Add, add_assign, +, T, E);
impl_oper_assign_overload!(SubAssign, Sub, sub_assign, -, T, E);
impl_oper_assign_overload!(MulAssign, Mul, mul_assign, *, T, E);
impl_oper_assign_overload!(DivAssign, Div, div_assign, /, T, E);

#[derive(Debug, PartialEq)]
pub struct ArrayView<'a, T> {
    buf: Rc<RefCell<&'a mut [u8]>>,
    _type: std::marker::PhantomData<T>,
}

impl<'a, T> ArrayView<'a, T> {
    pub fn mut_view(arr: &'a mut [u8]) -> (Self, &'a mut [u8]) {
        let (val, leftover) = arr.split_at_mut(std::mem::size_of::<T>());

        (
            Self {
                buf: Rc::new(RefCell::new(val)),
                _type: std::marker::PhantomData::<T>,
            },
            leftover,
        )
    }
}

impl<'a, const L: usize> ArrayView<'a, [u8; L]> {
    /// Get a mutable reference to the array
    pub fn as_mut_ref(&self) -> RefMut<&'a mut [u8]> {
        self.buf.borrow_mut()
    }

    /// Get a reference to the array
    pub fn as_ref(&self) -> Ref<&'a mut [u8]> {
        self.buf.borrow()
    }

    /// Return a reference counted pointer
    pub fn rc_clone(&self) -> Rc<RefCell<&'a mut [u8]>> {
        self.buf.clone()
    }

    /// Copy bytes from &[u8] source to the array
    pub fn set(&mut self, src: &[u8]) {
        let mut dst = self.buf.borrow_mut();

        for i in 0..L {
            dst[i] = src[i]
        }
    }
}
