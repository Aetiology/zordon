use crate::fmt_err;
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{Read, Write};

pub struct ArrayVal<const L: usize> {
    val: [u8; L],
    offset: u64,
}

impl<const L: usize> ArrayVal<L>
where
    [u8; L]: Default,
{
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, String> {
        let mut buf: [u8; L] = Default::default();
        let offset = reader.stream_position().map_err(|e| fmt_err!("{}", e))?;

        reader
            .read_exact(&mut buf)
            .map_err(|e| fmt_err!("Could not read bytes into buff: {}", e))?;

        Ok(Self {
            val: buf,
            offset,
        })
    }
}

pub struct PrimVal<T>
where
    T: Copy,
{
    val: T,
    offset: u64,
}

impl<T> PrimVal<T>
where
    T: std::fmt::Display + Copy + std::ops::AddAssign,
    PrimVal<T>: PrimValMod<T>,
{
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, String> {
        Ok(Self {
            val: Self::read(reader)?,
            offset: reader.stream_position().map_err(|e| fmt_err!("{}", e))?,
        })
    }

    pub fn val(&self) -> T {
        self.val
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    fn seek_to_val<S: Seek>(&mut self, seeker: &mut S) -> Result<u64, String> {
        seeker.seek(SeekFrom::Start(self.offset)).map_err(|e| {
            fmt_err!(
                "Failed to seek to offset: {} for val: {} - {}",
                self.offset,
                self.val,
                e
            )
        })
    }

    fn seek_write<W: Write + Seek>(&mut self, writer: &mut W) -> Result<(), String> {
        self.seek_to_val(writer)?;
        Self::write(writer, self.val)
    }

    pub fn set<W: Write + Seek>(&mut self, writer: &mut W, val: T) -> Result<(), String> {
        self.val = val;
        self.seek_write(writer)
    }

    pub fn add<W: Write + Seek>(&mut self, writer: &mut W, val: T) -> Result<(), String> {
        self.val += val;
        self.seek_write(writer)
    }
}

pub trait PrimValMod<T>
where
    T: Copy,
{
    fn read<R: Read + Seek>(reader: &mut R) -> Result<T, String>;
    fn write<W: Write + Seek>(writer: &mut W, val: T) -> Result<(), String>;
}

impl PrimValMod<u16> for PrimVal<u16> {
    fn write<W: Write + Seek>(writer: &mut W, val: u16) -> Result<(), String> {
        writer.write_u16::<LittleEndian>(val).map_err(|e| {
            fmt_err!(
                "Failed to write u16 val at: {:#?} - {}",
                writer.stream_position(),
                e
            )
        })?;

        Ok(())
    }

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
}

impl PrimValMod<u32> for PrimVal<u32> {
    fn write<W: Write + Seek>(writer: &mut W, val: u32) -> Result<(), String> {
        writer.write_u32::<LittleEndian>(val).map_err(|e| {
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
