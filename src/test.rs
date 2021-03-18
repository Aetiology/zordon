use crate::dos_hdr::*;
use crate::fmt_err;
use std::fs::File;
#[macro_use]
use assert_hex::assert_eq_hex;

#[cfg(test)]
#[test]
fn dos_hdr() -> Result<(), ()> {
    const TEST_PE: &str = "test_data/basic_test.exe";

    let pe: Vec<u8> = std::fs::read(TEST_PE)
        .map_err(|e| eprintln!("{}", fmt_err!("Could not read file: {} - {}", TEST_PE, e)))?;

    let mut buff = std::io::Cursor::new(pe);

    let dos_hdr = DosHeader::new(&mut buff)
        .map_err(|e| eprintln!("{}", fmt_err!("Could not create DosHeader: {}", e)))?;

    assert_eq_hex!(dos_hdr.mz_sig.val(), 0x5A4D);

    Ok(())
}
