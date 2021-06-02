#![warn(missing_docs, missing_doc_code_examples)]
// #![doc(test(no_crate_inject))]
#![allow(dead_code)]
//! # Zordon
//!
//! ![zordan_image](https://upload.wikimedia.org/wikipedia/en/b/bc/Zordon_power_rangers.jpg)
//!
//! `zordon` provides simple low-level abstractions for zero-copy parsing and mutation.
//!
//! 
//!
//!
//! ## Simple example
//! ```
//! use mut_view::MutView;
//! use zordon::types::{ByteView, MulByteView, ArrayView, BigEnd, ModByteView, ModMulByteView};
//!
//! #[derive(MutView)]
//! struct Example<'a> {
//!     u8_f: ByteView<'a, u8>,   
//!     u16_f: MulByteView<'a, u16, BigEnd>,   
//!     arr_f: ArrayView<'a, [u8; 3]>,
//! }
//!
//! fn main() {
//!   let mut input_buf = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
//!   let (mut example, _) = Example::mut_view(&mut input_buf);
//!
//!   assert_eq!(example.u8_f.val(), 0x00);    
//!   assert_eq!(example.u16_f.val(), 0x0102);    
//!   assert_eq!(*example.arr_f.as_ref(), [0x03, 0x04, 0x05]);    
//! }
//! ```
//! ### How it works
//! [`ByteView`]: types::ByteView
//! [`MulByteView`]: types::MulByteView
//! [`ArrayView`]: types::ArrayView
//!
//! #### Deriving `mut_view`
//! ```
//! # use mut_view::MutView;
//! # use zordon::types::{ByteView, MulByteView, ArrayView, BigEnd, ModByteView, ModMulByteView};
//!  #[derive(MutView)]
//!  struct Example<'a> {
//!     u8_f: ByteView<'a, u8>,   
//!     u16_f: MulByteView<'a, u16, BigEnd>,   
//!     arr_f: ArrayView<'a, [u8; 3]>,
//!  }
//! ```
//! The derive macro [`MutView`] implements a `mut_view` method for the `Example` struct.
//!
//! `mut_view` is allways called with a `&mut [u8]` as input, which it slices into different
//! length for each field, based on the type.
//!
//! - `ByteView<'a, u8>` specifies that the underlying data is a single byte value of type [`u8`]
//! - `MulByteView<'a, u16, LitEnd>` specifies that the underlying data is a little endian two byte
//! value of type [`u16`].
//!     - `LitEnd` can be swapped with `BigEnd` and the data will be treated as big endian.
//!     - `u16` can be swapped with u32-u128 or i16-i128
//! - `ArrayView<'a, [u8; 3]>` specifies that the underlying data is three byte value of type [`u8; 3`]
//!
//! #### Instantiating the struct
//! ```
//! # use mut_view::MutView;
//! # use zordon::types::{ByteView, MulByteView, ArrayView, BigEnd, ModByteView, ModMulByteView};
//! # #[derive(MutView)]
//! # struct Example<'a> {
//! #    u8_f: ByteView<'a, u8>,   
//! #    u16_f: MulByteView<'a, u16, BigEnd>,   
//! #    arr_f: ArrayView<'a, [u8; 3]>,
//! # }
//! # fn main() {
//! let mut input_buf = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
//! let (mut example, _) = Example::mut_view(&mut input_buf);
//! # }
//! ```
//! The derived `mut_view` method is called with a mutable reference to `input_buf` and
//! slices it for each fields based on it's type.
//!
//! In this case it will:
//! - First, split `input_buf` at an index of 1 (since the length of [`u8`] is one) - `u8_f` now owns this slice
//! - Second, split the remaining slice at an index of 2 (since the length of [`u16`] is two) - `u16_f` now owns this slice
//! - Finally,  it split the remaining slice at an index of 3 (since the length of [`u8; 3`] is three) - `arr_f` now owns this slice
//!
//! The return value of `mut_view` is always (Self, &'mut [u8]) where `Self` is the calling type and `&'mut [u8]`
//! is the remaining slice.
//!
//! ## Composite example
//! ```
//! use mut_view::MutView;
//! use zordon::types::{ByteView, MulByteView, ArrayView, BigEnd, ModByteView, ModMulByteView};
//!
//! #[derive(MutView)]
//! struct ExampleA<'a> {
//!     u8_f: ByteView<'a, u8>,   
//! }
//!
//! #[derive(MutView)]
//! struct ExampleB<'a> {
//!     u16_f: ByteView<'a, u16, BigEndian>,   
//! }
//!
//! #[derive(MutView)]
//! struct Composite<'a> {
//!     example_a: Example<'a>,  
//!     example_b: Example<'b>,   
//!     option_c: Option<u8>, 
//! }
//!
//! fn main() {
//!   let mut input_buf = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
//!   let (mut example, _) = Example::mut_view(&mut input_buf);
//!
//!   assert_eq!(example.u8_f.val(), 0x00);    
//!   assert_eq!(example.u16_f.val(), 0x0102);    
//!   assert_eq!(*example.arr_f.as_ref(), [0x03, 0x04, 0x05]);    
//! }
//! ``` 
//!
//!
//!
//! ## Features
//!
//! - Zero-copy -- Original buffer is split into mutable slices
//! - Able to parse the following types: `[u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, [u8; _]]`
//! - The types u16..u128 and i16..i128 can be treated as little endian or big endian
//! - Auto implementation of `mut_view` for structs via the [`MutView`] derive macro.
//!
//! ## Limitations
//!
//!
//!

mod tests;
pub mod types;
pub use mut_view::MutView;
