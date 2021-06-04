//! `zordon` types
//!
//! Contains all of the custom types implemented by `zordon`.

#[allow(unused_imports)]
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::rc::Rc;

/// For getting/setting single byte values.
pub trait ModByteView<'a, T> {
    /// Return a copy of the underlying value T.
    fn val(&self) -> T;
    /// Set the underlying value to T.
    fn set(&mut self, v: T);
}

/// For getting/setting multi byte values.
pub trait ModMulByteView<'a, T, E> {
    /// Return a copy of the underlying value T.
    fn val(&self) -> T;
    /// Set the underlying value to T.
    fn set(&mut self, v: T);
}

/// Marker type used with [`MulByteView`] as the E in MulByteView<'a, T, E> to specify a little endian view.
pub struct LitEnd;
/// Marker type used with [`MulByteView`] as the E in MulByteView<'a, T, E> to specify a big endian view.
pub struct BigEnd;

/// A mutable byte view for type T where the length of the view is always 1.
///
/// Check [`ModByteView`] implementations for valid T monomorphisms. 
#[derive(Debug, PartialEq)]
pub struct ByteView<'a, T> {
    val: &'a mut [u8],
    _type: std::marker::PhantomData<T>,
}

impl<'a, T> ByteView<'a, T> {
    /// Constructs a new [`ByteView`] and returns the leftover slice.
    ///
    /// # Panics
    ///
    /// Panics if `T.len() > arr.len()`
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

/// Template for implementing ModByteView<'a, _>
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

/// A mutable multi byte view for type T where the length of the view varies depending on T.
///
/// For valid types for T, check [`ModByteView`] implementations.
#[derive(Debug, PartialEq)]
pub struct MulByteView<'a, T, E> {
    val: &'a mut [u8],
    _type: std::marker::PhantomData<T>,
    _endian: std::marker::PhantomData<E>,
}

impl<'a, T, E> MulByteView<'a, T, E> {
    /// Returns a [`MulByteView`] and leftover slice.
    ///
    /// # Panics
    ///
    /// Panics if `T.len() > arr.len()`
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

/// Template for implementing ModMulByteView<'a, _, _>.
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

/// Template for implementing oper assign overloading
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

/// A mutable array view for type [u8; L] where L is a const.
#[derive(Debug, PartialEq)]
pub struct ArrayView<'a, T> {
    buf: Rc<RefCell<&'a mut [u8]>>,
    _type: std::marker::PhantomData<T>,
}

impl<'a, T> ArrayView<'a, T> {
    /// Returns an [`ArrayView`] and leftover slice.
    ///
    /// # Panics
    ///
    /// Panics if `T.len() > arr.len()`
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
    /// Returns a mutable reference to the array.
    pub fn as_mut_ref(&self) -> RefMut<&'a mut [u8]> {
        self.buf.borrow_mut()
    }

    /// Returns a reference to the array.
    pub fn as_ref(&self) -> Ref<&'a mut [u8]> {
        self.buf.borrow()
    }

    /// Retruns a clone of the reference counted pointer.
    pub fn rc_clone(&self) -> Rc<RefCell<&'a mut [u8]>> {
        self.buf.clone()
    }

    /// Copies bytes from a `&[u8]` source to the underlying array.
    pub fn set(&mut self, src: &[u8]) {
        let mut dst = self.buf.borrow_mut();

        for i in 0..L {
            dst[i] = src[i]
        }
    }
}
