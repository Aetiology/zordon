use crate::types::*;
use derive_header::MutViewNew;

#[derive(MutViewNew)]
pub struct ImportDescriptor<'a> {
    pub original_first_thunk: SimpleVal<'a, u32>,
    pub time_data_stamp: SimpleVal<'a, u32>,
    pub forwarder_chain: SimpleVal<'a, u32>,
    pub first_thunk: SimpleVal<'a, u32>,
}
