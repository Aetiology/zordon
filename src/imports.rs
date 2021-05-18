use crate::types::*;
use derive_header::GenValNew;
use std::io::prelude::*;
use std::io::Read;

#[derive(GenValNew)]
pub struct ImportDescriptor {
    pub original_first_thunk: GenVal<u32>,
    pub time_data_stamp: GenVal<u32>,
    pub forwarder_chain: GenVal<u32>,
    pub first_thunk: GenVal<u32>,
}
