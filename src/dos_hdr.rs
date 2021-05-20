use crate::types::*;
use derive_header::MutSlice;
use std::io::prelude::*;
use std::io::Read;
/*
#[derive(SimpleValNew)]
pub struct DosHeader {
    pub mz_sig: SimpleVal<u16>,
    pub used_bytes_in_last_page: SimpleVal<u16>,
    pub file_size_in_pages: SimpleVal<u16>,
    pub num_of_reloc_items: SimpleVal<u16>,
    pub header_size_in_paragraphs: SimpleVal<u16>,
    pub min_extra_paragraphs: SimpleVal<u16>,
    pub max_extra_paragraphs: SimpleVal<u16>,
    pub initial_relative_ss: SimpleVal<u16>,
    pub initial_sp: SimpleVal<u16>,
    pub checksum: SimpleVal<u16>,
    pub initial_ip: SimpleVal<u16>,
    pub initial_relative_cs: SimpleVal<u16>,
    pub addr_of_reloc_table: SimpleVal<u16>,
    pub overlay_number: SimpleVal<u16>,
    pub reserved_0: SimpleVal<[u8; 0x08]>,
    pub oem_id: SimpleVal<u16>,
    pub oem_info: SimpleVal<u16>,
    pub reserved_1: SimpleVal<[u8; 0x14]>,
    pub addr_of_new_exe_hdr: SimpleVal<u32>,
}
*/
