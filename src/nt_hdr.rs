use crate::fmt_err;
use crate::types::*;
use std::io::prelude::*;
use std::io::{Read, Write};

pub struct NtHeader {
    pub sig: GenVal<u16>,
    pub file_hdr: FileHeader,
    pub opt_hdr: OptHeader,
}

pub struct FileHeader {
    pub machine: GenVal<u16>,
    pub num_of_secs: GenVal<u16>,
    pub time_data_stamp: GenVal<u32>,
    pub ptr_to_symbol_table: GenVal<u32>,
    pub num_of_symbols: GenVal<u32>,
    pub opt_hdr_size: GenVal<u16>,
    pub file_characteristics: GenVal<u16>, // TODO: Think about making this into bitfields struct
}

pub struct OptHeader {
    pub magic: GenVal<u16>,
    pub major_linker_ver: GenVal<u8>,
    pub minor_linker_ver: GenVal<u8>,
    pub size_of_code: GenVal<u32>,
    pub size_of_init_data: GenVal<u32>,
    pub size_of_uninit_data: GenVal<u32>,
    pub addr_of_entrypoint: GenVal<u32>,
    pub base_of_code: GenVal<u32>,
    pub image_base: GenVal<u64>,
    pub sec_alignment: GenVal<u32>,
    pub file_alignment: GenVal<u32>,
    pub major_os_ver: GenVal<u16>,
    pub minor_os_ver: GenVal<u16>,
    pub major_image_ver: GenVal<u16>,
    pub minor_image_ver: GenVal<u16>,
    pub major_subsystem_ver: GenVal<u16>,
    pub minor_subsystem_ver: GenVal<u16>,
    pub win32_ver_val: GenVal<u32>,
    pub size_of_image: GenVal<u32>,
    pub size_of_hdrs: GenVal<u32>,
    pub checksum: GenVal<u32>,
    pub subsystem: GenVal<u16>,
    pub dll_characteristics: GenVal<u16>, // TODO: another one that can be made into bitfields struct
    pub size_of_stack_reservee: GenVal<u64>,
    pub size_of_stack_commit: GenVal<u64>,
    pub size_of_heap_reserve: GenVal<u64>,
    pub size_of_heap_commit: GenVal<u64>,
    pub loader_flags: GenVal<u32>,
    pub num_of_rva_and_sizes: GenVal<u32>,
    pub data_directories: DataDirectories,
}

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

pub struct DataDirectory {
    pub virtual_addr: GenVal<u32>,
    pub size: GenVal<u32>,
}

