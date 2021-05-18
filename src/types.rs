use crate::fmt_err;
#[allow(unused_imports)]
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use derive_header::GenValNew;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{Read, Write};
use std::ops::Deref;
#[allow(unused_attributes)]
#[macro_use]
#[allow(unused_imports)]
#[macro_use]
use assert_hex::assert_eq_hex;

pub trait ModGenVal<T> {
    fn read<R: Read + Seek>(reader: &mut R) -> Result<T, String>;
    fn write<W: Write + Seek>(writer: &mut W, val: &T) -> Result<(), String>;
    fn add_val(&mut self, val: T);
}

#[derive(Debug, PartialEq)]
pub struct GenVal<T> {
    val: T,
    offset: u64,
}

impl<T> Deref for GenVal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<T> GenVal<T>
where
    T: std::fmt::Debug,
    GenVal<T>: ModGenVal<T>,
{
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, String> {
        let offset = reader.stream_position().map_err(|e| fmt_err!("{}", e))?;

        Ok(Self {
            val: Self::read(reader)?,
            offset,
        })
    }

    pub fn get_ref(&self) -> &T {
        &self.val
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    fn seek_to_val<S: Seek>(&mut self, seeker: &mut S) -> Result<u64, String> {
        seeker.seek(SeekFrom::Start(self.offset)).map_err(|e| {
            fmt_err!(
                "Failed to seek to offset: {} for val: {:#X?} - {}",
                self.offset,
                self.val,
                e
            )
        })
    }

    fn seek_write<W: Write + Seek>(&mut self, writer: &mut W) -> Result<(), String> {
        self.seek_to_val(writer)?;
        Self::write(writer, &self.val)
    }

    /**
    ```
     # #[macro_use]
     # use assert_hex::assert_eq_hex;
     # use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
     # use derive_header::GenValNew;
     # use zordon::types::GenVal;
     # use std::io::{Read, Write, Seek};
    #[derive(GenValNew)]
    struct GenValTest {
        pub unsigned_8: GenVal<u8>,
    }

    let mut buf = std::io::Cursor::new(vec![0x0]);
    let mut genvaltest = GenValTest::new(&mut buf).unwrap();

    assert_eq_hex!(*genvaltest.unsigned_8, 0x0);

    genvaltest
        .unsigned_8
        .set(&mut buf, 0x10)
        .unwrap();

    assert_eq_hex!(*genvaltest.unsigned_8, 0x10);
    */
    pub fn set<W: Write + Seek>(&mut self, writer: &mut W, val: T) -> Result<(), String> {
        self.val = val;
        self.seek_write(writer)
    }
    /**
    ```
     # #[macro_use]
     # use assert_hex::assert_eq_hex;
     # use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
     # use derive_header::GenValNew;
     # use zordon::types::GenVal;
     # use std::io::{Read, Write, Seek};
    #[derive(GenValNew)]
    struct GenValTest {
        pub unsigned_8: GenVal<u8>,
    }

    let mut buf = std::io::Cursor::new(vec![0x10]);
    let mut genvaltest = GenValTest::new(&mut buf).unwrap();

    genvaltest
        .unsigned_8
        .add(&mut buf, 0x10)
        .unwrap();

    assert_eq_hex!(*genvaltest.unsigned_8, 0x20);
    */
    pub fn add<W: Write + Seek>(&mut self, writer: &mut W, val: T) -> Result<(), String> {
        self.add_val(val);
        self.seek_write(writer)
    }
}

impl<T, const L: usize> ModGenVal<[u8; L]> for GenVal<T>
where
    [u8; L]: Default,
{
    fn read<R: Read + Seek>(reader: &mut R) -> Result<[u8; L], String> {
        let mut buf: [u8; L] = Default::default();

        reader
            .read_exact(&mut buf)
            .map_err(|e| fmt_err!("Could not read bytes into buff: {}", e))?;

        Ok(buf)
    }

    fn write<W: Write + Seek>(writer: &mut W, val: &[u8; L]) -> Result<(), String> {
        writer.write_all(val).map_err(|e| {
            fmt_err!(
                "Failed to write bytes at offset: {:#?} - {}",
                writer.stream_position(),
                e
            )
        })?;

        Ok(())
    }

    fn add_val(&mut self, _: [u8; L]) {
        todo!()
    }
}

impl ModGenVal<u8> for GenVal<u8> {
    fn read<R: Read + Seek>(reader: &mut R) -> Result<u8, String> {
        let r = reader.read_u8().map_err(|e| {
            fmt_err!(
                "Failed to read u8 val at: {:#?} - {}",
                reader.stream_position(),
                e
            )
        })?;

        Ok(r)
    }

    fn write<W: Write + Seek>(writer: &mut W, val: &u8) -> Result<(), String> {
        writer.write_u8(*val).map_err(|e| {
            fmt_err!(
                "Failed to write u8 val at: {:#?} - {}",
                writer.stream_position(),
                e
            )
        })?;

        Ok(())
    }

    fn add_val(&mut self, val: u8) {
        self.val += val
    }
}

impl ModGenVal<u16> for GenVal<u16> {
    fn read<R: Read + Seek>(reader: &mut R) -> Result<u16, String> {
        let r = reader.read_u16::<LittleEndian>().map_err(|e| {
            fmt_err!(
                "Failed to read u16 val at: {:#?} - {}",
                reader.stream_position(),
                e
            )
        })?;

        Ok(r)
    }

    fn write<W: Write + Seek>(writer: &mut W, val: &u16) -> Result<(), String> {
        writer.write_u16::<LittleEndian>(*val).map_err(|e| {
            fmt_err!(
                "Failed to write u16 val at: {:#?} - {}",
                writer.stream_position(),
                e
            )
        })?;

        Ok(())
    }

    fn add_val(&mut self, val: u16) {
        self.val += val
    }
}

impl ModGenVal<u32> for GenVal<u32> {
    fn write<W: Write + Seek>(writer: &mut W, val: &u32) -> Result<(), String> {
        writer.write_u32::<LittleEndian>(*val).map_err(|e| {
            fmt_err!(
                "Failed to write u32 val at: {:#?} - {}",
                writer.stream_position(),
                e
            )
        })?;

        Ok(())
    }

    fn read<R: Read + Seek>(reader: &mut R) -> Result<u32, String> {
        let r = reader.read_u32::<LittleEndian>().map_err(|e| {
            fmt_err!(
                "Failed to read u32 val at: {:#?} - {}",
                reader.stream_position(),
                e
            )
        })?;

        Ok(r)
    }

    fn add_val(&mut self, val: u32) {
        self.val += val
    }
}

impl ModGenVal<u64> for GenVal<u64> {
    fn write<W: Write + Seek>(writer: &mut W, val: &u64) -> Result<(), String> {
        writer.write_u64::<LittleEndian>(*val).map_err(|e| {
            fmt_err!(
                "Failed to write u32 val at: {:#?} - {}",
                writer.stream_position(),
                e
            )
        })?;

        Ok(())
    }

    fn read<R: Read + Seek>(reader: &mut R) -> Result<u64, String> {
        let r = reader.read_u64::<LittleEndian>().map_err(|e| {
            fmt_err!(
                "Failed to read u32 val at: {:#?} - {}",
                reader.stream_position(),
                e
            )
        })?;

        Ok(r)
    }

    fn add_val(&mut self, val: u64) {
        self.val += val
    }
}

// ####
#[allow(dead_code)]
#[derive(GenValNew)]
struct GenValTest {
    pub unsigned_8: GenVal<u8>,
    pub unsigned_16: GenVal<u16>,
    pub unsigned_32: GenVal<u32>,
    pub unsigned_64: GenVal<u64>,
    pub unsigned_u8_arr: GenVal<[u8; 4]>,
}

#[allow(dead_code)]
const GENVAL_TESTDATA: [u8; 0x13] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF, 0x10, 0x11, 0x12, 0x13,
];

#[test]
fn genval_val() -> Result<(), ()> {
    let data = GENVAL_TESTDATA.to_vec();
    let mut buf = std::io::Cursor::new(data);

    let genvaltest = GenValTest::new(&mut buf).map_err(|e| eprintln!("{}", e))?;

    assert_eq_hex!(*genvaltest.unsigned_8, 01);
    assert_eq_hex!(*genvaltest.unsigned_16, 0x0302);
    assert_eq_hex!(*genvaltest.unsigned_32, 0x07060504);
    assert_eq_hex!(*genvaltest.unsigned_64, 0x0F0E0D0C0B0A0908);
    assert_eq_hex!(*genvaltest.unsigned_u8_arr, [0x10, 0x11, 0x12, 0x13]);

    Ok(())
}

#[test]
fn genval_set() -> Result<(), ()> {
    let data = GENVAL_TESTDATA.to_vec();
    let mut buf = std::io::Cursor::new(data);

    let mut genvaltest = GenValTest::new(&mut buf).map_err(|e| eprintln!("{}", e))?;

    genvaltest
        .unsigned_8
        .set(&mut buf, 0x13)
        .map_err(|e| eprintln!("{}", e))?;

    genvaltest
        .unsigned_16
        .set(&mut buf, 0x1112)
        .map_err(|e| eprintln!("{}", e))?;

    genvaltest
        .unsigned_32
        .set(&mut buf, 0x0D0E0F10)
        .map_err(|e| eprintln!("{}", e))?;

    genvaltest
        .unsigned_64
        .set(&mut buf, 0x05060708090A0B0C)
        .map_err(|e| eprintln!("{}", e))?;

    genvaltest
        .unsigned_u8_arr
        .set(&mut buf, [04, 03, 02, 01])
        .map_err(|e| eprintln!("{}", e))?;

    let data_ref = buf.get_ref();

    assert_eq_hex!(data_ref[0], 0x13);
    assert_eq_hex!(data_ref[1..3], [0x12, 0x11]);
    assert_eq_hex!(data_ref[3..7], [0x10, 0xF, 0xE, 0xD]);
    assert_eq_hex!(data_ref[7..15], [0xC, 0xB, 0xA, 0x9, 0x8, 0x7, 0x6, 0x5]);
    assert_eq_hex!(data_ref[15..19], [0x4, 0x3, 0x2, 0x1]);

    assert_eq_hex!(*genvaltest.unsigned_8, 0x13);
    assert_eq_hex!(*genvaltest.unsigned_16, 0x1112);
    assert_eq_hex!(*genvaltest.unsigned_32, 0x0D0E0F10);
    assert_eq_hex!(*genvaltest.unsigned_64, 0x05060708090A0B0C);
    assert_eq_hex!(*genvaltest.unsigned_u8_arr, [0x4, 0x3, 0x2, 0x1]);

    Ok(())
}

#[test]
fn genval_add() -> Result<(), ()> {
    let data = GENVAL_TESTDATA.to_vec();
    let mut buf = std::io::Cursor::new(data);

    let mut genvaltest = GenValTest::new(&mut buf).map_err(|e| eprintln!("{}", e))?;
    const VAL_TO_ADD: u8 = 0x10;

    // For the moment, we are not using Add for arrays

    genvaltest
        .unsigned_8
        .add(&mut buf, VAL_TO_ADD)
        .map_err(|e| eprintln!("{}", e))?;

    genvaltest
        .unsigned_16
        .add(&mut buf, VAL_TO_ADD as u16)
        .map_err(|e| eprintln!("{}", e))?;

    genvaltest
        .unsigned_32
        .add(&mut buf, VAL_TO_ADD as u32)
        .map_err(|e| eprintln!("{}", e))?;

    genvaltest
        .unsigned_64
        .add(&mut buf, VAL_TO_ADD as u64)
        .map_err(|e| eprintln!("{}", e))?;

    let data_ref = buf.get_ref();

    assert_eq_hex!(data_ref[0], 0x11);
    assert_eq_hex!(LittleEndian::read_u16(&data_ref[1..3]), 0x0312);
    assert_eq_hex!(LittleEndian::read_u32(&data_ref[3..7]), 0x07060514);
    assert_eq_hex!(LittleEndian::read_u64(&data_ref[7..15]), 0x0F0E0D0C0B0A0918);
    //assert_eq_hex!(data_ref[15..19], [0x4, 0x3, 0x2, 0x1]);

    Ok(())
}
