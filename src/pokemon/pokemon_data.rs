const SUBSTRUCTURE_ORDER: [&'static str; 24] = [
    "GAEM", "GAME", "GEAM", "GEMA", "GMAE", "GMEA", "AGEM", "AGME", "AEGM", "AEMG", "AMGE", "AMEG",
    "EGAM", "EGMA", "EAGM", "EAMG", "EMGA", "EMAG", "MGAE", "MGEA", "MAGE", "MAEG", "MEGA", "MEAG",
];

#[derive(Debug, Default)]
pub struct PokemonData {
    pub order: &'static str,
    pub growth: PokemonGrowth,
    pub attacks: PokemonAttacks,
    pub evs_condition: PokemonEvsCondition,
    pub miscellaneous: PokemonMiscellaneous,
}

impl PokemonData {
    pub fn new(data: &mut [u8], personality_value: u32, ot_id: u32) -> Self {
        assert!(data.len() == 48, "PokemonData: data length must be 48");

        let decryption_key = ot_id ^ personality_value;

        for chunk in data.chunks_exact_mut(4) {
            let encrypted_u32 = u32::from_le_bytes(chunk.try_into().unwrap());
            let decrypted_u32 = encrypted_u32 ^ decryption_key;
            chunk.copy_from_slice(&decrypted_u32.to_le_bytes());
        }

        let order_index = (personality_value % 24) as usize;
        let order = SUBSTRUCTURE_ORDER[order_index];

        let mut pokemon_data = PokemonData::default();
        pokemon_data.order = order;

        for (index, c) in order.as_bytes().iter().enumerate() {
            let start = index * 12;

            match c {
                b'G' => {
                    let species = u16::from_le_bytes(data[start..start + 2].try_into().unwrap());
                    let item = u16::from_le_bytes(data[start + 2..start + 4].try_into().unwrap());
                    let exp = u32::from_le_bytes(data[start + 4..start + 8].try_into().unwrap());
                    let pp_bonus = data[start + 8];
                    let friendship = data[start + 9];
                    let unused =
                        u16::from_le_bytes(data[start + 10..start + 12].try_into().unwrap());

                    pokemon_data.growth = PokemonGrowth {
                        species,
                        item,
                        exp,
                        pp_bonus,
                        friendship,
                        unused,
                    };
                }
                b'A' => {
                    let move1 = u16::from_le_bytes(data[start..start + 2].try_into().unwrap());
                    let move2 = u16::from_le_bytes(data[start + 2..start + 4].try_into().unwrap());
                    let move3 = u16::from_le_bytes(data[start + 4..start + 6].try_into().unwrap());
                    let move4 = u16::from_le_bytes(data[start + 6..start + 8].try_into().unwrap());
                    let pp1 = data[start + 8];
                    let pp2 = data[start + 9];
                    let pp3 = data[start + 10];
                    let pp4 = data[start + 11];

                    pokemon_data.attacks = PokemonAttacks {
                        move1,
                        move2,
                        move3,
                        move4,
                        pp1,
                        pp2,
                        pp3,
                        pp4,
                    };
                }
                b'M' => {
                    let pokerus_status = data[start];
                    let met_location = data[start + 1];
                    let origins_info =
                        u16::from_le_bytes(data[start + 2..start + 4].try_into().unwrap());
                    let ivs_egg_abbility =
                        u32::from_le_bytes(data[start + 4..start + 8].try_into().unwrap());
                    let ribbons_obedience =
                        u32::from_le_bytes(data[start + 8..start + 12].try_into().unwrap());
                    pokemon_data.miscellaneous = PokemonMiscellaneous {
                        pokerus_status,
                        met_location,
                        origins_info,
                        ivs_egg_abbility,
                        ribbons_obedience,
                    };
                }
                b'E' => {
                    let hp_ev = data[start];
                    let attack_ev = data[start + 1];
                    let defense_ev = data[start + 2];
                    let speed_ev = data[start + 3];
                    let sp_attack_ev = data[start + 4];
                    let sp_defense_ev = data[start + 5];
                    let coolness = data[start + 6];
                    let beauty = data[start + 7];
                    let cuteness = data[start + 8];
                    let smartness = data[start + 9];
                    let toughness = data[start + 10];
                    let feel = data[start + 11];

                    pokemon_data.evs_condition = PokemonEvsCondition {
                        hp_ev,
                        attack_ev,
                        defense_ev,
                        speed_ev,
                        sp_attack_ev,
                        sp_defense_ev,
                        coolness,
                        beauty,
                        cuteness,
                        smartness,
                        toughness,
                        feel,
                    }
                }
                u => panic!("PokemonData: {u} is not a valid order"),
            }
        }

        pokemon_data
    }
}

#[derive(Debug, Default)]
pub struct PokemonGrowth {
    pub species: u16,
    pub item: u16,
    pub exp: u32,
    pub pp_bonus: u8,
    pub friendship: u8,
    pub unused: u16,
}

#[derive(Debug, Default)]
pub struct PokemonAttacks {
    pub move1: u16,
    pub move2: u16,
    pub move3: u16,
    pub move4: u16,
    pub pp1: u8,
    pub pp2: u8,
    pub pp3: u8,
    pub pp4: u8,
}

#[derive(Debug, Default)]
pub struct PokemonEvsCondition {
    pub hp_ev: u8,
    pub attack_ev: u8,
    pub defense_ev: u8,
    pub speed_ev: u8,
    pub sp_attack_ev: u8,
    pub sp_defense_ev: u8,
    pub coolness: u8,
    pub beauty: u8,
    pub cuteness: u8,
    pub smartness: u8,
    pub toughness: u8,
    pub feel: u8,
}

#[derive(Debug, Default)]
pub struct PokemonMiscellaneous {
    pub pokerus_status: u8,
    pub met_location: u8,
    pub origins_info: u16,
    pub ivs_egg_abbility: u32,
    pub ribbons_obedience: u32,
}
