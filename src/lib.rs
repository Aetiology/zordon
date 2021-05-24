#![warn(missing_docs, missing_doc_code_examples)]
#![doc(test(no_crate_inject))]

//! # Zordon
//!
//! ![zordan_image](https://upload.wikimedia.org/wikipedia/en/b/bc/Zordon_power_rangers.jpg)
//!
//! `zordon` provides simple abstractions for zero-copy parsing and mutation.
//!
//! 
pub mod types;
mod tests;
pub use mut_view::MutViewNew;
