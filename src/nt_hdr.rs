/*
use crate::types::*;
use derive_header::MutSlice;
use std::io::prelude::*;
use std::io::Read;

#[derive(SimpleValNew)]
pub struct NtHeader {
    pub sig: SimpleVal<u32>,
    pub file_hdr: FileHeader,
    pub opt_hdr: OptHeader,
}

#[derive(SimpleValNew)]
pub struct FileHeader {
    pub machine: SimpleVal<u16>,
    pub num_of_secs: SimpleVal<u16>,
    pub time_data_stamp: SimpleVal<u32>,
    pub ptr_to_symbol_table: SimpleVal<u32>,
    pub num_of_symbols: SimpleVal<u32>,
    pub opt_hdr_size: SimpleVal<u16>,
    pub file_characteristics: SimpleVal<u16>, // TODO: Think about making this into bitfields struct
}
#[derive(SimpleValNew)]
pub struct OptHeader {
    pub magic: SimpleVal<u16>,
    pub major_linker_ver: SimpleVal<u8>,
    pub minor_linker_ver: SimpleVal<u8>,
    pub size_of_code: SimpleVal<u32>,
    pub size_of_init_data: SimpleVal<u32>,
    pub size_of_uninit_data: SimpleVal<u32>,
    pub addr_of_entrypoint: SimpleVal<u32>,
    pub base_of_code: SimpleVal<u32>,
    pub image_base: SimpleVal<u64>,
    pub sec_alignment: SimpleVal<u32>,
    pub file_alignment: SimpleVal<u32>,
    pub major_os_ver: SimpleVal<u16>,
    pub minor_os_ver: SimpleVal<u16>,
    pub major_image_ver: SimpleVal<u16>,
    pub minor_image_ver: SimpleVal<u16>,
    pub major_subsystem_ver: SimpleVal<u16>,
    pub minor_subsystem_ver: SimpleVal<u16>,
    pub win32_ver_val: SimpleVal<u32>,
    pub size_of_image: SimpleVal<u32>,
    pub size_of_hdrs: SimpleVal<u32>,
    pub checksum: SimpleVal<u32>,
    pub subsystem: SimpleVal<u16>,
    pub dll_characteristics: SimpleVal<u16>, // TODO: another one that can be made into bitfields struct
    pub size_of_stack_reservee: SimpleVal<u64>,
    pub size_of_stack_commit: SimpleVal<u64>,
    pub size_of_heap_reserve: SimpleVal<u64>,
    pub size_of_heap_commit: SimpleVal<u64>,
    pub loader_flags: SimpleVal<u32>,
    pub num_of_rva_and_sizes: SimpleVal<u32>,
    pub data_dirs: DataDirectories,
}
#[derive(SimpleValNew)]
pub struct DataDirectories {
    pub export: Option<DataDirectory>,
    pub import: Option<DataDirectory>,
    pub resource: Option<DataDirectory>,
    pub exception: Option<DataDirectory>,
    pub security: Option<DataDirectory>,
    pub base_reloc: Option<DataDirectory>,
    pub debug: Option<DataDirectory>,
    pub architecture: Option<DataDirectory>,
    pub global_ptr: Option<DataDirectory>,
    pub tls: Option<DataDirectory>,
    pub load_config: Option<DataDirectory>,
    pub bound_import: Option<DataDirectory>,
    pub iat: Option<DataDirectory>,
    pub delay_import: Option<DataDirectory>,
    pub com_descriptor: Option<DataDirectory>,
    pub reserved: Option<DataDirectory>,
}
#[derive(SimpleValNew)]
pub struct DataDirectory {
    pub virt_addr: SimpleVal<u32>,
    pub size: SimpleVal<u32>,
}
*/
