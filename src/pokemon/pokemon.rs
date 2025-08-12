use crate::pokemon::PokemonData;

// Personality value 	u32 	0x00 	4 	0
// OT ID 	u32 	0x04 	4 	4
// Nickname 	u8[10] 	0x08 	10 	8
// Language 	u8 	0x12 	1 	18
// Misc. Flags 	u8 	0x13 	1 	19
// OT name 	u8[7] 	0x14 	7 	20
// Markings 	u8 	0x1B 	1 	27
// Checksum 	u16 	0x1C 	2 	28
// ???? 	u16 	0x1E 	2 	30
// Data 	u8[48] 	0x20 	48 	32
// Status condition 	u32 	0x50 	4 	80
// Level 	u8 	0x54 	1 	84
// Mail ID 	u8 	0x55 	1 	85
// Current HP 	u16 	0x56 	2 	86
// Total HP 	u16 	0x58 	2 	88
// Attack 	u16 	0x5A 	2 	90
// Defense 	u16 	0x5C 	2 	92
// Speed 	u16 	0x5E 	2 	94
// Sp. Attack 	u16 	0x60 	2 	96
// Sp. Defense 	u16 	0x62 	2 	98
#[derive(Debug)]
pub struct Pokemon {
    pub personality_value: u32,
    pub ot_id: u32,
    pub nickname: [u8; 10],
    pub language: u8,
    pub misc_flags: u8,
    pub ot_name: [u8; 7],
    pub markings: u8,
    pub checksum: u16,
    pub unknown: u16,
    pub data: PokemonData,
    pub status_condition: u32,
    pub level: u8,
    pub mail_id: u8,
    pub current_hp: u16,
    pub total_hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
    pub sp_attack: u16,
    pub sp_defense: u16,
}

impl From<&[u8]> for Pokemon {
    fn from(value: &[u8]) -> Self {
        assert!(value.len() == 100, "Pokemon data must be length 100");

        let personality_value = u32::from_le_bytes(value[0..4].try_into().unwrap());
        let ot_id = u32::from_le_bytes(value[4..8].try_into().unwrap());
        let nickname = {
            let mut buf = [0u8; 10];
            buf.copy_from_slice(&value[8..18]);
            buf
        };
        let language = value[0x12];
        let misc_flags = value[0x13];
        let ot_name = {
            let mut buf = [0u8; 7];
            buf.copy_from_slice(&value[0x14..0x1B]);
            buf
        };
        let markings = value[0x1B];
        let checksum = u16::from_le_bytes(value[0x1C..0x1E].try_into().unwrap());
        let unknown = u16::from_le_bytes(value[0x1E..0x20].try_into().unwrap());
        let data = {
            let mut buf = [0u8; 48];
            buf.copy_from_slice(&value[0x20..0x50]);
            PokemonData::new(&mut buf, personality_value, ot_id)
        };
        let status_condition = u32::from_le_bytes(value[0x50..0x54].try_into().unwrap());
        let level = value[0x54];
        let mail_id = value[0x55];
        let current_hp = u16::from_le_bytes(value[0x56..0x58].try_into().unwrap());
        let total_hp = u16::from_le_bytes(value[0x58..0x5A].try_into().unwrap());
        let attack = u16::from_le_bytes(value[0x5A..0x5C].try_into().unwrap());
        let defense = u16::from_le_bytes(value[0x5C..0x5E].try_into().unwrap());
        let speed = u16::from_le_bytes(value[0x5E..0x60].try_into().unwrap());
        let sp_attack = u16::from_le_bytes(value[0x60..0x62].try_into().unwrap());
        let sp_defense = u16::from_le_bytes(value[0x62..0x64].try_into().unwrap());

        Pokemon {
            personality_value,
            ot_id,
            nickname,
            language,
            misc_flags,
            ot_name,
            markings,
            checksum,
            unknown,
            data,
            status_condition,
            level,
            mail_id,
            current_hp,
            total_hp,
            attack,
            defense,
            speed,
            sp_attack,
            sp_defense,
        }
    }
}

impl From<&Pokemon> for [u8; 100] {
    fn from(value: &Pokemon) -> Self {
        let mut buf = [0u8; 100];
        let (pokemon_data, checksum): ([u8; 48], u16) = From::from(&value.data);

        buf[0..4].copy_from_slice(&value.personality_value.to_le_bytes());
        buf[4..8].copy_from_slice(&value.ot_id.to_le_bytes());
        buf[8..18].copy_from_slice(&value.nickname);
        buf[0x12..0x14].copy_from_slice(&[value.language, value.misc_flags]);
        buf[0x14..0x1B].copy_from_slice(&value.ot_name);
        buf[0x1B] = value.markings;
        buf[0x1C..0x1E].copy_from_slice(&checksum.to_le_bytes());
        buf[0x1E..0x20].copy_from_slice(&value.unknown.to_le_bytes());

        buf[0x20..0x50].copy_from_slice(&pokemon_data);
        buf[0x50..0x54].copy_from_slice(&value.status_condition.to_le_bytes());
        buf[0x54..0x56].copy_from_slice(&[value.level, value.mail_id]);
        buf[0x56..0x58].copy_from_slice(&value.current_hp.to_le_bytes());
        buf[0x58..0x5A].copy_from_slice(&value.total_hp.to_le_bytes());
        buf[0x5A..0x5C].copy_from_slice(&value.attack.to_le_bytes());
        buf[0x5C..0x5E].copy_from_slice(&value.defense.to_le_bytes());
        buf[0x5E..0x60].copy_from_slice(&value.speed.to_le_bytes());
        buf[0x60..0x62].copy_from_slice(&value.sp_attack.to_le_bytes());
        buf[0x62..0x64].copy_from_slice(&value.sp_defense.to_le_bytes());

        buf
    }
}
