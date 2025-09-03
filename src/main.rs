use std::{
    fs::OpenOptions,
    io::{Read, Seek},
};

use emerald_save_editor::{
    block::Block,
    moves::MoveId,
    pokemon::{PokemonList, Species},
};

#[allow(dead_code)]
fn add_rayquaza(block: &mut Block) {
    let mut pokemon_list = PokemonList::from(&block.sections[1]);

    let mut new_pokemon = pokemon_list[0].clone();

    new_pokemon.data.growth.species = Species::Rayquaza;
    new_pokemon.data.attacks.move1 = MoveId::DragonDance;
    new_pokemon.data.attacks.move2 = MoveId::AerialAce;
    new_pokemon.data.attacks.move3 = MoveId::BrickBreak;
    new_pokemon.data.attacks.move4 = MoveId::Outrage;

    new_pokemon.max_out();

    pokemon_list.add(new_pokemon);

    block.sections[1].write_pokemon_list(&pokemon_list);
}

#[allow(dead_code)]
fn add_surf_pikachu(block: &mut Block) {
    let mut pokemon_list = PokemonList::from(&block.sections[1]);

    let mut new_pokemon = pokemon_list[0].clone();

    new_pokemon.data.growth.species = Species::Pikachu;
    new_pokemon.data.attacks.move1 = MoveId::Surf;
    new_pokemon.data.attacks.move2 = MoveId::Thunderbolt;
    new_pokemon.data.attacks.move3 = MoveId::Headbutt;
    new_pokemon.data.attacks.move4 = MoveId::ThunderWave;

    new_pokemon.max_out();

    pokemon_list.add(new_pokemon);

    block.sections[1].write_pokemon_list(&pokemon_list);
}

fn main() -> std::io::Result<()> {

    Ok(())
}
