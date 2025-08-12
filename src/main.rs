use std::{fs::OpenOptions, io::Read};

use emerald_save_editor::{block::Block, pokemon::PokemonList};

fn main() -> std::io::Result<()> {
    std::fs::copy(
        "./saves/pokemon-emerald-original.sav",
        "./saves/pokemon-emerald.sav",
    )?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("./saves/pokemon-emerald.sav")?;

    let mut buffer = [0u8; 57344];
    file.read_exact(&mut buffer)?;

    let mut block = Block::from(buffer);

    let pokemon_list = PokemonList::from(&block.sections[1]);

    block.sections[1].write_pokemon_list(&pokemon_list);
    block.write_section_to_file(1, &mut file)?;

    Ok(())
}
