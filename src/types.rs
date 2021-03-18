use crate::fmt_err;
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{Read, Write};
use std::ops::AddAssign;
use std::ops::{Deref, DerefMut};
pub trait GenValMod<T> {
    fn read<R: Read + Seek>(reader: &mut R) -> Result<T, String>;
    fn write<W: Write + Seek>(writer: &mut W, val: &T) -> Result<(), String>;
    fn add_val(&mut self, val: T);
}

pub struct GenVal<T> {
    val: T,
    offset: u64,
}

impl<T> GenVal<T>
where
    T: std::fmt::Debug,
    GenVal<T>: GenValMod<T>,
{
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, String> {
        let offset = reader.stream_position().map_err(|e| fmt_err!("{}", e))?;

        Ok(Self {
            val: Self::read(reader)?,
            offset,
        })
    }

    pub fn val(&self) -> &T {
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

    pub fn set<W: Write + Seek>(&mut self, writer: &mut W, val: T) -> Result<(), String> {
        self.val = val;
        self.seek_write(writer)
    }

    pub fn add<W: Write + Seek>(&mut self, writer: &mut W, val: T) -> Result<(), String> {
        self.add_val(val);
        self.seek_write(writer)
    }
}

impl<T, const L: usize> GenValMod<[u8; L]> for GenVal<T>
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

    fn add_val(&mut self, val: [u8; L]) {
        todo!()
    }
}

impl GenValMod<u16> for GenVal<u16> {
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

impl GenValMod<u32> for GenVal<u32> {
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