use crate::fmt_err;
use crate::types::*;
use crate::{dos_hdr::DosHeader, nt_hdr::*, sec_hdr::SectionHeader};
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{Cursor, Read, Write};

pub struct PeHeader {
    pub dos_hdr: DosHeader,
    pub nt_hdr: NtHeader,
    pub sec_hdrs: Vec<SectionHeader>,
    pub rwbuf: Cursor<Vec<u8>>,
}

impl PeHeader {
    pub fn new(buf: Vec<u8>) -> Result<Self, String> {
        let mut rwbuf = std::io::Cursor::new(buf);

        let dos_hdr = DosHeader::new(&mut rwbuf)?;

        rwbuf
            .seek(SeekFrom::Start(*dos_hdr.addr_of_new_exe_hdr as u64))
            .map_err(|e| fmt_err!("Could not seek to nt header start: {}", e))?;

        let nt_hdr = NtHeader::new(&mut rwbuf)?;

        let mut sec_hdrs: Vec<SectionHeader> = Vec::new();
        let num_of_secs = *nt_hdr.file_hdr.num_of_secs;

        for _ in 0..num_of_secs {
            sec_hdrs.push(SectionHeader::new(&mut rwbuf)?)
        }

        Ok(Self {
            dos_hdr,
            nt_hdr,
            sec_hdrs,
            rwbuf,
        })
    }

    pub fn virt_addr_to_sec_index(&self, section_va: u32) -> Result<usize, String> {
        for (i, s) in self.sec_hdrs.iter().enumerate() {
            if (*s.virt_addr <= section_va) && ((*s.virt_addr + *s.virt_size) > section_va) {
                return Ok(i);
            }
        }

        Err(fmt_err!(
            "Could not find section with va: {:#X}",
            section_va
        ))
    }

    pub fn entry_sec_index(&self) -> Result<usize, String> {
        self.virt_addr_to_sec_index(*self.nt_hdr.opt_hdr.addr_of_entrypoint)
    }

    pub fn entry_rel_sec_offset(&self) -> Result<usize, String> {
        Ok(*self.nt_hdr.opt_hdr.addr_of_entrypoint as usize
            - *self.entry_sec_ref()?.virt_addr as usize)
    }

    pub fn entry_sec_ref(&self) -> Result<&SectionHeader, String> {
        Ok(&self.sec_hdrs[self.entry_sec_index()?])
    }

    pub fn entry_sec_refmut(&mut self) -> Result<&mut SectionHeader, String> {
        let entry_sec_index = self.entry_sec_index()?;
        Ok(&mut self.sec_hdrs[entry_sec_index])
    }

    pub fn entry_ip(&self) -> Result<u64, String> {
        Ok(*self.nt_hdr.opt_hdr.image_base + *self.entry_sec_ref()?.virt_addr as u64)
    }

    pub fn entry_disk_offset(&self) -> Result<usize, String> {
        Ok(*self.entry_sec_ref()?.ptr_to_raw_data as usize + self.entry_rel_sec_offset()?)
    }

    pub fn calc_entry_sec_virt_size(&self) -> Result<u32, String> {
        Ok(((*self.entry_sec_ref()?.size_of_raw_data / 0x1000) + 1) * 0x1000)
    }
}

//Tests

#[test]
fn pe_virt_addr_to_sec_index() -> Result<(), String> {
    let mut pe_hdr = parse_test_pe()?;

    pe_hdr.sec_hdrs[0]
        .virt_addr
        .set(&mut pe_hdr.rwbuf, 0x1000)?;

    pe_hdr.sec_hdrs[0]
        .virt_size
        .set(&mut pe_hdr.rwbuf, 0x1000)?;

    pe_hdr.sec_hdrs[1]
        .virt_addr
        .set(&mut pe_hdr.rwbuf, 0x2000)?;

    pe_hdr.sec_hdrs[1]
        .virt_size
        .set(&mut pe_hdr.rwbuf, 0x1000)?;

    //assert_eq!(pe_hdr.virt_addr_to_sec_index(0x500), Err(_));
    assert_eq!(pe_hdr.virt_addr_to_sec_index(0x1000)?, 0);
    assert_eq!(pe_hdr.virt_addr_to_sec_index(0x1500)?, 0);
    assert_eq!(pe_hdr.virt_addr_to_sec_index(0x2000)?, 1);

    Ok(())
}

#[test]
fn pe_entry_sec_index() -> Result<(), String> {
    let mut pe_hdr = parse_test_pe()?;

    let new_entry_va = *pe_hdr.sec_hdrs[0].virt_addr;
    pe_hdr
        .nt_hdr
        .opt_hdr
        .addr_of_entrypoint
        .set(&mut pe_hdr.rwbuf, new_entry_va)?;

    assert_eq!(pe_hdr.entry_sec_index()?, 0);

    Ok(())
}

fn parse_test_pe() -> Result<PeHeader, String> {
    const TEST_PE: &str = "test_data/test_pe_hdr.bin";

    let pe_buf: Vec<u8> =
        std::fs::read(TEST_PE).map_err(|e| fmt_err!("Could not read file: {} - {}", TEST_PE, e))?;

    let pe_hdr = PeHeader::new(pe_buf).map_err(|e| fmt_err!("Could not create PeHeader: {}", e))?;

    Ok(pe_hdr)
}
