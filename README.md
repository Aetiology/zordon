# Zordon

![zordan_image](https://upload.wikimedia.org/wikipedia/en/b/bc/Zordon_power_rangers.jpg)

`zordon` provides simple low-level abstractions for zero-copy parsing and mutation. It is a no_std crate with alloc.

`zordon` types allow a single mutable u8 buffer to be treated as
a series of u8-u128, i8-i128 or [u8; _] values without the need to copy data in the original buffer. Setting, getting and
adding to the values is transparent via the methods implemented on the calling type.

The `[MutView]` derive macro can be used on a data structure whose fields use `zordon` types. Allowing a buffer to be
parsed and manipulated like a typical rust struct.

For usage examples and documentation check out: [docs](https://docs.rs/zordon)