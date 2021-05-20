use crate::types::*;
use derive_header::MutViewNew;

#[derive(MutViewNew)]
pub struct DosHeader<'a> {
    pub mz_sig: SimpleVal<'a, u16>,
    pub used_bytes_in_last_page: SimpleVal<'a, u16>,
    pub file_size_in_pages: SimpleVal<'a, u16>,
    pub num_of_reloc_items: SimpleVal<'a, u16>,
    pub header_size_in_paragraphs: SimpleVal<'a, u16>,
    pub min_extra_paragraphs: SimpleVal<'a, u16>,
    pub max_extra_paragraphs: SimpleVal<'a, u16>,
    pub initial_relative_ss: SimpleVal<'a, u16>,
    pub initial_sp: SimpleVal<'a, u16>,
    pub checksum: SimpleVal<'a, u16>,
    pub initial_ip: SimpleVal<'a, u16>,
    pub initial_relative_cs: SimpleVal<'a, u16>,
    pub addr_of_reloc_table: SimpleVal<'a, u16>,
    pub overlay_number: SimpleVal<'a, u16>,
    pub reserved_0: ArrayVal<'a, [u8; 0x08]>,
    pub oem_id: SimpleVal<'a, u16>,
    pub oem_info: SimpleVal<'a, u16>,
    pub reserved_1: ArrayVal<'a, [u8; 0x14]>,
    pub addr_of_new_exe_hdr: SimpleVal<'a, u32>,
}
