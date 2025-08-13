use std::ops::{Index, IndexMut};

use crate::{pokemon::Pokemon, section::Section};

#[derive(Debug)]
pub struct PokemonList(Vec<Pokemon>);

impl PokemonList {
    pub fn add(&mut self, pokemon: Pokemon) {
        if self.0.len() == 6 {
            return;
        }

        self.0.push(pokemon);
    }
}

impl Index<usize> for PokemonList {
    type Output = Pokemon;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for PokemonList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<&[u8]> for PokemonList {
    fn from(value: &[u8]) -> Self {
        assert!(value.len() == 604, "Team Pokemon list must be length 604");

        let length = u32::from_le_bytes(value[..4].try_into().unwrap()) as usize;
        let mut pokemon_list = Vec::with_capacity(length);

        for i in 0..6 {
            if i >= length {
                break;
            }

            let start = 4 + i * 100;
            let end = 4 + (i + 1) * 100;

            pokemon_list.push(Pokemon::from(&value[start..end]));
        }

        PokemonList(pokemon_list)
    }
}

impl From<&Section> for PokemonList {
    fn from(value: &Section) -> Self {
        assert!(value.id == 1, "Section id must be 1");

        PokemonList::from(&value.data[0x234..(0x234 + 604)])
    }
}

impl From<&PokemonList> for [u8; 604] {
    fn from(value: &PokemonList) -> Self {
        let mut buf = [0u8; 604];
        let length = value.0.len();

        buf[0..4].copy_from_slice(&(length as u32).to_le_bytes());

        for (i, pokemon) in value.0.iter().enumerate() {
            if i >= length {
                break;
            }

            let start = 4 + i * 100;
            let end = 4 + (i + 1) * 100;

            buf[start..end].copy_from_slice(&<[u8; 100]>::from(pokemon));
        }

        buf
    }
}
