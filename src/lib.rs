#![warn(missing_docs, missing_doc_code_examples)]
// #![doc(test(no_crate_inject))]
#![allow(dead_code)]
//! # Zordon
//!
//! ![zordan_image](https://upload.wikimedia.org/wikipedia/en/b/bc/Zordon_power_rangers.jpg)
//!
//! `zordon` provides simple low-level abstractions for zero-copy parsing and mutation.
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
//! ### Whats going on?
//! [`ByteView`]: types::ByteView
//! [`MulByteView`]: types::MulByteView
//! [`ArrayView`]: types::ArrayView
//!
//! #### The struct
//! ```
//! # use mut_view::MutView;
//! # use zordon::types::{ByteView, MulByteView, ArrayView, BigEnd, ModByteView, ModMulByteView};
//!
//!  #[derive(MutView)]
//!  struct Example<'a> {
//!     u8_f: ByteView<'a, u8>,   
//!     u16_f: MulByteView<'a, u16, BigEnd>,   
//!     arr_f: ArrayView<'a, [u8; 3]>,
//!  }
//! ```
//! The derive macro [`MutView`] implements a `new` method for the `Example` struct.
//!
//! The u8_f and u16_f fields are given the types [`ByteView`] and [`MulByteView`] respectively.
//!
//! - `ByteView<'a, u8>` specifies that the underlying data is a single byte value of type [`u8`]
//! - `MulByteView<'a, u16, LitEnd>` specifies that the underlying data is a little endian two byte
//! value of type [`u16`].
//!     - `LitEnd` can be swapped with `BigEnd` and the data will be treated as big endian.
//!     - `u16` can be swapped with u32-u128 or i16-i128 -- depending on how you wish to interprit the data.
//!
//! #### Instanciating the struct
//! ```
//! # use mut_view::MutView;
//! # use zordon::types::{ByteView, MulByteView, ArrayView, BigEnd, ModByteView, ModMulByteView};
//!
//! # #[derive(MutView)]
//! # struct Example<'a> {
//! #    u8_f: ByteView<'a, u8>,   
//! #    u16_f: MulByteView<'a, u16, BigEnd>,   
//! #    arr_f: ArrayView<'a, [u8; 3]>,
//! # }
//!
//! # fn main() {
//!   let mut input_buf = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
//!   let (mut example, _) = Example::mut_view(&mut input_buf);
//! # }
//! ```
//! The derived `new` method is called with a mutable reference to `input_buf`.
//!
//!
//! The `new` method slices the the input buffer based on the width of the type specfied in the type defintition.
//! In the case of example:
//! -
//!
//!
//!
//!
//! ## Features
//!
//! - Zero-copy -- Original buffer is split into mutable slices
//! - `zordon` can parse: `[u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, [u8; _]]`.
//! - `u16-u128` and `i16-i128` types can be parsed in little endian or big endian formats
//! - Auto implementation of `new` for struct via the [`MutView::MutView`] derive macro.
//!
//! ## Limitations
//!
//!
//!

mod tests;
pub mod types;
pub use mut_view::MutView;
