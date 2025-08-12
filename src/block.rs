use std::{
    fs::File,
    io::{Seek, Write},
};

use crate::{section::Section, tools::print_bytes};

pub enum SaveType {
    A,
    B,
}

#[derive(Debug)]
pub struct Block {
    pub sections: [Section; 14],
    pub zero_index: usize,
}

impl From<[u8; 57344]> for Block {
    fn from(value: [u8; 57344]) -> Self {
        let mut sections = [Section::default(); 14];
        let mut zero_index = 0;

        for (idx, chunk) in value.chunks_exact(4096).enumerate() {
            let section = Section::from(chunk);

            assert!(section.id < 14, "Block: Section id must be within 0-13!");
            sections[section.id as usize] = section;

            println!("Section {}", section.id);
            print_bytes(&section.data[0..16]);

            if section.id == 0 {
                zero_index = idx;
            }
        }

        Block {
            sections,
            zero_index,
        }
    }
}

impl Block {
    pub fn write_section_to_file(
        &mut self,
        section_index: usize,
        file: &mut File,
    ) -> std::io::Result<()> {
        assert!(
            section_index < 14,
            "Blcok: Section index must be lesser than 14"
        );

        let real_index = (self.zero_index + section_index) % 14;

        let start = real_index * 4096;
        let section = &self.sections[section_index];
        file.seek(std::io::SeekFrom::Start(start as u64))?;
        file.write(&section.data)?;
        file.seek(std::io::SeekFrom::Start((start + 0x0FF4) as u64))?;
        file.write(&section.id.to_le_bytes())?;
        file.write(&section.checksum.to_le_bytes())?;
        file.write(&section.signature.to_le_bytes())?;
        file.write(&section.save_index.to_le_bytes())?;
        Ok(())
    }
}
