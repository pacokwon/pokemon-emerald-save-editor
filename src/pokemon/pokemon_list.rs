use crate::{pokemon::Pokemon, section::Section};

#[derive(Debug)]
pub struct PokemonList(pub Vec<Pokemon>);

impl From<&[u8]> for PokemonList {
    fn from(value: &[u8]) -> Self {
        assert!(value.len() == 604, "Team Pokemon list must be length 604");

        let length = u32::from_le_bytes(value[..4].try_into().unwrap()) as usize;
        let mut pokemon_list = Vec::with_capacity(length);

        for i in 0..6 {
            if i >= length {
                break;
            }

            let start = (4 + i * 100) as usize;
            let end = (4 + (i + 1) * 100) as usize;

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
