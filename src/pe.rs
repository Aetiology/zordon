use crate::fmt_err;
use crate::types::*;
use crate::{dos_hdr::DosHeader, nt_hdr::*, sec_hdr::SectionHeader};
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

        let mut sec_hdrs: Vec<SectionHeader> = Vec::new();
        let num_of_secs = *nt_hdr.file_hdr.num_of_secs.val();

        for _ in 0..num_of_secs {
            sec_hdrs.push(SectionHeader::new(reader)?)
        }

        Ok(Self {
            dos_hdr,
            nt_hdr,
            sec_hdrs,
        })
    }

    pub fn virt_addr_to_sec_index(&self, section_va: u32) -> Result<usize, String> {
        for (i, s) in self.sec_hdrs.iter().enumerate() {
            if (*s.virt_addr.val() <= section_va)
                && ((*s.virt_addr.val() + *s.virt_size.val()) > section_va)
            {
                return Ok(i);
            }
        }

        Err(fmt_err!(
            "Could not find section with va: {:#X}",
            section_va
        ))
    }

    pub fn entry_sec_index(&self) -> Result<usize, String> {
        self.virt_addr_to_sec_index(*self.nt_hdr.opt_hdr.addr_of_entrypoint.val())
    }

    pub fn entry_ip(&self) -> Result<u64, String> {
        Ok(*self.nt_hdr.opt_hdr.image_base.val()
            + *self.sec_hdrs[self.entry_sec_index()?].virt_addr.val() as u64)
    }
}
