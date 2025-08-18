use std::{
    fs::File,
    io::{Read, Seek, Write},
};

use crate::section::Section;

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
    // Despite the save index being stored in each section, only the value in the last section is used to determine the most recent save.
    // If save A's value is bigger, then it is the most recent. Otherwise, save B is the most recent (this includes ties).
    pub fn get_save_start_address(file: &mut File) -> std::io::Result<u64> {
        let mut buf = [0u8; 4];

        // each section is 4096 bytes.
        let save_a_last_section = 0xdffc;
        file.seek(std::io::SeekFrom::Start(save_a_last_section))?;
        file.read_exact(&mut buf)?;
        let save_index_a = u32::from_le_bytes(buf);

        let save_b_last_section = 0x1bffc;
        file.seek(std::io::SeekFrom::Start(save_b_last_section))?;
        file.read_exact(&mut buf)?;
        let save_index_b = u32::from_le_bytes(buf);

        if save_index_a > save_index_b {
            Ok(0x0000u64)
        } else {
            Ok(0xe000u64)
        }
    }

    pub fn write_section_to_file(
        &mut self,
        section_index: usize,
        file: &mut File,
    ) -> std::io::Result<()> {
        assert!(
            section_index < 14,
            "Block: Section index must be lesser than 14"
        );

        let real_index = (self.zero_index + section_index) % 14;

        let start = real_index * 4096;
        let section = &self.sections[section_index];
        file.seek(std::io::SeekFrom::Start(start as u64))?;
        file.write(&section.data)?;
        file.seek(std::io::SeekFrom::Start((start + 0x0FF4) as u64))?;
        file.write(&section.id.to_le_bytes())?;
        file.write(&section.compute_checksum().to_le_bytes())?;
        file.write(&section.signature.to_le_bytes())?;
        file.write(&section.save_index.to_le_bytes())?;
        Ok(())
    }
}
