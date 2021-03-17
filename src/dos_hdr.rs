use crate::types::*;

pub struct DosHeader {
    mz_sig: PrimVal<u16>,
    used_bytes_in_last_page: PrimVal<u16>,
}
