use crate::fmt_err;
use crate::types::*;
use crate::{dos_hdr::DosHeader, nt_hdr::*, reloc::Relocations, sec_hdr::SectionHeader};
#[macro_use]
use assert_hex::assert_eq_hex;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{Cursor, Read, Write};

pub struct PeHeader {
    pub dos_hdr: DosHeader,
    pub nt_hdr: NtHeader,
    pub sec_hdrs: Vec<SectionHeader>,
    pub relocs: Option<Relocations>,
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

        let relocs = match &nt_hdr.opt_hdr.data_dirs.base_reloc {
            Some(base_relocs) => {
                let reloc_table_disk_offset =
                    Self::rva_to_file_offset(&sec_hdrs, *base_relocs.virt_addr)?;
                    
                rwbuf
                    .seek(SeekFrom::Start(reloc_table_disk_offset as u64))
                    .map_err(|e| {
                        fmt_err!(
                            "Failed to seek to reloc table disk offset: {} - {}",
                            reloc_table_disk_offset,
                            e,
                        )
                    })?;

                Some(Relocations::new(&mut rwbuf)?)
            }
            None => None,
        };

        Ok(Self {
            dos_hdr,
            nt_hdr,
            sec_hdrs,
            relocs,
            rwbuf,
        })
    }

    pub fn rva_to_file_offset(sec_hdrs: &Vec<SectionHeader>, rva: u32) -> Result<u32, String> {
        for s in sec_hdrs.iter() {
            if (*s.virt_addr <= rva) && ((*s.virt_addr + *s.virt_size) > rva) {
                return Ok((rva - *s.virt_addr) + *s.ptr_to_raw_data);
            }
        }

        Err(fmt_err!("Could find section rva resides in"))
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

    pub fn entry_sec_virt_ip(&self) -> Result<u64, String> {
        Ok(*self.nt_hdr.opt_hdr.image_base + *self.entry_sec_ref()?.virt_addr as u64)
    }

    pub fn entry_disk_offset(&self) -> Result<usize, String> {
        Ok(*self.entry_sec_ref()?.ptr_to_raw_data as usize + self.entry_rel_sec_offset()?)
    }

    pub fn entry_sec_virt_size(&self) -> Result<u32, String> {
        Ok(((*self.entry_sec_ref()?.size_of_raw_data / 0x1000) + 1) * 0x1000)
    }
}

//Tests

#[test]
fn virt_addr_to_sec_index() {
    let mut pe_hdr = parse_test_pe().unwrap();

    pe_hdr.sec_hdrs[0]
        .virt_addr
        .set(&mut pe_hdr.rwbuf, 0x1000)
        .unwrap();

    pe_hdr.sec_hdrs[0]
        .virt_size
        .set(&mut pe_hdr.rwbuf, 0x1000)
        .unwrap();

    pe_hdr.sec_hdrs[1]
        .virt_addr
        .set(&mut pe_hdr.rwbuf, 0x2000)
        .unwrap();

    pe_hdr.sec_hdrs[1]
        .virt_size
        .set(&mut pe_hdr.rwbuf, 0x1000)
        .unwrap();

    assert_eq_hex!(pe_hdr.virt_addr_to_sec_index(0x0).ok(), None);
    assert_eq_hex!(pe_hdr.virt_addr_to_sec_index(0x1000).ok(), Some(0));
    assert_eq_hex!(pe_hdr.virt_addr_to_sec_index(0x1500).ok(), Some(0));
    assert_eq_hex!(pe_hdr.virt_addr_to_sec_index(0x2000).ok(), Some(1));
}

#[test]
fn entry_sec_index() {
    let mut pe_hdr = parse_test_pe().unwrap();

    let new_entry_va = *pe_hdr.sec_hdrs[0].virt_addr;
    pe_hdr
        .nt_hdr
        .opt_hdr
        .addr_of_entrypoint
        .set(&mut pe_hdr.rwbuf, new_entry_va)
        .unwrap();

    assert_eq_hex!(pe_hdr.entry_sec_index().ok(), Some(0));
}

#[test]
fn entry_rel_sec_offset() {
    let mut pe_hdr = parse_test_pe().unwrap();

    pe_hdr
        .nt_hdr
        .opt_hdr
        .addr_of_entrypoint
        .set(&mut pe_hdr.rwbuf, *pe_hdr.sec_hdrs[0].virt_addr + 0x100)
        .unwrap();

    assert_eq_hex!(pe_hdr.entry_rel_sec_offset().ok(), Some(0x100));

    pe_hdr
        .nt_hdr
        .opt_hdr
        .addr_of_entrypoint
        .set(&mut pe_hdr.rwbuf, 0)
        .unwrap();

    assert_eq_hex!(pe_hdr.entry_rel_sec_offset().ok(), None);
}

#[test]
fn entry_sec_ref() {
    let mut pe_hdr = parse_test_pe().unwrap();

    pe_hdr
        .nt_hdr
        .opt_hdr
        .addr_of_entrypoint
        .set(&mut pe_hdr.rwbuf, 0x1500)
        .unwrap();

    pe_hdr.sec_hdrs[0]
        .virt_addr
        .set(&mut pe_hdr.rwbuf, 0x1000)
        .unwrap();

    pe_hdr.sec_hdrs[0]
        .virt_size
        .set(&mut pe_hdr.rwbuf, 0x1000)
        .unwrap();

    assert_eq_hex!(*pe_hdr.entry_sec_ref().unwrap(), pe_hdr.sec_hdrs[0]);
}

#[test]
pub fn entry_sec_virt_ip() {
    let mut pe_hdr = parse_test_pe().unwrap();

    pe_hdr
        .nt_hdr
        .opt_hdr
        .addr_of_entrypoint
        .set(&mut pe_hdr.rwbuf, 0x1000)
        .unwrap();

    pe_hdr.sec_hdrs[0]
        .virt_addr
        .set(&mut pe_hdr.rwbuf, 0x1000)
        .unwrap();
    pe_hdr
        .nt_hdr
        .opt_hdr
        .image_base
        .set(&mut pe_hdr.rwbuf, 0x500000)
        .unwrap();

    assert_eq_hex!(pe_hdr.entry_sec_virt_ip().unwrap(), 0x501000)
}

#[test]
pub fn entry_disk_offset() {
    let mut pe_hdr = parse_test_pe().unwrap();

    pe_hdr
        .nt_hdr
        .opt_hdr
        .addr_of_entrypoint
        .set(&mut pe_hdr.rwbuf, 0x1500)
        .unwrap();

    pe_hdr.sec_hdrs[0]
        .virt_addr
        .set(&mut pe_hdr.rwbuf, 0x1000)
        .unwrap();

    pe_hdr.sec_hdrs[0]
        .virt_size
        .set(&mut pe_hdr.rwbuf, 0x1000)
        .unwrap();

    pe_hdr.sec_hdrs[0]
        .ptr_to_raw_data
        .set(&mut pe_hdr.rwbuf, 0x400)
        .unwrap();

    assert_eq_hex!(pe_hdr.entry_disk_offset().unwrap(), 0x900);
}

#[test]
pub fn entry_sec_virt_size() {
    let mut pe_hdr = parse_test_pe().unwrap();

    pe_hdr
        .nt_hdr
        .opt_hdr
        .addr_of_entrypoint
        .set(&mut pe_hdr.rwbuf, 0x1500)
        .unwrap();

    pe_hdr.sec_hdrs[0]
        .virt_addr
        .set(&mut pe_hdr.rwbuf, 0x1000)
        .unwrap();

    pe_hdr.sec_hdrs[0]
        .size_of_raw_data
        .set(&mut pe_hdr.rwbuf, 0x442)
        .unwrap();

    assert_eq_hex!(pe_hdr.entry_sec_virt_size().unwrap(), 0x1000);

    pe_hdr.sec_hdrs[0]
        .size_of_raw_data
        .set(&mut pe_hdr.rwbuf, 0x0)
        .unwrap();

    assert_eq_hex!(pe_hdr.entry_sec_virt_size().unwrap(), 0x1000);

    pe_hdr.sec_hdrs[0]
        .size_of_raw_data
        .set(&mut pe_hdr.rwbuf, 0x1001)
        .unwrap();

    assert_eq_hex!(pe_hdr.entry_sec_virt_size().unwrap(), 0x2000);
}

fn parse_test_pe() -> Result<PeHeader, String> {
    const TEST_PE: &str = "test_data/test_pe_hdr.bin";

    let pe_buf: Vec<u8> =
        std::fs::read(TEST_PE).map_err(|e| fmt_err!("Could not read file: {} - {}", TEST_PE, e))?;

    let pe_hdr = PeHeader::new(pe_buf).map_err(|e| fmt_err!("Could not create PeHeader: {}", e))?;

    Ok(pe_hdr)
}
