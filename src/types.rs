use crate::fmt_err;
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::prelude::*;
use std::io::{Read, Write};

pub struct PrimVal<T>
where
    T: Copy,
{
    val: T,
    offset: u64,
}

impl<T> PrimVal<T>
where
    T: Copy + std::ops::AddAssign,
    PrimVal<T>: PrimValMod<T>,
{
    fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, String> {
        Ok(Self {
            val: Self::read(reader)?,
            offset: reader.stream_position().map_err(|e| fmt_err!("{}", e))?,
        })
    }

    fn set<W: Write + Seek>(&mut self, writer: &mut W, val: T) -> Result<(), String> {
        self.val = val;
        Self::write(self, writer, self.val)
    }

    fn add<W: Write + Seek>(&mut self, writer: &mut W, val: T) -> Result<(), String> {
        self.val += val;
        Self::write(self, writer, val)
    }
}

pub trait PrimValMod<T>
where
    T: Copy,
{
    fn read<R: Read + Seek>(reader: &mut R) -> Result<T, String>;
    fn write<W: Write + Seek>(&mut self, writer: &mut W, val: T) -> Result<(), String>;
}

impl PrimValMod<u16> for PrimVal<u16> {
    fn write<W: Write + Seek>(&mut self, writer: &mut W, val: u16) -> Result<(), String> {
        Ok(writer.write_u16::<LittleEndian>(val).map_err(|e| {
            fmt_err!(
                "Failed to u16 val at: {:#?} - {}",
                writer.stream_position(),
                e
            )
        })?)
    }

    fn read<R: Read + Seek>(reader: &mut R) -> Result<u16, String> {
        Ok(reader.read_u16::<LittleEndian>().map_err(|e| {
            fmt_err!(
                "Failed to u16 val at: {:#?} - {}",
                reader.stream_position(),
                e
            )
        })?)
    }
}

impl PrimValMod<u32> for PrimVal<u32> {
    fn write<W: Write + Seek>(&mut self, writer: &mut W, val: u32) -> Result<(), String> {
        Ok(writer.write_u32::<LittleEndian>(val).map_err(|e| {
            fmt_err!(
                "Failed to u16 val at: {:#?} - {}",
                writer.stream_position(),
                e
            )
        })?)
    }

    fn read<R: Read + Seek>(reader: &mut R) -> Result<u32, String> {
        Ok(reader.read_u32::<LittleEndian>().map_err(|e| {
            fmt_err!(
                "Failed to u32 val at: {:#?} - {}",
                reader.stream_position(),
                e
            )
        })?)
    }
}

/*
impl PrimValMod<u64> for PrimVal<u64> {
    fn read<R: Read + Seek>(reader: &mut R) -> Result<u64, String> {
        Ok(reader.read_u64::<LittleEndian>().map_err(|e| {
            fmt_err!(
                "Failed to u64 val at: {:#?} - {}",
                reader.stream_position(),
                e
            )
        })?)
    }
}
*/