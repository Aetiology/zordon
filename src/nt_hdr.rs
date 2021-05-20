use crate::types::*;
use derive_header::MutViewNew;

#[derive(MutViewNew)]
pub struct NtHeader<'a> {
    pub sig: SimpleVal<'a, u32>,
    pub file_hdr: FileHeader<'a>,
    pub opt_hdr: OptHeader<'a>,
}

#[derive(MutViewNew)]
pub struct FileHeader<'a> {
    pub machine: SimpleVal<'a, u16>,
    pub num_of_secs: SimpleVal<'a, u16>,
    pub time_data_stamp: SimpleVal<'a, u32>,
    pub ptr_to_symbol_table: SimpleVal<'a, u32>,
    pub num_of_symbols: SimpleVal<'a, u32>,
    pub opt_hdr_size: SimpleVal<'a, u16>,
    pub file_characteristics: SimpleVal<'a, u16>, // TODO: Think about making this into bitfields struct
}

#[derive(MutViewNew)]
pub struct OptHeader<'a> {
    pub magic: SimpleVal<'a, u16>,
    pub major_linker_ver: SimpleVal<'a, u8>,
    pub minor_linker_ver: SimpleVal<'a, u8>,
    pub size_of_code: SimpleVal<'a, u32>,
    pub size_of_init_data: SimpleVal<'a, u32>,
    pub size_of_uninit_data: SimpleVal<'a, u32>,
    pub addr_of_entrypoint: SimpleVal<'a, u32>,
    pub base_of_code: SimpleVal<'a, u32>,
    pub image_base: SimpleVal<'a, u64>,
    pub sec_alignment: SimpleVal<'a, u32>,
    pub file_alignment: SimpleVal<'a, u32>,
    pub major_os_ver: SimpleVal<'a, u16>,
    pub minor_os_ver: SimpleVal<'a, u16>,
    pub major_image_ver: SimpleVal<'a, u16>,
    pub minor_image_ver: SimpleVal<'a, u16>,
    pub major_subsystem_ver: SimpleVal<'a, u16>,
    pub minor_subsystem_ver: SimpleVal<'a, u16>,
    pub win32_ver_val: SimpleVal<'a, u32>,
    pub size_of_image: SimpleVal<'a, u32>,
    pub size_of_hdrs: SimpleVal<'a, u32>,
    pub checksum: SimpleVal<'a, u32>,
    pub subsystem: SimpleVal<'a, u16>,
    pub dll_characteristics: SimpleVal<'a, u16>, // TODO: another one that can be made into bitfields struct
    pub size_of_stack_reservee: SimpleVal<'a, u64>,
    pub size_of_stack_commit: SimpleVal<'a, u64>,
    pub size_of_heap_reserve: SimpleVal<'a, u64>,
    pub size_of_heap_commit: SimpleVal<'a, u64>,
    pub loader_flags: SimpleVal<'a, u32>,
    pub num_of_rva_and_sizes: SimpleVal<'a, u32>,
    pub data_dirs: DataDirectories<'a>,
}

#[derive(MutViewNew)]
pub struct DataDirectories<'a> {
    pub export: Option<DataDirectory<'a>>,
    pub import: Option<DataDirectory<'a>>,
    pub resource: Option<DataDirectory<'a>>,
    pub exception: Option<DataDirectory<'a>>,
    pub security: Option<DataDirectory<'a>>,
    pub base_reloc: Option<DataDirectory<'a>>,
    pub debug: Option<DataDirectory<'a>>,
    pub architecture: Option<DataDirectory<'a>>,
    pub global_ptr: Option<DataDirectory<'a>>,
    pub tls: Option<DataDirectory<'a>>,
    pub load_config: Option<DataDirectory<'a>>,
    pub bound_import: Option<DataDirectory<'a>>,
    pub iat: Option<DataDirectory<'a>>,
    pub delay_import: Option<DataDirectory<'a>>,
    pub com_descriptor: Option<DataDirectory<'a>>,
    pub reserved: Option<DataDirectory<'a>>,
}
#[derive(MutViewNew)]
pub struct DataDirectory<'a> {
    pub virt_addr: SimpleVal<'a, u32>,
    pub size: SimpleVal<'a, u32>,
}
