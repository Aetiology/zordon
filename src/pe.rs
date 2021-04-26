use crate::fmt_err;
use crate::types::*;
use crate::{dos_hdr::DosHeader, sec_hdr::SectionHeader nt_hdr::*};
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{Read, Write};

pub struct Pe {
    pub dos_hdr: DosHeader,
    pub nt_hdr: NtHeader,
    pub sec_hdrs: Vec<SectionHeader>,
}

impl Pe {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, String> {
        let dos_hdr = DosHeader::new(reader)?;

        reader
            .seek(SeekFrom::Start(*dos_hdr.addr_of_new_exe_hdr.val() as u64))
            .map_err(|e| fmt_err!("Could not seek to nt header start: {}", e))?;

        let nt_hdr = NtHeader::new(reader)?;

        let sec_hdrs: Vec<SectionHeader> = Vec::new();
        let num_of_secs = *nt_hdr.file_hdr.num_of_secs.val();

        for _  in 0..num_of_secs{
            sec_hdrs.push(SectionHeader::new(reader)?)
        }

        Ok(Self { dos_hdr, nt_hdr, sec_hdrs })
    }
}
