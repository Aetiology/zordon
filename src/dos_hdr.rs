use crate::fmt_err;
use crate::types::*;
use std::io::prelude::*;
use std::io::{Read, Write};

const RESERVED_0_SIZE: usize = 4;
const RESERVED_1_SIZE: usize = 10;

pub struct DosHeader {
    pub mz_sig: PrimVal<u16>,
    pub used_bytes_in_last_page: PrimVal<u16>,
    pub file_size_in_pages: PrimVal<u16>,
    pub num_of_reloc_items: PrimVal<u16>,
    pub header_size_in_paragraphs: PrimVal<u16>,
    pub min_extra_paragraphs: PrimVal<u16>,
    pub max_extra_paragraphs: PrimVal<u16>,
    pub initial_relative_ss_: PrimVal<u16>,
    pub initial_sp: PrimVal<u16>,
    pub checksum: PrimVal<u16>,
    pub initial_ip: PrimVal<u16>,
    pub initial_relative_cs: PrimVal<u16>,
    pub addr_of_reloc_table: PrimVal<u16>,
    pub overlay_number: PrimVal<u16>,
    pub reserved_0: ArrayVal<{ RESERVED_0_SIZE }>,
    pub oem_id: PrimVal<u16>,
    pub oem_info: PrimVal<u16>,
    pub reserved_1: ArrayVal<{ RESERVED_1_SIZE }>,
    pub addr_of_new_exe_hdr: PrimVal<u32>,
}

impl DosHeader {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, String> {
        Ok(Self {
            mz_sig: PrimVal::new(reader)?,
            used_bytes_in_last_page: PrimVal::new(reader)?,
            file_size_in_pages: PrimVal::new(reader)?,
            num_of_reloc_items: PrimVal::new(reader)?,
            header_size_in_paragraphs: PrimVal::new(reader)?,
            min_extra_paragraphs: PrimVal::new(reader)?,
            max_extra_paragraphs: PrimVal::new(reader)?,
            initial_relative_ss_: PrimVal::new(reader)?,
            initial_sp: PrimVal::new(reader)?,
            checksum: PrimVal::new(reader)?,
            initial_ip: PrimVal::new(reader)?,
            initial_relative_cs: PrimVal::new(reader)?,
            addr_of_reloc_table: PrimVal::new(reader)?,
            overlay_number: PrimVal::new(reader)?,
            reserved_0: ArrayVal::new(reader)?,
            oem_id: PrimVal::new(reader)?,
            oem_info: PrimVal::new(reader)?,
            reserved_1: ArrayVal::new(reader)?,
            addr_of_new_exe_hdr: PrimVal::new(reader)?,
        })
    }
}
