#[repr(u16)]
#[derive(Debug, Clone, Copy, Default)]
pub enum MoveId {
    #[default]
    None = 0,
    Pound = 1,
    KarateChop = 2,
    DoubleSlap = 3,
    CometPunch = 4,
    MegaPunch = 5,
    PayDay = 6,
    FirePunch = 7,
    IcePunch = 8,
    ThunderPunch = 9,
    Scratch = 10,
    ViceGrip = 11,
    Guillotine = 12,
    RazorWind = 13,
    SwordsDance = 14,
    Cut = 15,
    Gust = 16,
    WingAttack = 17,
    Whirlwind = 18,
    Fly = 19,
    Bind = 20,
    Slam = 21,
    VineWhip = 22,
    Stomp = 23,
    DoubleKick = 24,
    MegaKick = 25,
    JumpKick = 26,
    RollingKick = 27,
    SandAttack = 28,
    Headbutt = 29,
    HornAttack = 30,
    FuryAttack = 31,
    HornDrill = 32,
    Tackle = 33,
    BodySlam = 34,
    Wrap = 35,
    TakeDown = 36,
    Thrash = 37,
    DoubleEdge = 38,
    TailWhip = 39,
    PoisonSting = 40,
    Twineedle = 41,
    PinMissile = 42,
    Leer = 43,
    Bite = 44,
    Growl = 45,
    Roar = 46,
    Sing = 47,
    Supersonic = 48,
    SonicBoom = 49,
    Disable = 50,
    Acid = 51,
    Ember = 52,
    Flamethrower = 53,
    Mist = 54,
    WaterGun = 55,
    HydroPump = 56,
    Surf = 57,
    IceBeam = 58,
    Blizzard = 59,
    Psybeam = 60,
    BubbleBeam = 61,
    AuroraBeam = 62,
    HyperBeam = 63,
    Peck = 64,
    DrillPeck = 65,
    Submission = 66,
    LowKick = 67,
    Counter = 68,
    SeismicToss = 69,
    Strength = 70,
    Absorb = 71,
    MegaDrain = 72,
    LeechSeed = 73,
    Growth = 74,
    RazorLeaf = 75,
    SolarBeam = 76,
    PoisonPowder = 77,
    StunSpore = 78,
    SleepPowder = 79,
    PetalDance = 80,
    StringShot = 81,
    DragonRage = 82,
    FireSpin = 83,
    ThunderShock = 84,
    Thunderbolt = 85,
    ThunderWave = 86,
    Thunder = 87,
    RockThrow = 88,
    Earthquake = 89,
    Fissure = 90,
    Dig = 91,
    Toxic = 92,
    Confusion = 93,
    Psychic = 94,
    Hypnosis = 95,
    Meditate = 96,
    Agility = 97,
    QuickAttack = 98,
    Rage = 99,
    Teleport = 100,
    NightShade = 101,
    Mimic = 102,
    Screech = 103,
    DoubleTeam = 104,
    Recover = 105,
    Harden = 106,
    Minimize = 107,
    Smokescreen = 108,
    ConfuseRay = 109,
    Withdraw = 110,
    DefenseCurl = 111,
    Barrier = 112,
    LightScreen = 113,
    Haze = 114,
    Reflect = 115,
    FocusEnergy = 116,
    Bide = 117,
    Metronome = 118,
    MirrorMove = 119,
    SelfDestruct = 120,
    EggBomb = 121,
    Lick = 122,
    Smog = 123,
    Sludge = 124,
    BoneClub = 125,
    FireBlast = 126,
    Waterfall = 127,
    Clamp = 128,
    Swift = 129,
    SkullBash = 130,
    SpikeCannon = 131,
    Constrict = 132,
    Amnesia = 133,
    Kinesis = 134,
    SoftBoiled = 135,
    HiJumpKick = 136,
    Glare = 137,
    DreamEater = 138,
    PoisonGas = 139,
    Barrage = 140,
    LeechLife = 141,
    LovelyKiss = 142,
    SkyAttack = 143,
    Transform = 144,
    Bubble = 145,
    DizzyPunch = 146,
    Spore = 147,
    Flash = 148,
    Psywave = 149,
    Splash = 150,
    AcidArmor = 151,
    Crabhammer = 152,
    Explosion = 153,
    FurySwipes = 154,
    Bonemerang = 155,
    Rest = 156,
    RockSlide = 157,
    HyperFang = 158,
    Sharpen = 159,
    Conversion = 160,
    TriAttack = 161,
    SuperFang = 162,
    Slash = 163,
    Substitute = 164,
    Struggle = 165,
    Sketch = 166,
    TripleKick = 167,
    Thief = 168,
    SpiderWeb = 169,
    MindReader = 170,
    Nightmare = 171,
    FlameWheel = 172,
    Snore = 173,
    Curse = 174,
    Flail = 175,
    Conversion2 = 176,
    Aeroblast = 177,
    CottonSpore = 178,
    Reversal = 179,
    Spite = 180,
    PowderSnow = 181,
    Protect = 182,
    MachPunch = 183,
    ScaryFace = 184,
    FaintAttack = 185,
    SweetKiss = 186,
    BellyDrum = 187,
    SludgeBomb = 188,
    MudSlap = 189,
    Octazooka = 190,
    Spikes = 191,
    ZapCannon = 192,
    Foresight = 193,
    DestinyBond = 194,
    PerishSong = 195,
    IcyWind = 196,
    Detect = 197,
    BoneRush = 198,
    LockOn = 199,
    Outrage = 200,
    Sandstorm = 201,
    GigaDrain = 202,
    Endure = 203,
    Charm = 204,
    Rollout = 205,
    FalseSwipe = 206,
    Swagger = 207,
    MilkDrink = 208,
    Spark = 209,
    FuryCutter = 210,
    SteelWing = 211,
    MeanLook = 212,
    Attract = 213,
    SleepTalk = 214,
    HealBell = 215,
    Return = 216,
    Present = 217,
    Frustration = 218,
    Safeguard = 219,
    PainSplit = 220,
    SacredFire = 221,
    Magnitude = 222,
    DynamicPunch = 223,
    Megahorn = 224,
    DragonBreath = 225,
    BatonPass = 226,
    Encore = 227,
    Pursuit = 228,
    RapidSpin = 229,
    SweetScent = 230,
    IronTail = 231,
    MetalClaw = 232,
    VitalThrow = 233,
    MorningSun = 234,
    Synthesis = 235,
    Moonlight = 236,
    HiddenPower = 237,
    CrossChop = 238,
    Twister = 239,
    RainDance = 240,
    SunnyDay = 241,
    Crunch = 242,
    MirrorCoat = 243,
    PsychUp = 244,
    ExtremeSpeed = 245,
    AncientPower = 246,
    ShadowBall = 247,
    FutureSight = 248,
    RockSmash = 249,
    Whirlpool = 250,
    BeatUp = 251,
    FakeOut = 252,
    Uproar = 253,
    Stockpile = 254,
    SpitUp = 255,
    Swallow = 256,
    HeatWave = 257,
    Hail = 258,
    Torment = 259,
    Flatter = 260,
    WillOWisp = 261,
    Memento = 262,
    Facade = 263,
    FocusPunch = 264,
    SmellingSalt = 265,
    FollowMe = 266,
    NaturePower = 267,
    Charge = 268,
    Taunt = 269,
    HelpingHand = 270,
    Trick = 271,
    RolePlay = 272,
    Wish = 273,
    Assist = 274,
    Ingrain = 275,
    Superpower = 276,
    MagicCoat = 277,
    Recycle = 278,
    Revenge = 279,
    BrickBreak = 280,
    Yawn = 281,
    KnockOff = 282,
    Endeavor = 283,
    Eruption = 284,
    SkillSwap = 285,
    Imprison = 286,
    Refresh = 287,
    Grudge = 288,
    Snatch = 289,
    SecretPower = 290,
    Dive = 291,
    ArmThrust = 292,
    Camouflage = 293,
    TailGlow = 294,
    LusterPurge = 295,
    MistBall = 296,
    FeatherDance = 297,
    TeeterDance = 298,
    BlazeKick = 299,
    MudSport = 300,
    IceBall = 301,
    NeedleArm = 302,
    SlackOff = 303,
    HyperVoice = 304,
    PoisonFang = 305,
    CrushClaw = 306,
    BlastBurn = 307,
    HydroCannon = 308,
    MeteorMash = 309,
    Astonish = 310,
    WeatherBall = 311,
    Aromatherapy = 312,
    FakeTears = 313,
    AirCutter = 314,
    Overheat = 315,
    OdorSleuth = 316,
    RockTomb = 317,
    SilverWind = 318,
    MetalSound = 319,
    GrassWhistle = 320,
    Tickle = 321,
    CosmicPower = 322,
    WaterSpout = 323,
    SignalBeam = 324,
    ShadowPunch = 325,
    Extrasensory = 326,
    SkyUppercut = 327,
    SandTomb = 328,
    SheerCold = 329,
    MuddyWater = 330,
    BulletSeed = 331,
    AerialAce = 332,
    IcicleSpear = 333,
    IronDefense = 334,
    Block = 335,
    Howl = 336,
    DragonClaw = 337,
    FrenzyPlant = 338,
    BulkUp = 339,
    Bounce = 340,
    MudShot = 341,
    PoisonTail = 342,
    Covet = 343,
    VoltTackle = 344,
    MagicalLeaf = 345,
    WaterSport = 346,
    CalmMind = 347,
    LeafBlade = 348,
    DragonDance = 349,
    RockBlast = 350,
    ShockWave = 351,
    WaterPulse = 352,
    DoomDesire = 353,
    PsychoBoost = 354,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Move {
    pub move_id: MoveId,
    pub base_pp: u8,
}

const MOVES: &[Move] = &[
    Move {
        move_id: MoveId::None,
        base_pp: 0,
    },
    Move {
        move_id: MoveId::Pound,
        base_pp: 35,
    },
    Move {
        move_id: MoveId::KarateChop,
        base_pp: 25,
    },
    Move {
        move_id: MoveId::DoubleSlap,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::CometPunch,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::MegaPunch,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::PayDay,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::FirePunch,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::IcePunch,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::ThunderPunch,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Scratch,
        base_pp: 35,
    },
    Move {
        move_id: MoveId::ViceGrip,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::Guillotine,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::RazorWind,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::SwordsDance,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::Cut,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::Gust,
        base_pp: 35,
    },
    Move {
        move_id: MoveId::WingAttack,
        base_pp: 35,
    },
    Move {
        move_id: MoveId::Whirlwind,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Fly,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Bind,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Slam,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::VineWhip,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Stomp,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::DoubleKick,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::MegaKick,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::JumpKick,
        base_pp: 25,
    },
    Move {
        move_id: MoveId::RollingKick,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::SandAttack,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Headbutt,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::HornAttack,
        base_pp: 25,
    },
    Move {
        move_id: MoveId::FuryAttack,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::HornDrill,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Tackle,
        base_pp: 35,
    },
    Move {
        move_id: MoveId::BodySlam,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Wrap,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::TakeDown,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Thrash,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::DoubleEdge,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::TailWhip,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::PoisonSting,
        base_pp: 35,
    },
    Move {
        move_id: MoveId::Twineedle,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::PinMissile,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Leer,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::Bite,
        base_pp: 25,
    },
    Move {
        move_id: MoveId::Growl,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::Roar,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Sing,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Supersonic,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::SonicBoom,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Disable,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Acid,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::Ember,
        base_pp: 25,
    },
    Move {
        move_id: MoveId::Flamethrower,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Mist,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::WaterGun,
        base_pp: 25,
    },
    Move {
        move_id: MoveId::HydroPump,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Surf,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::IceBeam,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Blizzard,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Psybeam,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::BubbleBeam,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::AuroraBeam,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::HyperBeam,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Peck,
        base_pp: 35,
    },
    Move {
        move_id: MoveId::DrillPeck,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Submission,
        base_pp: 25,
    },
    Move {
        move_id: MoveId::LowKick,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Counter,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::SeismicToss,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Strength,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Absorb,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::MegaDrain,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::LeechSeed,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Growth,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::RazorLeaf,
        base_pp: 25,
    },
    Move {
        move_id: MoveId::SolarBeam,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::PoisonPowder,
        base_pp: 35,
    },
    Move {
        move_id: MoveId::StunSpore,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::SleepPowder,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::PetalDance,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::StringShot,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::DragonRage,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::FireSpin,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::ThunderShock,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::Thunderbolt,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::ThunderWave,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Thunder,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::RockThrow,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Earthquake,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Fissure,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Dig,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Toxic,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Confusion,
        base_pp: 25,
    },
    Move {
        move_id: MoveId::Psychic,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Hypnosis,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Meditate,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::Agility,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::QuickAttack,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::Rage,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Teleport,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::NightShade,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Mimic,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Screech,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::DoubleTeam,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Recover,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Harden,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::Minimize,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Smokescreen,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::ConfuseRay,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Withdraw,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::DefenseCurl,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::Barrier,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::LightScreen,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::Haze,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::Reflect,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::FocusEnergy,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::Bide,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Metronome,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::MirrorMove,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::SelfDestruct,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::EggBomb,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Lick,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::Smog,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Sludge,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::BoneClub,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::FireBlast,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Waterfall,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Clamp,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Swift,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::SkullBash,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::SpikeCannon,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Constrict,
        base_pp: 35,
    },
    Move {
        move_id: MoveId::Amnesia,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Kinesis,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::SoftBoiled,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::HiJumpKick,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Glare,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::DreamEater,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::PoisonGas,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::Barrage,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::LeechLife,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::LovelyKiss,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::SkyAttack,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Transform,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Bubble,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::DizzyPunch,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Spore,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Flash,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Psywave,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Splash,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::AcidArmor,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::Crabhammer,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Explosion,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::FurySwipes,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Bonemerang,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Rest,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::RockSlide,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::HyperFang,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Sharpen,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::Conversion,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::TriAttack,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::SuperFang,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Slash,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Substitute,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Struggle,
        base_pp: 1,
    },
    Move {
        move_id: MoveId::Sketch,
        base_pp: 1,
    },
    Move {
        move_id: MoveId::TripleKick,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Thief,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::SpiderWeb,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::MindReader,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Nightmare,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::FlameWheel,
        base_pp: 25,
    },
    Move {
        move_id: MoveId::Snore,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Curse,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Flail,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Conversion2,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::Aeroblast,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::CottonSpore,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::Reversal,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Spite,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::PowderSnow,
        base_pp: 25,
    },
    Move {
        move_id: MoveId::Protect,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::MachPunch,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::ScaryFace,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::FaintAttack,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::SweetKiss,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::BellyDrum,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::SludgeBomb,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::MudSlap,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Octazooka,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Spikes,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::ZapCannon,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Foresight,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::DestinyBond,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::PerishSong,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::IcyWind,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Detect,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::BoneRush,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::LockOn,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Outrage,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Sandstorm,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::GigaDrain,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Endure,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Charm,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Rollout,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::FalseSwipe,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::Swagger,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::MilkDrink,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Spark,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::FuryCutter,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::SteelWing,
        base_pp: 25,
    },
    Move {
        move_id: MoveId::MeanLook,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Attract,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::SleepTalk,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::HealBell,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Return,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Present,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Frustration,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Safeguard,
        base_pp: 25,
    },
    Move {
        move_id: MoveId::PainSplit,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::SacredFire,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Magnitude,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::DynamicPunch,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Megahorn,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::DragonBreath,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::BatonPass,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::Encore,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Pursuit,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::RapidSpin,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::SweetScent,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::IronTail,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::MetalClaw,
        base_pp: 35,
    },
    Move {
        move_id: MoveId::VitalThrow,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::MorningSun,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Synthesis,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Moonlight,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::HiddenPower,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::CrossChop,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Twister,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::RainDance,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::SunnyDay,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Crunch,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::MirrorCoat,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::PsychUp,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::ExtremeSpeed,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::AncientPower,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::ShadowBall,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::FutureSight,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::RockSmash,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Whirlpool,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::BeatUp,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::FakeOut,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Uproar,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Stockpile,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::SpitUp,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Swallow,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::HeatWave,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Hail,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Torment,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Flatter,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::WillOWisp,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Memento,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Facade,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::FocusPunch,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::SmellingSalt,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::FollowMe,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::NaturePower,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Charge,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Taunt,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::HelpingHand,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Trick,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::RolePlay,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Wish,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Assist,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Ingrain,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Superpower,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::MagicCoat,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Recycle,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Revenge,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::BrickBreak,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Yawn,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::KnockOff,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Endeavor,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Eruption,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::SkillSwap,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Imprison,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Refresh,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Grudge,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Snatch,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::SecretPower,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Dive,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::ArmThrust,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Camouflage,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::TailGlow,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::LusterPurge,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::MistBall,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::FeatherDance,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::TeeterDance,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::BlazeKick,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::MudSport,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::IceBall,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::NeedleArm,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::SlackOff,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::HyperVoice,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::PoisonFang,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::CrushClaw,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::BlastBurn,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::HydroCannon,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::MeteorMash,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Astonish,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::WeatherBall,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::Aromatherapy,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::FakeTears,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::AirCutter,
        base_pp: 25,
    },
    Move {
        move_id: MoveId::Overheat,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::OdorSleuth,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::RockTomb,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::SilverWind,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::MetalSound,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::GrassWhistle,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Tickle,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::CosmicPower,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::WaterSpout,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::SignalBeam,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::ShadowPunch,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Extrasensory,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::SkyUppercut,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::SandTomb,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::SheerCold,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::MuddyWater,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::BulletSeed,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::AerialAce,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::IcicleSpear,
        base_pp: 30,
    },
    Move {
        move_id: MoveId::IronDefense,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::Block,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::Howl,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::DragonClaw,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::FrenzyPlant,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::BulkUp,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::Bounce,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::MudShot,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::PoisonTail,
        base_pp: 25,
    },
    Move {
        move_id: MoveId::Covet,
        base_pp: 40,
    },
    Move {
        move_id: MoveId::VoltTackle,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::MagicalLeaf,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::WaterSport,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::CalmMind,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::LeafBlade,
        base_pp: 15,
    },
    Move {
        move_id: MoveId::DragonDance,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::RockBlast,
        base_pp: 10,
    },
    Move {
        move_id: MoveId::ShockWave,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::WaterPulse,
        base_pp: 20,
    },
    Move {
        move_id: MoveId::DoomDesire,
        base_pp: 5,
    },
    Move {
        move_id: MoveId::PsychoBoost,
        base_pp: 5,
    },
];

impl From<u16> for Move {
    fn from(value: u16) -> Self {
        MOVES[value as usize]
    }
}

impl Move {
    pub fn to_le_bytes(&self) -> [u8; 2] {
        (self.move_id as u16).to_le_bytes()
    }
}

impl From<u16> for MoveId {
    fn from(value: u16) -> Self {
        unsafe { std::mem::transmute(value as u16) }
    }
}

impl MoveId {
    pub fn to_le_bytes(&self) -> [u8; 2] {
        (*self as u16).to_le_bytes()
    }

    pub fn base_pp(&self) -> u8 {
        MOVES[*self as usize].base_pp
    }
}
