use std::{
    fs::OpenOptions,
    io::{Read, Seek},
};

use emerald_save_editor::{
    block::Block,
    moves::Move,
    pokemon::{PokemonList, Species},
};

#[allow(dead_code)]
fn add_rayquaza(block: &mut Block) {
    let mut pokemon_list = PokemonList::from(&block.sections[1]);

    let mut new_pokemon = pokemon_list[0].clone();
    new_pokemon.max_out();
    new_pokemon.data.growth.species = Species::Rayquaza;
    new_pokemon.data.attacks.move1 = Move::DragonDance;
    new_pokemon.data.attacks.move2 = Move::AerialAce;
    new_pokemon.data.attacks.move3 = Move::BrickBreak;
    new_pokemon.data.attacks.move4 = Move::Outrage;
    pokemon_list.add(new_pokemon);

    block.sections[1].write_pokemon_list(&pokemon_list);
}

fn main() -> std::io::Result<()> {
    std::fs::copy(
        "./saves/pokemon-emerald-original.sav",
        "./saves/pokemon-emerald.sav",
    )?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("./saves/pokemon-emerald.sav")?;

    let start_address = Block::get_save_start_address(&mut file)?;
    file.seek(std::io::SeekFrom::Start(start_address))?;
    let mut buffer = [0u8; 57344];
    file.read_exact(&mut buffer)?;

    let mut block = Block::from(buffer);
    add_rayquaza(&mut block);
    block.write_section_to_file(1, &mut file)?;

    Ok(())
}
