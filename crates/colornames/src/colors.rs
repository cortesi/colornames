use crate::COLORS;
use once_cell::sync::Lazy;
use std::collections::HashMap;
#[doc = r" Normalize a color name by lowercasing and removing whitespace"]
fn norm_name(name: &str) -> String {
    name.replace(" ", "").to_lowercase()
}
#[doc = r" A list of named colors"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    BlackBlue,
    Night,
    Charcoal,
    Oil,
    StormyGray,
    LightBlack,
    DarkSteampunk,
    BlackCat,
    Iridium,
    BlackEel,
    BlackCow,
    GrayWolf,
    VampireGray,
    IronGray,
    GrayDolphin,
    CarbonGray,
    AshGray,
    DimGray,
    DimGrey,
    NardoGray,
    CloudyGray,
    SmokeyGray,
    AlienGray,
    SonicSilver,
    PlatinumGray,
    Granite,
    Gray,
    Grey,
    BattleshipGray,
    SheetMetal,
    DarkGainsboro,
    GunmetalGray,
    ColdMetal,
    StainlessSteelGray,
    DarkGrey,
    DarkGray,
    ChromeAluminum,
    GrayCloud,
    Metal,
    Silver,
    Steampunk,
    PaleSilver,
    GearSteelGray,
    GrayGoose,
    PlatinumSilver,
    LightGrey,
    LightGray,
    SilverWhite,
    Gainsboro,
    LightSteelGray,
    WhiteSmoke,
    WhiteGray,
    Platinum,
    MetallicSilver,
    BlueGray,
    RomanSilver,
    LightSlateGrey,
    LightSlateGray,
    SlateGrey,
    SlateGray,
    RatGray,
    SlateGraniteGray,
    JetGray,
    MistBlue,
    SteelGray,
    MarbleBlue,
    SlateBlueGray,
    LightPurpleBlue,
    AzureBlue,
    EstorilBlue,
    BlueJay,
    CharcoalBlue,
    DarkBlueGray,
    DarkSlate,
    DeepSeaBlue,
    NightBlue,
    MidnightBlue,
    Navy,
    DenimDarkBlue,
    DarkBlue,
    LapisBlue,
    NewMidnightBlue,
    EarthBlue,
    CobaltBlue,
    MediumBlue,
    BlueberryBlue,
    CanaryBlue,
    Blue,
    SamcoBlue,
    BrightBlue,
    BlueOrchid,
    SapphireBlue,
    BlueEyes,
    BrightNavyBlue,
    BalloonBlue,
    RoyalBlue,
    OceanBlue,
    DarkSkyBlue,
    BlueRibbon,
    BlueDress,
    NeonBlue,
    DodgerBlue,
    GlacialBlueIce,
    SteelBlue,
    SilkBlue,
    WindowsBlue,
    BlueIvy,
    CyanBlue,
    BlueKoi,
    ColumbiaBlue,
    BabyBlue,
    CornflowerBlue,
    SkyBlueDress,
    Iceberg,
    ButterflyBlue,
    DeepSkyBlue,
    MiddayBlue,
    CrystalBlue,
    DenimBlue,
    DaySkyBlue,
    LightSkyBlue,
    SkyBlue,
    JeansBlue,
    BlueAngel,
    PastelBlue,
    LightDayBlue,
    SeaBlue,
    HeavenlyBlue,
    RobinEggBlue,
    PowderBlue,
    CoralBlue,
    LightBlue,
    LightSteelBlue,
    GulfBlue,
    PastelLightBlue,
    LavenderBlue,
    WhiteBlue,
    Lavender,
    Water,
    AliceBlue,
    GhostWhite,
    Azure,
    LightCyan,
    LightSlate,
    ElectricBlue,
    TronBlue,
    BlueZircon,
    Cyan,
    Aqua,
    BrightCyan,
    Celeste,
    BlueDiamond,
    BrightTurquoise,
    BlueLagoon,
    PaleTurquoise,
    PaleBlueLily,
    LightTeal,
    TiffanyBlue,
    BlueHosta,
    CyanOpaque,
    NorthernLightsBlue,
    BlueGreen,
    MediumAquaMarine,
    AquaSeafoamGreen,
    MagicMint,
    LightAquamarine,
    Aquamarine,
    BrightTeal,
    Turquoise,
    MediumTurquoise,
    DeepTurquoise,
    Jellyfish,
    BlueTurquoise,
    DarkTurquoise,
    MacawBlueGreen,
    LightSeaGreen,
    SeafoamGreen,
    CadetBlue,
    DeepSea,
    DarkCyan,
    TealGreen,
    Teal,
    TealBlue,
    MediumTeal,
    DarkTeal,
    DeepTeal,
    DarkSlateGray,
    DarkSlateGrey,
    Gunmetal,
    BlueMossGreen,
    BeetleGreen,
    GrayishTurquoise,
    GreenishBlue,
    AquamarineStone,
    SeaTurtleGreen,
    DullSeaGreen,
    DarkGreenBlue,
    DeepSeaGreen,
    BottleGreen,
    SeaGreen,
    ElfGreen,
    DarkMint,
    Jade,
    EarthGreen,
    ChromeGreen,
    Mint,
    Emerald,
    IsleOfManGreen,
    MediumSeaGreen,
    MetallicGreen,
    CamouflageGreen,
    SageGreen,
    HazelGreen,
    VenomGreen,
    OliveDrab,
    Olive,
    Ebony,
    DarkOliveGreen,
    MilitaryGreen,
    GreenLeaves,
    ArmyGreen,
    FernGreen,
    FallForestGreen,
    IrishGreen,
    PineGreen,
    MediumForestGreen,
    RacingGreen,
    JungleGreen,
    CactusGreen,
    ForestGreen,
    Green,
    DarkGreen,
    DeepGreen,
    DeepEmeraldGreen,
    HunterGreen,
    DarkForestGreen,
    LotusGreen,
    BroccoliGreen,
    SeaweedGreen,
    ShamrockGreen,
    GreenOnion,
    MossGreen,
    GrassGreen,
    GreenPepper,
    DarkLimeGreen,
    ParrotGreen,
    CloverGreen,
    DinosaurGreen,
    GreenSnake,
    AlienGreen,
    GreenApple,
    LimeGreen,
    PeaGreen,
    KellyGreen,
    ZombieGreen,
    GreenPeas,
    DollarBillGreen,
    FrogGreen,
    TurquoiseGreen,
    DarkSeaGreen,
    BasilGreen,
    GrayGreen,
    LightOliveGreen,
    IguanaGreen,
    CitronGreen,
    AcidGreen,
    AvocadoGreen,
    PistachioGreen,
    SaladGreen,
    YellowGreen,
    PastelGreen,
    HummingbirdGreen,
    NebulaGreen,
    StoplightGoGreen,
    NeonGreen,
    JadeGreen,
    SpringGreen,
    OceanGreen,
    LimeMintGreen,
    MediumSpringGreen,
    AquaGreen,
    EmeraldGreen,
    Lime,
    LawnGreen,
    BrightGreen,
    Chartreuse,
    YellowLawnGreen,
    AloeVeraGreen,
    DullGreenYellow,
    LemonGreen,
    GreenYellow,
    ChameleonGreen,
    NeonYellowGreen,
    YellowGreenGrosbeak,
    TeaGreen,
    SlimeGreen,
    AlgaeGreen,
    LightGreen,
    DragonGreen,
    PaleGreen,
    MintGreen,
    GreenThumb,
    OrganicBrown,
    LightJade,
    LightMintGreen,
    LightRoseGreen,
    ChromeWhite,
    HoneyDew,
    MintCream,
    LemonChiffon,
    Parchment,
    Cream,
    CreamWhite,
    LightGoldenRodYellow,
    LightYellow,
    Beige,
    WhiteYellow,
    Cornsilk,
    Blonde,
    AntiqueWhite,
    LightBeige,
    PapayaWhip,
    Champagne,
    BlanchedAlmond,
    Bisque,
    Wheat,
    Moccasin,
    Peach,
    LightOrange,
    PeachPuff,
    CoralPeach,
    NavajoWhite,
    GoldenBlonde,
    GoldenSilk,
    DarkBlonde,
    LightGold,
    Vanilla,
    TanBrown,
    DirtyWhite,
    PaleGoldenRod,
    Khaki,
    CardboardBrown,
    HarvestGold,
    SunYellow,
    CornYellow,
    PastelYellow,
    NeonYellow,
    Yellow,
    LemonYellow,
    CanaryYellow,
    BananaYellow,
    MustardYellow,
    GoldenYellow,
    BoldYellow,
    SafetyYellow,
    RubberDuckyYellow,
    Gold,
    BrightGold,
    ChromeGold,
    GoldenBrown,
    DeepYellow,
    MacaroniandCheese,
    Amber,
    Saffron,
    NeonGold,
    Beer,
    YellowOrange,
    OrangeYellow,
    Cantaloupe,
    CheeseOrange,
    Orange,
    BrownSand,
    SandyBrown,
    BrownSugar,
    CamelBrown,
    DeerBrown,
    BurlyWood,
    Tan,
    LightFrenchBeige,
    Sand,
    SoftHazel,
    Sage,
    FallLeafBrown,
    GingerBrown,
    BronzeGold,
    DarkKhaki,
    OliveGreen,
    Brass,
    CookieBrown,
    MetallicGold,
    Mustard,
    BeeYellow,
    SchoolBusYellow,
    GoldenRod,
    OrangeGold,
    Caramel,
    DarkGoldenRod,
    Cinnamon,
    Peru,
    Bronze,
    PumpkinPie,
    TigerOrange,
    Copper,
    DarkGold,
    MetallicBronze,
    DarkAlmond,
    Wood,
    KhakiBrown,
    OakBrown,
    AntiqueBronze,
    Hazel,
    DarkYellow,
    DarkMoccasin,
    KhakiGreen,
    MillenniumJade,
    DarkBeige,
    BulletShell,
    ArmyBrown,
    Sandstone,
    Taupe,
    DarkGrayishOlive,
    DarkHazelBrown,
    Mocha,
    MilkChocolate,
    GrayBrown,
    DarkCoffee,
    WesternCharcoal,
    OldBurgundy,
    RedBrown,
    BakersBrown,
    PullmanBrown,
    DarkBrown,
    SepiaBrown,
    DarkBronze,
    Coffee,
    BrownBear,
    RedDirt,
    Sepia,
    Sienna,
    SaddleBrown,
    DarkSienna,
    Sangria,
    BloodRed,
    Chestnut,
    CoralBrown,
    DeepAmber,
    ChestnutRed,
    GingerRed,
    Mahogany,
    RedGold,
    RedFox,
    DarkBisque,
    LightBrown,
    PetraGold,
    BrownRust,
    Rust,
    CopperRed,
    OrangeSalmon,
    Chocolate,
    Sedona,
    PapayaOrange,
    HalloweenOrange,
    NeonOrange,
    BrightOrange,
    FluroOrange,
    PumpkinOrange,
    SafetyOrange,
    CarrotOrange,
    DarkOrange,
    ConstructionConeOrange,
    IndianSaffron,
    SunriseOrange,
    MangoOrange,
    Coral,
    BasketBallOrange,
    LightSalmonRose,
    LightSalmon,
    PinkOrange,
    DarkSalmon,
    Tangerine,
    LightCopper,
    SalmonPink,
    Salmon,
    PeachPink,
    LightCoral,
    PastelRed,
    PinkCoral,
    BeanRed,
    ValentineRed,
    IndianRed,
    Tomato,
    ShockingOrange,
    OrangeRed,
    Red,
    NeonRed,
    ScarletRed,
    RubyRed,
    FerrariRed,
    FireEngineRed,
    LavaRed,
    LoveRed,
    Grapefruit,
    StrawberryRed,
    CherryRed,
    ChilliPepper,
    FireBrick,
    TomatoSauceRed,
    Brown,
    CarbonRed,
    Cranberry,
    SaffronRed,
    CrimsonRed,
    RedWine,
    WineRed,
    DarkRed,
    MaroonRed,
    Maroon,
    Burgundy,
    Vermilion,
    DeepRed,
    GarnetRed,
    RedBlood,
    BloodNight,
    DarkScarlet,
    ChocolateBrown,
    BlackBean,
    DarkMaroon,
    Midnight,
    PurpleLily,
    PurpleMaroon,
    PlumPie,
    PlumVelvet,
    DarkRaspberry,
    VelvetMaroon,
    RosyFinch,
    DullPurple,
    Puce,
    RoseDust,
    PastelBrown,
    RosyPink,
    RosyBrown,
    KhakiRose,
    LipstickPink,
    DuskyPink,
    PinkBrown,
    OldRose,
    DustyPink,
    PinkDaisy,
    Rose,
    DustyRose,
    SilverPink,
    GoldPink,
    RoseGold,
    DeepPeach,
    PastelOrange,
    DesertSand,
    UnbleachedSilk,
    PigPink,
    PalePink,
    Blush,
    MistyRose,
    PinkBubbleGum,
    LightRose,
    LightRed,
    RoseQuartz,
    WarmPink,
    DeepRose,
    Pink,
    LightPink,
    SoftPink,
    PowderPink,
    DonutPink,
    BabyPink,
    FlamingoPink,
    PastelPink,
    RosePink,
    CadillacPink,
    CarnationPink,
    PastelRose,
    BlushRed,
    PaleVioletRed,
    PurplePink,
    TulipPink,
    BashfulPink,
    DarkPink,
    DarkHotPink,
    HotPink,
    WatermelonPink,
    VioletRed,
    HotDeepPink,
    BrightPink,
    RedMagenta,
    DeepPink,
    NeonPink,
    ChromePink,
    NeonHotPink,
    PinkCupcake,
    RoyalPink,
    DimorphothecaMagenta,
    BarbiePink,
    PinkLemonade,
    RedPink,
    Raspberry,
    Crimson,
    BrightMaroon,
    RoseRed,
    RoguePink,
    BurntPink,
    PinkViolet,
    MagentaPink,
    MediumVioletRed,
    DarkCarnationPink,
    RaspberryPurple,
    PinkPlum,
    Orchid,
    DeepMauve,
    Violet,
    FuchsiaPink,
    BrightNeonPink,
    Magenta,
    Fuchsia,
    CrimsonPurple,
    HeliotropePurple,
    TyrianPurple,
    MediumOrchid,
    PurpleFlower,
    OrchidPurple,
    RichLilac,
    PastelViolet,
    Rosy,
    MauveTaupe,
    ViolaPurple,
    Eggplant,
    PlumPurple,
    Grape,
    PurpleNavy,
    SlateBlue,
    BlueLotus,
    Blurple,
    LightSlateBlue,
    MediumSlateBlue,
    PeriwinklePurple,
    VeryPeri,
    BrightGrape,
    BrightPurple,
    PurpleAmethyst,
    BlueMagenta,
    DarkBlurple,
    DeepPeriwinkle,
    DarkSlateBlue,
    PurpleHaze,
    PurpleIris,
    DarkPurple,
    DeepPurple,
    MidnightPurple,
    PurpleMonster,
    Indigo,
    BlueWhale,
    RebeccaPurple,
    PurpleJam,
    DarkMagenta,
    Purple,
    FrenchLilac,
    DarkOrchid,
    DarkViolet,
    PurpleViolet,
    JasminePurple,
    PurpleDaffodil,
    ClematisViolet,
    BlueViolet,
    PurpleSageBush,
    LovelyPurple,
    NeonPurple,
    PurplePlum,
    AztechPurple,
    MediumPurple,
    LightPurple,
    CrocusPurple,
    PurpleMimosa,
    PastelIndigo,
    LavenderPurple,
    RosePurple,
    Viola,
    Periwinkle,
    PaleLilac,
    Lilac,
    Mauve,
    BrightLilac,
    PurpleDragon,
    Plum,
    BlushPink,
    PastelPurple,
    BlossomPink,
    WisteriaPurple,
    PurpleThistle,
    Thistle,
    PurpleWhite,
    PeriwinklePink,
    CottonCandy,
    LavenderPinocchio,
    DarkWhite,
    AshWhite,
    WarmWhite,
    WhiteChocolate,
    CreamyWhite,
    OffWhite,
    SoftIvory,
    CosmicLatte,
    PearlWhite,
    RedWhite,
    LavenderBlush,
    Pearl,
    EggShell,
    OldLace,
    WhiteIce,
    Linen,
    SeaShell,
    BoneWhite,
    Rice,
    FloralWhite,
    Ivory,
    WhiteGold,
    LightWhite,
    Cotton,
    Snow,
    MilkWhite,
    HalfWhite,
    White,
    Rgb(u8, u8, u8),
}
#[doc = r" Maps normalized color names to array offsets"]
static NAME_MAP: Lazy<HashMap<String, usize>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for (idx, (name, _)) in COLORS.iter().enumerate() {
        m.insert(norm_name(name), idx);
    }
    m
});
#[doc = r" Array of color variants matching the order of COLORS array"]
pub static VARIANTS: &[Color] = &[
    Color::Black,
    Color::BlackBlue,
    Color::Night,
    Color::Charcoal,
    Color::Oil,
    Color::StormyGray,
    Color::LightBlack,
    Color::DarkSteampunk,
    Color::BlackCat,
    Color::Iridium,
    Color::BlackEel,
    Color::BlackCow,
    Color::GrayWolf,
    Color::VampireGray,
    Color::IronGray,
    Color::GrayDolphin,
    Color::CarbonGray,
    Color::AshGray,
    Color::DimGray,
    Color::DimGrey,
    Color::NardoGray,
    Color::CloudyGray,
    Color::SmokeyGray,
    Color::AlienGray,
    Color::SonicSilver,
    Color::PlatinumGray,
    Color::Granite,
    Color::Gray,
    Color::Grey,
    Color::BattleshipGray,
    Color::SheetMetal,
    Color::DarkGainsboro,
    Color::GunmetalGray,
    Color::ColdMetal,
    Color::StainlessSteelGray,
    Color::DarkGrey,
    Color::DarkGray,
    Color::ChromeAluminum,
    Color::GrayCloud,
    Color::Metal,
    Color::Silver,
    Color::Steampunk,
    Color::PaleSilver,
    Color::GearSteelGray,
    Color::GrayGoose,
    Color::PlatinumSilver,
    Color::LightGrey,
    Color::LightGray,
    Color::SilverWhite,
    Color::Gainsboro,
    Color::LightSteelGray,
    Color::WhiteSmoke,
    Color::WhiteGray,
    Color::Platinum,
    Color::MetallicSilver,
    Color::BlueGray,
    Color::RomanSilver,
    Color::LightSlateGrey,
    Color::LightSlateGray,
    Color::SlateGrey,
    Color::SlateGray,
    Color::RatGray,
    Color::SlateGraniteGray,
    Color::JetGray,
    Color::MistBlue,
    Color::SteelGray,
    Color::MarbleBlue,
    Color::SlateBlueGray,
    Color::LightPurpleBlue,
    Color::AzureBlue,
    Color::EstorilBlue,
    Color::BlueJay,
    Color::CharcoalBlue,
    Color::DarkBlueGray,
    Color::DarkSlate,
    Color::DeepSeaBlue,
    Color::NightBlue,
    Color::MidnightBlue,
    Color::Navy,
    Color::DenimDarkBlue,
    Color::DarkBlue,
    Color::LapisBlue,
    Color::NewMidnightBlue,
    Color::EarthBlue,
    Color::CobaltBlue,
    Color::MediumBlue,
    Color::BlueberryBlue,
    Color::CanaryBlue,
    Color::Blue,
    Color::SamcoBlue,
    Color::BrightBlue,
    Color::BlueOrchid,
    Color::SapphireBlue,
    Color::BlueEyes,
    Color::BrightNavyBlue,
    Color::BalloonBlue,
    Color::RoyalBlue,
    Color::OceanBlue,
    Color::DarkSkyBlue,
    Color::BlueRibbon,
    Color::BlueDress,
    Color::NeonBlue,
    Color::DodgerBlue,
    Color::GlacialBlueIce,
    Color::SteelBlue,
    Color::SilkBlue,
    Color::WindowsBlue,
    Color::BlueIvy,
    Color::CyanBlue,
    Color::BlueKoi,
    Color::ColumbiaBlue,
    Color::BabyBlue,
    Color::CornflowerBlue,
    Color::SkyBlueDress,
    Color::Iceberg,
    Color::ButterflyBlue,
    Color::DeepSkyBlue,
    Color::MiddayBlue,
    Color::CrystalBlue,
    Color::DenimBlue,
    Color::DaySkyBlue,
    Color::LightSkyBlue,
    Color::SkyBlue,
    Color::JeansBlue,
    Color::BlueAngel,
    Color::PastelBlue,
    Color::LightDayBlue,
    Color::SeaBlue,
    Color::HeavenlyBlue,
    Color::RobinEggBlue,
    Color::PowderBlue,
    Color::CoralBlue,
    Color::LightBlue,
    Color::LightSteelBlue,
    Color::GulfBlue,
    Color::PastelLightBlue,
    Color::LavenderBlue,
    Color::WhiteBlue,
    Color::Lavender,
    Color::Water,
    Color::AliceBlue,
    Color::GhostWhite,
    Color::Azure,
    Color::LightCyan,
    Color::LightSlate,
    Color::ElectricBlue,
    Color::TronBlue,
    Color::BlueZircon,
    Color::Cyan,
    Color::Aqua,
    Color::BrightCyan,
    Color::Celeste,
    Color::BlueDiamond,
    Color::BrightTurquoise,
    Color::BlueLagoon,
    Color::PaleTurquoise,
    Color::PaleBlueLily,
    Color::LightTeal,
    Color::TiffanyBlue,
    Color::BlueHosta,
    Color::CyanOpaque,
    Color::NorthernLightsBlue,
    Color::BlueGreen,
    Color::MediumAquaMarine,
    Color::AquaSeafoamGreen,
    Color::MagicMint,
    Color::LightAquamarine,
    Color::Aquamarine,
    Color::BrightTeal,
    Color::Turquoise,
    Color::MediumTurquoise,
    Color::DeepTurquoise,
    Color::Jellyfish,
    Color::BlueTurquoise,
    Color::DarkTurquoise,
    Color::MacawBlueGreen,
    Color::LightSeaGreen,
    Color::SeafoamGreen,
    Color::CadetBlue,
    Color::DeepSea,
    Color::DarkCyan,
    Color::TealGreen,
    Color::Teal,
    Color::TealBlue,
    Color::MediumTeal,
    Color::DarkTeal,
    Color::DeepTeal,
    Color::DarkSlateGray,
    Color::DarkSlateGrey,
    Color::Gunmetal,
    Color::BlueMossGreen,
    Color::BeetleGreen,
    Color::GrayishTurquoise,
    Color::GreenishBlue,
    Color::AquamarineStone,
    Color::SeaTurtleGreen,
    Color::DullSeaGreen,
    Color::DarkGreenBlue,
    Color::DeepSeaGreen,
    Color::BottleGreen,
    Color::SeaGreen,
    Color::ElfGreen,
    Color::DarkMint,
    Color::Jade,
    Color::EarthGreen,
    Color::ChromeGreen,
    Color::Mint,
    Color::Emerald,
    Color::IsleOfManGreen,
    Color::MediumSeaGreen,
    Color::MetallicGreen,
    Color::CamouflageGreen,
    Color::SageGreen,
    Color::HazelGreen,
    Color::VenomGreen,
    Color::OliveDrab,
    Color::Olive,
    Color::Ebony,
    Color::DarkOliveGreen,
    Color::MilitaryGreen,
    Color::GreenLeaves,
    Color::ArmyGreen,
    Color::FernGreen,
    Color::FallForestGreen,
    Color::IrishGreen,
    Color::PineGreen,
    Color::MediumForestGreen,
    Color::RacingGreen,
    Color::JungleGreen,
    Color::CactusGreen,
    Color::ForestGreen,
    Color::Green,
    Color::DarkGreen,
    Color::DeepGreen,
    Color::DeepEmeraldGreen,
    Color::HunterGreen,
    Color::DarkForestGreen,
    Color::LotusGreen,
    Color::BroccoliGreen,
    Color::SeaweedGreen,
    Color::ShamrockGreen,
    Color::GreenOnion,
    Color::MossGreen,
    Color::GrassGreen,
    Color::GreenPepper,
    Color::DarkLimeGreen,
    Color::ParrotGreen,
    Color::CloverGreen,
    Color::DinosaurGreen,
    Color::GreenSnake,
    Color::AlienGreen,
    Color::GreenApple,
    Color::LimeGreen,
    Color::PeaGreen,
    Color::KellyGreen,
    Color::ZombieGreen,
    Color::GreenPeas,
    Color::DollarBillGreen,
    Color::FrogGreen,
    Color::TurquoiseGreen,
    Color::DarkSeaGreen,
    Color::BasilGreen,
    Color::GrayGreen,
    Color::LightOliveGreen,
    Color::IguanaGreen,
    Color::CitronGreen,
    Color::AcidGreen,
    Color::AvocadoGreen,
    Color::PistachioGreen,
    Color::SaladGreen,
    Color::YellowGreen,
    Color::PastelGreen,
    Color::HummingbirdGreen,
    Color::NebulaGreen,
    Color::StoplightGoGreen,
    Color::NeonGreen,
    Color::JadeGreen,
    Color::SpringGreen,
    Color::OceanGreen,
    Color::LimeMintGreen,
    Color::MediumSpringGreen,
    Color::AquaGreen,
    Color::EmeraldGreen,
    Color::Lime,
    Color::LawnGreen,
    Color::BrightGreen,
    Color::Chartreuse,
    Color::YellowLawnGreen,
    Color::AloeVeraGreen,
    Color::DullGreenYellow,
    Color::LemonGreen,
    Color::GreenYellow,
    Color::ChameleonGreen,
    Color::NeonYellowGreen,
    Color::YellowGreenGrosbeak,
    Color::TeaGreen,
    Color::SlimeGreen,
    Color::AlgaeGreen,
    Color::LightGreen,
    Color::DragonGreen,
    Color::PaleGreen,
    Color::MintGreen,
    Color::GreenThumb,
    Color::OrganicBrown,
    Color::LightJade,
    Color::LightMintGreen,
    Color::LightRoseGreen,
    Color::ChromeWhite,
    Color::HoneyDew,
    Color::MintCream,
    Color::LemonChiffon,
    Color::Parchment,
    Color::Cream,
    Color::CreamWhite,
    Color::LightGoldenRodYellow,
    Color::LightYellow,
    Color::Beige,
    Color::WhiteYellow,
    Color::Cornsilk,
    Color::Blonde,
    Color::AntiqueWhite,
    Color::LightBeige,
    Color::PapayaWhip,
    Color::Champagne,
    Color::BlanchedAlmond,
    Color::Bisque,
    Color::Wheat,
    Color::Moccasin,
    Color::Peach,
    Color::LightOrange,
    Color::PeachPuff,
    Color::CoralPeach,
    Color::NavajoWhite,
    Color::GoldenBlonde,
    Color::GoldenSilk,
    Color::DarkBlonde,
    Color::LightGold,
    Color::Vanilla,
    Color::TanBrown,
    Color::DirtyWhite,
    Color::PaleGoldenRod,
    Color::Khaki,
    Color::CardboardBrown,
    Color::HarvestGold,
    Color::SunYellow,
    Color::CornYellow,
    Color::PastelYellow,
    Color::NeonYellow,
    Color::Yellow,
    Color::LemonYellow,
    Color::CanaryYellow,
    Color::BananaYellow,
    Color::MustardYellow,
    Color::GoldenYellow,
    Color::BoldYellow,
    Color::SafetyYellow,
    Color::RubberDuckyYellow,
    Color::Gold,
    Color::BrightGold,
    Color::ChromeGold,
    Color::GoldenBrown,
    Color::DeepYellow,
    Color::MacaroniandCheese,
    Color::Amber,
    Color::Saffron,
    Color::NeonGold,
    Color::Beer,
    Color::YellowOrange,
    Color::OrangeYellow,
    Color::Cantaloupe,
    Color::CheeseOrange,
    Color::Orange,
    Color::BrownSand,
    Color::SandyBrown,
    Color::BrownSugar,
    Color::CamelBrown,
    Color::DeerBrown,
    Color::BurlyWood,
    Color::Tan,
    Color::LightFrenchBeige,
    Color::Sand,
    Color::SoftHazel,
    Color::Sage,
    Color::FallLeafBrown,
    Color::GingerBrown,
    Color::BronzeGold,
    Color::DarkKhaki,
    Color::OliveGreen,
    Color::Brass,
    Color::CookieBrown,
    Color::MetallicGold,
    Color::Mustard,
    Color::BeeYellow,
    Color::SchoolBusYellow,
    Color::GoldenRod,
    Color::OrangeGold,
    Color::Caramel,
    Color::DarkGoldenRod,
    Color::Cinnamon,
    Color::Peru,
    Color::Bronze,
    Color::PumpkinPie,
    Color::TigerOrange,
    Color::Copper,
    Color::DarkGold,
    Color::MetallicBronze,
    Color::DarkAlmond,
    Color::Wood,
    Color::KhakiBrown,
    Color::OakBrown,
    Color::AntiqueBronze,
    Color::Hazel,
    Color::DarkYellow,
    Color::DarkMoccasin,
    Color::KhakiGreen,
    Color::MillenniumJade,
    Color::DarkBeige,
    Color::BulletShell,
    Color::ArmyBrown,
    Color::Sandstone,
    Color::Taupe,
    Color::DarkGrayishOlive,
    Color::DarkHazelBrown,
    Color::Mocha,
    Color::MilkChocolate,
    Color::GrayBrown,
    Color::DarkCoffee,
    Color::WesternCharcoal,
    Color::OldBurgundy,
    Color::RedBrown,
    Color::BakersBrown,
    Color::PullmanBrown,
    Color::DarkBrown,
    Color::SepiaBrown,
    Color::DarkBronze,
    Color::Coffee,
    Color::BrownBear,
    Color::RedDirt,
    Color::Sepia,
    Color::Sienna,
    Color::SaddleBrown,
    Color::DarkSienna,
    Color::Sangria,
    Color::BloodRed,
    Color::Chestnut,
    Color::CoralBrown,
    Color::DeepAmber,
    Color::ChestnutRed,
    Color::GingerRed,
    Color::Mahogany,
    Color::RedGold,
    Color::RedFox,
    Color::DarkBisque,
    Color::LightBrown,
    Color::PetraGold,
    Color::BrownRust,
    Color::Rust,
    Color::CopperRed,
    Color::OrangeSalmon,
    Color::Chocolate,
    Color::Sedona,
    Color::PapayaOrange,
    Color::HalloweenOrange,
    Color::NeonOrange,
    Color::BrightOrange,
    Color::FluroOrange,
    Color::PumpkinOrange,
    Color::SafetyOrange,
    Color::CarrotOrange,
    Color::DarkOrange,
    Color::ConstructionConeOrange,
    Color::IndianSaffron,
    Color::SunriseOrange,
    Color::MangoOrange,
    Color::Coral,
    Color::BasketBallOrange,
    Color::LightSalmonRose,
    Color::LightSalmon,
    Color::PinkOrange,
    Color::DarkSalmon,
    Color::Tangerine,
    Color::LightCopper,
    Color::SalmonPink,
    Color::Salmon,
    Color::PeachPink,
    Color::LightCoral,
    Color::PastelRed,
    Color::PinkCoral,
    Color::BeanRed,
    Color::ValentineRed,
    Color::IndianRed,
    Color::Tomato,
    Color::ShockingOrange,
    Color::OrangeRed,
    Color::Red,
    Color::NeonRed,
    Color::ScarletRed,
    Color::RubyRed,
    Color::FerrariRed,
    Color::FireEngineRed,
    Color::LavaRed,
    Color::LoveRed,
    Color::Grapefruit,
    Color::StrawberryRed,
    Color::CherryRed,
    Color::ChilliPepper,
    Color::FireBrick,
    Color::TomatoSauceRed,
    Color::Brown,
    Color::CarbonRed,
    Color::Cranberry,
    Color::SaffronRed,
    Color::CrimsonRed,
    Color::RedWine,
    Color::WineRed,
    Color::DarkRed,
    Color::MaroonRed,
    Color::Maroon,
    Color::Burgundy,
    Color::Vermilion,
    Color::DeepRed,
    Color::GarnetRed,
    Color::RedBlood,
    Color::BloodNight,
    Color::DarkScarlet,
    Color::ChocolateBrown,
    Color::BlackBean,
    Color::DarkMaroon,
    Color::Midnight,
    Color::PurpleLily,
    Color::PurpleMaroon,
    Color::PlumPie,
    Color::PlumVelvet,
    Color::DarkRaspberry,
    Color::VelvetMaroon,
    Color::RosyFinch,
    Color::DullPurple,
    Color::Puce,
    Color::RoseDust,
    Color::PastelBrown,
    Color::RosyPink,
    Color::RosyBrown,
    Color::KhakiRose,
    Color::LipstickPink,
    Color::DuskyPink,
    Color::PinkBrown,
    Color::OldRose,
    Color::DustyPink,
    Color::PinkDaisy,
    Color::Rose,
    Color::DustyRose,
    Color::SilverPink,
    Color::GoldPink,
    Color::RoseGold,
    Color::DeepPeach,
    Color::PastelOrange,
    Color::DesertSand,
    Color::UnbleachedSilk,
    Color::PigPink,
    Color::PalePink,
    Color::Blush,
    Color::MistyRose,
    Color::PinkBubbleGum,
    Color::LightRose,
    Color::LightRed,
    Color::RoseQuartz,
    Color::WarmPink,
    Color::DeepRose,
    Color::Pink,
    Color::LightPink,
    Color::SoftPink,
    Color::PowderPink,
    Color::DonutPink,
    Color::BabyPink,
    Color::FlamingoPink,
    Color::PastelPink,
    Color::RosePink,
    Color::CadillacPink,
    Color::CarnationPink,
    Color::PastelRose,
    Color::BlushRed,
    Color::PaleVioletRed,
    Color::PurplePink,
    Color::TulipPink,
    Color::BashfulPink,
    Color::DarkPink,
    Color::DarkHotPink,
    Color::HotPink,
    Color::WatermelonPink,
    Color::VioletRed,
    Color::HotDeepPink,
    Color::BrightPink,
    Color::RedMagenta,
    Color::DeepPink,
    Color::NeonPink,
    Color::ChromePink,
    Color::NeonHotPink,
    Color::PinkCupcake,
    Color::RoyalPink,
    Color::DimorphothecaMagenta,
    Color::BarbiePink,
    Color::PinkLemonade,
    Color::RedPink,
    Color::Raspberry,
    Color::Crimson,
    Color::BrightMaroon,
    Color::RoseRed,
    Color::RoguePink,
    Color::BurntPink,
    Color::PinkViolet,
    Color::MagentaPink,
    Color::MediumVioletRed,
    Color::DarkCarnationPink,
    Color::RaspberryPurple,
    Color::PinkPlum,
    Color::Orchid,
    Color::DeepMauve,
    Color::Violet,
    Color::FuchsiaPink,
    Color::BrightNeonPink,
    Color::Magenta,
    Color::Fuchsia,
    Color::CrimsonPurple,
    Color::HeliotropePurple,
    Color::TyrianPurple,
    Color::MediumOrchid,
    Color::PurpleFlower,
    Color::OrchidPurple,
    Color::RichLilac,
    Color::PastelViolet,
    Color::Rosy,
    Color::MauveTaupe,
    Color::ViolaPurple,
    Color::Eggplant,
    Color::PlumPurple,
    Color::Grape,
    Color::PurpleNavy,
    Color::SlateBlue,
    Color::BlueLotus,
    Color::Blurple,
    Color::LightSlateBlue,
    Color::MediumSlateBlue,
    Color::PeriwinklePurple,
    Color::VeryPeri,
    Color::BrightGrape,
    Color::BrightPurple,
    Color::PurpleAmethyst,
    Color::BlueMagenta,
    Color::DarkBlurple,
    Color::DeepPeriwinkle,
    Color::DarkSlateBlue,
    Color::PurpleHaze,
    Color::PurpleIris,
    Color::DarkPurple,
    Color::DeepPurple,
    Color::MidnightPurple,
    Color::PurpleMonster,
    Color::Indigo,
    Color::BlueWhale,
    Color::RebeccaPurple,
    Color::PurpleJam,
    Color::DarkMagenta,
    Color::Purple,
    Color::FrenchLilac,
    Color::DarkOrchid,
    Color::DarkViolet,
    Color::PurpleViolet,
    Color::JasminePurple,
    Color::PurpleDaffodil,
    Color::ClematisViolet,
    Color::BlueViolet,
    Color::PurpleSageBush,
    Color::LovelyPurple,
    Color::NeonPurple,
    Color::PurplePlum,
    Color::AztechPurple,
    Color::MediumPurple,
    Color::LightPurple,
    Color::CrocusPurple,
    Color::PurpleMimosa,
    Color::PastelIndigo,
    Color::LavenderPurple,
    Color::RosePurple,
    Color::Viola,
    Color::Periwinkle,
    Color::PaleLilac,
    Color::Lilac,
    Color::Mauve,
    Color::BrightLilac,
    Color::PurpleDragon,
    Color::Plum,
    Color::BlushPink,
    Color::PastelPurple,
    Color::BlossomPink,
    Color::WisteriaPurple,
    Color::PurpleThistle,
    Color::Thistle,
    Color::PurpleWhite,
    Color::PeriwinklePink,
    Color::CottonCandy,
    Color::LavenderPinocchio,
    Color::DarkWhite,
    Color::AshWhite,
    Color::WarmWhite,
    Color::WhiteChocolate,
    Color::CreamyWhite,
    Color::OffWhite,
    Color::SoftIvory,
    Color::CosmicLatte,
    Color::PearlWhite,
    Color::RedWhite,
    Color::LavenderBlush,
    Color::Pearl,
    Color::EggShell,
    Color::OldLace,
    Color::WhiteIce,
    Color::Linen,
    Color::SeaShell,
    Color::BoneWhite,
    Color::Rice,
    Color::FloralWhite,
    Color::Ivory,
    Color::WhiteGold,
    Color::LightWhite,
    Color::Cotton,
    Color::Snow,
    Color::MilkWhite,
    Color::HalfWhite,
    Color::White,
];
#[doc = r" Maps hex codes to array indices"]
static RGB_MAP: Lazy<HashMap<&'static str, usize>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("#000000", 0usize);
    m.insert("#040720", 1usize);
    m.insert("#0C090A", 2usize);
    m.insert("#34282C", 3usize);
    m.insert("#3B3131", 4usize);
    m.insert("#3A3B3C", 5usize);
    m.insert("#454545", 6usize);
    m.insert("#4D4D4F", 7usize);
    m.insert("#413839", 8usize);
    m.insert("#3D3C3A", 9usize);
    m.insert("#463E3F", 10usize);
    m.insert("#4C4646", 11usize);
    m.insert("#504A4B", 12usize);
    m.insert("#565051", 13usize);
    m.insert("#52595D", 14usize);
    m.insert("#5C5858", 15usize);
    m.insert("#625D5D", 16usize);
    m.insert("#666362", 17usize);
    m.insert("#696969", 18usize);
    m.insert("#696969", 19usize);
    m.insert("#686A6C", 20usize);
    m.insert("#6D6968", 21usize);
    m.insert("#726E6D", 22usize);
    m.insert("#736F6E", 23usize);
    m.insert("#757575", 24usize);
    m.insert("#797979", 25usize);
    m.insert("#837E7C", 26usize);
    m.insert("#808080", 27usize);
    m.insert("#808080", 28usize);
    m.insert("#848482", 29usize);
    m.insert("#888B90", 30usize);
    m.insert("#8C8C8C", 31usize);
    m.insert("#8D918D", 32usize);
    m.insert("#9B9A96", 33usize);
    m.insert("#99A3A3", 34usize);
    m.insert("#A9A9A9", 35usize);
    m.insert("#A9A9A9", 36usize);
    m.insert("#A8A9AD", 37usize);
    m.insert("#B6B6B4", 38usize);
    m.insert("#B6B6B6", 39usize);
    m.insert("#C0C0C0", 40usize);
    m.insert("#C9C1C1", 41usize);
    m.insert("#C9C0BB", 42usize);
    m.insert("#C0C6C7", 43usize);
    m.insert("#D1D0CE", 44usize);
    m.insert("#CECECE", 45usize);
    m.insert("#D3D3D3", 46usize);
    m.insert("#D3D3D3", 47usize);
    m.insert("#DADBDD", 48usize);
    m.insert("#DCDCDC", 49usize);
    m.insert("#E0E5E5", 50usize);
    m.insert("#F5F5F5", 51usize);
    m.insert("#EEEEEE", 52usize);
    m.insert("#E5E4E2", 53usize);
    m.insert("#BCC6CC", 54usize);
    m.insert("#98AFC7", 55usize);
    m.insert("#838996", 56usize);
    m.insert("#778899", 57usize);
    m.insert("#778899", 58usize);
    m.insert("#708090", 59usize);
    m.insert("#708090", 60usize);
    m.insert("#6D7B8D", 61usize);
    m.insert("#657383", 62usize);
    m.insert("#616D7E", 63usize);
    m.insert("#646D7E", 64usize);
    m.insert("#71797E", 65usize);
    m.insert("#566D7E", 66usize);
    m.insert("#737CA1", 67usize);
    m.insert("#728FCE", 68usize);
    m.insert("#4863A0", 69usize);
    m.insert("#2F539B", 70usize);
    m.insert("#2B547E", 71usize);
    m.insert("#36454F", 72usize);
    m.insert("#29465B", 73usize);
    m.insert("#2B3856", 74usize);
    m.insert("#123456", 75usize);
    m.insert("#151B54", 76usize);
    m.insert("#191970", 77usize);
    m.insert("#000080", 78usize);
    m.insert("#151B8D", 79usize);
    m.insert("#00008B", 80usize);
    m.insert("#15317E", 81usize);
    m.insert("#0000A0", 82usize);
    m.insert("#0000A5", 83usize);
    m.insert("#0020C2", 84usize);
    m.insert("#0000CD", 85usize);
    m.insert("#0041C2", 86usize);
    m.insert("#2916F5", 87usize);
    m.insert("#0000FF", 88usize);
    m.insert("#0002FF", 89usize);
    m.insert("#0909FF", 90usize);
    m.insert("#1F45FC", 91usize);
    m.insert("#2554C7", 92usize);
    m.insert("#1569C7", 93usize);
    m.insert("#1974D2", 94usize);
    m.insert("#2B60DE", 95usize);
    m.insert("#4169E1", 96usize);
    m.insert("#2B65EC", 97usize);
    m.insert("#0059FF", 98usize);
    m.insert("#306EFF", 99usize);
    m.insert("#157DEC", 100usize);
    m.insert("#1589FF", 101usize);
    m.insert("#1E90FF", 102usize);
    m.insert("#368BC1", 103usize);
    m.insert("#4682B4", 104usize);
    m.insert("#488AC7", 105usize);
    m.insert("#357EC7", 106usize);
    m.insert("#3090C7", 107usize);
    m.insert("#14A3C7", 108usize);
    m.insert("#659EC7", 109usize);
    m.insert("#87AFC7", 110usize);
    m.insert("#95B9C7", 111usize);
    m.insert("#6495ED", 112usize);
    m.insert("#6698FF", 113usize);
    m.insert("#56A5EC", 114usize);
    m.insert("#38ACEC", 115usize);
    m.insert("#00BFFF", 116usize);
    m.insert("#3BB9FF", 117usize);
    m.insert("#5CB3FF", 118usize);
    m.insert("#79BAEC", 119usize);
    m.insert("#82CAFF", 120usize);
    m.insert("#87CEFA", 121usize);
    m.insert("#87CEEB", 122usize);
    m.insert("#A0CFEC", 123usize);
    m.insert("#B7CEEC", 124usize);
    m.insert("#B4CFEC", 125usize);
    m.insert("#ADDFFF", 126usize);
    m.insert("#C2DFFF", 127usize);
    m.insert("#C6DEFF", 128usize);
    m.insert("#BDEDFF", 129usize);
    m.insert("#B0E0E6", 130usize);
    m.insert("#AFDCEC", 131usize);
    m.insert("#ADD8E6", 132usize);
    m.insert("#B0CFDE", 133usize);
    m.insert("#C9DFEC", 134usize);
    m.insert("#D5D6EA", 135usize);
    m.insert("#E3E4FA", 136usize);
    m.insert("#DBE9FA", 137usize);
    m.insert("#E6E6FA", 138usize);
    m.insert("#EBF4FA", 139usize);
    m.insert("#F0F8FF", 140usize);
    m.insert("#F8F8FF", 141usize);
    m.insert("#F0FFFF", 142usize);
    m.insert("#E0FFFF", 143usize);
    m.insert("#CCFFFF", 144usize);
    m.insert("#9AFEFF", 145usize);
    m.insert("#7DFDFE", 146usize);
    m.insert("#57FEFF", 147usize);
    m.insert("#00FFFF", 148usize);
    m.insert("#00FFFF", 149usize);
    m.insert("#0AFFFF", 150usize);
    m.insert("#50EBEC", 151usize);
    m.insert("#4EE2EC", 152usize);
    m.insert("#16E2F5", 153usize);
    m.insert("#8EEBEC", 154usize);
    m.insert("#AFEEEE", 155usize);
    m.insert("#CFECEC", 156usize);
    m.insert("#B3D9D9", 157usize);
    m.insert("#81D8D0", 158usize);
    m.insert("#77BFC7", 159usize);
    m.insert("#92C7C7", 160usize);
    m.insert("#78C7C7", 161usize);
    m.insert("#7BCCB5", 162usize);
    m.insert("#66CDAA", 163usize);
    m.insert("#93E9BE", 164usize);
    m.insert("#AAF0D1", 165usize);
    m.insert("#93FFE8", 166usize);
    m.insert("#7FFFD4", 167usize);
    m.insert("#01F9C6", 168usize);
    m.insert("#40E0D0", 169usize);
    m.insert("#48D1CC", 170usize);
    m.insert("#48CCCD", 171usize);
    m.insert("#46C7C7", 172usize);
    m.insert("#43C6DB", 173usize);
    m.insert("#00CED1", 174usize);
    m.insert("#43BFC7", 175usize);
    m.insert("#20B2AA", 176usize);
    m.insert("#3EA99F", 177usize);
    m.insert("#5F9EA0", 178usize);
    m.insert("#3B9C9C", 179usize);
    m.insert("#008B8B", 180usize);
    m.insert("#00827F", 181usize);
    m.insert("#008080", 182usize);
    m.insert("#007C80", 183usize);
    m.insert("#045F5F", 184usize);
    m.insert("#045D5D", 185usize);
    m.insert("#033E3E", 186usize);
    m.insert("#25383C", 187usize);
    m.insert("#25383C", 188usize);
    m.insert("#2C3539", 189usize);
    m.insert("#3C565B", 190usize);
    m.insert("#4C787E", 191usize);
    m.insert("#5E7D7E", 192usize);
    m.insert("#307D7E", 193usize);
    m.insert("#348781", 194usize);
    m.insert("#438D80", 195usize);
    m.insert("#4E8975", 196usize);
    m.insert("#1F6357", 197usize);
    m.insert("#306754", 198usize);
    m.insert("#006A4E", 199usize);
    m.insert("#2E8B57", 200usize);
    m.insert("#1B8A6B", 201usize);
    m.insert("#31906E", 202usize);
    m.insert("#00A36C", 203usize);
    m.insert("#34A56F", 204usize);
    m.insert("#1AA260", 205usize);
    m.insert("#3EB489", 206usize);
    m.insert("#50C878", 207usize);
    m.insert("#22CE83", 208usize);
    m.insert("#3CB371", 209usize);
    m.insert("#7C9D8E", 210usize);
    m.insert("#78866B", 211usize);
    m.insert("#848B79", 212usize);
    m.insert("#617C58", 213usize);
    m.insert("#728C00", 214usize);
    m.insert("#6B8E23", 215usize);
    m.insert("#808000", 216usize);
    m.insert("#555D50", 217usize);
    m.insert("#556B2F", 218usize);
    m.insert("#4E5B31", 219usize);
    m.insert("#3A5F0B", 220usize);
    m.insert("#4B5320", 221usize);
    m.insert("#667C26", 222usize);
    m.insert("#4E9258", 223usize);
    m.insert("#08A04B", 224usize);
    m.insert("#387C44", 225usize);
    m.insert("#347235", 226usize);
    m.insert("#27742C", 227usize);
    m.insert("#347C2C", 228usize);
    m.insert("#227442", 229usize);
    m.insert("#228B22", 230usize);
    m.insert("#008000", 231usize);
    m.insert("#006400", 232usize);
    m.insert("#056608", 233usize);
    m.insert("#046307", 234usize);
    m.insert("#355E3B", 235usize);
    m.insert("#254117", 236usize);
    m.insert("#004225", 237usize);
    m.insert("#026C3D", 238usize);
    m.insert("#437C17", 239usize);
    m.insert("#347C17", 240usize);
    m.insert("#6AA121", 241usize);
    m.insert("#8A9A5B", 242usize);
    m.insert("#3F9B0B", 243usize);
    m.insert("#4AA02C", 244usize);
    m.insert("#41A317", 245usize);
    m.insert("#12AD2B", 246usize);
    m.insert("#3EA055", 247usize);
    m.insert("#73A16C", 248usize);
    m.insert("#6CBB3C", 249usize);
    m.insert("#6CC417", 250usize);
    m.insert("#4CC417", 251usize);
    m.insert("#32CD32", 252usize);
    m.insert("#52D017", 253usize);
    m.insert("#4CC552", 254usize);
    m.insert("#54C571", 255usize);
    m.insert("#89C35C", 256usize);
    m.insert("#85BB65", 257usize);
    m.insert("#99C68E", 258usize);
    m.insert("#A0D6B4", 259usize);
    m.insert("#8FBC8F", 260usize);
    m.insert("#829F82", 261usize);
    m.insert("#A2AD9C", 262usize);
    m.insert("#B8BC86", 263usize);
    m.insert("#9CB071", 264usize);
    m.insert("#8FB31D", 265usize);
    m.insert("#B0BF1A", 266usize);
    m.insert("#B2C248", 267usize);
    m.insert("#9DC209", 268usize);
    m.insert("#A1C935", 269usize);
    m.insert("#9ACD32", 270usize);
    m.insert("#77DD77", 271usize);
    m.insert("#7FE817", 272usize);
    m.insert("#59E817", 273usize);
    m.insert("#57E964", 274usize);
    m.insert("#16F529", 275usize);
    m.insert("#5EFB6E", 276usize);
    m.insert("#00FF7F", 277usize);
    m.insert("#00FF80", 278usize);
    m.insert("#36F57F", 279usize);
    m.insert("#00FA9A", 280usize);
    m.insert("#12E193", 281usize);
    m.insert("#5FFB17", 282usize);
    m.insert("#00FF00", 283usize);
    m.insert("#7CFC00", 284usize);
    m.insert("#66FF00", 285usize);
    m.insert("#7FFF00", 286usize);
    m.insert("#87F717", 287usize);
    m.insert("#98F516", 288usize);
    m.insert("#B1FB17", 289usize);
    m.insert("#ADF802", 290usize);
    m.insert("#ADFF2F", 291usize);
    m.insert("#BDF516", 292usize);
    m.insert("#DAEE01", 293usize);
    m.insert("#E2F516", 294usize);
    m.insert("#CCFB5D", 295usize);
    m.insert("#BCE954", 296usize);
    m.insert("#64E986", 297usize);
    m.insert("#90EE90", 298usize);
    m.insert("#6AFB92", 299usize);
    m.insert("#98FB98", 300usize);
    m.insert("#98FF98", 301usize);
    m.insert("#B5EAAA", 302usize);
    m.insert("#E3F9A6", 303usize);
    m.insert("#C3FDB8", 304usize);
    m.insert("#C2E5D3", 305usize);
    m.insert("#DBF9DB", 306usize);
    m.insert("#E8F1D4", 307usize);
    m.insert("#F0FFF0", 308usize);
    m.insert("#F5FFFA", 309usize);
    m.insert("#FFFACD", 310usize);
    m.insert("#FFFFC2", 311usize);
    m.insert("#FFFFCC", 312usize);
    m.insert("#FFFDD0", 313usize);
    m.insert("#FAFAD2", 314usize);
    m.insert("#FFFFE0", 315usize);
    m.insert("#F5F5DC", 316usize);
    m.insert("#F2F0DF", 317usize);
    m.insert("#FFF8DC", 318usize);
    m.insert("#FBF6D9", 319usize);
    m.insert("#FAEBD7", 320usize);
    m.insert("#FFF0DB", 321usize);
    m.insert("#FFEFD5", 322usize);
    m.insert("#F7E7CE", 323usize);
    m.insert("#FFEBCD", 324usize);
    m.insert("#FFE4C4", 325usize);
    m.insert("#F5DEB3", 326usize);
    m.insert("#FFE4B5", 327usize);
    m.insert("#FFE5B4", 328usize);
    m.insert("#FED8B1", 329usize);
    m.insert("#FFDAB9", 330usize);
    m.insert("#FBD5AB", 331usize);
    m.insert("#FFDEAD", 332usize);
    m.insert("#FBE7A1", 333usize);
    m.insert("#F3E3C3", 334usize);
    m.insert("#F0E2B6", 335usize);
    m.insert("#F1E5AC", 336usize);
    m.insert("#F3E5AB", 337usize);
    m.insert("#ECE5B6", 338usize);
    m.insert("#E8E4C9", 339usize);
    m.insert("#EEE8AA", 340usize);
    m.insert("#F0E68C", 341usize);
    m.insert("#EDDA74", 342usize);
    m.insert("#EDE275", 343usize);
    m.insert("#FFE87C", 344usize);
    m.insert("#FFF380", 345usize);
    m.insert("#FAF884", 346usize);
    m.insert("#FFFF33", 347usize);
    m.insert("#FFFF00", 348usize);
    m.insert("#FEF250", 349usize);
    m.insert("#FFEF00", 350usize);
    m.insert("#F5E216", 351usize);
    m.insert("#FFDB58", 352usize);
    m.insert("#FFDF00", 353usize);
    m.insert("#F9DB24", 354usize);
    m.insert("#EED202", 355usize);
    m.insert("#FFD801", 356usize);
    m.insert("#FFD700", 357usize);
    m.insert("#FDD017", 358usize);
    m.insert("#FFCE44", 359usize);
    m.insert("#EAC117", 360usize);
    m.insert("#F6BE00", 361usize);
    m.insert("#F2BB66", 362usize);
    m.insert("#FFBF00", 363usize);
    m.insert("#FBB917", 364usize);
    m.insert("#FDBD01", 365usize);
    m.insert("#FBB117", 366usize);
    m.insert("#FFAE42", 367usize);
    m.insert("#FFAE42", 368usize);
    m.insert("#FFA62F", 369usize);
    m.insert("#FFA600", 370usize);
    m.insert("#FFA500", 371usize);
    m.insert("#EE9A4D", 372usize);
    m.insert("#F4A460", 373usize);
    m.insert("#E2A76F", 374usize);
    m.insert("#C19A6B", 375usize);
    m.insert("#E6BF83", 376usize);
    m.insert("#DEB887", 377usize);
    m.insert("#D2B48C", 378usize);
    m.insert("#C8AD7F", 379usize);
    m.insert("#C2B280", 380usize);
    m.insert("#C6BA8B", 381usize);
    m.insert("#BCB88A", 382usize);
    m.insert("#C8B560", 383usize);
    m.insert("#C9BE62", 384usize);
    m.insert("#C9AE5D", 385usize);
    m.insert("#BDB76B", 386usize);
    m.insert("#BAB86C", 387usize);
    m.insert("#B5A642", 388usize);
    m.insert("#C7A317", 389usize);
    m.insert("#D4AF37", 390usize);
    m.insert("#E1AD01", 391usize);
    m.insert("#E9AB17", 392usize);
    m.insert("#E8A317", 393usize);
    m.insert("#DAA520", 394usize);
    m.insert("#D4A017", 395usize);
    m.insert("#C68E17", 396usize);
    m.insert("#B8860B", 397usize);
    m.insert("#C58917", 398usize);
    m.insert("#CD853F", 399usize);
    m.insert("#CD7F32", 400usize);
    m.insert("#CA762B", 401usize);
    m.insert("#C88141", 402usize);
    m.insert("#B87333", 403usize);
    m.insert("#AA6C39", 404usize);
    m.insert("#A97142", 405usize);
    m.insert("#AB784E", 406usize);
    m.insert("#966F33", 407usize);
    m.insert("#906E3E", 408usize);
    m.insert("#806517", 409usize);
    m.insert("#665D1E", 410usize);
    m.insert("#8E7618", 411usize);
    m.insert("#8B8000", 412usize);
    m.insert("#827839", 413usize);
    m.insert("#8A865D", 414usize);
    m.insert("#93917C", 415usize);
    m.insert("#9F8C76", 416usize);
    m.insert("#AF9B60", 417usize);
    m.insert("#827B60", 418usize);
    m.insert("#786D5F", 419usize);
    m.insert("#483C32", 420usize);
    m.insert("#4A412A", 421usize);
    m.insert("#473810", 422usize);
    m.insert("#493D26", 423usize);
    m.insert("#513B1C", 424usize);
    m.insert("#3D3635", 425usize);
    m.insert("#3B2F2F", 426usize);
    m.insert("#49413F", 427usize);
    m.insert("#43302E", 428usize);
    m.insert("#622F22", 429usize);
    m.insert("#5C3317", 430usize);
    m.insert("#644117", 431usize);
    m.insert("#654321", 432usize);
    m.insert("#704214", 433usize);
    m.insert("#804A00", 434usize);
    m.insert("#6F4E37", 435usize);
    m.insert("#835C3B", 436usize);
    m.insert("#7F5217", 437usize);
    m.insert("#7F462C", 438usize);
    m.insert("#A0522D", 439usize);
    m.insert("#8B4513", 440usize);
    m.insert("#8A4117", 441usize);
    m.insert("#7E3817", 442usize);
    m.insert("#7E3517", 443usize);
    m.insert("#954535", 444usize);
    m.insert("#9E4638", 445usize);
    m.insert("#A05544", 446usize);
    m.insert("#C34A2C", 447usize);
    m.insert("#B83C08", 448usize);
    m.insert("#C04000", 449usize);
    m.insert("#EB5406", 450usize);
    m.insert("#C35817", 451usize);
    m.insert("#B86500", 452usize);
    m.insert("#B5651D", 453usize);
    m.insert("#B76734", 454usize);
    m.insert("#A55D35", 455usize);
    m.insert("#C36241", 456usize);
    m.insert("#CB6D51", 457usize);
    m.insert("#C47451", 458usize);
    m.insert("#D2691E", 459usize);
    m.insert("#CC6600", 460usize);
    m.insert("#E56717", 461usize);
    m.insert("#E66C2C", 462usize);
    m.insert("#FF6700", 463usize);
    m.insert("#FF5F1F", 464usize);
    m.insert("#FE632A", 465usize);
    m.insert("#F87217", 466usize);
    m.insert("#FF7900", 467usize);
    m.insert("#F88017", 468usize);
    m.insert("#FF8C00", 469usize);
    m.insert("#F87431", 470usize);
    m.insert("#FF7722", 471usize);
    m.insert("#E67451", 472usize);
    m.insert("#FF8040", 473usize);
    m.insert("#FF7F50", 474usize);
    m.insert("#F88158", 475usize);
    m.insert("#F9966B", 476usize);
    m.insert("#FFA07A", 477usize);
    m.insert("#F89880", 478usize);
    m.insert("#E9967A", 479usize);
    m.insert("#E78A61", 480usize);
    m.insert("#DA8A67", 481usize);
    m.insert("#FF8674", 482usize);
    m.insert("#FA8072", 483usize);
    m.insert("#F98B88", 484usize);
    m.insert("#F08080", 485usize);
    m.insert("#F67280", 486usize);
    m.insert("#E77471", 487usize);
    m.insert("#F75D59", 488usize);
    m.insert("#E55451", 489usize);
    m.insert("#CD5C5C", 490usize);
    m.insert("#FF6347", 491usize);
    m.insert("#E55B3C", 492usize);
    m.insert("#FF4500", 493usize);
    m.insert("#FF0000", 494usize);
    m.insert("#FD1C03", 495usize);
    m.insert("#FF2400", 496usize);
    m.insert("#F62217", 497usize);
    m.insert("#F70D1A", 498usize);
    m.insert("#F62817", 499usize);
    m.insert("#E42217", 500usize);
    m.insert("#E41B17", 501usize);
    m.insert("#DC381F", 502usize);
    m.insert("#C83F49", 503usize);
    m.insert("#C24641", 504usize);
    m.insert("#C11B17", 505usize);
    m.insert("#B22222", 506usize);
    m.insert("#B21807", 507usize);
    m.insert("#A52A2A", 508usize);
    m.insert("#A70D2A", 509usize);
    m.insert("#9F000F", 510usize);
    m.insert("#931314", 511usize);
    m.insert("#990000", 512usize);
    m.insert("#990012", 513usize);
    m.insert("#990012", 514usize);
    m.insert("#8B0000", 515usize);
    m.insert("#8F0B0B", 516usize);
    m.insert("#800000", 517usize);
    m.insert("#8C001A", 518usize);
    m.insert("#7E191B", 519usize);
    m.insert("#800517", 520usize);
    m.insert("#733635", 521usize);
    m.insert("#660000", 522usize);
    m.insert("#551606", 523usize);
    m.insert("#560319", 524usize);
    m.insert("#3F000F", 525usize);
    m.insert("#3D0C02", 526usize);
    m.insert("#2F0909", 527usize);
    m.insert("#2B1B17", 528usize);
    m.insert("#550A35", 529usize);
    m.insert("#810541", 530usize);
    m.insert("#7D0541", 531usize);
    m.insert("#7D0552", 532usize);
    m.insert("#872657", 533usize);
    m.insert("#7E354D", 534usize);
    m.insert("#7F4E52", 535usize);
    m.insert("#7F525D", 536usize);
    m.insert("#7F5A58", 537usize);
    m.insert("#997070", 538usize);
    m.insert("#B1907F", 539usize);
    m.insert("#B38481", 540usize);
    m.insert("#BC8F8F", 541usize);
    m.insert("#C5908E", 542usize);
    m.insert("#C48793", 543usize);
    m.insert("#CC7A8B", 544usize);
    m.insert("#C48189", 545usize);
    m.insert("#C08081", 546usize);
    m.insert("#D58A94", 547usize);
    m.insert("#E799A3", 548usize);
    m.insert("#E8ADAA", 549usize);
    m.insert("#C9A9A6", 550usize);
    m.insert("#C4AEAD", 551usize);
    m.insert("#E6C7C2", 552usize);
    m.insert("#ECC5C0", 553usize);
    m.insert("#FFCBA4", 554usize);
    m.insert("#F8B88B", 555usize);
    m.insert("#EDC9AF", 556usize);
    m.insert("#FFDDCA", 557usize);
    m.insert("#FDD7E4", 558usize);
    m.insert("#F2D4D7", 559usize);
    m.insert("#FFE6E8", 560usize);
    m.insert("#FFE4E1", 561usize);
    m.insert("#FFDFDD", 562usize);
    m.insert("#FBCFCD", 563usize);
    m.insert("#FFCCCB", 564usize);
    m.insert("#F7CAC9", 565usize);
    m.insert("#F6C6BD", 566usize);
    m.insert("#FBBBB9", 567usize);
    m.insert("#FFC0CB", 568usize);
    m.insert("#FFB6C1", 569usize);
    m.insert("#FFB8BF", 570usize);
    m.insert("#FFB2D0", 571usize);
    m.insert("#FAAFBE", 572usize);
    m.insert("#FAAFBA", 573usize);
    m.insert("#F9A7B0", 574usize);
    m.insert("#FEA3AA", 575usize);
    m.insert("#E7A1B0", 576usize);
    m.insert("#E38AAE", 577usize);
    m.insert("#F778A1", 578usize);
    m.insert("#E5788F", 579usize);
    m.insert("#E56E94", 580usize);
    m.insert("#DB7093", 581usize);
    m.insert("#D16587", 582usize);
    m.insert("#C25A7C", 583usize);
    m.insert("#C25283", 584usize);
    m.insert("#E75480", 585usize);
    m.insert("#F660AB", 586usize);
    m.insert("#FF69B4", 587usize);
    m.insert("#FC6C85", 588usize);
    m.insert("#F6358A", 589usize);
    m.insert("#F52887", 590usize);
    m.insert("#FF007F", 591usize);
    m.insert("#FF0080", 592usize);
    m.insert("#FF1493", 593usize);
    m.insert("#F535AA", 594usize);
    m.insert("#FF33AA", 595usize);
    m.insert("#FD349C", 596usize);
    m.insert("#E45E9D", 597usize);
    m.insert("#E759AC", 598usize);
    m.insert("#E3319D", 599usize);
    m.insert("#DA1884", 600usize);
    m.insert("#E4287C", 601usize);
    m.insert("#FA2A55", 602usize);
    m.insert("#E30B5D", 603usize);
    m.insert("#DC143C", 604usize);
    m.insert("#C32148", 605usize);
    m.insert("#C21E56", 606usize);
    m.insert("#C12869", 607usize);
    m.insert("#C12267", 608usize);
    m.insert("#CA226B", 609usize);
    m.insert("#CC338B", 610usize);
    m.insert("#C71585", 611usize);
    m.insert("#C12283", 612usize);
    m.insert("#B3446C", 613usize);
    m.insert("#B93B8F", 614usize);
    m.insert("#DA70D6", 615usize);
    m.insert("#DF73D4", 616usize);
    m.insert("#EE82EE", 617usize);
    m.insert("#FF77FF", 618usize);
    m.insert("#F433FF", 619usize);
    m.insert("#FF00FF", 620usize);
    m.insert("#FF00FF", 621usize);
    m.insert("#E238EC", 622usize);
    m.insert("#D462FF", 623usize);
    m.insert("#C45AEC", 624usize);
    m.insert("#BA55D3", 625usize);
    m.insert("#A74AC7", 626usize);
    m.insert("#B048B5", 627usize);
    m.insert("#B666D2", 628usize);
    m.insert("#D291BC", 629usize);
    m.insert("#A17188", 630usize);
    m.insert("#915F6D", 631usize);
    m.insert("#7E587E", 632usize);
    m.insert("#614051", 633usize);
    m.insert("#583759", 634usize);
    m.insert("#5E5A80", 635usize);
    m.insert("#4E5180", 636usize);
    m.insert("#6A5ACD", 637usize);
    m.insert("#6960EC", 638usize);
    m.insert("#5865F2", 639usize);
    m.insert("#736AFF", 640usize);
    m.insert("#7B68EE", 641usize);
    m.insert("#7575CF", 642usize);
    m.insert("#6667AB", 643usize);
    m.insert("#6F2DA8", 644usize);
    m.insert("#6A0DAD", 645usize);
    m.insert("#6C2DC7", 646usize);
    m.insert("#822EFF", 647usize);
    m.insert("#5539CC", 648usize);
    m.insert("#5453A6", 649usize);
    m.insert("#483D8B", 650usize);
    m.insert("#4E387E", 651usize);
    m.insert("#571B7E", 652usize);
    m.insert("#4B0150", 653usize);
    m.insert("#36013F", 654usize);
    m.insert("#2E1A47", 655usize);
    m.insert("#461B7E", 656usize);
    m.insert("#4B0082", 657usize);
    m.insert("#342D7E", 658usize);
    m.insert("#663399", 659usize);
    m.insert("#6A287E", 660usize);
    m.insert("#8B008B", 661usize);
    m.insert("#800080", 662usize);
    m.insert("#86608E", 663usize);
    m.insert("#9932CC", 664usize);
    m.insert("#9400D3", 665usize);
    m.insert("#8D38C9", 666usize);
    m.insert("#A23BEC", 667usize);
    m.insert("#B041FF", 668usize);
    m.insert("#842DCE", 669usize);
    m.insert("#8A2BE2", 670usize);
    m.insert("#7A5DC7", 671usize);
    m.insert("#7F38EC", 672usize);
    m.insert("#9D00FF", 673usize);
    m.insert("#8E35EF", 674usize);
    m.insert("#893BFF", 675usize);
    m.insert("#9370DB", 676usize);
    m.insert("#8467D7", 677usize);
    m.insert("#9172EC", 678usize);
    m.insert("#9E7BFF", 679usize);
    m.insert("#8686AF", 680usize);
    m.insert("#967BB6", 681usize);
    m.insert("#B09FCA", 682usize);
    m.insert("#C8C4DF", 683usize);
    m.insert("#CCCCFF", 684usize);
    m.insert("#DCD0FF", 685usize);
    m.insert("#C8A2C8", 686usize);
    m.insert("#E0B0FF", 687usize);
    m.insert("#D891EF", 688usize);
    m.insert("#C38EC7", 689usize);
    m.insert("#DDA0DD", 690usize);
    m.insert("#E6A9EC", 691usize);
    m.insert("#F2A2E8", 692usize);
    m.insert("#F9B7FF", 693usize);
    m.insert("#C6AEC7", 694usize);
    m.insert("#D2B9D3", 695usize);
    m.insert("#D8BFD8", 696usize);
    m.insert("#DFD3E3", 697usize);
    m.insert("#E9CFEC", 698usize);
    m.insert("#FCDFFF", 699usize);
    m.insert("#EBDDE2", 700usize);
    m.insert("#E1D9D1", 701usize);
    m.insert("#E9E4D4", 702usize);
    m.insert("#EFEBD8", 703usize);
    m.insert("#EDE6D6", 704usize);
    m.insert("#F0E9D6", 705usize);
    m.insert("#F8F0E3", 706usize);
    m.insert("#FAF0DD", 707usize);
    m.insert("#FFF8E7", 708usize);
    m.insert("#F8F6F0", 709usize);
    m.insert("#F3E8EA", 710usize);
    m.insert("#FFF0F5", 711usize);
    m.insert("#FDEEF4", 712usize);
    m.insert("#FFF9E3", 713usize);
    m.insert("#FEF0E3", 714usize);
    m.insert("#EAEEE9", 715usize);
    m.insert("#FAF0E6", 716usize);
    m.insert("#FFF5EE", 717usize);
    m.insert("#F9F6EE", 718usize);
    m.insert("#FAF5EF", 719usize);
    m.insert("#FFFAF0", 720usize);
    m.insert("#FFFFF0", 721usize);
    m.insert("#FFFFF4", 722usize);
    m.insert("#FFFFF7", 723usize);
    m.insert("#FBFBF9", 724usize);
    m.insert("#FFFAFA", 725usize);
    m.insert("#FEFCFF", 726usize);
    m.insert("#FFFEFA", 727usize);
    m.insert("#FFFFFF", 728usize);
    m
});
impl Color {
    pub fn convert_str(name: &str) -> Option<Self> {
        if let Some(hex) = name.strip_prefix('#') {
            let (r, g, b) = match hex.len() {
                6 => {
                    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                    (r, g, b)
                }
                3 => {
                    let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                    let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                    let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                    (r, g, b)
                }
                _ => return None,
            };
            let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
            RGB_MAP
                .get(hex.as_str())
                .map(|&idx| VARIANTS[idx])
                .or(Some(Color::Rgb(r, g, b)))
        } else {
            NAME_MAP.get(&norm_name(name)).map(|&idx| VARIANTS[idx])
        }
    }
    #[doc = r" Get the RGB values of a color"]
    pub fn rgb(&self) -> (u8, u8, u8) {
        let hex = self.rgb_hex();
        (
            u8::from_str_radix(&hex[1..3], 16).unwrap(),
            u8::from_str_radix(&hex[3..5], 16).unwrap(),
            u8::from_str_radix(&hex[5..7], 16).unwrap(),
        )
    }
    #[doc = r" Get the hex value of a color"]
    pub fn rgb_hex(&self) -> String {
        match self {
            Self::Rgb(r, g, b) => format!("#{:02X}{:02X}{:02X}", r, g, b),
            _ => {
                if let Some(idx) = self.offset() {
                    COLORS[idx].1.to_string()
                } else {
                    unreachable!()
                }
            }
        }
    }
    #[doc = r" Get the name of the color as a string"]
    pub fn name(&self) -> String {
        match self {
            Self::Rgb(r, g, b) => format!("rgb({}, {}, {})", r, g, b),
            _ => {
                if let Some(idx) = self.offset() {
                    COLORS[idx].0.to_string()
                } else {
                    unreachable!()
                }
            }
        }
    }
    #[doc = r" Get the offset of this color in the COLORS array"]
    fn offset(&self) -> Option<usize> {
        match self {
            Self::Black => Some(0usize),
            Self::BlackBlue => Some(1usize),
            Self::Night => Some(2usize),
            Self::Charcoal => Some(3usize),
            Self::Oil => Some(4usize),
            Self::StormyGray => Some(5usize),
            Self::LightBlack => Some(6usize),
            Self::DarkSteampunk => Some(7usize),
            Self::BlackCat => Some(8usize),
            Self::Iridium => Some(9usize),
            Self::BlackEel => Some(10usize),
            Self::BlackCow => Some(11usize),
            Self::GrayWolf => Some(12usize),
            Self::VampireGray => Some(13usize),
            Self::IronGray => Some(14usize),
            Self::GrayDolphin => Some(15usize),
            Self::CarbonGray => Some(16usize),
            Self::AshGray => Some(17usize),
            Self::DimGray => Some(18usize),
            Self::DimGrey => Some(19usize),
            Self::NardoGray => Some(20usize),
            Self::CloudyGray => Some(21usize),
            Self::SmokeyGray => Some(22usize),
            Self::AlienGray => Some(23usize),
            Self::SonicSilver => Some(24usize),
            Self::PlatinumGray => Some(25usize),
            Self::Granite => Some(26usize),
            Self::Gray => Some(27usize),
            Self::Grey => Some(28usize),
            Self::BattleshipGray => Some(29usize),
            Self::SheetMetal => Some(30usize),
            Self::DarkGainsboro => Some(31usize),
            Self::GunmetalGray => Some(32usize),
            Self::ColdMetal => Some(33usize),
            Self::StainlessSteelGray => Some(34usize),
            Self::DarkGrey => Some(35usize),
            Self::DarkGray => Some(36usize),
            Self::ChromeAluminum => Some(37usize),
            Self::GrayCloud => Some(38usize),
            Self::Metal => Some(39usize),
            Self::Silver => Some(40usize),
            Self::Steampunk => Some(41usize),
            Self::PaleSilver => Some(42usize),
            Self::GearSteelGray => Some(43usize),
            Self::GrayGoose => Some(44usize),
            Self::PlatinumSilver => Some(45usize),
            Self::LightGrey => Some(46usize),
            Self::LightGray => Some(47usize),
            Self::SilverWhite => Some(48usize),
            Self::Gainsboro => Some(49usize),
            Self::LightSteelGray => Some(50usize),
            Self::WhiteSmoke => Some(51usize),
            Self::WhiteGray => Some(52usize),
            Self::Platinum => Some(53usize),
            Self::MetallicSilver => Some(54usize),
            Self::BlueGray => Some(55usize),
            Self::RomanSilver => Some(56usize),
            Self::LightSlateGrey => Some(57usize),
            Self::LightSlateGray => Some(58usize),
            Self::SlateGrey => Some(59usize),
            Self::SlateGray => Some(60usize),
            Self::RatGray => Some(61usize),
            Self::SlateGraniteGray => Some(62usize),
            Self::JetGray => Some(63usize),
            Self::MistBlue => Some(64usize),
            Self::SteelGray => Some(65usize),
            Self::MarbleBlue => Some(66usize),
            Self::SlateBlueGray => Some(67usize),
            Self::LightPurpleBlue => Some(68usize),
            Self::AzureBlue => Some(69usize),
            Self::EstorilBlue => Some(70usize),
            Self::BlueJay => Some(71usize),
            Self::CharcoalBlue => Some(72usize),
            Self::DarkBlueGray => Some(73usize),
            Self::DarkSlate => Some(74usize),
            Self::DeepSeaBlue => Some(75usize),
            Self::NightBlue => Some(76usize),
            Self::MidnightBlue => Some(77usize),
            Self::Navy => Some(78usize),
            Self::DenimDarkBlue => Some(79usize),
            Self::DarkBlue => Some(80usize),
            Self::LapisBlue => Some(81usize),
            Self::NewMidnightBlue => Some(82usize),
            Self::EarthBlue => Some(83usize),
            Self::CobaltBlue => Some(84usize),
            Self::MediumBlue => Some(85usize),
            Self::BlueberryBlue => Some(86usize),
            Self::CanaryBlue => Some(87usize),
            Self::Blue => Some(88usize),
            Self::SamcoBlue => Some(89usize),
            Self::BrightBlue => Some(90usize),
            Self::BlueOrchid => Some(91usize),
            Self::SapphireBlue => Some(92usize),
            Self::BlueEyes => Some(93usize),
            Self::BrightNavyBlue => Some(94usize),
            Self::BalloonBlue => Some(95usize),
            Self::RoyalBlue => Some(96usize),
            Self::OceanBlue => Some(97usize),
            Self::DarkSkyBlue => Some(98usize),
            Self::BlueRibbon => Some(99usize),
            Self::BlueDress => Some(100usize),
            Self::NeonBlue => Some(101usize),
            Self::DodgerBlue => Some(102usize),
            Self::GlacialBlueIce => Some(103usize),
            Self::SteelBlue => Some(104usize),
            Self::SilkBlue => Some(105usize),
            Self::WindowsBlue => Some(106usize),
            Self::BlueIvy => Some(107usize),
            Self::CyanBlue => Some(108usize),
            Self::BlueKoi => Some(109usize),
            Self::ColumbiaBlue => Some(110usize),
            Self::BabyBlue => Some(111usize),
            Self::CornflowerBlue => Some(112usize),
            Self::SkyBlueDress => Some(113usize),
            Self::Iceberg => Some(114usize),
            Self::ButterflyBlue => Some(115usize),
            Self::DeepSkyBlue => Some(116usize),
            Self::MiddayBlue => Some(117usize),
            Self::CrystalBlue => Some(118usize),
            Self::DenimBlue => Some(119usize),
            Self::DaySkyBlue => Some(120usize),
            Self::LightSkyBlue => Some(121usize),
            Self::SkyBlue => Some(122usize),
            Self::JeansBlue => Some(123usize),
            Self::BlueAngel => Some(124usize),
            Self::PastelBlue => Some(125usize),
            Self::LightDayBlue => Some(126usize),
            Self::SeaBlue => Some(127usize),
            Self::HeavenlyBlue => Some(128usize),
            Self::RobinEggBlue => Some(129usize),
            Self::PowderBlue => Some(130usize),
            Self::CoralBlue => Some(131usize),
            Self::LightBlue => Some(132usize),
            Self::LightSteelBlue => Some(133usize),
            Self::GulfBlue => Some(134usize),
            Self::PastelLightBlue => Some(135usize),
            Self::LavenderBlue => Some(136usize),
            Self::WhiteBlue => Some(137usize),
            Self::Lavender => Some(138usize),
            Self::Water => Some(139usize),
            Self::AliceBlue => Some(140usize),
            Self::GhostWhite => Some(141usize),
            Self::Azure => Some(142usize),
            Self::LightCyan => Some(143usize),
            Self::LightSlate => Some(144usize),
            Self::ElectricBlue => Some(145usize),
            Self::TronBlue => Some(146usize),
            Self::BlueZircon => Some(147usize),
            Self::Cyan => Some(148usize),
            Self::Aqua => Some(149usize),
            Self::BrightCyan => Some(150usize),
            Self::Celeste => Some(151usize),
            Self::BlueDiamond => Some(152usize),
            Self::BrightTurquoise => Some(153usize),
            Self::BlueLagoon => Some(154usize),
            Self::PaleTurquoise => Some(155usize),
            Self::PaleBlueLily => Some(156usize),
            Self::LightTeal => Some(157usize),
            Self::TiffanyBlue => Some(158usize),
            Self::BlueHosta => Some(159usize),
            Self::CyanOpaque => Some(160usize),
            Self::NorthernLightsBlue => Some(161usize),
            Self::BlueGreen => Some(162usize),
            Self::MediumAquaMarine => Some(163usize),
            Self::AquaSeafoamGreen => Some(164usize),
            Self::MagicMint => Some(165usize),
            Self::LightAquamarine => Some(166usize),
            Self::Aquamarine => Some(167usize),
            Self::BrightTeal => Some(168usize),
            Self::Turquoise => Some(169usize),
            Self::MediumTurquoise => Some(170usize),
            Self::DeepTurquoise => Some(171usize),
            Self::Jellyfish => Some(172usize),
            Self::BlueTurquoise => Some(173usize),
            Self::DarkTurquoise => Some(174usize),
            Self::MacawBlueGreen => Some(175usize),
            Self::LightSeaGreen => Some(176usize),
            Self::SeafoamGreen => Some(177usize),
            Self::CadetBlue => Some(178usize),
            Self::DeepSea => Some(179usize),
            Self::DarkCyan => Some(180usize),
            Self::TealGreen => Some(181usize),
            Self::Teal => Some(182usize),
            Self::TealBlue => Some(183usize),
            Self::MediumTeal => Some(184usize),
            Self::DarkTeal => Some(185usize),
            Self::DeepTeal => Some(186usize),
            Self::DarkSlateGray => Some(187usize),
            Self::DarkSlateGrey => Some(188usize),
            Self::Gunmetal => Some(189usize),
            Self::BlueMossGreen => Some(190usize),
            Self::BeetleGreen => Some(191usize),
            Self::GrayishTurquoise => Some(192usize),
            Self::GreenishBlue => Some(193usize),
            Self::AquamarineStone => Some(194usize),
            Self::SeaTurtleGreen => Some(195usize),
            Self::DullSeaGreen => Some(196usize),
            Self::DarkGreenBlue => Some(197usize),
            Self::DeepSeaGreen => Some(198usize),
            Self::BottleGreen => Some(199usize),
            Self::SeaGreen => Some(200usize),
            Self::ElfGreen => Some(201usize),
            Self::DarkMint => Some(202usize),
            Self::Jade => Some(203usize),
            Self::EarthGreen => Some(204usize),
            Self::ChromeGreen => Some(205usize),
            Self::Mint => Some(206usize),
            Self::Emerald => Some(207usize),
            Self::IsleOfManGreen => Some(208usize),
            Self::MediumSeaGreen => Some(209usize),
            Self::MetallicGreen => Some(210usize),
            Self::CamouflageGreen => Some(211usize),
            Self::SageGreen => Some(212usize),
            Self::HazelGreen => Some(213usize),
            Self::VenomGreen => Some(214usize),
            Self::OliveDrab => Some(215usize),
            Self::Olive => Some(216usize),
            Self::Ebony => Some(217usize),
            Self::DarkOliveGreen => Some(218usize),
            Self::MilitaryGreen => Some(219usize),
            Self::GreenLeaves => Some(220usize),
            Self::ArmyGreen => Some(221usize),
            Self::FernGreen => Some(222usize),
            Self::FallForestGreen => Some(223usize),
            Self::IrishGreen => Some(224usize),
            Self::PineGreen => Some(225usize),
            Self::MediumForestGreen => Some(226usize),
            Self::RacingGreen => Some(227usize),
            Self::JungleGreen => Some(228usize),
            Self::CactusGreen => Some(229usize),
            Self::ForestGreen => Some(230usize),
            Self::Green => Some(231usize),
            Self::DarkGreen => Some(232usize),
            Self::DeepGreen => Some(233usize),
            Self::DeepEmeraldGreen => Some(234usize),
            Self::HunterGreen => Some(235usize),
            Self::DarkForestGreen => Some(236usize),
            Self::LotusGreen => Some(237usize),
            Self::BroccoliGreen => Some(238usize),
            Self::SeaweedGreen => Some(239usize),
            Self::ShamrockGreen => Some(240usize),
            Self::GreenOnion => Some(241usize),
            Self::MossGreen => Some(242usize),
            Self::GrassGreen => Some(243usize),
            Self::GreenPepper => Some(244usize),
            Self::DarkLimeGreen => Some(245usize),
            Self::ParrotGreen => Some(246usize),
            Self::CloverGreen => Some(247usize),
            Self::DinosaurGreen => Some(248usize),
            Self::GreenSnake => Some(249usize),
            Self::AlienGreen => Some(250usize),
            Self::GreenApple => Some(251usize),
            Self::LimeGreen => Some(252usize),
            Self::PeaGreen => Some(253usize),
            Self::KellyGreen => Some(254usize),
            Self::ZombieGreen => Some(255usize),
            Self::GreenPeas => Some(256usize),
            Self::DollarBillGreen => Some(257usize),
            Self::FrogGreen => Some(258usize),
            Self::TurquoiseGreen => Some(259usize),
            Self::DarkSeaGreen => Some(260usize),
            Self::BasilGreen => Some(261usize),
            Self::GrayGreen => Some(262usize),
            Self::LightOliveGreen => Some(263usize),
            Self::IguanaGreen => Some(264usize),
            Self::CitronGreen => Some(265usize),
            Self::AcidGreen => Some(266usize),
            Self::AvocadoGreen => Some(267usize),
            Self::PistachioGreen => Some(268usize),
            Self::SaladGreen => Some(269usize),
            Self::YellowGreen => Some(270usize),
            Self::PastelGreen => Some(271usize),
            Self::HummingbirdGreen => Some(272usize),
            Self::NebulaGreen => Some(273usize),
            Self::StoplightGoGreen => Some(274usize),
            Self::NeonGreen => Some(275usize),
            Self::JadeGreen => Some(276usize),
            Self::SpringGreen => Some(277usize),
            Self::OceanGreen => Some(278usize),
            Self::LimeMintGreen => Some(279usize),
            Self::MediumSpringGreen => Some(280usize),
            Self::AquaGreen => Some(281usize),
            Self::EmeraldGreen => Some(282usize),
            Self::Lime => Some(283usize),
            Self::LawnGreen => Some(284usize),
            Self::BrightGreen => Some(285usize),
            Self::Chartreuse => Some(286usize),
            Self::YellowLawnGreen => Some(287usize),
            Self::AloeVeraGreen => Some(288usize),
            Self::DullGreenYellow => Some(289usize),
            Self::LemonGreen => Some(290usize),
            Self::GreenYellow => Some(291usize),
            Self::ChameleonGreen => Some(292usize),
            Self::NeonYellowGreen => Some(293usize),
            Self::YellowGreenGrosbeak => Some(294usize),
            Self::TeaGreen => Some(295usize),
            Self::SlimeGreen => Some(296usize),
            Self::AlgaeGreen => Some(297usize),
            Self::LightGreen => Some(298usize),
            Self::DragonGreen => Some(299usize),
            Self::PaleGreen => Some(300usize),
            Self::MintGreen => Some(301usize),
            Self::GreenThumb => Some(302usize),
            Self::OrganicBrown => Some(303usize),
            Self::LightJade => Some(304usize),
            Self::LightMintGreen => Some(305usize),
            Self::LightRoseGreen => Some(306usize),
            Self::ChromeWhite => Some(307usize),
            Self::HoneyDew => Some(308usize),
            Self::MintCream => Some(309usize),
            Self::LemonChiffon => Some(310usize),
            Self::Parchment => Some(311usize),
            Self::Cream => Some(312usize),
            Self::CreamWhite => Some(313usize),
            Self::LightGoldenRodYellow => Some(314usize),
            Self::LightYellow => Some(315usize),
            Self::Beige => Some(316usize),
            Self::WhiteYellow => Some(317usize),
            Self::Cornsilk => Some(318usize),
            Self::Blonde => Some(319usize),
            Self::AntiqueWhite => Some(320usize),
            Self::LightBeige => Some(321usize),
            Self::PapayaWhip => Some(322usize),
            Self::Champagne => Some(323usize),
            Self::BlanchedAlmond => Some(324usize),
            Self::Bisque => Some(325usize),
            Self::Wheat => Some(326usize),
            Self::Moccasin => Some(327usize),
            Self::Peach => Some(328usize),
            Self::LightOrange => Some(329usize),
            Self::PeachPuff => Some(330usize),
            Self::CoralPeach => Some(331usize),
            Self::NavajoWhite => Some(332usize),
            Self::GoldenBlonde => Some(333usize),
            Self::GoldenSilk => Some(334usize),
            Self::DarkBlonde => Some(335usize),
            Self::LightGold => Some(336usize),
            Self::Vanilla => Some(337usize),
            Self::TanBrown => Some(338usize),
            Self::DirtyWhite => Some(339usize),
            Self::PaleGoldenRod => Some(340usize),
            Self::Khaki => Some(341usize),
            Self::CardboardBrown => Some(342usize),
            Self::HarvestGold => Some(343usize),
            Self::SunYellow => Some(344usize),
            Self::CornYellow => Some(345usize),
            Self::PastelYellow => Some(346usize),
            Self::NeonYellow => Some(347usize),
            Self::Yellow => Some(348usize),
            Self::LemonYellow => Some(349usize),
            Self::CanaryYellow => Some(350usize),
            Self::BananaYellow => Some(351usize),
            Self::MustardYellow => Some(352usize),
            Self::GoldenYellow => Some(353usize),
            Self::BoldYellow => Some(354usize),
            Self::SafetyYellow => Some(355usize),
            Self::RubberDuckyYellow => Some(356usize),
            Self::Gold => Some(357usize),
            Self::BrightGold => Some(358usize),
            Self::ChromeGold => Some(359usize),
            Self::GoldenBrown => Some(360usize),
            Self::DeepYellow => Some(361usize),
            Self::MacaroniandCheese => Some(362usize),
            Self::Amber => Some(363usize),
            Self::Saffron => Some(364usize),
            Self::NeonGold => Some(365usize),
            Self::Beer => Some(366usize),
            Self::YellowOrange => Some(367usize),
            Self::OrangeYellow => Some(368usize),
            Self::Cantaloupe => Some(369usize),
            Self::CheeseOrange => Some(370usize),
            Self::Orange => Some(371usize),
            Self::BrownSand => Some(372usize),
            Self::SandyBrown => Some(373usize),
            Self::BrownSugar => Some(374usize),
            Self::CamelBrown => Some(375usize),
            Self::DeerBrown => Some(376usize),
            Self::BurlyWood => Some(377usize),
            Self::Tan => Some(378usize),
            Self::LightFrenchBeige => Some(379usize),
            Self::Sand => Some(380usize),
            Self::SoftHazel => Some(381usize),
            Self::Sage => Some(382usize),
            Self::FallLeafBrown => Some(383usize),
            Self::GingerBrown => Some(384usize),
            Self::BronzeGold => Some(385usize),
            Self::DarkKhaki => Some(386usize),
            Self::OliveGreen => Some(387usize),
            Self::Brass => Some(388usize),
            Self::CookieBrown => Some(389usize),
            Self::MetallicGold => Some(390usize),
            Self::Mustard => Some(391usize),
            Self::BeeYellow => Some(392usize),
            Self::SchoolBusYellow => Some(393usize),
            Self::GoldenRod => Some(394usize),
            Self::OrangeGold => Some(395usize),
            Self::Caramel => Some(396usize),
            Self::DarkGoldenRod => Some(397usize),
            Self::Cinnamon => Some(398usize),
            Self::Peru => Some(399usize),
            Self::Bronze => Some(400usize),
            Self::PumpkinPie => Some(401usize),
            Self::TigerOrange => Some(402usize),
            Self::Copper => Some(403usize),
            Self::DarkGold => Some(404usize),
            Self::MetallicBronze => Some(405usize),
            Self::DarkAlmond => Some(406usize),
            Self::Wood => Some(407usize),
            Self::KhakiBrown => Some(408usize),
            Self::OakBrown => Some(409usize),
            Self::AntiqueBronze => Some(410usize),
            Self::Hazel => Some(411usize),
            Self::DarkYellow => Some(412usize),
            Self::DarkMoccasin => Some(413usize),
            Self::KhakiGreen => Some(414usize),
            Self::MillenniumJade => Some(415usize),
            Self::DarkBeige => Some(416usize),
            Self::BulletShell => Some(417usize),
            Self::ArmyBrown => Some(418usize),
            Self::Sandstone => Some(419usize),
            Self::Taupe => Some(420usize),
            Self::DarkGrayishOlive => Some(421usize),
            Self::DarkHazelBrown => Some(422usize),
            Self::Mocha => Some(423usize),
            Self::MilkChocolate => Some(424usize),
            Self::GrayBrown => Some(425usize),
            Self::DarkCoffee => Some(426usize),
            Self::WesternCharcoal => Some(427usize),
            Self::OldBurgundy => Some(428usize),
            Self::RedBrown => Some(429usize),
            Self::BakersBrown => Some(430usize),
            Self::PullmanBrown => Some(431usize),
            Self::DarkBrown => Some(432usize),
            Self::SepiaBrown => Some(433usize),
            Self::DarkBronze => Some(434usize),
            Self::Coffee => Some(435usize),
            Self::BrownBear => Some(436usize),
            Self::RedDirt => Some(437usize),
            Self::Sepia => Some(438usize),
            Self::Sienna => Some(439usize),
            Self::SaddleBrown => Some(440usize),
            Self::DarkSienna => Some(441usize),
            Self::Sangria => Some(442usize),
            Self::BloodRed => Some(443usize),
            Self::Chestnut => Some(444usize),
            Self::CoralBrown => Some(445usize),
            Self::DeepAmber => Some(446usize),
            Self::ChestnutRed => Some(447usize),
            Self::GingerRed => Some(448usize),
            Self::Mahogany => Some(449usize),
            Self::RedGold => Some(450usize),
            Self::RedFox => Some(451usize),
            Self::DarkBisque => Some(452usize),
            Self::LightBrown => Some(453usize),
            Self::PetraGold => Some(454usize),
            Self::BrownRust => Some(455usize),
            Self::Rust => Some(456usize),
            Self::CopperRed => Some(457usize),
            Self::OrangeSalmon => Some(458usize),
            Self::Chocolate => Some(459usize),
            Self::Sedona => Some(460usize),
            Self::PapayaOrange => Some(461usize),
            Self::HalloweenOrange => Some(462usize),
            Self::NeonOrange => Some(463usize),
            Self::BrightOrange => Some(464usize),
            Self::FluroOrange => Some(465usize),
            Self::PumpkinOrange => Some(466usize),
            Self::SafetyOrange => Some(467usize),
            Self::CarrotOrange => Some(468usize),
            Self::DarkOrange => Some(469usize),
            Self::ConstructionConeOrange => Some(470usize),
            Self::IndianSaffron => Some(471usize),
            Self::SunriseOrange => Some(472usize),
            Self::MangoOrange => Some(473usize),
            Self::Coral => Some(474usize),
            Self::BasketBallOrange => Some(475usize),
            Self::LightSalmonRose => Some(476usize),
            Self::LightSalmon => Some(477usize),
            Self::PinkOrange => Some(478usize),
            Self::DarkSalmon => Some(479usize),
            Self::Tangerine => Some(480usize),
            Self::LightCopper => Some(481usize),
            Self::SalmonPink => Some(482usize),
            Self::Salmon => Some(483usize),
            Self::PeachPink => Some(484usize),
            Self::LightCoral => Some(485usize),
            Self::PastelRed => Some(486usize),
            Self::PinkCoral => Some(487usize),
            Self::BeanRed => Some(488usize),
            Self::ValentineRed => Some(489usize),
            Self::IndianRed => Some(490usize),
            Self::Tomato => Some(491usize),
            Self::ShockingOrange => Some(492usize),
            Self::OrangeRed => Some(493usize),
            Self::Red => Some(494usize),
            Self::NeonRed => Some(495usize),
            Self::ScarletRed => Some(496usize),
            Self::RubyRed => Some(497usize),
            Self::FerrariRed => Some(498usize),
            Self::FireEngineRed => Some(499usize),
            Self::LavaRed => Some(500usize),
            Self::LoveRed => Some(501usize),
            Self::Grapefruit => Some(502usize),
            Self::StrawberryRed => Some(503usize),
            Self::CherryRed => Some(504usize),
            Self::ChilliPepper => Some(505usize),
            Self::FireBrick => Some(506usize),
            Self::TomatoSauceRed => Some(507usize),
            Self::Brown => Some(508usize),
            Self::CarbonRed => Some(509usize),
            Self::Cranberry => Some(510usize),
            Self::SaffronRed => Some(511usize),
            Self::CrimsonRed => Some(512usize),
            Self::RedWine => Some(513usize),
            Self::WineRed => Some(514usize),
            Self::DarkRed => Some(515usize),
            Self::MaroonRed => Some(516usize),
            Self::Maroon => Some(517usize),
            Self::Burgundy => Some(518usize),
            Self::Vermilion => Some(519usize),
            Self::DeepRed => Some(520usize),
            Self::GarnetRed => Some(521usize),
            Self::RedBlood => Some(522usize),
            Self::BloodNight => Some(523usize),
            Self::DarkScarlet => Some(524usize),
            Self::ChocolateBrown => Some(525usize),
            Self::BlackBean => Some(526usize),
            Self::DarkMaroon => Some(527usize),
            Self::Midnight => Some(528usize),
            Self::PurpleLily => Some(529usize),
            Self::PurpleMaroon => Some(530usize),
            Self::PlumPie => Some(531usize),
            Self::PlumVelvet => Some(532usize),
            Self::DarkRaspberry => Some(533usize),
            Self::VelvetMaroon => Some(534usize),
            Self::RosyFinch => Some(535usize),
            Self::DullPurple => Some(536usize),
            Self::Puce => Some(537usize),
            Self::RoseDust => Some(538usize),
            Self::PastelBrown => Some(539usize),
            Self::RosyPink => Some(540usize),
            Self::RosyBrown => Some(541usize),
            Self::KhakiRose => Some(542usize),
            Self::LipstickPink => Some(543usize),
            Self::DuskyPink => Some(544usize),
            Self::PinkBrown => Some(545usize),
            Self::OldRose => Some(546usize),
            Self::DustyPink => Some(547usize),
            Self::PinkDaisy => Some(548usize),
            Self::Rose => Some(549usize),
            Self::DustyRose => Some(550usize),
            Self::SilverPink => Some(551usize),
            Self::GoldPink => Some(552usize),
            Self::RoseGold => Some(553usize),
            Self::DeepPeach => Some(554usize),
            Self::PastelOrange => Some(555usize),
            Self::DesertSand => Some(556usize),
            Self::UnbleachedSilk => Some(557usize),
            Self::PigPink => Some(558usize),
            Self::PalePink => Some(559usize),
            Self::Blush => Some(560usize),
            Self::MistyRose => Some(561usize),
            Self::PinkBubbleGum => Some(562usize),
            Self::LightRose => Some(563usize),
            Self::LightRed => Some(564usize),
            Self::RoseQuartz => Some(565usize),
            Self::WarmPink => Some(566usize),
            Self::DeepRose => Some(567usize),
            Self::Pink => Some(568usize),
            Self::LightPink => Some(569usize),
            Self::SoftPink => Some(570usize),
            Self::PowderPink => Some(571usize),
            Self::DonutPink => Some(572usize),
            Self::BabyPink => Some(573usize),
            Self::FlamingoPink => Some(574usize),
            Self::PastelPink => Some(575usize),
            Self::RosePink => Some(576usize),
            Self::CadillacPink => Some(577usize),
            Self::CarnationPink => Some(578usize),
            Self::PastelRose => Some(579usize),
            Self::BlushRed => Some(580usize),
            Self::PaleVioletRed => Some(581usize),
            Self::PurplePink => Some(582usize),
            Self::TulipPink => Some(583usize),
            Self::BashfulPink => Some(584usize),
            Self::DarkPink => Some(585usize),
            Self::DarkHotPink => Some(586usize),
            Self::HotPink => Some(587usize),
            Self::WatermelonPink => Some(588usize),
            Self::VioletRed => Some(589usize),
            Self::HotDeepPink => Some(590usize),
            Self::BrightPink => Some(591usize),
            Self::RedMagenta => Some(592usize),
            Self::DeepPink => Some(593usize),
            Self::NeonPink => Some(594usize),
            Self::ChromePink => Some(595usize),
            Self::NeonHotPink => Some(596usize),
            Self::PinkCupcake => Some(597usize),
            Self::RoyalPink => Some(598usize),
            Self::DimorphothecaMagenta => Some(599usize),
            Self::BarbiePink => Some(600usize),
            Self::PinkLemonade => Some(601usize),
            Self::RedPink => Some(602usize),
            Self::Raspberry => Some(603usize),
            Self::Crimson => Some(604usize),
            Self::BrightMaroon => Some(605usize),
            Self::RoseRed => Some(606usize),
            Self::RoguePink => Some(607usize),
            Self::BurntPink => Some(608usize),
            Self::PinkViolet => Some(609usize),
            Self::MagentaPink => Some(610usize),
            Self::MediumVioletRed => Some(611usize),
            Self::DarkCarnationPink => Some(612usize),
            Self::RaspberryPurple => Some(613usize),
            Self::PinkPlum => Some(614usize),
            Self::Orchid => Some(615usize),
            Self::DeepMauve => Some(616usize),
            Self::Violet => Some(617usize),
            Self::FuchsiaPink => Some(618usize),
            Self::BrightNeonPink => Some(619usize),
            Self::Magenta => Some(620usize),
            Self::Fuchsia => Some(621usize),
            Self::CrimsonPurple => Some(622usize),
            Self::HeliotropePurple => Some(623usize),
            Self::TyrianPurple => Some(624usize),
            Self::MediumOrchid => Some(625usize),
            Self::PurpleFlower => Some(626usize),
            Self::OrchidPurple => Some(627usize),
            Self::RichLilac => Some(628usize),
            Self::PastelViolet => Some(629usize),
            Self::Rosy => Some(630usize),
            Self::MauveTaupe => Some(631usize),
            Self::ViolaPurple => Some(632usize),
            Self::Eggplant => Some(633usize),
            Self::PlumPurple => Some(634usize),
            Self::Grape => Some(635usize),
            Self::PurpleNavy => Some(636usize),
            Self::SlateBlue => Some(637usize),
            Self::BlueLotus => Some(638usize),
            Self::Blurple => Some(639usize),
            Self::LightSlateBlue => Some(640usize),
            Self::MediumSlateBlue => Some(641usize),
            Self::PeriwinklePurple => Some(642usize),
            Self::VeryPeri => Some(643usize),
            Self::BrightGrape => Some(644usize),
            Self::BrightPurple => Some(645usize),
            Self::PurpleAmethyst => Some(646usize),
            Self::BlueMagenta => Some(647usize),
            Self::DarkBlurple => Some(648usize),
            Self::DeepPeriwinkle => Some(649usize),
            Self::DarkSlateBlue => Some(650usize),
            Self::PurpleHaze => Some(651usize),
            Self::PurpleIris => Some(652usize),
            Self::DarkPurple => Some(653usize),
            Self::DeepPurple => Some(654usize),
            Self::MidnightPurple => Some(655usize),
            Self::PurpleMonster => Some(656usize),
            Self::Indigo => Some(657usize),
            Self::BlueWhale => Some(658usize),
            Self::RebeccaPurple => Some(659usize),
            Self::PurpleJam => Some(660usize),
            Self::DarkMagenta => Some(661usize),
            Self::Purple => Some(662usize),
            Self::FrenchLilac => Some(663usize),
            Self::DarkOrchid => Some(664usize),
            Self::DarkViolet => Some(665usize),
            Self::PurpleViolet => Some(666usize),
            Self::JasminePurple => Some(667usize),
            Self::PurpleDaffodil => Some(668usize),
            Self::ClematisViolet => Some(669usize),
            Self::BlueViolet => Some(670usize),
            Self::PurpleSageBush => Some(671usize),
            Self::LovelyPurple => Some(672usize),
            Self::NeonPurple => Some(673usize),
            Self::PurplePlum => Some(674usize),
            Self::AztechPurple => Some(675usize),
            Self::MediumPurple => Some(676usize),
            Self::LightPurple => Some(677usize),
            Self::CrocusPurple => Some(678usize),
            Self::PurpleMimosa => Some(679usize),
            Self::PastelIndigo => Some(680usize),
            Self::LavenderPurple => Some(681usize),
            Self::RosePurple => Some(682usize),
            Self::Viola => Some(683usize),
            Self::Periwinkle => Some(684usize),
            Self::PaleLilac => Some(685usize),
            Self::Lilac => Some(686usize),
            Self::Mauve => Some(687usize),
            Self::BrightLilac => Some(688usize),
            Self::PurpleDragon => Some(689usize),
            Self::Plum => Some(690usize),
            Self::BlushPink => Some(691usize),
            Self::PastelPurple => Some(692usize),
            Self::BlossomPink => Some(693usize),
            Self::WisteriaPurple => Some(694usize),
            Self::PurpleThistle => Some(695usize),
            Self::Thistle => Some(696usize),
            Self::PurpleWhite => Some(697usize),
            Self::PeriwinklePink => Some(698usize),
            Self::CottonCandy => Some(699usize),
            Self::LavenderPinocchio => Some(700usize),
            Self::DarkWhite => Some(701usize),
            Self::AshWhite => Some(702usize),
            Self::WarmWhite => Some(703usize),
            Self::WhiteChocolate => Some(704usize),
            Self::CreamyWhite => Some(705usize),
            Self::OffWhite => Some(706usize),
            Self::SoftIvory => Some(707usize),
            Self::CosmicLatte => Some(708usize),
            Self::PearlWhite => Some(709usize),
            Self::RedWhite => Some(710usize),
            Self::LavenderBlush => Some(711usize),
            Self::Pearl => Some(712usize),
            Self::EggShell => Some(713usize),
            Self::OldLace => Some(714usize),
            Self::WhiteIce => Some(715usize),
            Self::Linen => Some(716usize),
            Self::SeaShell => Some(717usize),
            Self::BoneWhite => Some(718usize),
            Self::Rice => Some(719usize),
            Self::FloralWhite => Some(720usize),
            Self::Ivory => Some(721usize),
            Self::WhiteGold => Some(722usize),
            Self::LightWhite => Some(723usize),
            Self::Cotton => Some(724usize),
            Self::Snow => Some(725usize),
            Self::MilkWhite => Some(726usize),
            Self::HalfWhite => Some(727usize),
            Self::White => Some(728usize),
            Self::Rgb(_, _, _) => None,
        }
    }
}
impl TryFrom<&str> for Color {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::convert_str(s).ok_or_else(|| format!("invalid color: {}", s))
    }
}
impl TryFrom<String> for Color {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}
impl TryFrom<&String> for Color {
    type Error = String;
    fn try_from(s: &String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}
