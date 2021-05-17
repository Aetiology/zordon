use crate::fmt_err;
use crate::types::*;
use derive_header::GenValNew;
use std::io::prelude::*;
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy)]
pub enum RelocationType {
    // For now skip/ignore the other types
    ImageRelBasedAbsolute,
    ImageRelBasedHigh,
    ImageRelBasedLow,
    ImageRelBasedHighLow,
    ImageRelBasedHighAdj,
    ImageRelBasedMipsJmpAddr,
}

impl RelocationType {
    pub fn new(reloc_type: u8) -> Self {
        match reloc_type {
            0 => Self::ImageRelBasedAbsolute,
            1 => Self::ImageRelBasedHigh,
            2 => Self::ImageRelBasedLow,
            3 => Self::ImageRelBasedHighLow,
            4 => Self::ImageRelBasedHighAdj,
            5 => Self::ImageRelBasedMipsJmpAddr,
            _ => unimplemented!("reloc_type: {}", reloc_type),
        }
    }
}

pub struct Relocations {
    pub virt_addr: GenVal<u32>,
    pub size_of_block: GenVal<u32>,
    pub block: Vec<GenVal<u16>>,
}

impl Relocations {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, String> {
        let virt_addr = GenVal::new(reader)?;
        let size_of_block: GenVal<u32> = GenVal::new(reader)?;

        let entry_count = ((*size_of_block.get_ref() - 8) / 2) as usize;
        let mut block: Vec<GenVal<u16>> = Vec::with_capacity(entry_count);

        for _ in 0..entry_count {
            block.push(GenVal::new(reader)?);
        }

        Ok(Self {
            virt_addr,
            size_of_block,
            block,
        })
    }

    pub fn to_type(type_offset_pair: u16) -> RelocationType {
        RelocationType::new(((type_offset_pair & 0xF000) >> 12) as u8)
    }

    pub fn to_offset(type_offset_pair: u16) -> RelocationType {
        RelocationType::new((type_offset_pair & 0xFFF) as u8)
    }
}

const RELOC_TESTDATA: [u8; 0x10] = [
    0, 0x10, 0, 0, 0x0C, 0, 0, 0, 0x17, 0x30, 0x1F, 0x30, 0, 0, 0, 0,
];

/*
#[test]
fn relocations_new() {
    let reloc_data = RELOC_TESTDATA.to_vec();
    let mut buf = std::io::Cursor::new(reloc_data);

    let relocs = Relocations::new()
}
*/
