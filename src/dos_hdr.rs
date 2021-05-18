use crate::types::*;
use derive_header::GenValNew;
use std::io::prelude::*;
use std::io::{Read};

#[derive(GenValNew)]
pub struct DosHeader {
    pub mz_sig: GenVal<u16>,
    pub used_bytes_in_last_page: GenVal<u16>,
    pub file_size_in_pages: GenVal<u16>,
    pub num_of_reloc_items: GenVal<u16>,
    pub header_size_in_paragraphs: GenVal<u16>,
    pub min_extra_paragraphs: GenVal<u16>,
    pub max_extra_paragraphs: GenVal<u16>,
    pub initial_relative_ss: GenVal<u16>,
    pub initial_sp: GenVal<u16>,
    pub checksum: GenVal<u16>,
    pub initial_ip: GenVal<u16>,
    pub initial_relative_cs: GenVal<u16>,
    pub addr_of_reloc_table: GenVal<u16>,
    pub overlay_number: GenVal<u16>,
    pub reserved_0: GenVal<[u8; 0x08]>,
    pub oem_id: GenVal<u16>,
    pub oem_info: GenVal<u16>,
    pub reserved_1: GenVal<[u8; 0x14]>,
    pub addr_of_new_exe_hdr: GenVal<u32>,
}