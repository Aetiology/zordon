use crate::types::*;
use derive_header::GenValNew;
use std::io::prelude::*;
use std::io::Read;

#[derive(GenValNew, Debug, PartialEq)]
pub struct SectionHeader {
    pub name: GenVal<[u8; 0x08]>,
    pub virt_size: GenVal<u32>,
    pub virt_addr: GenVal<u32>,
    pub size_of_raw_data: GenVal<u32>,
    pub ptr_to_raw_data: GenVal<u32>,
    pub ptr_to_relocs: GenVal<u32>,
    pub ptr_to_line_nums: GenVal<u32>,
    pub num_of_relocs: GenVal<u16>,
    pub num_of_line_nums: GenVal<u16>,
    pub characteristics: GenVal<u32>,
}
