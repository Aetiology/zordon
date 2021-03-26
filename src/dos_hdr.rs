use crate::fmt_err;
use crate::types::*;
use std::io::prelude::*;
use std::io::{Read, Write};
use derive_header::HeaderNew;

const RESERVED_0_SIZE: usize = 4;
const RESERVED_1_SIZE: usize = 10;

#[derive(HeaderNew)]
pub struct DosHeader {
    pub mz_sig: GenVal<u16>,
    pub used_bytes_in_last_page: GenVal<u16>,
    pub file_size_in_pages: GenVal<u16>,
    pub num_of_reloc_items: GenVal<u16>,
    pub header_size_in_paragraphs: GenVal<u16>,
    pub min_extra_paragraphs: GenVal<u16>,
    pub max_extra_paragraphs: GenVal<u16>,
    pub initial_relative_ss_: GenVal<u16>,
    pub initial_sp: GenVal<u16>,
    pub checksum: GenVal<u16>,
    pub initial_ip: GenVal<u16>,
    pub initial_relative_cs: GenVal<u16>,
    pub addr_of_reloc_table: GenVal<u16>,
    pub overlay_number: GenVal<u16>,
    pub reserved_0: GenVal<[u8; RESERVED_0_SIZE]>,
    pub oem_id: GenVal<u16>,
    pub oem_info: GenVal<u16>,
    pub reserved_1: GenVal<[u8; RESERVED_1_SIZE]>,
    pub addr_of_new_exe_hdr: GenVal<u32>,
}

/*
impl DosHeader {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, String> {
        Ok(Self {
            mz_sig: GenVal::new(reader)?,
            used_bytes_in_last_page: GenVal::new(reader)?,
            file_size_in_pages: GenVal::new(reader)?,
            num_of_reloc_items: GenVal::new(reader)?,
            header_size_in_paragraphs: GenVal::new(reader)?,
            min_extra_paragraphs: GenVal::new(reader)?,
            max_extra_paragraphs: GenVal::new(reader)?,
            initial_relative_ss_: GenVal::new(reader)?,
            initial_sp: GenVal::new(reader)?,
            checksum: GenVal::new(reader)?,
            initial_ip: GenVal::new(reader)?,
            initial_relative_cs: GenVal::new(reader)?,
            addr_of_reloc_table: GenVal::new(reader)?,
            overlay_number: GenVal::new(reader)?,
            reserved_0: GenVal::new(reader)?,
            oem_id: GenVal::new(reader)?,
            oem_info: GenVal::new(reader)?,
            reserved_1: GenVal::new(reader)?,
            addr_of_new_exe_hdr: GenVal::new(reader)?,
        })
    }
}
*/