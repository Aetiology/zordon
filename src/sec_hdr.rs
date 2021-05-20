use crate::types::*;
use derive_header::MutViewNew;

#[derive(MutViewNew, Debug, PartialEq)]
pub struct SectionHeader<'a> {
    pub name: ArrayVal<'a, [u8; 0x08]>,
    pub virt_size: SimpleVal<'a, u32>,
    pub virt_addr: SimpleVal<'a, u32>,
    pub size_of_raw_data: SimpleVal<'a, u32>,
    pub ptr_to_raw_data: SimpleVal<'a, u32>,
    pub ptr_to_relocs: SimpleVal<'a, u32>,
    pub ptr_to_line_nums: SimpleVal<'a, u32>,
    pub num_of_relocs: SimpleVal<'a, u16>,
    pub num_of_line_nums: SimpleVal<'a, u16>,
    pub characteristics: SimpleVal<'a, u32>,
}
