#[allow(unused_attributes)]
#[macro_use]
#[allow(unused_imports)]
use assert_hex::assert_eq_hex;
/*
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelocationType {
    // For now skip/ignore the other types
    ImageRelBasedAbsolute,
    ImageRelBasedHigh,
    ImageRelBasedLow,
    ImageRelBasedHighLow,
    ImageRelBasedHighAdj,
    ImageRelBasedMipsJmpAddr,
}

impl RelocationType {
    pub fn new(reloc_type: u8) -> Self {
        match reloc_type {
            0 => Self::ImageRelBasedAbsolute,
            1 => Self::ImageRelBasedHigh,
            2 => Self::ImageRelBasedLow,
            3 => Self::ImageRelBasedHighLow,
            4 => Self::ImageRelBasedHighAdj,
            5 => Self::ImageRelBasedMipsJmpAddr,
            _ => unimplemented!("reloc_type: {}", reloc_type),
        }
    }
}

#[derive(PartialEq)]
pub struct Relocations {
    pub virt_addr: SimpleVal<u32>,
    pub size_of_block: SimpleVal<u32>,
    pub block: Vec<SimpleVal<u16>>,
}

impl Relocations {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, String> {
        let virt_addr = SimpleVal::new(reader)?;
        let size_of_block: SimpleVal<u32> = SimpleVal::new(reader)?;

        let entry_count = ((*size_of_block - 8) / 2) as usize;
        let mut block: Vec<SimpleVal<u16>> = Vec::with_capacity(entry_count);

        for _ in 0..entry_count {
            block.push(SimpleVal::new(reader)?);
        }

        Ok(Self {
            virt_addr,
            size_of_block,
            block,
        })
    }

    pub fn to_type(type_offset_pair: u16) -> RelocationType {
        RelocationType::new(((type_offset_pair & 0xF000) >> 12) as u8)
    }

    pub fn to_offset(type_offset_pair: u16) -> u16 {
        type_offset_pair & 0xFFF
    }
}

#[allow(dead_code)]
const RELOC_TESTDATA: [u8; 0x10] = [
    0, 0x10, 0, 0, 0x0C, 0, 0, 0, 0x17, 0x30, 0x1F, 0x30, 0, 0, 0, 0,
];

#[test]
fn relocations_new() {
    let reloc_data = RELOC_TESTDATA.to_vec();
    let mut buf = std::io::Cursor::new(reloc_data);

    let relocs = Relocations::new(&mut buf).unwrap();

    assert_eq_hex!(*relocs.virt_addr, 0x1000);
    assert_eq_hex!(*relocs.size_of_block, 0x0C);
    assert_eq_hex!(*relocs.block[0], 0x3017);
    assert_eq_hex!(*relocs.block[1], 0x301F);

    assert_eq!(
        Relocations::to_type(*relocs.block[0]),
        RelocationType::ImageRelBasedHighLow
    );
    assert_eq!(
        Relocations::to_type(*relocs.block[1]),
        RelocationType::ImageRelBasedHighLow
    );

    assert_eq!(Relocations::to_offset(*relocs.block[0]), 0x17);
    assert_eq!(Relocations::to_offset(*relocs.block[1]), 0x1F);
}

*/
