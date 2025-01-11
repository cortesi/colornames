use crate::COLORS;
use once_cell::sync::Lazy;
use std::collections::HashMap;
#[doc = r" Normalize a color name by lowercasing and removing whitespace"]
fn norm_name(name: &str) -> String {
    name.replace(" ", "").to_lowercase()
}
#[doc = r" An enum of named colors, with a catch-all Rgb variant"]
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
static VARIANTS: &[Color] = &[
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
    m.insert("#000000", 0);
    m.insert("#040720", 1);
    m.insert("#0C090A", 2);
    m.insert("#34282C", 3);
    m.insert("#3B3131", 4);
    m.insert("#3A3B3C", 5);
    m.insert("#454545", 6);
    m.insert("#4D4D4F", 7);
    m.insert("#413839", 8);
    m.insert("#3D3C3A", 9);
    m.insert("#463E3F", 10);
    m.insert("#4C4646", 11);
    m.insert("#504A4B", 12);
    m.insert("#565051", 13);
    m.insert("#52595D", 14);
    m.insert("#5C5858", 15);
    m.insert("#625D5D", 16);
    m.insert("#666362", 17);
    m.insert("#696969", 18);
    m.insert("#696969", 19);
    m.insert("#686A6C", 20);
    m.insert("#6D6968", 21);
    m.insert("#726E6D", 22);
    m.insert("#736F6E", 23);
    m.insert("#757575", 24);
    m.insert("#797979", 25);
    m.insert("#837E7C", 26);
    m.insert("#808080", 27);
    m.insert("#808080", 28);
    m.insert("#848482", 29);
    m.insert("#888B90", 30);
    m.insert("#8C8C8C", 31);
    m.insert("#8D918D", 32);
    m.insert("#9B9A96", 33);
    m.insert("#99A3A3", 34);
    m.insert("#A9A9A9", 35);
    m.insert("#A9A9A9", 36);
    m.insert("#A8A9AD", 37);
    m.insert("#B6B6B4", 38);
    m.insert("#B6B6B6", 39);
    m.insert("#C0C0C0", 40);
    m.insert("#C9C1C1", 41);
    m.insert("#C9C0BB", 42);
    m.insert("#C0C6C7", 43);
    m.insert("#D1D0CE", 44);
    m.insert("#CECECE", 45);
    m.insert("#D3D3D3", 46);
    m.insert("#D3D3D3", 47);
    m.insert("#DADBDD", 48);
    m.insert("#DCDCDC", 49);
    m.insert("#E0E5E5", 50);
    m.insert("#F5F5F5", 51);
    m.insert("#EEEEEE", 52);
    m.insert("#E5E4E2", 53);
    m.insert("#BCC6CC", 54);
    m.insert("#98AFC7", 55);
    m.insert("#838996", 56);
    m.insert("#778899", 57);
    m.insert("#778899", 58);
    m.insert("#708090", 59);
    m.insert("#708090", 60);
    m.insert("#6D7B8D", 61);
    m.insert("#657383", 62);
    m.insert("#616D7E", 63);
    m.insert("#646D7E", 64);
    m.insert("#71797E", 65);
    m.insert("#566D7E", 66);
    m.insert("#737CA1", 67);
    m.insert("#728FCE", 68);
    m.insert("#4863A0", 69);
    m.insert("#2F539B", 70);
    m.insert("#2B547E", 71);
    m.insert("#36454F", 72);
    m.insert("#29465B", 73);
    m.insert("#2B3856", 74);
    m.insert("#123456", 75);
    m.insert("#151B54", 76);
    m.insert("#191970", 77);
    m.insert("#000080", 78);
    m.insert("#151B8D", 79);
    m.insert("#00008B", 80);
    m.insert("#15317E", 81);
    m.insert("#0000A0", 82);
    m.insert("#0000A5", 83);
    m.insert("#0020C2", 84);
    m.insert("#0000CD", 85);
    m.insert("#0041C2", 86);
    m.insert("#2916F5", 87);
    m.insert("#0000FF", 88);
    m.insert("#0002FF", 89);
    m.insert("#0909FF", 90);
    m.insert("#1F45FC", 91);
    m.insert("#2554C7", 92);
    m.insert("#1569C7", 93);
    m.insert("#1974D2", 94);
    m.insert("#2B60DE", 95);
    m.insert("#4169E1", 96);
    m.insert("#2B65EC", 97);
    m.insert("#0059FF", 98);
    m.insert("#306EFF", 99);
    m.insert("#157DEC", 100);
    m.insert("#1589FF", 101);
    m.insert("#1E90FF", 102);
    m.insert("#368BC1", 103);
    m.insert("#4682B4", 104);
    m.insert("#488AC7", 105);
    m.insert("#357EC7", 106);
    m.insert("#3090C7", 107);
    m.insert("#14A3C7", 108);
    m.insert("#659EC7", 109);
    m.insert("#87AFC7", 110);
    m.insert("#95B9C7", 111);
    m.insert("#6495ED", 112);
    m.insert("#6698FF", 113);
    m.insert("#56A5EC", 114);
    m.insert("#38ACEC", 115);
    m.insert("#00BFFF", 116);
    m.insert("#3BB9FF", 117);
    m.insert("#5CB3FF", 118);
    m.insert("#79BAEC", 119);
    m.insert("#82CAFF", 120);
    m.insert("#87CEFA", 121);
    m.insert("#87CEEB", 122);
    m.insert("#A0CFEC", 123);
    m.insert("#B7CEEC", 124);
    m.insert("#B4CFEC", 125);
    m.insert("#ADDFFF", 126);
    m.insert("#C2DFFF", 127);
    m.insert("#C6DEFF", 128);
    m.insert("#BDEDFF", 129);
    m.insert("#B0E0E6", 130);
    m.insert("#AFDCEC", 131);
    m.insert("#ADD8E6", 132);
    m.insert("#B0CFDE", 133);
    m.insert("#C9DFEC", 134);
    m.insert("#D5D6EA", 135);
    m.insert("#E3E4FA", 136);
    m.insert("#DBE9FA", 137);
    m.insert("#E6E6FA", 138);
    m.insert("#EBF4FA", 139);
    m.insert("#F0F8FF", 140);
    m.insert("#F8F8FF", 141);
    m.insert("#F0FFFF", 142);
    m.insert("#E0FFFF", 143);
    m.insert("#CCFFFF", 144);
    m.insert("#9AFEFF", 145);
    m.insert("#7DFDFE", 146);
    m.insert("#57FEFF", 147);
    m.insert("#00FFFF", 148);
    m.insert("#00FFFF", 149);
    m.insert("#0AFFFF", 150);
    m.insert("#50EBEC", 151);
    m.insert("#4EE2EC", 152);
    m.insert("#16E2F5", 153);
    m.insert("#8EEBEC", 154);
    m.insert("#AFEEEE", 155);
    m.insert("#CFECEC", 156);
    m.insert("#B3D9D9", 157);
    m.insert("#81D8D0", 158);
    m.insert("#77BFC7", 159);
    m.insert("#92C7C7", 160);
    m.insert("#78C7C7", 161);
    m.insert("#7BCCB5", 162);
    m.insert("#66CDAA", 163);
    m.insert("#93E9BE", 164);
    m.insert("#AAF0D1", 165);
    m.insert("#93FFE8", 166);
    m.insert("#7FFFD4", 167);
    m.insert("#01F9C6", 168);
    m.insert("#40E0D0", 169);
    m.insert("#48D1CC", 170);
    m.insert("#48CCCD", 171);
    m.insert("#46C7C7", 172);
    m.insert("#43C6DB", 173);
    m.insert("#00CED1", 174);
    m.insert("#43BFC7", 175);
    m.insert("#20B2AA", 176);
    m.insert("#3EA99F", 177);
    m.insert("#5F9EA0", 178);
    m.insert("#3B9C9C", 179);
    m.insert("#008B8B", 180);
    m.insert("#00827F", 181);
    m.insert("#008080", 182);
    m.insert("#007C80", 183);
    m.insert("#045F5F", 184);
    m.insert("#045D5D", 185);
    m.insert("#033E3E", 186);
    m.insert("#25383C", 187);
    m.insert("#25383C", 188);
    m.insert("#2C3539", 189);
    m.insert("#3C565B", 190);
    m.insert("#4C787E", 191);
    m.insert("#5E7D7E", 192);
    m.insert("#307D7E", 193);
    m.insert("#348781", 194);
    m.insert("#438D80", 195);
    m.insert("#4E8975", 196);
    m.insert("#1F6357", 197);
    m.insert("#306754", 198);
    m.insert("#006A4E", 199);
    m.insert("#2E8B57", 200);
    m.insert("#1B8A6B", 201);
    m.insert("#31906E", 202);
    m.insert("#00A36C", 203);
    m.insert("#34A56F", 204);
    m.insert("#1AA260", 205);
    m.insert("#3EB489", 206);
    m.insert("#50C878", 207);
    m.insert("#22CE83", 208);
    m.insert("#3CB371", 209);
    m.insert("#7C9D8E", 210);
    m.insert("#78866B", 211);
    m.insert("#848B79", 212);
    m.insert("#617C58", 213);
    m.insert("#728C00", 214);
    m.insert("#6B8E23", 215);
    m.insert("#808000", 216);
    m.insert("#555D50", 217);
    m.insert("#556B2F", 218);
    m.insert("#4E5B31", 219);
    m.insert("#3A5F0B", 220);
    m.insert("#4B5320", 221);
    m.insert("#667C26", 222);
    m.insert("#4E9258", 223);
    m.insert("#08A04B", 224);
    m.insert("#387C44", 225);
    m.insert("#347235", 226);
    m.insert("#27742C", 227);
    m.insert("#347C2C", 228);
    m.insert("#227442", 229);
    m.insert("#228B22", 230);
    m.insert("#008000", 231);
    m.insert("#006400", 232);
    m.insert("#056608", 233);
    m.insert("#046307", 234);
    m.insert("#355E3B", 235);
    m.insert("#254117", 236);
    m.insert("#004225", 237);
    m.insert("#026C3D", 238);
    m.insert("#437C17", 239);
    m.insert("#347C17", 240);
    m.insert("#6AA121", 241);
    m.insert("#8A9A5B", 242);
    m.insert("#3F9B0B", 243);
    m.insert("#4AA02C", 244);
    m.insert("#41A317", 245);
    m.insert("#12AD2B", 246);
    m.insert("#3EA055", 247);
    m.insert("#73A16C", 248);
    m.insert("#6CBB3C", 249);
    m.insert("#6CC417", 250);
    m.insert("#4CC417", 251);
    m.insert("#32CD32", 252);
    m.insert("#52D017", 253);
    m.insert("#4CC552", 254);
    m.insert("#54C571", 255);
    m.insert("#89C35C", 256);
    m.insert("#85BB65", 257);
    m.insert("#99C68E", 258);
    m.insert("#A0D6B4", 259);
    m.insert("#8FBC8F", 260);
    m.insert("#829F82", 261);
    m.insert("#A2AD9C", 262);
    m.insert("#B8BC86", 263);
    m.insert("#9CB071", 264);
    m.insert("#8FB31D", 265);
    m.insert("#B0BF1A", 266);
    m.insert("#B2C248", 267);
    m.insert("#9DC209", 268);
    m.insert("#A1C935", 269);
    m.insert("#9ACD32", 270);
    m.insert("#77DD77", 271);
    m.insert("#7FE817", 272);
    m.insert("#59E817", 273);
    m.insert("#57E964", 274);
    m.insert("#16F529", 275);
    m.insert("#5EFB6E", 276);
    m.insert("#00FF7F", 277);
    m.insert("#00FF80", 278);
    m.insert("#36F57F", 279);
    m.insert("#00FA9A", 280);
    m.insert("#12E193", 281);
    m.insert("#5FFB17", 282);
    m.insert("#00FF00", 283);
    m.insert("#7CFC00", 284);
    m.insert("#66FF00", 285);
    m.insert("#7FFF00", 286);
    m.insert("#87F717", 287);
    m.insert("#98F516", 288);
    m.insert("#B1FB17", 289);
    m.insert("#ADF802", 290);
    m.insert("#ADFF2F", 291);
    m.insert("#BDF516", 292);
    m.insert("#DAEE01", 293);
    m.insert("#E2F516", 294);
    m.insert("#CCFB5D", 295);
    m.insert("#BCE954", 296);
    m.insert("#64E986", 297);
    m.insert("#90EE90", 298);
    m.insert("#6AFB92", 299);
    m.insert("#98FB98", 300);
    m.insert("#98FF98", 301);
    m.insert("#B5EAAA", 302);
    m.insert("#E3F9A6", 303);
    m.insert("#C3FDB8", 304);
    m.insert("#C2E5D3", 305);
    m.insert("#DBF9DB", 306);
    m.insert("#E8F1D4", 307);
    m.insert("#F0FFF0", 308);
    m.insert("#F5FFFA", 309);
    m.insert("#FFFACD", 310);
    m.insert("#FFFFC2", 311);
    m.insert("#FFFFCC", 312);
    m.insert("#FFFDD0", 313);
    m.insert("#FAFAD2", 314);
    m.insert("#FFFFE0", 315);
    m.insert("#F5F5DC", 316);
    m.insert("#F2F0DF", 317);
    m.insert("#FFF8DC", 318);
    m.insert("#FBF6D9", 319);
    m.insert("#FAEBD7", 320);
    m.insert("#FFF0DB", 321);
    m.insert("#FFEFD5", 322);
    m.insert("#F7E7CE", 323);
    m.insert("#FFEBCD", 324);
    m.insert("#FFE4C4", 325);
    m.insert("#F5DEB3", 326);
    m.insert("#FFE4B5", 327);
    m.insert("#FFE5B4", 328);
    m.insert("#FED8B1", 329);
    m.insert("#FFDAB9", 330);
    m.insert("#FBD5AB", 331);
    m.insert("#FFDEAD", 332);
    m.insert("#FBE7A1", 333);
    m.insert("#F3E3C3", 334);
    m.insert("#F0E2B6", 335);
    m.insert("#F1E5AC", 336);
    m.insert("#F3E5AB", 337);
    m.insert("#ECE5B6", 338);
    m.insert("#E8E4C9", 339);
    m.insert("#EEE8AA", 340);
    m.insert("#F0E68C", 341);
    m.insert("#EDDA74", 342);
    m.insert("#EDE275", 343);
    m.insert("#FFE87C", 344);
    m.insert("#FFF380", 345);
    m.insert("#FAF884", 346);
    m.insert("#FFFF33", 347);
    m.insert("#FFFF00", 348);
    m.insert("#FEF250", 349);
    m.insert("#FFEF00", 350);
    m.insert("#F5E216", 351);
    m.insert("#FFDB58", 352);
    m.insert("#FFDF00", 353);
    m.insert("#F9DB24", 354);
    m.insert("#EED202", 355);
    m.insert("#FFD801", 356);
    m.insert("#FFD700", 357);
    m.insert("#FDD017", 358);
    m.insert("#FFCE44", 359);
    m.insert("#EAC117", 360);
    m.insert("#F6BE00", 361);
    m.insert("#F2BB66", 362);
    m.insert("#FFBF00", 363);
    m.insert("#FBB917", 364);
    m.insert("#FDBD01", 365);
    m.insert("#FBB117", 366);
    m.insert("#FFAE42", 367);
    m.insert("#FFAE42", 368);
    m.insert("#FFA62F", 369);
    m.insert("#FFA600", 370);
    m.insert("#FFA500", 371);
    m.insert("#EE9A4D", 372);
    m.insert("#F4A460", 373);
    m.insert("#E2A76F", 374);
    m.insert("#C19A6B", 375);
    m.insert("#E6BF83", 376);
    m.insert("#DEB887", 377);
    m.insert("#D2B48C", 378);
    m.insert("#C8AD7F", 379);
    m.insert("#C2B280", 380);
    m.insert("#C6BA8B", 381);
    m.insert("#BCB88A", 382);
    m.insert("#C8B560", 383);
    m.insert("#C9BE62", 384);
    m.insert("#C9AE5D", 385);
    m.insert("#BDB76B", 386);
    m.insert("#BAB86C", 387);
    m.insert("#B5A642", 388);
    m.insert("#C7A317", 389);
    m.insert("#D4AF37", 390);
    m.insert("#E1AD01", 391);
    m.insert("#E9AB17", 392);
    m.insert("#E8A317", 393);
    m.insert("#DAA520", 394);
    m.insert("#D4A017", 395);
    m.insert("#C68E17", 396);
    m.insert("#B8860B", 397);
    m.insert("#C58917", 398);
    m.insert("#CD853F", 399);
    m.insert("#CD7F32", 400);
    m.insert("#CA762B", 401);
    m.insert("#C88141", 402);
    m.insert("#B87333", 403);
    m.insert("#AA6C39", 404);
    m.insert("#A97142", 405);
    m.insert("#AB784E", 406);
    m.insert("#966F33", 407);
    m.insert("#906E3E", 408);
    m.insert("#806517", 409);
    m.insert("#665D1E", 410);
    m.insert("#8E7618", 411);
    m.insert("#8B8000", 412);
    m.insert("#827839", 413);
    m.insert("#8A865D", 414);
    m.insert("#93917C", 415);
    m.insert("#9F8C76", 416);
    m.insert("#AF9B60", 417);
    m.insert("#827B60", 418);
    m.insert("#786D5F", 419);
    m.insert("#483C32", 420);
    m.insert("#4A412A", 421);
    m.insert("#473810", 422);
    m.insert("#493D26", 423);
    m.insert("#513B1C", 424);
    m.insert("#3D3635", 425);
    m.insert("#3B2F2F", 426);
    m.insert("#49413F", 427);
    m.insert("#43302E", 428);
    m.insert("#622F22", 429);
    m.insert("#5C3317", 430);
    m.insert("#644117", 431);
    m.insert("#654321", 432);
    m.insert("#704214", 433);
    m.insert("#804A00", 434);
    m.insert("#6F4E37", 435);
    m.insert("#835C3B", 436);
    m.insert("#7F5217", 437);
    m.insert("#7F462C", 438);
    m.insert("#A0522D", 439);
    m.insert("#8B4513", 440);
    m.insert("#8A4117", 441);
    m.insert("#7E3817", 442);
    m.insert("#7E3517", 443);
    m.insert("#954535", 444);
    m.insert("#9E4638", 445);
    m.insert("#A05544", 446);
    m.insert("#C34A2C", 447);
    m.insert("#B83C08", 448);
    m.insert("#C04000", 449);
    m.insert("#EB5406", 450);
    m.insert("#C35817", 451);
    m.insert("#B86500", 452);
    m.insert("#B5651D", 453);
    m.insert("#B76734", 454);
    m.insert("#A55D35", 455);
    m.insert("#C36241", 456);
    m.insert("#CB6D51", 457);
    m.insert("#C47451", 458);
    m.insert("#D2691E", 459);
    m.insert("#CC6600", 460);
    m.insert("#E56717", 461);
    m.insert("#E66C2C", 462);
    m.insert("#FF6700", 463);
    m.insert("#FF5F1F", 464);
    m.insert("#FE632A", 465);
    m.insert("#F87217", 466);
    m.insert("#FF7900", 467);
    m.insert("#F88017", 468);
    m.insert("#FF8C00", 469);
    m.insert("#F87431", 470);
    m.insert("#FF7722", 471);
    m.insert("#E67451", 472);
    m.insert("#FF8040", 473);
    m.insert("#FF7F50", 474);
    m.insert("#F88158", 475);
    m.insert("#F9966B", 476);
    m.insert("#FFA07A", 477);
    m.insert("#F89880", 478);
    m.insert("#E9967A", 479);
    m.insert("#E78A61", 480);
    m.insert("#DA8A67", 481);
    m.insert("#FF8674", 482);
    m.insert("#FA8072", 483);
    m.insert("#F98B88", 484);
    m.insert("#F08080", 485);
    m.insert("#F67280", 486);
    m.insert("#E77471", 487);
    m.insert("#F75D59", 488);
    m.insert("#E55451", 489);
    m.insert("#CD5C5C", 490);
    m.insert("#FF6347", 491);
    m.insert("#E55B3C", 492);
    m.insert("#FF4500", 493);
    m.insert("#FF0000", 494);
    m.insert("#FD1C03", 495);
    m.insert("#FF2400", 496);
    m.insert("#F62217", 497);
    m.insert("#F70D1A", 498);
    m.insert("#F62817", 499);
    m.insert("#E42217", 500);
    m.insert("#E41B17", 501);
    m.insert("#DC381F", 502);
    m.insert("#C83F49", 503);
    m.insert("#C24641", 504);
    m.insert("#C11B17", 505);
    m.insert("#B22222", 506);
    m.insert("#B21807", 507);
    m.insert("#A52A2A", 508);
    m.insert("#A70D2A", 509);
    m.insert("#9F000F", 510);
    m.insert("#931314", 511);
    m.insert("#990000", 512);
    m.insert("#990012", 513);
    m.insert("#990012", 514);
    m.insert("#8B0000", 515);
    m.insert("#8F0B0B", 516);
    m.insert("#800000", 517);
    m.insert("#8C001A", 518);
    m.insert("#7E191B", 519);
    m.insert("#800517", 520);
    m.insert("#733635", 521);
    m.insert("#660000", 522);
    m.insert("#551606", 523);
    m.insert("#560319", 524);
    m.insert("#3F000F", 525);
    m.insert("#3D0C02", 526);
    m.insert("#2F0909", 527);
    m.insert("#2B1B17", 528);
    m.insert("#550A35", 529);
    m.insert("#810541", 530);
    m.insert("#7D0541", 531);
    m.insert("#7D0552", 532);
    m.insert("#872657", 533);
    m.insert("#7E354D", 534);
    m.insert("#7F4E52", 535);
    m.insert("#7F525D", 536);
    m.insert("#7F5A58", 537);
    m.insert("#997070", 538);
    m.insert("#B1907F", 539);
    m.insert("#B38481", 540);
    m.insert("#BC8F8F", 541);
    m.insert("#C5908E", 542);
    m.insert("#C48793", 543);
    m.insert("#CC7A8B", 544);
    m.insert("#C48189", 545);
    m.insert("#C08081", 546);
    m.insert("#D58A94", 547);
    m.insert("#E799A3", 548);
    m.insert("#E8ADAA", 549);
    m.insert("#C9A9A6", 550);
    m.insert("#C4AEAD", 551);
    m.insert("#E6C7C2", 552);
    m.insert("#ECC5C0", 553);
    m.insert("#FFCBA4", 554);
    m.insert("#F8B88B", 555);
    m.insert("#EDC9AF", 556);
    m.insert("#FFDDCA", 557);
    m.insert("#FDD7E4", 558);
    m.insert("#F2D4D7", 559);
    m.insert("#FFE6E8", 560);
    m.insert("#FFE4E1", 561);
    m.insert("#FFDFDD", 562);
    m.insert("#FBCFCD", 563);
    m.insert("#FFCCCB", 564);
    m.insert("#F7CAC9", 565);
    m.insert("#F6C6BD", 566);
    m.insert("#FBBBB9", 567);
    m.insert("#FFC0CB", 568);
    m.insert("#FFB6C1", 569);
    m.insert("#FFB8BF", 570);
    m.insert("#FFB2D0", 571);
    m.insert("#FAAFBE", 572);
    m.insert("#FAAFBA", 573);
    m.insert("#F9A7B0", 574);
    m.insert("#FEA3AA", 575);
    m.insert("#E7A1B0", 576);
    m.insert("#E38AAE", 577);
    m.insert("#F778A1", 578);
    m.insert("#E5788F", 579);
    m.insert("#E56E94", 580);
    m.insert("#DB7093", 581);
    m.insert("#D16587", 582);
    m.insert("#C25A7C", 583);
    m.insert("#C25283", 584);
    m.insert("#E75480", 585);
    m.insert("#F660AB", 586);
    m.insert("#FF69B4", 587);
    m.insert("#FC6C85", 588);
    m.insert("#F6358A", 589);
    m.insert("#F52887", 590);
    m.insert("#FF007F", 591);
    m.insert("#FF0080", 592);
    m.insert("#FF1493", 593);
    m.insert("#F535AA", 594);
    m.insert("#FF33AA", 595);
    m.insert("#FD349C", 596);
    m.insert("#E45E9D", 597);
    m.insert("#E759AC", 598);
    m.insert("#E3319D", 599);
    m.insert("#DA1884", 600);
    m.insert("#E4287C", 601);
    m.insert("#FA2A55", 602);
    m.insert("#E30B5D", 603);
    m.insert("#DC143C", 604);
    m.insert("#C32148", 605);
    m.insert("#C21E56", 606);
    m.insert("#C12869", 607);
    m.insert("#C12267", 608);
    m.insert("#CA226B", 609);
    m.insert("#CC338B", 610);
    m.insert("#C71585", 611);
    m.insert("#C12283", 612);
    m.insert("#B3446C", 613);
    m.insert("#B93B8F", 614);
    m.insert("#DA70D6", 615);
    m.insert("#DF73D4", 616);
    m.insert("#EE82EE", 617);
    m.insert("#FF77FF", 618);
    m.insert("#F433FF", 619);
    m.insert("#FF00FF", 620);
    m.insert("#FF00FF", 621);
    m.insert("#E238EC", 622);
    m.insert("#D462FF", 623);
    m.insert("#C45AEC", 624);
    m.insert("#BA55D3", 625);
    m.insert("#A74AC7", 626);
    m.insert("#B048B5", 627);
    m.insert("#B666D2", 628);
    m.insert("#D291BC", 629);
    m.insert("#A17188", 630);
    m.insert("#915F6D", 631);
    m.insert("#7E587E", 632);
    m.insert("#614051", 633);
    m.insert("#583759", 634);
    m.insert("#5E5A80", 635);
    m.insert("#4E5180", 636);
    m.insert("#6A5ACD", 637);
    m.insert("#6960EC", 638);
    m.insert("#5865F2", 639);
    m.insert("#736AFF", 640);
    m.insert("#7B68EE", 641);
    m.insert("#7575CF", 642);
    m.insert("#6667AB", 643);
    m.insert("#6F2DA8", 644);
    m.insert("#6A0DAD", 645);
    m.insert("#6C2DC7", 646);
    m.insert("#822EFF", 647);
    m.insert("#5539CC", 648);
    m.insert("#5453A6", 649);
    m.insert("#483D8B", 650);
    m.insert("#4E387E", 651);
    m.insert("#571B7E", 652);
    m.insert("#4B0150", 653);
    m.insert("#36013F", 654);
    m.insert("#2E1A47", 655);
    m.insert("#461B7E", 656);
    m.insert("#4B0082", 657);
    m.insert("#342D7E", 658);
    m.insert("#663399", 659);
    m.insert("#6A287E", 660);
    m.insert("#8B008B", 661);
    m.insert("#800080", 662);
    m.insert("#86608E", 663);
    m.insert("#9932CC", 664);
    m.insert("#9400D3", 665);
    m.insert("#8D38C9", 666);
    m.insert("#A23BEC", 667);
    m.insert("#B041FF", 668);
    m.insert("#842DCE", 669);
    m.insert("#8A2BE2", 670);
    m.insert("#7A5DC7", 671);
    m.insert("#7F38EC", 672);
    m.insert("#9D00FF", 673);
    m.insert("#8E35EF", 674);
    m.insert("#893BFF", 675);
    m.insert("#9370DB", 676);
    m.insert("#8467D7", 677);
    m.insert("#9172EC", 678);
    m.insert("#9E7BFF", 679);
    m.insert("#8686AF", 680);
    m.insert("#967BB6", 681);
    m.insert("#B09FCA", 682);
    m.insert("#C8C4DF", 683);
    m.insert("#CCCCFF", 684);
    m.insert("#DCD0FF", 685);
    m.insert("#C8A2C8", 686);
    m.insert("#E0B0FF", 687);
    m.insert("#D891EF", 688);
    m.insert("#C38EC7", 689);
    m.insert("#DDA0DD", 690);
    m.insert("#E6A9EC", 691);
    m.insert("#F2A2E8", 692);
    m.insert("#F9B7FF", 693);
    m.insert("#C6AEC7", 694);
    m.insert("#D2B9D3", 695);
    m.insert("#D8BFD8", 696);
    m.insert("#DFD3E3", 697);
    m.insert("#E9CFEC", 698);
    m.insert("#FCDFFF", 699);
    m.insert("#EBDDE2", 700);
    m.insert("#E1D9D1", 701);
    m.insert("#E9E4D4", 702);
    m.insert("#EFEBD8", 703);
    m.insert("#EDE6D6", 704);
    m.insert("#F0E9D6", 705);
    m.insert("#F8F0E3", 706);
    m.insert("#FAF0DD", 707);
    m.insert("#FFF8E7", 708);
    m.insert("#F8F6F0", 709);
    m.insert("#F3E8EA", 710);
    m.insert("#FFF0F5", 711);
    m.insert("#FDEEF4", 712);
    m.insert("#FFF9E3", 713);
    m.insert("#FEF0E3", 714);
    m.insert("#EAEEE9", 715);
    m.insert("#FAF0E6", 716);
    m.insert("#FFF5EE", 717);
    m.insert("#F9F6EE", 718);
    m.insert("#FAF5EF", 719);
    m.insert("#FFFAF0", 720);
    m.insert("#FFFFF0", 721);
    m.insert("#FFFFF4", 722);
    m.insert("#FFFFF7", 723);
    m.insert("#FBFBF9", 724);
    m.insert("#FFFAFA", 725);
    m.insert("#FEFCFF", 726);
    m.insert("#FFFEFA", 727);
    m.insert("#FFFFFF", 728);
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
    pub fn name(&self) -> Option<String> {
        match self {
            Self::Rgb(_, _, _) => None,
            _ => {
                if let Some(idx) = self.offset() {
                    Some(COLORS[idx].0.to_string())
                } else {
                    unreachable!()
                }
            }
        }
    }
    #[doc = r" Get the offset of this color in the COLORS array"]
    fn offset(&self) -> Option<usize> {
        Some(match self {
            Self::Black => 0,
            Self::BlackBlue => 1,
            Self::Night => 2,
            Self::Charcoal => 3,
            Self::Oil => 4,
            Self::StormyGray => 5,
            Self::LightBlack => 6,
            Self::DarkSteampunk => 7,
            Self::BlackCat => 8,
            Self::Iridium => 9,
            Self::BlackEel => 10,
            Self::BlackCow => 11,
            Self::GrayWolf => 12,
            Self::VampireGray => 13,
            Self::IronGray => 14,
            Self::GrayDolphin => 15,
            Self::CarbonGray => 16,
            Self::AshGray => 17,
            Self::DimGray => 18,
            Self::DimGrey => 19,
            Self::NardoGray => 20,
            Self::CloudyGray => 21,
            Self::SmokeyGray => 22,
            Self::AlienGray => 23,
            Self::SonicSilver => 24,
            Self::PlatinumGray => 25,
            Self::Granite => 26,
            Self::Gray => 27,
            Self::Grey => 28,
            Self::BattleshipGray => 29,
            Self::SheetMetal => 30,
            Self::DarkGainsboro => 31,
            Self::GunmetalGray => 32,
            Self::ColdMetal => 33,
            Self::StainlessSteelGray => 34,
            Self::DarkGrey => 35,
            Self::DarkGray => 36,
            Self::ChromeAluminum => 37,
            Self::GrayCloud => 38,
            Self::Metal => 39,
            Self::Silver => 40,
            Self::Steampunk => 41,
            Self::PaleSilver => 42,
            Self::GearSteelGray => 43,
            Self::GrayGoose => 44,
            Self::PlatinumSilver => 45,
            Self::LightGrey => 46,
            Self::LightGray => 47,
            Self::SilverWhite => 48,
            Self::Gainsboro => 49,
            Self::LightSteelGray => 50,
            Self::WhiteSmoke => 51,
            Self::WhiteGray => 52,
            Self::Platinum => 53,
            Self::MetallicSilver => 54,
            Self::BlueGray => 55,
            Self::RomanSilver => 56,
            Self::LightSlateGrey => 57,
            Self::LightSlateGray => 58,
            Self::SlateGrey => 59,
            Self::SlateGray => 60,
            Self::RatGray => 61,
            Self::SlateGraniteGray => 62,
            Self::JetGray => 63,
            Self::MistBlue => 64,
            Self::SteelGray => 65,
            Self::MarbleBlue => 66,
            Self::SlateBlueGray => 67,
            Self::LightPurpleBlue => 68,
            Self::AzureBlue => 69,
            Self::EstorilBlue => 70,
            Self::BlueJay => 71,
            Self::CharcoalBlue => 72,
            Self::DarkBlueGray => 73,
            Self::DarkSlate => 74,
            Self::DeepSeaBlue => 75,
            Self::NightBlue => 76,
            Self::MidnightBlue => 77,
            Self::Navy => 78,
            Self::DenimDarkBlue => 79,
            Self::DarkBlue => 80,
            Self::LapisBlue => 81,
            Self::NewMidnightBlue => 82,
            Self::EarthBlue => 83,
            Self::CobaltBlue => 84,
            Self::MediumBlue => 85,
            Self::BlueberryBlue => 86,
            Self::CanaryBlue => 87,
            Self::Blue => 88,
            Self::SamcoBlue => 89,
            Self::BrightBlue => 90,
            Self::BlueOrchid => 91,
            Self::SapphireBlue => 92,
            Self::BlueEyes => 93,
            Self::BrightNavyBlue => 94,
            Self::BalloonBlue => 95,
            Self::RoyalBlue => 96,
            Self::OceanBlue => 97,
            Self::DarkSkyBlue => 98,
            Self::BlueRibbon => 99,
            Self::BlueDress => 100,
            Self::NeonBlue => 101,
            Self::DodgerBlue => 102,
            Self::GlacialBlueIce => 103,
            Self::SteelBlue => 104,
            Self::SilkBlue => 105,
            Self::WindowsBlue => 106,
            Self::BlueIvy => 107,
            Self::CyanBlue => 108,
            Self::BlueKoi => 109,
            Self::ColumbiaBlue => 110,
            Self::BabyBlue => 111,
            Self::CornflowerBlue => 112,
            Self::SkyBlueDress => 113,
            Self::Iceberg => 114,
            Self::ButterflyBlue => 115,
            Self::DeepSkyBlue => 116,
            Self::MiddayBlue => 117,
            Self::CrystalBlue => 118,
            Self::DenimBlue => 119,
            Self::DaySkyBlue => 120,
            Self::LightSkyBlue => 121,
            Self::SkyBlue => 122,
            Self::JeansBlue => 123,
            Self::BlueAngel => 124,
            Self::PastelBlue => 125,
            Self::LightDayBlue => 126,
            Self::SeaBlue => 127,
            Self::HeavenlyBlue => 128,
            Self::RobinEggBlue => 129,
            Self::PowderBlue => 130,
            Self::CoralBlue => 131,
            Self::LightBlue => 132,
            Self::LightSteelBlue => 133,
            Self::GulfBlue => 134,
            Self::PastelLightBlue => 135,
            Self::LavenderBlue => 136,
            Self::WhiteBlue => 137,
            Self::Lavender => 138,
            Self::Water => 139,
            Self::AliceBlue => 140,
            Self::GhostWhite => 141,
            Self::Azure => 142,
            Self::LightCyan => 143,
            Self::LightSlate => 144,
            Self::ElectricBlue => 145,
            Self::TronBlue => 146,
            Self::BlueZircon => 147,
            Self::Cyan => 148,
            Self::Aqua => 149,
            Self::BrightCyan => 150,
            Self::Celeste => 151,
            Self::BlueDiamond => 152,
            Self::BrightTurquoise => 153,
            Self::BlueLagoon => 154,
            Self::PaleTurquoise => 155,
            Self::PaleBlueLily => 156,
            Self::LightTeal => 157,
            Self::TiffanyBlue => 158,
            Self::BlueHosta => 159,
            Self::CyanOpaque => 160,
            Self::NorthernLightsBlue => 161,
            Self::BlueGreen => 162,
            Self::MediumAquaMarine => 163,
            Self::AquaSeafoamGreen => 164,
            Self::MagicMint => 165,
            Self::LightAquamarine => 166,
            Self::Aquamarine => 167,
            Self::BrightTeal => 168,
            Self::Turquoise => 169,
            Self::MediumTurquoise => 170,
            Self::DeepTurquoise => 171,
            Self::Jellyfish => 172,
            Self::BlueTurquoise => 173,
            Self::DarkTurquoise => 174,
            Self::MacawBlueGreen => 175,
            Self::LightSeaGreen => 176,
            Self::SeafoamGreen => 177,
            Self::CadetBlue => 178,
            Self::DeepSea => 179,
            Self::DarkCyan => 180,
            Self::TealGreen => 181,
            Self::Teal => 182,
            Self::TealBlue => 183,
            Self::MediumTeal => 184,
            Self::DarkTeal => 185,
            Self::DeepTeal => 186,
            Self::DarkSlateGray => 187,
            Self::DarkSlateGrey => 188,
            Self::Gunmetal => 189,
            Self::BlueMossGreen => 190,
            Self::BeetleGreen => 191,
            Self::GrayishTurquoise => 192,
            Self::GreenishBlue => 193,
            Self::AquamarineStone => 194,
            Self::SeaTurtleGreen => 195,
            Self::DullSeaGreen => 196,
            Self::DarkGreenBlue => 197,
            Self::DeepSeaGreen => 198,
            Self::BottleGreen => 199,
            Self::SeaGreen => 200,
            Self::ElfGreen => 201,
            Self::DarkMint => 202,
            Self::Jade => 203,
            Self::EarthGreen => 204,
            Self::ChromeGreen => 205,
            Self::Mint => 206,
            Self::Emerald => 207,
            Self::IsleOfManGreen => 208,
            Self::MediumSeaGreen => 209,
            Self::MetallicGreen => 210,
            Self::CamouflageGreen => 211,
            Self::SageGreen => 212,
            Self::HazelGreen => 213,
            Self::VenomGreen => 214,
            Self::OliveDrab => 215,
            Self::Olive => 216,
            Self::Ebony => 217,
            Self::DarkOliveGreen => 218,
            Self::MilitaryGreen => 219,
            Self::GreenLeaves => 220,
            Self::ArmyGreen => 221,
            Self::FernGreen => 222,
            Self::FallForestGreen => 223,
            Self::IrishGreen => 224,
            Self::PineGreen => 225,
            Self::MediumForestGreen => 226,
            Self::RacingGreen => 227,
            Self::JungleGreen => 228,
            Self::CactusGreen => 229,
            Self::ForestGreen => 230,
            Self::Green => 231,
            Self::DarkGreen => 232,
            Self::DeepGreen => 233,
            Self::DeepEmeraldGreen => 234,
            Self::HunterGreen => 235,
            Self::DarkForestGreen => 236,
            Self::LotusGreen => 237,
            Self::BroccoliGreen => 238,
            Self::SeaweedGreen => 239,
            Self::ShamrockGreen => 240,
            Self::GreenOnion => 241,
            Self::MossGreen => 242,
            Self::GrassGreen => 243,
            Self::GreenPepper => 244,
            Self::DarkLimeGreen => 245,
            Self::ParrotGreen => 246,
            Self::CloverGreen => 247,
            Self::DinosaurGreen => 248,
            Self::GreenSnake => 249,
            Self::AlienGreen => 250,
            Self::GreenApple => 251,
            Self::LimeGreen => 252,
            Self::PeaGreen => 253,
            Self::KellyGreen => 254,
            Self::ZombieGreen => 255,
            Self::GreenPeas => 256,
            Self::DollarBillGreen => 257,
            Self::FrogGreen => 258,
            Self::TurquoiseGreen => 259,
            Self::DarkSeaGreen => 260,
            Self::BasilGreen => 261,
            Self::GrayGreen => 262,
            Self::LightOliveGreen => 263,
            Self::IguanaGreen => 264,
            Self::CitronGreen => 265,
            Self::AcidGreen => 266,
            Self::AvocadoGreen => 267,
            Self::PistachioGreen => 268,
            Self::SaladGreen => 269,
            Self::YellowGreen => 270,
            Self::PastelGreen => 271,
            Self::HummingbirdGreen => 272,
            Self::NebulaGreen => 273,
            Self::StoplightGoGreen => 274,
            Self::NeonGreen => 275,
            Self::JadeGreen => 276,
            Self::SpringGreen => 277,
            Self::OceanGreen => 278,
            Self::LimeMintGreen => 279,
            Self::MediumSpringGreen => 280,
            Self::AquaGreen => 281,
            Self::EmeraldGreen => 282,
            Self::Lime => 283,
            Self::LawnGreen => 284,
            Self::BrightGreen => 285,
            Self::Chartreuse => 286,
            Self::YellowLawnGreen => 287,
            Self::AloeVeraGreen => 288,
            Self::DullGreenYellow => 289,
            Self::LemonGreen => 290,
            Self::GreenYellow => 291,
            Self::ChameleonGreen => 292,
            Self::NeonYellowGreen => 293,
            Self::YellowGreenGrosbeak => 294,
            Self::TeaGreen => 295,
            Self::SlimeGreen => 296,
            Self::AlgaeGreen => 297,
            Self::LightGreen => 298,
            Self::DragonGreen => 299,
            Self::PaleGreen => 300,
            Self::MintGreen => 301,
            Self::GreenThumb => 302,
            Self::OrganicBrown => 303,
            Self::LightJade => 304,
            Self::LightMintGreen => 305,
            Self::LightRoseGreen => 306,
            Self::ChromeWhite => 307,
            Self::HoneyDew => 308,
            Self::MintCream => 309,
            Self::LemonChiffon => 310,
            Self::Parchment => 311,
            Self::Cream => 312,
            Self::CreamWhite => 313,
            Self::LightGoldenRodYellow => 314,
            Self::LightYellow => 315,
            Self::Beige => 316,
            Self::WhiteYellow => 317,
            Self::Cornsilk => 318,
            Self::Blonde => 319,
            Self::AntiqueWhite => 320,
            Self::LightBeige => 321,
            Self::PapayaWhip => 322,
            Self::Champagne => 323,
            Self::BlanchedAlmond => 324,
            Self::Bisque => 325,
            Self::Wheat => 326,
            Self::Moccasin => 327,
            Self::Peach => 328,
            Self::LightOrange => 329,
            Self::PeachPuff => 330,
            Self::CoralPeach => 331,
            Self::NavajoWhite => 332,
            Self::GoldenBlonde => 333,
            Self::GoldenSilk => 334,
            Self::DarkBlonde => 335,
            Self::LightGold => 336,
            Self::Vanilla => 337,
            Self::TanBrown => 338,
            Self::DirtyWhite => 339,
            Self::PaleGoldenRod => 340,
            Self::Khaki => 341,
            Self::CardboardBrown => 342,
            Self::HarvestGold => 343,
            Self::SunYellow => 344,
            Self::CornYellow => 345,
            Self::PastelYellow => 346,
            Self::NeonYellow => 347,
            Self::Yellow => 348,
            Self::LemonYellow => 349,
            Self::CanaryYellow => 350,
            Self::BananaYellow => 351,
            Self::MustardYellow => 352,
            Self::GoldenYellow => 353,
            Self::BoldYellow => 354,
            Self::SafetyYellow => 355,
            Self::RubberDuckyYellow => 356,
            Self::Gold => 357,
            Self::BrightGold => 358,
            Self::ChromeGold => 359,
            Self::GoldenBrown => 360,
            Self::DeepYellow => 361,
            Self::MacaroniandCheese => 362,
            Self::Amber => 363,
            Self::Saffron => 364,
            Self::NeonGold => 365,
            Self::Beer => 366,
            Self::YellowOrange => 367,
            Self::OrangeYellow => 368,
            Self::Cantaloupe => 369,
            Self::CheeseOrange => 370,
            Self::Orange => 371,
            Self::BrownSand => 372,
            Self::SandyBrown => 373,
            Self::BrownSugar => 374,
            Self::CamelBrown => 375,
            Self::DeerBrown => 376,
            Self::BurlyWood => 377,
            Self::Tan => 378,
            Self::LightFrenchBeige => 379,
            Self::Sand => 380,
            Self::SoftHazel => 381,
            Self::Sage => 382,
            Self::FallLeafBrown => 383,
            Self::GingerBrown => 384,
            Self::BronzeGold => 385,
            Self::DarkKhaki => 386,
            Self::OliveGreen => 387,
            Self::Brass => 388,
            Self::CookieBrown => 389,
            Self::MetallicGold => 390,
            Self::Mustard => 391,
            Self::BeeYellow => 392,
            Self::SchoolBusYellow => 393,
            Self::GoldenRod => 394,
            Self::OrangeGold => 395,
            Self::Caramel => 396,
            Self::DarkGoldenRod => 397,
            Self::Cinnamon => 398,
            Self::Peru => 399,
            Self::Bronze => 400,
            Self::PumpkinPie => 401,
            Self::TigerOrange => 402,
            Self::Copper => 403,
            Self::DarkGold => 404,
            Self::MetallicBronze => 405,
            Self::DarkAlmond => 406,
            Self::Wood => 407,
            Self::KhakiBrown => 408,
            Self::OakBrown => 409,
            Self::AntiqueBronze => 410,
            Self::Hazel => 411,
            Self::DarkYellow => 412,
            Self::DarkMoccasin => 413,
            Self::KhakiGreen => 414,
            Self::MillenniumJade => 415,
            Self::DarkBeige => 416,
            Self::BulletShell => 417,
            Self::ArmyBrown => 418,
            Self::Sandstone => 419,
            Self::Taupe => 420,
            Self::DarkGrayishOlive => 421,
            Self::DarkHazelBrown => 422,
            Self::Mocha => 423,
            Self::MilkChocolate => 424,
            Self::GrayBrown => 425,
            Self::DarkCoffee => 426,
            Self::WesternCharcoal => 427,
            Self::OldBurgundy => 428,
            Self::RedBrown => 429,
            Self::BakersBrown => 430,
            Self::PullmanBrown => 431,
            Self::DarkBrown => 432,
            Self::SepiaBrown => 433,
            Self::DarkBronze => 434,
            Self::Coffee => 435,
            Self::BrownBear => 436,
            Self::RedDirt => 437,
            Self::Sepia => 438,
            Self::Sienna => 439,
            Self::SaddleBrown => 440,
            Self::DarkSienna => 441,
            Self::Sangria => 442,
            Self::BloodRed => 443,
            Self::Chestnut => 444,
            Self::CoralBrown => 445,
            Self::DeepAmber => 446,
            Self::ChestnutRed => 447,
            Self::GingerRed => 448,
            Self::Mahogany => 449,
            Self::RedGold => 450,
            Self::RedFox => 451,
            Self::DarkBisque => 452,
            Self::LightBrown => 453,
            Self::PetraGold => 454,
            Self::BrownRust => 455,
            Self::Rust => 456,
            Self::CopperRed => 457,
            Self::OrangeSalmon => 458,
            Self::Chocolate => 459,
            Self::Sedona => 460,
            Self::PapayaOrange => 461,
            Self::HalloweenOrange => 462,
            Self::NeonOrange => 463,
            Self::BrightOrange => 464,
            Self::FluroOrange => 465,
            Self::PumpkinOrange => 466,
            Self::SafetyOrange => 467,
            Self::CarrotOrange => 468,
            Self::DarkOrange => 469,
            Self::ConstructionConeOrange => 470,
            Self::IndianSaffron => 471,
            Self::SunriseOrange => 472,
            Self::MangoOrange => 473,
            Self::Coral => 474,
            Self::BasketBallOrange => 475,
            Self::LightSalmonRose => 476,
            Self::LightSalmon => 477,
            Self::PinkOrange => 478,
            Self::DarkSalmon => 479,
            Self::Tangerine => 480,
            Self::LightCopper => 481,
            Self::SalmonPink => 482,
            Self::Salmon => 483,
            Self::PeachPink => 484,
            Self::LightCoral => 485,
            Self::PastelRed => 486,
            Self::PinkCoral => 487,
            Self::BeanRed => 488,
            Self::ValentineRed => 489,
            Self::IndianRed => 490,
            Self::Tomato => 491,
            Self::ShockingOrange => 492,
            Self::OrangeRed => 493,
            Self::Red => 494,
            Self::NeonRed => 495,
            Self::ScarletRed => 496,
            Self::RubyRed => 497,
            Self::FerrariRed => 498,
            Self::FireEngineRed => 499,
            Self::LavaRed => 500,
            Self::LoveRed => 501,
            Self::Grapefruit => 502,
            Self::StrawberryRed => 503,
            Self::CherryRed => 504,
            Self::ChilliPepper => 505,
            Self::FireBrick => 506,
            Self::TomatoSauceRed => 507,
            Self::Brown => 508,
            Self::CarbonRed => 509,
            Self::Cranberry => 510,
            Self::SaffronRed => 511,
            Self::CrimsonRed => 512,
            Self::RedWine => 513,
            Self::WineRed => 514,
            Self::DarkRed => 515,
            Self::MaroonRed => 516,
            Self::Maroon => 517,
            Self::Burgundy => 518,
            Self::Vermilion => 519,
            Self::DeepRed => 520,
            Self::GarnetRed => 521,
            Self::RedBlood => 522,
            Self::BloodNight => 523,
            Self::DarkScarlet => 524,
            Self::ChocolateBrown => 525,
            Self::BlackBean => 526,
            Self::DarkMaroon => 527,
            Self::Midnight => 528,
            Self::PurpleLily => 529,
            Self::PurpleMaroon => 530,
            Self::PlumPie => 531,
            Self::PlumVelvet => 532,
            Self::DarkRaspberry => 533,
            Self::VelvetMaroon => 534,
            Self::RosyFinch => 535,
            Self::DullPurple => 536,
            Self::Puce => 537,
            Self::RoseDust => 538,
            Self::PastelBrown => 539,
            Self::RosyPink => 540,
            Self::RosyBrown => 541,
            Self::KhakiRose => 542,
            Self::LipstickPink => 543,
            Self::DuskyPink => 544,
            Self::PinkBrown => 545,
            Self::OldRose => 546,
            Self::DustyPink => 547,
            Self::PinkDaisy => 548,
            Self::Rose => 549,
            Self::DustyRose => 550,
            Self::SilverPink => 551,
            Self::GoldPink => 552,
            Self::RoseGold => 553,
            Self::DeepPeach => 554,
            Self::PastelOrange => 555,
            Self::DesertSand => 556,
            Self::UnbleachedSilk => 557,
            Self::PigPink => 558,
            Self::PalePink => 559,
            Self::Blush => 560,
            Self::MistyRose => 561,
            Self::PinkBubbleGum => 562,
            Self::LightRose => 563,
            Self::LightRed => 564,
            Self::RoseQuartz => 565,
            Self::WarmPink => 566,
            Self::DeepRose => 567,
            Self::Pink => 568,
            Self::LightPink => 569,
            Self::SoftPink => 570,
            Self::PowderPink => 571,
            Self::DonutPink => 572,
            Self::BabyPink => 573,
            Self::FlamingoPink => 574,
            Self::PastelPink => 575,
            Self::RosePink => 576,
            Self::CadillacPink => 577,
            Self::CarnationPink => 578,
            Self::PastelRose => 579,
            Self::BlushRed => 580,
            Self::PaleVioletRed => 581,
            Self::PurplePink => 582,
            Self::TulipPink => 583,
            Self::BashfulPink => 584,
            Self::DarkPink => 585,
            Self::DarkHotPink => 586,
            Self::HotPink => 587,
            Self::WatermelonPink => 588,
            Self::VioletRed => 589,
            Self::HotDeepPink => 590,
            Self::BrightPink => 591,
            Self::RedMagenta => 592,
            Self::DeepPink => 593,
            Self::NeonPink => 594,
            Self::ChromePink => 595,
            Self::NeonHotPink => 596,
            Self::PinkCupcake => 597,
            Self::RoyalPink => 598,
            Self::DimorphothecaMagenta => 599,
            Self::BarbiePink => 600,
            Self::PinkLemonade => 601,
            Self::RedPink => 602,
            Self::Raspberry => 603,
            Self::Crimson => 604,
            Self::BrightMaroon => 605,
            Self::RoseRed => 606,
            Self::RoguePink => 607,
            Self::BurntPink => 608,
            Self::PinkViolet => 609,
            Self::MagentaPink => 610,
            Self::MediumVioletRed => 611,
            Self::DarkCarnationPink => 612,
            Self::RaspberryPurple => 613,
            Self::PinkPlum => 614,
            Self::Orchid => 615,
            Self::DeepMauve => 616,
            Self::Violet => 617,
            Self::FuchsiaPink => 618,
            Self::BrightNeonPink => 619,
            Self::Magenta => 620,
            Self::Fuchsia => 621,
            Self::CrimsonPurple => 622,
            Self::HeliotropePurple => 623,
            Self::TyrianPurple => 624,
            Self::MediumOrchid => 625,
            Self::PurpleFlower => 626,
            Self::OrchidPurple => 627,
            Self::RichLilac => 628,
            Self::PastelViolet => 629,
            Self::Rosy => 630,
            Self::MauveTaupe => 631,
            Self::ViolaPurple => 632,
            Self::Eggplant => 633,
            Self::PlumPurple => 634,
            Self::Grape => 635,
            Self::PurpleNavy => 636,
            Self::SlateBlue => 637,
            Self::BlueLotus => 638,
            Self::Blurple => 639,
            Self::LightSlateBlue => 640,
            Self::MediumSlateBlue => 641,
            Self::PeriwinklePurple => 642,
            Self::VeryPeri => 643,
            Self::BrightGrape => 644,
            Self::BrightPurple => 645,
            Self::PurpleAmethyst => 646,
            Self::BlueMagenta => 647,
            Self::DarkBlurple => 648,
            Self::DeepPeriwinkle => 649,
            Self::DarkSlateBlue => 650,
            Self::PurpleHaze => 651,
            Self::PurpleIris => 652,
            Self::DarkPurple => 653,
            Self::DeepPurple => 654,
            Self::MidnightPurple => 655,
            Self::PurpleMonster => 656,
            Self::Indigo => 657,
            Self::BlueWhale => 658,
            Self::RebeccaPurple => 659,
            Self::PurpleJam => 660,
            Self::DarkMagenta => 661,
            Self::Purple => 662,
            Self::FrenchLilac => 663,
            Self::DarkOrchid => 664,
            Self::DarkViolet => 665,
            Self::PurpleViolet => 666,
            Self::JasminePurple => 667,
            Self::PurpleDaffodil => 668,
            Self::ClematisViolet => 669,
            Self::BlueViolet => 670,
            Self::PurpleSageBush => 671,
            Self::LovelyPurple => 672,
            Self::NeonPurple => 673,
            Self::PurplePlum => 674,
            Self::AztechPurple => 675,
            Self::MediumPurple => 676,
            Self::LightPurple => 677,
            Self::CrocusPurple => 678,
            Self::PurpleMimosa => 679,
            Self::PastelIndigo => 680,
            Self::LavenderPurple => 681,
            Self::RosePurple => 682,
            Self::Viola => 683,
            Self::Periwinkle => 684,
            Self::PaleLilac => 685,
            Self::Lilac => 686,
            Self::Mauve => 687,
            Self::BrightLilac => 688,
            Self::PurpleDragon => 689,
            Self::Plum => 690,
            Self::BlushPink => 691,
            Self::PastelPurple => 692,
            Self::BlossomPink => 693,
            Self::WisteriaPurple => 694,
            Self::PurpleThistle => 695,
            Self::Thistle => 696,
            Self::PurpleWhite => 697,
            Self::PeriwinklePink => 698,
            Self::CottonCandy => 699,
            Self::LavenderPinocchio => 700,
            Self::DarkWhite => 701,
            Self::AshWhite => 702,
            Self::WarmWhite => 703,
            Self::WhiteChocolate => 704,
            Self::CreamyWhite => 705,
            Self::OffWhite => 706,
            Self::SoftIvory => 707,
            Self::CosmicLatte => 708,
            Self::PearlWhite => 709,
            Self::RedWhite => 710,
            Self::LavenderBlush => 711,
            Self::Pearl => 712,
            Self::EggShell => 713,
            Self::OldLace => 714,
            Self::WhiteIce => 715,
            Self::Linen => 716,
            Self::SeaShell => 717,
            Self::BoneWhite => 718,
            Self::Rice => 719,
            Self::FloralWhite => 720,
            Self::Ivory => 721,
            Self::WhiteGold => 722,
            Self::LightWhite => 723,
            Self::Cotton => 724,
            Self::Snow => 725,
            Self::MilkWhite => 726,
            Self::HalfWhite => 727,
            Self::White => 728,
            Self::Rgb(_, _, _) => return None,
        })
    }
}
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.name() {
            Some(color_name) => write!(f, "{}", color_name.replace(" ", "")),
            None => write!(f, "{}", self.rgb_hex()),
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
