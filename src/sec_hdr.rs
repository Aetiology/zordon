use crate::types::*;
use derive_header::MutSlice;
use std::io::prelude::*;
use std::io::Read;
/*
#[derive(SimpleValNew, Debug, PartialEq)]
pub struct SectionHeader {
    pub name: SimpleVal<[u8; 0x08]>,
    pub virt_size: SimpleVal<u32>,
    pub virt_addr: SimpleVal<u32>,
    pub size_of_raw_data: SimpleVal<u32>,
    pub ptr_to_raw_data: SimpleVal<u32>,
    pub ptr_to_relocs: SimpleVal<u32>,
    pub ptr_to_line_nums: SimpleVal<u32>,
    pub num_of_relocs: SimpleVal<u16>,
    pub num_of_line_nums: SimpleVal<u16>,
    pub characteristics: SimpleVal<u32>,
}
*/
