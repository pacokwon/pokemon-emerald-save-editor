use crate::{section::Section, tools::print_bytes};

pub enum SaveType {
    A,
    B,
}

#[derive(Debug)]
pub struct Block {
    pub sections: [Section; 14],
}

impl From<[u8; 57344]> for Block {
    fn from(value: [u8; 57344]) -> Self {
        let mut sections = [Section::default(); 14];

        for chunk in value.chunks_exact(4096) {
            let section = Section::from(chunk);

            assert!(section.id < 14, "Block: Section id must be within 0-13!");
            sections[section.id as usize] = section;

            println!("Section {}", section.id);
            print_bytes(&section.data[0..16]);
        }

        Block { sections }
    }
}

impl Block {
    pub fn write_section(&mut self, section: &Section) {}
}
