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
#[doc = r" Convert a hex color string to a `Color` variant"]
static RGB_MAP: Lazy<HashMap<&'static str, Color>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("#000000", Color::Black);
    m.insert("#040720", Color::BlackBlue);
    m.insert("#0C090A", Color::Night);
    m.insert("#34282C", Color::Charcoal);
    m.insert("#3B3131", Color::Oil);
    m.insert("#3A3B3C", Color::StormyGray);
    m.insert("#454545", Color::LightBlack);
    m.insert("#4D4D4F", Color::DarkSteampunk);
    m.insert("#413839", Color::BlackCat);
    m.insert("#3D3C3A", Color::Iridium);
    m.insert("#463E3F", Color::BlackEel);
    m.insert("#4C4646", Color::BlackCow);
    m.insert("#504A4B", Color::GrayWolf);
    m.insert("#565051", Color::VampireGray);
    m.insert("#52595D", Color::IronGray);
    m.insert("#5C5858", Color::GrayDolphin);
    m.insert("#625D5D", Color::CarbonGray);
    m.insert("#666362", Color::AshGray);
    m.insert("#696969", Color::DimGray);
    m.insert("#696969", Color::DimGrey);
    m.insert("#686A6C", Color::NardoGray);
    m.insert("#6D6968", Color::CloudyGray);
    m.insert("#726E6D", Color::SmokeyGray);
    m.insert("#736F6E", Color::AlienGray);
    m.insert("#757575", Color::SonicSilver);
    m.insert("#797979", Color::PlatinumGray);
    m.insert("#837E7C", Color::Granite);
    m.insert("#808080", Color::Gray);
    m.insert("#808080", Color::Grey);
    m.insert("#848482", Color::BattleshipGray);
    m.insert("#888B90", Color::SheetMetal);
    m.insert("#8C8C8C", Color::DarkGainsboro);
    m.insert("#8D918D", Color::GunmetalGray);
    m.insert("#9B9A96", Color::ColdMetal);
    m.insert("#99A3A3", Color::StainlessSteelGray);
    m.insert("#A9A9A9", Color::DarkGrey);
    m.insert("#A9A9A9", Color::DarkGray);
    m.insert("#A8A9AD", Color::ChromeAluminum);
    m.insert("#B6B6B4", Color::GrayCloud);
    m.insert("#B6B6B6", Color::Metal);
    m.insert("#C0C0C0", Color::Silver);
    m.insert("#C9C1C1", Color::Steampunk);
    m.insert("#C9C0BB", Color::PaleSilver);
    m.insert("#C0C6C7", Color::GearSteelGray);
    m.insert("#D1D0CE", Color::GrayGoose);
    m.insert("#CECECE", Color::PlatinumSilver);
    m.insert("#D3D3D3", Color::LightGrey);
    m.insert("#D3D3D3", Color::LightGray);
    m.insert("#DADBDD", Color::SilverWhite);
    m.insert("#DCDCDC", Color::Gainsboro);
    m.insert("#E0E5E5", Color::LightSteelGray);
    m.insert("#F5F5F5", Color::WhiteSmoke);
    m.insert("#EEEEEE", Color::WhiteGray);
    m.insert("#E5E4E2", Color::Platinum);
    m.insert("#BCC6CC", Color::MetallicSilver);
    m.insert("#98AFC7", Color::BlueGray);
    m.insert("#838996", Color::RomanSilver);
    m.insert("#778899", Color::LightSlateGrey);
    m.insert("#778899", Color::LightSlateGray);
    m.insert("#708090", Color::SlateGrey);
    m.insert("#708090", Color::SlateGray);
    m.insert("#6D7B8D", Color::RatGray);
    m.insert("#657383", Color::SlateGraniteGray);
    m.insert("#616D7E", Color::JetGray);
    m.insert("#646D7E", Color::MistBlue);
    m.insert("#71797E", Color::SteelGray);
    m.insert("#566D7E", Color::MarbleBlue);
    m.insert("#737CA1", Color::SlateBlueGray);
    m.insert("#728FCE", Color::LightPurpleBlue);
    m.insert("#4863A0", Color::AzureBlue);
    m.insert("#2F539B", Color::EstorilBlue);
    m.insert("#2B547E", Color::BlueJay);
    m.insert("#36454F", Color::CharcoalBlue);
    m.insert("#29465B", Color::DarkBlueGray);
    m.insert("#2B3856", Color::DarkSlate);
    m.insert("#123456", Color::DeepSeaBlue);
    m.insert("#151B54", Color::NightBlue);
    m.insert("#191970", Color::MidnightBlue);
    m.insert("#000080", Color::Navy);
    m.insert("#151B8D", Color::DenimDarkBlue);
    m.insert("#00008B", Color::DarkBlue);
    m.insert("#15317E", Color::LapisBlue);
    m.insert("#0000A0", Color::NewMidnightBlue);
    m.insert("#0000A5", Color::EarthBlue);
    m.insert("#0020C2", Color::CobaltBlue);
    m.insert("#0000CD", Color::MediumBlue);
    m.insert("#0041C2", Color::BlueberryBlue);
    m.insert("#2916F5", Color::CanaryBlue);
    m.insert("#0000FF", Color::Blue);
    m.insert("#0002FF", Color::SamcoBlue);
    m.insert("#0909FF", Color::BrightBlue);
    m.insert("#1F45FC", Color::BlueOrchid);
    m.insert("#2554C7", Color::SapphireBlue);
    m.insert("#1569C7", Color::BlueEyes);
    m.insert("#1974D2", Color::BrightNavyBlue);
    m.insert("#2B60DE", Color::BalloonBlue);
    m.insert("#4169E1", Color::RoyalBlue);
    m.insert("#2B65EC", Color::OceanBlue);
    m.insert("#0059FF", Color::DarkSkyBlue);
    m.insert("#306EFF", Color::BlueRibbon);
    m.insert("#157DEC", Color::BlueDress);
    m.insert("#1589FF", Color::NeonBlue);
    m.insert("#1E90FF", Color::DodgerBlue);
    m.insert("#368BC1", Color::GlacialBlueIce);
    m.insert("#4682B4", Color::SteelBlue);
    m.insert("#488AC7", Color::SilkBlue);
    m.insert("#357EC7", Color::WindowsBlue);
    m.insert("#3090C7", Color::BlueIvy);
    m.insert("#14A3C7", Color::CyanBlue);
    m.insert("#659EC7", Color::BlueKoi);
    m.insert("#87AFC7", Color::ColumbiaBlue);
    m.insert("#95B9C7", Color::BabyBlue);
    m.insert("#6495ED", Color::CornflowerBlue);
    m.insert("#6698FF", Color::SkyBlueDress);
    m.insert("#56A5EC", Color::Iceberg);
    m.insert("#38ACEC", Color::ButterflyBlue);
    m.insert("#00BFFF", Color::DeepSkyBlue);
    m.insert("#3BB9FF", Color::MiddayBlue);
    m.insert("#5CB3FF", Color::CrystalBlue);
    m.insert("#79BAEC", Color::DenimBlue);
    m.insert("#82CAFF", Color::DaySkyBlue);
    m.insert("#87CEFA", Color::LightSkyBlue);
    m.insert("#87CEEB", Color::SkyBlue);
    m.insert("#A0CFEC", Color::JeansBlue);
    m.insert("#B7CEEC", Color::BlueAngel);
    m.insert("#B4CFEC", Color::PastelBlue);
    m.insert("#ADDFFF", Color::LightDayBlue);
    m.insert("#C2DFFF", Color::SeaBlue);
    m.insert("#C6DEFF", Color::HeavenlyBlue);
    m.insert("#BDEDFF", Color::RobinEggBlue);
    m.insert("#B0E0E6", Color::PowderBlue);
    m.insert("#AFDCEC", Color::CoralBlue);
    m.insert("#ADD8E6", Color::LightBlue);
    m.insert("#B0CFDE", Color::LightSteelBlue);
    m.insert("#C9DFEC", Color::GulfBlue);
    m.insert("#D5D6EA", Color::PastelLightBlue);
    m.insert("#E3E4FA", Color::LavenderBlue);
    m.insert("#DBE9FA", Color::WhiteBlue);
    m.insert("#E6E6FA", Color::Lavender);
    m.insert("#EBF4FA", Color::Water);
    m.insert("#F0F8FF", Color::AliceBlue);
    m.insert("#F8F8FF", Color::GhostWhite);
    m.insert("#F0FFFF", Color::Azure);
    m.insert("#E0FFFF", Color::LightCyan);
    m.insert("#CCFFFF", Color::LightSlate);
    m.insert("#9AFEFF", Color::ElectricBlue);
    m.insert("#7DFDFE", Color::TronBlue);
    m.insert("#57FEFF", Color::BlueZircon);
    m.insert("#00FFFF", Color::Cyan);
    m.insert("#00FFFF", Color::Aqua);
    m.insert("#0AFFFF", Color::BrightCyan);
    m.insert("#50EBEC", Color::Celeste);
    m.insert("#4EE2EC", Color::BlueDiamond);
    m.insert("#16E2F5", Color::BrightTurquoise);
    m.insert("#8EEBEC", Color::BlueLagoon);
    m.insert("#AFEEEE", Color::PaleTurquoise);
    m.insert("#CFECEC", Color::PaleBlueLily);
    m.insert("#B3D9D9", Color::LightTeal);
    m.insert("#81D8D0", Color::TiffanyBlue);
    m.insert("#77BFC7", Color::BlueHosta);
    m.insert("#92C7C7", Color::CyanOpaque);
    m.insert("#78C7C7", Color::NorthernLightsBlue);
    m.insert("#7BCCB5", Color::BlueGreen);
    m.insert("#66CDAA", Color::MediumAquaMarine);
    m.insert("#93E9BE", Color::AquaSeafoamGreen);
    m.insert("#AAF0D1", Color::MagicMint);
    m.insert("#93FFE8", Color::LightAquamarine);
    m.insert("#7FFFD4", Color::Aquamarine);
    m.insert("#01F9C6", Color::BrightTeal);
    m.insert("#40E0D0", Color::Turquoise);
    m.insert("#48D1CC", Color::MediumTurquoise);
    m.insert("#48CCCD", Color::DeepTurquoise);
    m.insert("#46C7C7", Color::Jellyfish);
    m.insert("#43C6DB", Color::BlueTurquoise);
    m.insert("#00CED1", Color::DarkTurquoise);
    m.insert("#43BFC7", Color::MacawBlueGreen);
    m.insert("#20B2AA", Color::LightSeaGreen);
    m.insert("#3EA99F", Color::SeafoamGreen);
    m.insert("#5F9EA0", Color::CadetBlue);
    m.insert("#3B9C9C", Color::DeepSea);
    m.insert("#008B8B", Color::DarkCyan);
    m.insert("#00827F", Color::TealGreen);
    m.insert("#008080", Color::Teal);
    m.insert("#007C80", Color::TealBlue);
    m.insert("#045F5F", Color::MediumTeal);
    m.insert("#045D5D", Color::DarkTeal);
    m.insert("#033E3E", Color::DeepTeal);
    m.insert("#25383C", Color::DarkSlateGray);
    m.insert("#25383C", Color::DarkSlateGrey);
    m.insert("#2C3539", Color::Gunmetal);
    m.insert("#3C565B", Color::BlueMossGreen);
    m.insert("#4C787E", Color::BeetleGreen);
    m.insert("#5E7D7E", Color::GrayishTurquoise);
    m.insert("#307D7E", Color::GreenishBlue);
    m.insert("#348781", Color::AquamarineStone);
    m.insert("#438D80", Color::SeaTurtleGreen);
    m.insert("#4E8975", Color::DullSeaGreen);
    m.insert("#1F6357", Color::DarkGreenBlue);
    m.insert("#306754", Color::DeepSeaGreen);
    m.insert("#006A4E", Color::BottleGreen);
    m.insert("#2E8B57", Color::SeaGreen);
    m.insert("#1B8A6B", Color::ElfGreen);
    m.insert("#31906E", Color::DarkMint);
    m.insert("#00A36C", Color::Jade);
    m.insert("#34A56F", Color::EarthGreen);
    m.insert("#1AA260", Color::ChromeGreen);
    m.insert("#3EB489", Color::Mint);
    m.insert("#50C878", Color::Emerald);
    m.insert("#22CE83", Color::IsleOfManGreen);
    m.insert("#3CB371", Color::MediumSeaGreen);
    m.insert("#7C9D8E", Color::MetallicGreen);
    m.insert("#78866B", Color::CamouflageGreen);
    m.insert("#848B79", Color::SageGreen);
    m.insert("#617C58", Color::HazelGreen);
    m.insert("#728C00", Color::VenomGreen);
    m.insert("#6B8E23", Color::OliveDrab);
    m.insert("#808000", Color::Olive);
    m.insert("#555D50", Color::Ebony);
    m.insert("#556B2F", Color::DarkOliveGreen);
    m.insert("#4E5B31", Color::MilitaryGreen);
    m.insert("#3A5F0B", Color::GreenLeaves);
    m.insert("#4B5320", Color::ArmyGreen);
    m.insert("#667C26", Color::FernGreen);
    m.insert("#4E9258", Color::FallForestGreen);
    m.insert("#08A04B", Color::IrishGreen);
    m.insert("#387C44", Color::PineGreen);
    m.insert("#347235", Color::MediumForestGreen);
    m.insert("#27742C", Color::RacingGreen);
    m.insert("#347C2C", Color::JungleGreen);
    m.insert("#227442", Color::CactusGreen);
    m.insert("#228B22", Color::ForestGreen);
    m.insert("#008000", Color::Green);
    m.insert("#006400", Color::DarkGreen);
    m.insert("#056608", Color::DeepGreen);
    m.insert("#046307", Color::DeepEmeraldGreen);
    m.insert("#355E3B", Color::HunterGreen);
    m.insert("#254117", Color::DarkForestGreen);
    m.insert("#004225", Color::LotusGreen);
    m.insert("#026C3D", Color::BroccoliGreen);
    m.insert("#437C17", Color::SeaweedGreen);
    m.insert("#347C17", Color::ShamrockGreen);
    m.insert("#6AA121", Color::GreenOnion);
    m.insert("#8A9A5B", Color::MossGreen);
    m.insert("#3F9B0B", Color::GrassGreen);
    m.insert("#4AA02C", Color::GreenPepper);
    m.insert("#41A317", Color::DarkLimeGreen);
    m.insert("#12AD2B", Color::ParrotGreen);
    m.insert("#3EA055", Color::CloverGreen);
    m.insert("#73A16C", Color::DinosaurGreen);
    m.insert("#6CBB3C", Color::GreenSnake);
    m.insert("#6CC417", Color::AlienGreen);
    m.insert("#4CC417", Color::GreenApple);
    m.insert("#32CD32", Color::LimeGreen);
    m.insert("#52D017", Color::PeaGreen);
    m.insert("#4CC552", Color::KellyGreen);
    m.insert("#54C571", Color::ZombieGreen);
    m.insert("#89C35C", Color::GreenPeas);
    m.insert("#85BB65", Color::DollarBillGreen);
    m.insert("#99C68E", Color::FrogGreen);
    m.insert("#A0D6B4", Color::TurquoiseGreen);
    m.insert("#8FBC8F", Color::DarkSeaGreen);
    m.insert("#829F82", Color::BasilGreen);
    m.insert("#A2AD9C", Color::GrayGreen);
    m.insert("#B8BC86", Color::LightOliveGreen);
    m.insert("#9CB071", Color::IguanaGreen);
    m.insert("#8FB31D", Color::CitronGreen);
    m.insert("#B0BF1A", Color::AcidGreen);
    m.insert("#B2C248", Color::AvocadoGreen);
    m.insert("#9DC209", Color::PistachioGreen);
    m.insert("#A1C935", Color::SaladGreen);
    m.insert("#9ACD32", Color::YellowGreen);
    m.insert("#77DD77", Color::PastelGreen);
    m.insert("#7FE817", Color::HummingbirdGreen);
    m.insert("#59E817", Color::NebulaGreen);
    m.insert("#57E964", Color::StoplightGoGreen);
    m.insert("#16F529", Color::NeonGreen);
    m.insert("#5EFB6E", Color::JadeGreen);
    m.insert("#00FF7F", Color::SpringGreen);
    m.insert("#00FF80", Color::OceanGreen);
    m.insert("#36F57F", Color::LimeMintGreen);
    m.insert("#00FA9A", Color::MediumSpringGreen);
    m.insert("#12E193", Color::AquaGreen);
    m.insert("#5FFB17", Color::EmeraldGreen);
    m.insert("#00FF00", Color::Lime);
    m.insert("#7CFC00", Color::LawnGreen);
    m.insert("#66FF00", Color::BrightGreen);
    m.insert("#7FFF00", Color::Chartreuse);
    m.insert("#87F717", Color::YellowLawnGreen);
    m.insert("#98F516", Color::AloeVeraGreen);
    m.insert("#B1FB17", Color::DullGreenYellow);
    m.insert("#ADF802", Color::LemonGreen);
    m.insert("#ADFF2F", Color::GreenYellow);
    m.insert("#BDF516", Color::ChameleonGreen);
    m.insert("#DAEE01", Color::NeonYellowGreen);
    m.insert("#E2F516", Color::YellowGreenGrosbeak);
    m.insert("#CCFB5D", Color::TeaGreen);
    m.insert("#BCE954", Color::SlimeGreen);
    m.insert("#64E986", Color::AlgaeGreen);
    m.insert("#90EE90", Color::LightGreen);
    m.insert("#6AFB92", Color::DragonGreen);
    m.insert("#98FB98", Color::PaleGreen);
    m.insert("#98FF98", Color::MintGreen);
    m.insert("#B5EAAA", Color::GreenThumb);
    m.insert("#E3F9A6", Color::OrganicBrown);
    m.insert("#C3FDB8", Color::LightJade);
    m.insert("#C2E5D3", Color::LightMintGreen);
    m.insert("#DBF9DB", Color::LightRoseGreen);
    m.insert("#E8F1D4", Color::ChromeWhite);
    m.insert("#F0FFF0", Color::HoneyDew);
    m.insert("#F5FFFA", Color::MintCream);
    m.insert("#FFFACD", Color::LemonChiffon);
    m.insert("#FFFFC2", Color::Parchment);
    m.insert("#FFFFCC", Color::Cream);
    m.insert("#FFFDD0", Color::CreamWhite);
    m.insert("#FAFAD2", Color::LightGoldenRodYellow);
    m.insert("#FFFFE0", Color::LightYellow);
    m.insert("#F5F5DC", Color::Beige);
    m.insert("#F2F0DF", Color::WhiteYellow);
    m.insert("#FFF8DC", Color::Cornsilk);
    m.insert("#FBF6D9", Color::Blonde);
    m.insert("#FAEBD7", Color::AntiqueWhite);
    m.insert("#FFF0DB", Color::LightBeige);
    m.insert("#FFEFD5", Color::PapayaWhip);
    m.insert("#F7E7CE", Color::Champagne);
    m.insert("#FFEBCD", Color::BlanchedAlmond);
    m.insert("#FFE4C4", Color::Bisque);
    m.insert("#F5DEB3", Color::Wheat);
    m.insert("#FFE4B5", Color::Moccasin);
    m.insert("#FFE5B4", Color::Peach);
    m.insert("#FED8B1", Color::LightOrange);
    m.insert("#FFDAB9", Color::PeachPuff);
    m.insert("#FBD5AB", Color::CoralPeach);
    m.insert("#FFDEAD", Color::NavajoWhite);
    m.insert("#FBE7A1", Color::GoldenBlonde);
    m.insert("#F3E3C3", Color::GoldenSilk);
    m.insert("#F0E2B6", Color::DarkBlonde);
    m.insert("#F1E5AC", Color::LightGold);
    m.insert("#F3E5AB", Color::Vanilla);
    m.insert("#ECE5B6", Color::TanBrown);
    m.insert("#E8E4C9", Color::DirtyWhite);
    m.insert("#EEE8AA", Color::PaleGoldenRod);
    m.insert("#F0E68C", Color::Khaki);
    m.insert("#EDDA74", Color::CardboardBrown);
    m.insert("#EDE275", Color::HarvestGold);
    m.insert("#FFE87C", Color::SunYellow);
    m.insert("#FFF380", Color::CornYellow);
    m.insert("#FAF884", Color::PastelYellow);
    m.insert("#FFFF33", Color::NeonYellow);
    m.insert("#FFFF00", Color::Yellow);
    m.insert("#FEF250", Color::LemonYellow);
    m.insert("#FFEF00", Color::CanaryYellow);
    m.insert("#F5E216", Color::BananaYellow);
    m.insert("#FFDB58", Color::MustardYellow);
    m.insert("#FFDF00", Color::GoldenYellow);
    m.insert("#F9DB24", Color::BoldYellow);
    m.insert("#EED202", Color::SafetyYellow);
    m.insert("#FFD801", Color::RubberDuckyYellow);
    m.insert("#FFD700", Color::Gold);
    m.insert("#FDD017", Color::BrightGold);
    m.insert("#FFCE44", Color::ChromeGold);
    m.insert("#EAC117", Color::GoldenBrown);
    m.insert("#F6BE00", Color::DeepYellow);
    m.insert("#F2BB66", Color::MacaroniandCheese);
    m.insert("#FFBF00", Color::Amber);
    m.insert("#FBB917", Color::Saffron);
    m.insert("#FDBD01", Color::NeonGold);
    m.insert("#FBB117", Color::Beer);
    m.insert("#FFAE42", Color::YellowOrange);
    m.insert("#FFAE42", Color::OrangeYellow);
    m.insert("#FFA62F", Color::Cantaloupe);
    m.insert("#FFA600", Color::CheeseOrange);
    m.insert("#FFA500", Color::Orange);
    m.insert("#EE9A4D", Color::BrownSand);
    m.insert("#F4A460", Color::SandyBrown);
    m.insert("#E2A76F", Color::BrownSugar);
    m.insert("#C19A6B", Color::CamelBrown);
    m.insert("#E6BF83", Color::DeerBrown);
    m.insert("#DEB887", Color::BurlyWood);
    m.insert("#D2B48C", Color::Tan);
    m.insert("#C8AD7F", Color::LightFrenchBeige);
    m.insert("#C2B280", Color::Sand);
    m.insert("#C6BA8B", Color::SoftHazel);
    m.insert("#BCB88A", Color::Sage);
    m.insert("#C8B560", Color::FallLeafBrown);
    m.insert("#C9BE62", Color::GingerBrown);
    m.insert("#C9AE5D", Color::BronzeGold);
    m.insert("#BDB76B", Color::DarkKhaki);
    m.insert("#BAB86C", Color::OliveGreen);
    m.insert("#B5A642", Color::Brass);
    m.insert("#C7A317", Color::CookieBrown);
    m.insert("#D4AF37", Color::MetallicGold);
    m.insert("#E1AD01", Color::Mustard);
    m.insert("#E9AB17", Color::BeeYellow);
    m.insert("#E8A317", Color::SchoolBusYellow);
    m.insert("#DAA520", Color::GoldenRod);
    m.insert("#D4A017", Color::OrangeGold);
    m.insert("#C68E17", Color::Caramel);
    m.insert("#B8860B", Color::DarkGoldenRod);
    m.insert("#C58917", Color::Cinnamon);
    m.insert("#CD853F", Color::Peru);
    m.insert("#CD7F32", Color::Bronze);
    m.insert("#CA762B", Color::PumpkinPie);
    m.insert("#C88141", Color::TigerOrange);
    m.insert("#B87333", Color::Copper);
    m.insert("#AA6C39", Color::DarkGold);
    m.insert("#A97142", Color::MetallicBronze);
    m.insert("#AB784E", Color::DarkAlmond);
    m.insert("#966F33", Color::Wood);
    m.insert("#906E3E", Color::KhakiBrown);
    m.insert("#806517", Color::OakBrown);
    m.insert("#665D1E", Color::AntiqueBronze);
    m.insert("#8E7618", Color::Hazel);
    m.insert("#8B8000", Color::DarkYellow);
    m.insert("#827839", Color::DarkMoccasin);
    m.insert("#8A865D", Color::KhakiGreen);
    m.insert("#93917C", Color::MillenniumJade);
    m.insert("#9F8C76", Color::DarkBeige);
    m.insert("#AF9B60", Color::BulletShell);
    m.insert("#827B60", Color::ArmyBrown);
    m.insert("#786D5F", Color::Sandstone);
    m.insert("#483C32", Color::Taupe);
    m.insert("#4A412A", Color::DarkGrayishOlive);
    m.insert("#473810", Color::DarkHazelBrown);
    m.insert("#493D26", Color::Mocha);
    m.insert("#513B1C", Color::MilkChocolate);
    m.insert("#3D3635", Color::GrayBrown);
    m.insert("#3B2F2F", Color::DarkCoffee);
    m.insert("#49413F", Color::WesternCharcoal);
    m.insert("#43302E", Color::OldBurgundy);
    m.insert("#622F22", Color::RedBrown);
    m.insert("#5C3317", Color::BakersBrown);
    m.insert("#644117", Color::PullmanBrown);
    m.insert("#654321", Color::DarkBrown);
    m.insert("#704214", Color::SepiaBrown);
    m.insert("#804A00", Color::DarkBronze);
    m.insert("#6F4E37", Color::Coffee);
    m.insert("#835C3B", Color::BrownBear);
    m.insert("#7F5217", Color::RedDirt);
    m.insert("#7F462C", Color::Sepia);
    m.insert("#A0522D", Color::Sienna);
    m.insert("#8B4513", Color::SaddleBrown);
    m.insert("#8A4117", Color::DarkSienna);
    m.insert("#7E3817", Color::Sangria);
    m.insert("#7E3517", Color::BloodRed);
    m.insert("#954535", Color::Chestnut);
    m.insert("#9E4638", Color::CoralBrown);
    m.insert("#A05544", Color::DeepAmber);
    m.insert("#C34A2C", Color::ChestnutRed);
    m.insert("#B83C08", Color::GingerRed);
    m.insert("#C04000", Color::Mahogany);
    m.insert("#EB5406", Color::RedGold);
    m.insert("#C35817", Color::RedFox);
    m.insert("#B86500", Color::DarkBisque);
    m.insert("#B5651D", Color::LightBrown);
    m.insert("#B76734", Color::PetraGold);
    m.insert("#A55D35", Color::BrownRust);
    m.insert("#C36241", Color::Rust);
    m.insert("#CB6D51", Color::CopperRed);
    m.insert("#C47451", Color::OrangeSalmon);
    m.insert("#D2691E", Color::Chocolate);
    m.insert("#CC6600", Color::Sedona);
    m.insert("#E56717", Color::PapayaOrange);
    m.insert("#E66C2C", Color::HalloweenOrange);
    m.insert("#FF6700", Color::NeonOrange);
    m.insert("#FF5F1F", Color::BrightOrange);
    m.insert("#FE632A", Color::FluroOrange);
    m.insert("#F87217", Color::PumpkinOrange);
    m.insert("#FF7900", Color::SafetyOrange);
    m.insert("#F88017", Color::CarrotOrange);
    m.insert("#FF8C00", Color::DarkOrange);
    m.insert("#F87431", Color::ConstructionConeOrange);
    m.insert("#FF7722", Color::IndianSaffron);
    m.insert("#E67451", Color::SunriseOrange);
    m.insert("#FF8040", Color::MangoOrange);
    m.insert("#FF7F50", Color::Coral);
    m.insert("#F88158", Color::BasketBallOrange);
    m.insert("#F9966B", Color::LightSalmonRose);
    m.insert("#FFA07A", Color::LightSalmon);
    m.insert("#F89880", Color::PinkOrange);
    m.insert("#E9967A", Color::DarkSalmon);
    m.insert("#E78A61", Color::Tangerine);
    m.insert("#DA8A67", Color::LightCopper);
    m.insert("#FF8674", Color::SalmonPink);
    m.insert("#FA8072", Color::Salmon);
    m.insert("#F98B88", Color::PeachPink);
    m.insert("#F08080", Color::LightCoral);
    m.insert("#F67280", Color::PastelRed);
    m.insert("#E77471", Color::PinkCoral);
    m.insert("#F75D59", Color::BeanRed);
    m.insert("#E55451", Color::ValentineRed);
    m.insert("#CD5C5C", Color::IndianRed);
    m.insert("#FF6347", Color::Tomato);
    m.insert("#E55B3C", Color::ShockingOrange);
    m.insert("#FF4500", Color::OrangeRed);
    m.insert("#FF0000", Color::Red);
    m.insert("#FD1C03", Color::NeonRed);
    m.insert("#FF2400", Color::ScarletRed);
    m.insert("#F62217", Color::RubyRed);
    m.insert("#F70D1A", Color::FerrariRed);
    m.insert("#F62817", Color::FireEngineRed);
    m.insert("#E42217", Color::LavaRed);
    m.insert("#E41B17", Color::LoveRed);
    m.insert("#DC381F", Color::Grapefruit);
    m.insert("#C83F49", Color::StrawberryRed);
    m.insert("#C24641", Color::CherryRed);
    m.insert("#C11B17", Color::ChilliPepper);
    m.insert("#B22222", Color::FireBrick);
    m.insert("#B21807", Color::TomatoSauceRed);
    m.insert("#A52A2A", Color::Brown);
    m.insert("#A70D2A", Color::CarbonRed);
    m.insert("#9F000F", Color::Cranberry);
    m.insert("#931314", Color::SaffronRed);
    m.insert("#990000", Color::CrimsonRed);
    m.insert("#990012", Color::RedWine);
    m.insert("#990012", Color::WineRed);
    m.insert("#8B0000", Color::DarkRed);
    m.insert("#8F0B0B", Color::MaroonRed);
    m.insert("#800000", Color::Maroon);
    m.insert("#8C001A", Color::Burgundy);
    m.insert("#7E191B", Color::Vermilion);
    m.insert("#800517", Color::DeepRed);
    m.insert("#733635", Color::GarnetRed);
    m.insert("#660000", Color::RedBlood);
    m.insert("#551606", Color::BloodNight);
    m.insert("#560319", Color::DarkScarlet);
    m.insert("#3F000F", Color::ChocolateBrown);
    m.insert("#3D0C02", Color::BlackBean);
    m.insert("#2F0909", Color::DarkMaroon);
    m.insert("#2B1B17", Color::Midnight);
    m.insert("#550A35", Color::PurpleLily);
    m.insert("#810541", Color::PurpleMaroon);
    m.insert("#7D0541", Color::PlumPie);
    m.insert("#7D0552", Color::PlumVelvet);
    m.insert("#872657", Color::DarkRaspberry);
    m.insert("#7E354D", Color::VelvetMaroon);
    m.insert("#7F4E52", Color::RosyFinch);
    m.insert("#7F525D", Color::DullPurple);
    m.insert("#7F5A58", Color::Puce);
    m.insert("#997070", Color::RoseDust);
    m.insert("#B1907F", Color::PastelBrown);
    m.insert("#B38481", Color::RosyPink);
    m.insert("#BC8F8F", Color::RosyBrown);
    m.insert("#C5908E", Color::KhakiRose);
    m.insert("#C48793", Color::LipstickPink);
    m.insert("#CC7A8B", Color::DuskyPink);
    m.insert("#C48189", Color::PinkBrown);
    m.insert("#C08081", Color::OldRose);
    m.insert("#D58A94", Color::DustyPink);
    m.insert("#E799A3", Color::PinkDaisy);
    m.insert("#E8ADAA", Color::Rose);
    m.insert("#C9A9A6", Color::DustyRose);
    m.insert("#C4AEAD", Color::SilverPink);
    m.insert("#E6C7C2", Color::GoldPink);
    m.insert("#ECC5C0", Color::RoseGold);
    m.insert("#FFCBA4", Color::DeepPeach);
    m.insert("#F8B88B", Color::PastelOrange);
    m.insert("#EDC9AF", Color::DesertSand);
    m.insert("#FFDDCA", Color::UnbleachedSilk);
    m.insert("#FDD7E4", Color::PigPink);
    m.insert("#F2D4D7", Color::PalePink);
    m.insert("#FFE6E8", Color::Blush);
    m.insert("#FFE4E1", Color::MistyRose);
    m.insert("#FFDFDD", Color::PinkBubbleGum);
    m.insert("#FBCFCD", Color::LightRose);
    m.insert("#FFCCCB", Color::LightRed);
    m.insert("#F7CAC9", Color::RoseQuartz);
    m.insert("#F6C6BD", Color::WarmPink);
    m.insert("#FBBBB9", Color::DeepRose);
    m.insert("#FFC0CB", Color::Pink);
    m.insert("#FFB6C1", Color::LightPink);
    m.insert("#FFB8BF", Color::SoftPink);
    m.insert("#FFB2D0", Color::PowderPink);
    m.insert("#FAAFBE", Color::DonutPink);
    m.insert("#FAAFBA", Color::BabyPink);
    m.insert("#F9A7B0", Color::FlamingoPink);
    m.insert("#FEA3AA", Color::PastelPink);
    m.insert("#E7A1B0", Color::RosePink);
    m.insert("#E38AAE", Color::CadillacPink);
    m.insert("#F778A1", Color::CarnationPink);
    m.insert("#E5788F", Color::PastelRose);
    m.insert("#E56E94", Color::BlushRed);
    m.insert("#DB7093", Color::PaleVioletRed);
    m.insert("#D16587", Color::PurplePink);
    m.insert("#C25A7C", Color::TulipPink);
    m.insert("#C25283", Color::BashfulPink);
    m.insert("#E75480", Color::DarkPink);
    m.insert("#F660AB", Color::DarkHotPink);
    m.insert("#FF69B4", Color::HotPink);
    m.insert("#FC6C85", Color::WatermelonPink);
    m.insert("#F6358A", Color::VioletRed);
    m.insert("#F52887", Color::HotDeepPink);
    m.insert("#FF007F", Color::BrightPink);
    m.insert("#FF0080", Color::RedMagenta);
    m.insert("#FF1493", Color::DeepPink);
    m.insert("#F535AA", Color::NeonPink);
    m.insert("#FF33AA", Color::ChromePink);
    m.insert("#FD349C", Color::NeonHotPink);
    m.insert("#E45E9D", Color::PinkCupcake);
    m.insert("#E759AC", Color::RoyalPink);
    m.insert("#E3319D", Color::DimorphothecaMagenta);
    m.insert("#DA1884", Color::BarbiePink);
    m.insert("#E4287C", Color::PinkLemonade);
    m.insert("#FA2A55", Color::RedPink);
    m.insert("#E30B5D", Color::Raspberry);
    m.insert("#DC143C", Color::Crimson);
    m.insert("#C32148", Color::BrightMaroon);
    m.insert("#C21E56", Color::RoseRed);
    m.insert("#C12869", Color::RoguePink);
    m.insert("#C12267", Color::BurntPink);
    m.insert("#CA226B", Color::PinkViolet);
    m.insert("#CC338B", Color::MagentaPink);
    m.insert("#C71585", Color::MediumVioletRed);
    m.insert("#C12283", Color::DarkCarnationPink);
    m.insert("#B3446C", Color::RaspberryPurple);
    m.insert("#B93B8F", Color::PinkPlum);
    m.insert("#DA70D6", Color::Orchid);
    m.insert("#DF73D4", Color::DeepMauve);
    m.insert("#EE82EE", Color::Violet);
    m.insert("#FF77FF", Color::FuchsiaPink);
    m.insert("#F433FF", Color::BrightNeonPink);
    m.insert("#FF00FF", Color::Magenta);
    m.insert("#FF00FF", Color::Fuchsia);
    m.insert("#E238EC", Color::CrimsonPurple);
    m.insert("#D462FF", Color::HeliotropePurple);
    m.insert("#C45AEC", Color::TyrianPurple);
    m.insert("#BA55D3", Color::MediumOrchid);
    m.insert("#A74AC7", Color::PurpleFlower);
    m.insert("#B048B5", Color::OrchidPurple);
    m.insert("#B666D2", Color::RichLilac);
    m.insert("#D291BC", Color::PastelViolet);
    m.insert("#A17188", Color::Rosy);
    m.insert("#915F6D", Color::MauveTaupe);
    m.insert("#7E587E", Color::ViolaPurple);
    m.insert("#614051", Color::Eggplant);
    m.insert("#583759", Color::PlumPurple);
    m.insert("#5E5A80", Color::Grape);
    m.insert("#4E5180", Color::PurpleNavy);
    m.insert("#6A5ACD", Color::SlateBlue);
    m.insert("#6960EC", Color::BlueLotus);
    m.insert("#5865F2", Color::Blurple);
    m.insert("#736AFF", Color::LightSlateBlue);
    m.insert("#7B68EE", Color::MediumSlateBlue);
    m.insert("#7575CF", Color::PeriwinklePurple);
    m.insert("#6667AB", Color::VeryPeri);
    m.insert("#6F2DA8", Color::BrightGrape);
    m.insert("#6A0DAD", Color::BrightPurple);
    m.insert("#6C2DC7", Color::PurpleAmethyst);
    m.insert("#822EFF", Color::BlueMagenta);
    m.insert("#5539CC", Color::DarkBlurple);
    m.insert("#5453A6", Color::DeepPeriwinkle);
    m.insert("#483D8B", Color::DarkSlateBlue);
    m.insert("#4E387E", Color::PurpleHaze);
    m.insert("#571B7E", Color::PurpleIris);
    m.insert("#4B0150", Color::DarkPurple);
    m.insert("#36013F", Color::DeepPurple);
    m.insert("#2E1A47", Color::MidnightPurple);
    m.insert("#461B7E", Color::PurpleMonster);
    m.insert("#4B0082", Color::Indigo);
    m.insert("#342D7E", Color::BlueWhale);
    m.insert("#663399", Color::RebeccaPurple);
    m.insert("#6A287E", Color::PurpleJam);
    m.insert("#8B008B", Color::DarkMagenta);
    m.insert("#800080", Color::Purple);
    m.insert("#86608E", Color::FrenchLilac);
    m.insert("#9932CC", Color::DarkOrchid);
    m.insert("#9400D3", Color::DarkViolet);
    m.insert("#8D38C9", Color::PurpleViolet);
    m.insert("#A23BEC", Color::JasminePurple);
    m.insert("#B041FF", Color::PurpleDaffodil);
    m.insert("#842DCE", Color::ClematisViolet);
    m.insert("#8A2BE2", Color::BlueViolet);
    m.insert("#7A5DC7", Color::PurpleSageBush);
    m.insert("#7F38EC", Color::LovelyPurple);
    m.insert("#9D00FF", Color::NeonPurple);
    m.insert("#8E35EF", Color::PurplePlum);
    m.insert("#893BFF", Color::AztechPurple);
    m.insert("#9370DB", Color::MediumPurple);
    m.insert("#8467D7", Color::LightPurple);
    m.insert("#9172EC", Color::CrocusPurple);
    m.insert("#9E7BFF", Color::PurpleMimosa);
    m.insert("#8686AF", Color::PastelIndigo);
    m.insert("#967BB6", Color::LavenderPurple);
    m.insert("#B09FCA", Color::RosePurple);
    m.insert("#C8C4DF", Color::Viola);
    m.insert("#CCCCFF", Color::Periwinkle);
    m.insert("#DCD0FF", Color::PaleLilac);
    m.insert("#C8A2C8", Color::Lilac);
    m.insert("#E0B0FF", Color::Mauve);
    m.insert("#D891EF", Color::BrightLilac);
    m.insert("#C38EC7", Color::PurpleDragon);
    m.insert("#DDA0DD", Color::Plum);
    m.insert("#E6A9EC", Color::BlushPink);
    m.insert("#F2A2E8", Color::PastelPurple);
    m.insert("#F9B7FF", Color::BlossomPink);
    m.insert("#C6AEC7", Color::WisteriaPurple);
    m.insert("#D2B9D3", Color::PurpleThistle);
    m.insert("#D8BFD8", Color::Thistle);
    m.insert("#DFD3E3", Color::PurpleWhite);
    m.insert("#E9CFEC", Color::PeriwinklePink);
    m.insert("#FCDFFF", Color::CottonCandy);
    m.insert("#EBDDE2", Color::LavenderPinocchio);
    m.insert("#E1D9D1", Color::DarkWhite);
    m.insert("#E9E4D4", Color::AshWhite);
    m.insert("#EFEBD8", Color::WarmWhite);
    m.insert("#EDE6D6", Color::WhiteChocolate);
    m.insert("#F0E9D6", Color::CreamyWhite);
    m.insert("#F8F0E3", Color::OffWhite);
    m.insert("#FAF0DD", Color::SoftIvory);
    m.insert("#FFF8E7", Color::CosmicLatte);
    m.insert("#F8F6F0", Color::PearlWhite);
    m.insert("#F3E8EA", Color::RedWhite);
    m.insert("#FFF0F5", Color::LavenderBlush);
    m.insert("#FDEEF4", Color::Pearl);
    m.insert("#FFF9E3", Color::EggShell);
    m.insert("#FEF0E3", Color::OldLace);
    m.insert("#EAEEE9", Color::WhiteIce);
    m.insert("#FAF0E6", Color::Linen);
    m.insert("#FFF5EE", Color::SeaShell);
    m.insert("#F9F6EE", Color::BoneWhite);
    m.insert("#FAF5EF", Color::Rice);
    m.insert("#FFFAF0", Color::FloralWhite);
    m.insert("#FFFFF0", Color::Ivory);
    m.insert("#FFFFF4", Color::WhiteGold);
    m.insert("#FFFFF7", Color::LightWhite);
    m.insert("#FBFBF9", Color::Cotton);
    m.insert("#FFFAFA", Color::Snow);
    m.insert("#FEFCFF", Color::MilkWhite);
    m.insert("#FFFEFA", Color::HalfWhite);
    m.insert("#FFFFFF", Color::White);
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
                .copied()
                .or(Some(Color::Rgb(r, g, b)))
        } else {
            NAME_MAP.get(&norm_name(name)).and_then(|&idx| {
                if idx == 0usize {
                    Some(Color::Black)
                } else if idx == 1usize {
                    Some(Color::BlackBlue)
                } else if idx == 2usize {
                    Some(Color::Night)
                } else if idx == 3usize {
                    Some(Color::Charcoal)
                } else if idx == 4usize {
                    Some(Color::Oil)
                } else if idx == 5usize {
                    Some(Color::StormyGray)
                } else if idx == 6usize {
                    Some(Color::LightBlack)
                } else if idx == 7usize {
                    Some(Color::DarkSteampunk)
                } else if idx == 8usize {
                    Some(Color::BlackCat)
                } else if idx == 9usize {
                    Some(Color::Iridium)
                } else if idx == 10usize {
                    Some(Color::BlackEel)
                } else if idx == 11usize {
                    Some(Color::BlackCow)
                } else if idx == 12usize {
                    Some(Color::GrayWolf)
                } else if idx == 13usize {
                    Some(Color::VampireGray)
                } else if idx == 14usize {
                    Some(Color::IronGray)
                } else if idx == 15usize {
                    Some(Color::GrayDolphin)
                } else if idx == 16usize {
                    Some(Color::CarbonGray)
                } else if idx == 17usize {
                    Some(Color::AshGray)
                } else if idx == 18usize {
                    Some(Color::DimGray)
                } else if idx == 19usize {
                    Some(Color::DimGrey)
                } else if idx == 20usize {
                    Some(Color::NardoGray)
                } else if idx == 21usize {
                    Some(Color::CloudyGray)
                } else if idx == 22usize {
                    Some(Color::SmokeyGray)
                } else if idx == 23usize {
                    Some(Color::AlienGray)
                } else if idx == 24usize {
                    Some(Color::SonicSilver)
                } else if idx == 25usize {
                    Some(Color::PlatinumGray)
                } else if idx == 26usize {
                    Some(Color::Granite)
                } else if idx == 27usize {
                    Some(Color::Gray)
                } else if idx == 28usize {
                    Some(Color::Grey)
                } else if idx == 29usize {
                    Some(Color::BattleshipGray)
                } else if idx == 30usize {
                    Some(Color::SheetMetal)
                } else if idx == 31usize {
                    Some(Color::DarkGainsboro)
                } else if idx == 32usize {
                    Some(Color::GunmetalGray)
                } else if idx == 33usize {
                    Some(Color::ColdMetal)
                } else if idx == 34usize {
                    Some(Color::StainlessSteelGray)
                } else if idx == 35usize {
                    Some(Color::DarkGrey)
                } else if idx == 36usize {
                    Some(Color::DarkGray)
                } else if idx == 37usize {
                    Some(Color::ChromeAluminum)
                } else if idx == 38usize {
                    Some(Color::GrayCloud)
                } else if idx == 39usize {
                    Some(Color::Metal)
                } else if idx == 40usize {
                    Some(Color::Silver)
                } else if idx == 41usize {
                    Some(Color::Steampunk)
                } else if idx == 42usize {
                    Some(Color::PaleSilver)
                } else if idx == 43usize {
                    Some(Color::GearSteelGray)
                } else if idx == 44usize {
                    Some(Color::GrayGoose)
                } else if idx == 45usize {
                    Some(Color::PlatinumSilver)
                } else if idx == 46usize {
                    Some(Color::LightGrey)
                } else if idx == 47usize {
                    Some(Color::LightGray)
                } else if idx == 48usize {
                    Some(Color::SilverWhite)
                } else if idx == 49usize {
                    Some(Color::Gainsboro)
                } else if idx == 50usize {
                    Some(Color::LightSteelGray)
                } else if idx == 51usize {
                    Some(Color::WhiteSmoke)
                } else if idx == 52usize {
                    Some(Color::WhiteGray)
                } else if idx == 53usize {
                    Some(Color::Platinum)
                } else if idx == 54usize {
                    Some(Color::MetallicSilver)
                } else if idx == 55usize {
                    Some(Color::BlueGray)
                } else if idx == 56usize {
                    Some(Color::RomanSilver)
                } else if idx == 57usize {
                    Some(Color::LightSlateGrey)
                } else if idx == 58usize {
                    Some(Color::LightSlateGray)
                } else if idx == 59usize {
                    Some(Color::SlateGrey)
                } else if idx == 60usize {
                    Some(Color::SlateGray)
                } else if idx == 61usize {
                    Some(Color::RatGray)
                } else if idx == 62usize {
                    Some(Color::SlateGraniteGray)
                } else if idx == 63usize {
                    Some(Color::JetGray)
                } else if idx == 64usize {
                    Some(Color::MistBlue)
                } else if idx == 65usize {
                    Some(Color::SteelGray)
                } else if idx == 66usize {
                    Some(Color::MarbleBlue)
                } else if idx == 67usize {
                    Some(Color::SlateBlueGray)
                } else if idx == 68usize {
                    Some(Color::LightPurpleBlue)
                } else if idx == 69usize {
                    Some(Color::AzureBlue)
                } else if idx == 70usize {
                    Some(Color::EstorilBlue)
                } else if idx == 71usize {
                    Some(Color::BlueJay)
                } else if idx == 72usize {
                    Some(Color::CharcoalBlue)
                } else if idx == 73usize {
                    Some(Color::DarkBlueGray)
                } else if idx == 74usize {
                    Some(Color::DarkSlate)
                } else if idx == 75usize {
                    Some(Color::DeepSeaBlue)
                } else if idx == 76usize {
                    Some(Color::NightBlue)
                } else if idx == 77usize {
                    Some(Color::MidnightBlue)
                } else if idx == 78usize {
                    Some(Color::Navy)
                } else if idx == 79usize {
                    Some(Color::DenimDarkBlue)
                } else if idx == 80usize {
                    Some(Color::DarkBlue)
                } else if idx == 81usize {
                    Some(Color::LapisBlue)
                } else if idx == 82usize {
                    Some(Color::NewMidnightBlue)
                } else if idx == 83usize {
                    Some(Color::EarthBlue)
                } else if idx == 84usize {
                    Some(Color::CobaltBlue)
                } else if idx == 85usize {
                    Some(Color::MediumBlue)
                } else if idx == 86usize {
                    Some(Color::BlueberryBlue)
                } else if idx == 87usize {
                    Some(Color::CanaryBlue)
                } else if idx == 88usize {
                    Some(Color::Blue)
                } else if idx == 89usize {
                    Some(Color::SamcoBlue)
                } else if idx == 90usize {
                    Some(Color::BrightBlue)
                } else if idx == 91usize {
                    Some(Color::BlueOrchid)
                } else if idx == 92usize {
                    Some(Color::SapphireBlue)
                } else if idx == 93usize {
                    Some(Color::BlueEyes)
                } else if idx == 94usize {
                    Some(Color::BrightNavyBlue)
                } else if idx == 95usize {
                    Some(Color::BalloonBlue)
                } else if idx == 96usize {
                    Some(Color::RoyalBlue)
                } else if idx == 97usize {
                    Some(Color::OceanBlue)
                } else if idx == 98usize {
                    Some(Color::DarkSkyBlue)
                } else if idx == 99usize {
                    Some(Color::BlueRibbon)
                } else if idx == 100usize {
                    Some(Color::BlueDress)
                } else if idx == 101usize {
                    Some(Color::NeonBlue)
                } else if idx == 102usize {
                    Some(Color::DodgerBlue)
                } else if idx == 103usize {
                    Some(Color::GlacialBlueIce)
                } else if idx == 104usize {
                    Some(Color::SteelBlue)
                } else if idx == 105usize {
                    Some(Color::SilkBlue)
                } else if idx == 106usize {
                    Some(Color::WindowsBlue)
                } else if idx == 107usize {
                    Some(Color::BlueIvy)
                } else if idx == 108usize {
                    Some(Color::CyanBlue)
                } else if idx == 109usize {
                    Some(Color::BlueKoi)
                } else if idx == 110usize {
                    Some(Color::ColumbiaBlue)
                } else if idx == 111usize {
                    Some(Color::BabyBlue)
                } else if idx == 112usize {
                    Some(Color::CornflowerBlue)
                } else if idx == 113usize {
                    Some(Color::SkyBlueDress)
                } else if idx == 114usize {
                    Some(Color::Iceberg)
                } else if idx == 115usize {
                    Some(Color::ButterflyBlue)
                } else if idx == 116usize {
                    Some(Color::DeepSkyBlue)
                } else if idx == 117usize {
                    Some(Color::MiddayBlue)
                } else if idx == 118usize {
                    Some(Color::CrystalBlue)
                } else if idx == 119usize {
                    Some(Color::DenimBlue)
                } else if idx == 120usize {
                    Some(Color::DaySkyBlue)
                } else if idx == 121usize {
                    Some(Color::LightSkyBlue)
                } else if idx == 122usize {
                    Some(Color::SkyBlue)
                } else if idx == 123usize {
                    Some(Color::JeansBlue)
                } else if idx == 124usize {
                    Some(Color::BlueAngel)
                } else if idx == 125usize {
                    Some(Color::PastelBlue)
                } else if idx == 126usize {
                    Some(Color::LightDayBlue)
                } else if idx == 127usize {
                    Some(Color::SeaBlue)
                } else if idx == 128usize {
                    Some(Color::HeavenlyBlue)
                } else if idx == 129usize {
                    Some(Color::RobinEggBlue)
                } else if idx == 130usize {
                    Some(Color::PowderBlue)
                } else if idx == 131usize {
                    Some(Color::CoralBlue)
                } else if idx == 132usize {
                    Some(Color::LightBlue)
                } else if idx == 133usize {
                    Some(Color::LightSteelBlue)
                } else if idx == 134usize {
                    Some(Color::GulfBlue)
                } else if idx == 135usize {
                    Some(Color::PastelLightBlue)
                } else if idx == 136usize {
                    Some(Color::LavenderBlue)
                } else if idx == 137usize {
                    Some(Color::WhiteBlue)
                } else if idx == 138usize {
                    Some(Color::Lavender)
                } else if idx == 139usize {
                    Some(Color::Water)
                } else if idx == 140usize {
                    Some(Color::AliceBlue)
                } else if idx == 141usize {
                    Some(Color::GhostWhite)
                } else if idx == 142usize {
                    Some(Color::Azure)
                } else if idx == 143usize {
                    Some(Color::LightCyan)
                } else if idx == 144usize {
                    Some(Color::LightSlate)
                } else if idx == 145usize {
                    Some(Color::ElectricBlue)
                } else if idx == 146usize {
                    Some(Color::TronBlue)
                } else if idx == 147usize {
                    Some(Color::BlueZircon)
                } else if idx == 148usize {
                    Some(Color::Cyan)
                } else if idx == 149usize {
                    Some(Color::Aqua)
                } else if idx == 150usize {
                    Some(Color::BrightCyan)
                } else if idx == 151usize {
                    Some(Color::Celeste)
                } else if idx == 152usize {
                    Some(Color::BlueDiamond)
                } else if idx == 153usize {
                    Some(Color::BrightTurquoise)
                } else if idx == 154usize {
                    Some(Color::BlueLagoon)
                } else if idx == 155usize {
                    Some(Color::PaleTurquoise)
                } else if idx == 156usize {
                    Some(Color::PaleBlueLily)
                } else if idx == 157usize {
                    Some(Color::LightTeal)
                } else if idx == 158usize {
                    Some(Color::TiffanyBlue)
                } else if idx == 159usize {
                    Some(Color::BlueHosta)
                } else if idx == 160usize {
                    Some(Color::CyanOpaque)
                } else if idx == 161usize {
                    Some(Color::NorthernLightsBlue)
                } else if idx == 162usize {
                    Some(Color::BlueGreen)
                } else if idx == 163usize {
                    Some(Color::MediumAquaMarine)
                } else if idx == 164usize {
                    Some(Color::AquaSeafoamGreen)
                } else if idx == 165usize {
                    Some(Color::MagicMint)
                } else if idx == 166usize {
                    Some(Color::LightAquamarine)
                } else if idx == 167usize {
                    Some(Color::Aquamarine)
                } else if idx == 168usize {
                    Some(Color::BrightTeal)
                } else if idx == 169usize {
                    Some(Color::Turquoise)
                } else if idx == 170usize {
                    Some(Color::MediumTurquoise)
                } else if idx == 171usize {
                    Some(Color::DeepTurquoise)
                } else if idx == 172usize {
                    Some(Color::Jellyfish)
                } else if idx == 173usize {
                    Some(Color::BlueTurquoise)
                } else if idx == 174usize {
                    Some(Color::DarkTurquoise)
                } else if idx == 175usize {
                    Some(Color::MacawBlueGreen)
                } else if idx == 176usize {
                    Some(Color::LightSeaGreen)
                } else if idx == 177usize {
                    Some(Color::SeafoamGreen)
                } else if idx == 178usize {
                    Some(Color::CadetBlue)
                } else if idx == 179usize {
                    Some(Color::DeepSea)
                } else if idx == 180usize {
                    Some(Color::DarkCyan)
                } else if idx == 181usize {
                    Some(Color::TealGreen)
                } else if idx == 182usize {
                    Some(Color::Teal)
                } else if idx == 183usize {
                    Some(Color::TealBlue)
                } else if idx == 184usize {
                    Some(Color::MediumTeal)
                } else if idx == 185usize {
                    Some(Color::DarkTeal)
                } else if idx == 186usize {
                    Some(Color::DeepTeal)
                } else if idx == 187usize {
                    Some(Color::DarkSlateGray)
                } else if idx == 188usize {
                    Some(Color::DarkSlateGrey)
                } else if idx == 189usize {
                    Some(Color::Gunmetal)
                } else if idx == 190usize {
                    Some(Color::BlueMossGreen)
                } else if idx == 191usize {
                    Some(Color::BeetleGreen)
                } else if idx == 192usize {
                    Some(Color::GrayishTurquoise)
                } else if idx == 193usize {
                    Some(Color::GreenishBlue)
                } else if idx == 194usize {
                    Some(Color::AquamarineStone)
                } else if idx == 195usize {
                    Some(Color::SeaTurtleGreen)
                } else if idx == 196usize {
                    Some(Color::DullSeaGreen)
                } else if idx == 197usize {
                    Some(Color::DarkGreenBlue)
                } else if idx == 198usize {
                    Some(Color::DeepSeaGreen)
                } else if idx == 199usize {
                    Some(Color::BottleGreen)
                } else if idx == 200usize {
                    Some(Color::SeaGreen)
                } else if idx == 201usize {
                    Some(Color::ElfGreen)
                } else if idx == 202usize {
                    Some(Color::DarkMint)
                } else if idx == 203usize {
                    Some(Color::Jade)
                } else if idx == 204usize {
                    Some(Color::EarthGreen)
                } else if idx == 205usize {
                    Some(Color::ChromeGreen)
                } else if idx == 206usize {
                    Some(Color::Mint)
                } else if idx == 207usize {
                    Some(Color::Emerald)
                } else if idx == 208usize {
                    Some(Color::IsleOfManGreen)
                } else if idx == 209usize {
                    Some(Color::MediumSeaGreen)
                } else if idx == 210usize {
                    Some(Color::MetallicGreen)
                } else if idx == 211usize {
                    Some(Color::CamouflageGreen)
                } else if idx == 212usize {
                    Some(Color::SageGreen)
                } else if idx == 213usize {
                    Some(Color::HazelGreen)
                } else if idx == 214usize {
                    Some(Color::VenomGreen)
                } else if idx == 215usize {
                    Some(Color::OliveDrab)
                } else if idx == 216usize {
                    Some(Color::Olive)
                } else if idx == 217usize {
                    Some(Color::Ebony)
                } else if idx == 218usize {
                    Some(Color::DarkOliveGreen)
                } else if idx == 219usize {
                    Some(Color::MilitaryGreen)
                } else if idx == 220usize {
                    Some(Color::GreenLeaves)
                } else if idx == 221usize {
                    Some(Color::ArmyGreen)
                } else if idx == 222usize {
                    Some(Color::FernGreen)
                } else if idx == 223usize {
                    Some(Color::FallForestGreen)
                } else if idx == 224usize {
                    Some(Color::IrishGreen)
                } else if idx == 225usize {
                    Some(Color::PineGreen)
                } else if idx == 226usize {
                    Some(Color::MediumForestGreen)
                } else if idx == 227usize {
                    Some(Color::RacingGreen)
                } else if idx == 228usize {
                    Some(Color::JungleGreen)
                } else if idx == 229usize {
                    Some(Color::CactusGreen)
                } else if idx == 230usize {
                    Some(Color::ForestGreen)
                } else if idx == 231usize {
                    Some(Color::Green)
                } else if idx == 232usize {
                    Some(Color::DarkGreen)
                } else if idx == 233usize {
                    Some(Color::DeepGreen)
                } else if idx == 234usize {
                    Some(Color::DeepEmeraldGreen)
                } else if idx == 235usize {
                    Some(Color::HunterGreen)
                } else if idx == 236usize {
                    Some(Color::DarkForestGreen)
                } else if idx == 237usize {
                    Some(Color::LotusGreen)
                } else if idx == 238usize {
                    Some(Color::BroccoliGreen)
                } else if idx == 239usize {
                    Some(Color::SeaweedGreen)
                } else if idx == 240usize {
                    Some(Color::ShamrockGreen)
                } else if idx == 241usize {
                    Some(Color::GreenOnion)
                } else if idx == 242usize {
                    Some(Color::MossGreen)
                } else if idx == 243usize {
                    Some(Color::GrassGreen)
                } else if idx == 244usize {
                    Some(Color::GreenPepper)
                } else if idx == 245usize {
                    Some(Color::DarkLimeGreen)
                } else if idx == 246usize {
                    Some(Color::ParrotGreen)
                } else if idx == 247usize {
                    Some(Color::CloverGreen)
                } else if idx == 248usize {
                    Some(Color::DinosaurGreen)
                } else if idx == 249usize {
                    Some(Color::GreenSnake)
                } else if idx == 250usize {
                    Some(Color::AlienGreen)
                } else if idx == 251usize {
                    Some(Color::GreenApple)
                } else if idx == 252usize {
                    Some(Color::LimeGreen)
                } else if idx == 253usize {
                    Some(Color::PeaGreen)
                } else if idx == 254usize {
                    Some(Color::KellyGreen)
                } else if idx == 255usize {
                    Some(Color::ZombieGreen)
                } else if idx == 256usize {
                    Some(Color::GreenPeas)
                } else if idx == 257usize {
                    Some(Color::DollarBillGreen)
                } else if idx == 258usize {
                    Some(Color::FrogGreen)
                } else if idx == 259usize {
                    Some(Color::TurquoiseGreen)
                } else if idx == 260usize {
                    Some(Color::DarkSeaGreen)
                } else if idx == 261usize {
                    Some(Color::BasilGreen)
                } else if idx == 262usize {
                    Some(Color::GrayGreen)
                } else if idx == 263usize {
                    Some(Color::LightOliveGreen)
                } else if idx == 264usize {
                    Some(Color::IguanaGreen)
                } else if idx == 265usize {
                    Some(Color::CitronGreen)
                } else if idx == 266usize {
                    Some(Color::AcidGreen)
                } else if idx == 267usize {
                    Some(Color::AvocadoGreen)
                } else if idx == 268usize {
                    Some(Color::PistachioGreen)
                } else if idx == 269usize {
                    Some(Color::SaladGreen)
                } else if idx == 270usize {
                    Some(Color::YellowGreen)
                } else if idx == 271usize {
                    Some(Color::PastelGreen)
                } else if idx == 272usize {
                    Some(Color::HummingbirdGreen)
                } else if idx == 273usize {
                    Some(Color::NebulaGreen)
                } else if idx == 274usize {
                    Some(Color::StoplightGoGreen)
                } else if idx == 275usize {
                    Some(Color::NeonGreen)
                } else if idx == 276usize {
                    Some(Color::JadeGreen)
                } else if idx == 277usize {
                    Some(Color::SpringGreen)
                } else if idx == 278usize {
                    Some(Color::OceanGreen)
                } else if idx == 279usize {
                    Some(Color::LimeMintGreen)
                } else if idx == 280usize {
                    Some(Color::MediumSpringGreen)
                } else if idx == 281usize {
                    Some(Color::AquaGreen)
                } else if idx == 282usize {
                    Some(Color::EmeraldGreen)
                } else if idx == 283usize {
                    Some(Color::Lime)
                } else if idx == 284usize {
                    Some(Color::LawnGreen)
                } else if idx == 285usize {
                    Some(Color::BrightGreen)
                } else if idx == 286usize {
                    Some(Color::Chartreuse)
                } else if idx == 287usize {
                    Some(Color::YellowLawnGreen)
                } else if idx == 288usize {
                    Some(Color::AloeVeraGreen)
                } else if idx == 289usize {
                    Some(Color::DullGreenYellow)
                } else if idx == 290usize {
                    Some(Color::LemonGreen)
                } else if idx == 291usize {
                    Some(Color::GreenYellow)
                } else if idx == 292usize {
                    Some(Color::ChameleonGreen)
                } else if idx == 293usize {
                    Some(Color::NeonYellowGreen)
                } else if idx == 294usize {
                    Some(Color::YellowGreenGrosbeak)
                } else if idx == 295usize {
                    Some(Color::TeaGreen)
                } else if idx == 296usize {
                    Some(Color::SlimeGreen)
                } else if idx == 297usize {
                    Some(Color::AlgaeGreen)
                } else if idx == 298usize {
                    Some(Color::LightGreen)
                } else if idx == 299usize {
                    Some(Color::DragonGreen)
                } else if idx == 300usize {
                    Some(Color::PaleGreen)
                } else if idx == 301usize {
                    Some(Color::MintGreen)
                } else if idx == 302usize {
                    Some(Color::GreenThumb)
                } else if idx == 303usize {
                    Some(Color::OrganicBrown)
                } else if idx == 304usize {
                    Some(Color::LightJade)
                } else if idx == 305usize {
                    Some(Color::LightMintGreen)
                } else if idx == 306usize {
                    Some(Color::LightRoseGreen)
                } else if idx == 307usize {
                    Some(Color::ChromeWhite)
                } else if idx == 308usize {
                    Some(Color::HoneyDew)
                } else if idx == 309usize {
                    Some(Color::MintCream)
                } else if idx == 310usize {
                    Some(Color::LemonChiffon)
                } else if idx == 311usize {
                    Some(Color::Parchment)
                } else if idx == 312usize {
                    Some(Color::Cream)
                } else if idx == 313usize {
                    Some(Color::CreamWhite)
                } else if idx == 314usize {
                    Some(Color::LightGoldenRodYellow)
                } else if idx == 315usize {
                    Some(Color::LightYellow)
                } else if idx == 316usize {
                    Some(Color::Beige)
                } else if idx == 317usize {
                    Some(Color::WhiteYellow)
                } else if idx == 318usize {
                    Some(Color::Cornsilk)
                } else if idx == 319usize {
                    Some(Color::Blonde)
                } else if idx == 320usize {
                    Some(Color::AntiqueWhite)
                } else if idx == 321usize {
                    Some(Color::LightBeige)
                } else if idx == 322usize {
                    Some(Color::PapayaWhip)
                } else if idx == 323usize {
                    Some(Color::Champagne)
                } else if idx == 324usize {
                    Some(Color::BlanchedAlmond)
                } else if idx == 325usize {
                    Some(Color::Bisque)
                } else if idx == 326usize {
                    Some(Color::Wheat)
                } else if idx == 327usize {
                    Some(Color::Moccasin)
                } else if idx == 328usize {
                    Some(Color::Peach)
                } else if idx == 329usize {
                    Some(Color::LightOrange)
                } else if idx == 330usize {
                    Some(Color::PeachPuff)
                } else if idx == 331usize {
                    Some(Color::CoralPeach)
                } else if idx == 332usize {
                    Some(Color::NavajoWhite)
                } else if idx == 333usize {
                    Some(Color::GoldenBlonde)
                } else if idx == 334usize {
                    Some(Color::GoldenSilk)
                } else if idx == 335usize {
                    Some(Color::DarkBlonde)
                } else if idx == 336usize {
                    Some(Color::LightGold)
                } else if idx == 337usize {
                    Some(Color::Vanilla)
                } else if idx == 338usize {
                    Some(Color::TanBrown)
                } else if idx == 339usize {
                    Some(Color::DirtyWhite)
                } else if idx == 340usize {
                    Some(Color::PaleGoldenRod)
                } else if idx == 341usize {
                    Some(Color::Khaki)
                } else if idx == 342usize {
                    Some(Color::CardboardBrown)
                } else if idx == 343usize {
                    Some(Color::HarvestGold)
                } else if idx == 344usize {
                    Some(Color::SunYellow)
                } else if idx == 345usize {
                    Some(Color::CornYellow)
                } else if idx == 346usize {
                    Some(Color::PastelYellow)
                } else if idx == 347usize {
                    Some(Color::NeonYellow)
                } else if idx == 348usize {
                    Some(Color::Yellow)
                } else if idx == 349usize {
                    Some(Color::LemonYellow)
                } else if idx == 350usize {
                    Some(Color::CanaryYellow)
                } else if idx == 351usize {
                    Some(Color::BananaYellow)
                } else if idx == 352usize {
                    Some(Color::MustardYellow)
                } else if idx == 353usize {
                    Some(Color::GoldenYellow)
                } else if idx == 354usize {
                    Some(Color::BoldYellow)
                } else if idx == 355usize {
                    Some(Color::SafetyYellow)
                } else if idx == 356usize {
                    Some(Color::RubberDuckyYellow)
                } else if idx == 357usize {
                    Some(Color::Gold)
                } else if idx == 358usize {
                    Some(Color::BrightGold)
                } else if idx == 359usize {
                    Some(Color::ChromeGold)
                } else if idx == 360usize {
                    Some(Color::GoldenBrown)
                } else if idx == 361usize {
                    Some(Color::DeepYellow)
                } else if idx == 362usize {
                    Some(Color::MacaroniandCheese)
                } else if idx == 363usize {
                    Some(Color::Amber)
                } else if idx == 364usize {
                    Some(Color::Saffron)
                } else if idx == 365usize {
                    Some(Color::NeonGold)
                } else if idx == 366usize {
                    Some(Color::Beer)
                } else if idx == 367usize {
                    Some(Color::YellowOrange)
                } else if idx == 368usize {
                    Some(Color::OrangeYellow)
                } else if idx == 369usize {
                    Some(Color::Cantaloupe)
                } else if idx == 370usize {
                    Some(Color::CheeseOrange)
                } else if idx == 371usize {
                    Some(Color::Orange)
                } else if idx == 372usize {
                    Some(Color::BrownSand)
                } else if idx == 373usize {
                    Some(Color::SandyBrown)
                } else if idx == 374usize {
                    Some(Color::BrownSugar)
                } else if idx == 375usize {
                    Some(Color::CamelBrown)
                } else if idx == 376usize {
                    Some(Color::DeerBrown)
                } else if idx == 377usize {
                    Some(Color::BurlyWood)
                } else if idx == 378usize {
                    Some(Color::Tan)
                } else if idx == 379usize {
                    Some(Color::LightFrenchBeige)
                } else if idx == 380usize {
                    Some(Color::Sand)
                } else if idx == 381usize {
                    Some(Color::SoftHazel)
                } else if idx == 382usize {
                    Some(Color::Sage)
                } else if idx == 383usize {
                    Some(Color::FallLeafBrown)
                } else if idx == 384usize {
                    Some(Color::GingerBrown)
                } else if idx == 385usize {
                    Some(Color::BronzeGold)
                } else if idx == 386usize {
                    Some(Color::DarkKhaki)
                } else if idx == 387usize {
                    Some(Color::OliveGreen)
                } else if idx == 388usize {
                    Some(Color::Brass)
                } else if idx == 389usize {
                    Some(Color::CookieBrown)
                } else if idx == 390usize {
                    Some(Color::MetallicGold)
                } else if idx == 391usize {
                    Some(Color::Mustard)
                } else if idx == 392usize {
                    Some(Color::BeeYellow)
                } else if idx == 393usize {
                    Some(Color::SchoolBusYellow)
                } else if idx == 394usize {
                    Some(Color::GoldenRod)
                } else if idx == 395usize {
                    Some(Color::OrangeGold)
                } else if idx == 396usize {
                    Some(Color::Caramel)
                } else if idx == 397usize {
                    Some(Color::DarkGoldenRod)
                } else if idx == 398usize {
                    Some(Color::Cinnamon)
                } else if idx == 399usize {
                    Some(Color::Peru)
                } else if idx == 400usize {
                    Some(Color::Bronze)
                } else if idx == 401usize {
                    Some(Color::PumpkinPie)
                } else if idx == 402usize {
                    Some(Color::TigerOrange)
                } else if idx == 403usize {
                    Some(Color::Copper)
                } else if idx == 404usize {
                    Some(Color::DarkGold)
                } else if idx == 405usize {
                    Some(Color::MetallicBronze)
                } else if idx == 406usize {
                    Some(Color::DarkAlmond)
                } else if idx == 407usize {
                    Some(Color::Wood)
                } else if idx == 408usize {
                    Some(Color::KhakiBrown)
                } else if idx == 409usize {
                    Some(Color::OakBrown)
                } else if idx == 410usize {
                    Some(Color::AntiqueBronze)
                } else if idx == 411usize {
                    Some(Color::Hazel)
                } else if idx == 412usize {
                    Some(Color::DarkYellow)
                } else if idx == 413usize {
                    Some(Color::DarkMoccasin)
                } else if idx == 414usize {
                    Some(Color::KhakiGreen)
                } else if idx == 415usize {
                    Some(Color::MillenniumJade)
                } else if idx == 416usize {
                    Some(Color::DarkBeige)
                } else if idx == 417usize {
                    Some(Color::BulletShell)
                } else if idx == 418usize {
                    Some(Color::ArmyBrown)
                } else if idx == 419usize {
                    Some(Color::Sandstone)
                } else if idx == 420usize {
                    Some(Color::Taupe)
                } else if idx == 421usize {
                    Some(Color::DarkGrayishOlive)
                } else if idx == 422usize {
                    Some(Color::DarkHazelBrown)
                } else if idx == 423usize {
                    Some(Color::Mocha)
                } else if idx == 424usize {
                    Some(Color::MilkChocolate)
                } else if idx == 425usize {
                    Some(Color::GrayBrown)
                } else if idx == 426usize {
                    Some(Color::DarkCoffee)
                } else if idx == 427usize {
                    Some(Color::WesternCharcoal)
                } else if idx == 428usize {
                    Some(Color::OldBurgundy)
                } else if idx == 429usize {
                    Some(Color::RedBrown)
                } else if idx == 430usize {
                    Some(Color::BakersBrown)
                } else if idx == 431usize {
                    Some(Color::PullmanBrown)
                } else if idx == 432usize {
                    Some(Color::DarkBrown)
                } else if idx == 433usize {
                    Some(Color::SepiaBrown)
                } else if idx == 434usize {
                    Some(Color::DarkBronze)
                } else if idx == 435usize {
                    Some(Color::Coffee)
                } else if idx == 436usize {
                    Some(Color::BrownBear)
                } else if idx == 437usize {
                    Some(Color::RedDirt)
                } else if idx == 438usize {
                    Some(Color::Sepia)
                } else if idx == 439usize {
                    Some(Color::Sienna)
                } else if idx == 440usize {
                    Some(Color::SaddleBrown)
                } else if idx == 441usize {
                    Some(Color::DarkSienna)
                } else if idx == 442usize {
                    Some(Color::Sangria)
                } else if idx == 443usize {
                    Some(Color::BloodRed)
                } else if idx == 444usize {
                    Some(Color::Chestnut)
                } else if idx == 445usize {
                    Some(Color::CoralBrown)
                } else if idx == 446usize {
                    Some(Color::DeepAmber)
                } else if idx == 447usize {
                    Some(Color::ChestnutRed)
                } else if idx == 448usize {
                    Some(Color::GingerRed)
                } else if idx == 449usize {
                    Some(Color::Mahogany)
                } else if idx == 450usize {
                    Some(Color::RedGold)
                } else if idx == 451usize {
                    Some(Color::RedFox)
                } else if idx == 452usize {
                    Some(Color::DarkBisque)
                } else if idx == 453usize {
                    Some(Color::LightBrown)
                } else if idx == 454usize {
                    Some(Color::PetraGold)
                } else if idx == 455usize {
                    Some(Color::BrownRust)
                } else if idx == 456usize {
                    Some(Color::Rust)
                } else if idx == 457usize {
                    Some(Color::CopperRed)
                } else if idx == 458usize {
                    Some(Color::OrangeSalmon)
                } else if idx == 459usize {
                    Some(Color::Chocolate)
                } else if idx == 460usize {
                    Some(Color::Sedona)
                } else if idx == 461usize {
                    Some(Color::PapayaOrange)
                } else if idx == 462usize {
                    Some(Color::HalloweenOrange)
                } else if idx == 463usize {
                    Some(Color::NeonOrange)
                } else if idx == 464usize {
                    Some(Color::BrightOrange)
                } else if idx == 465usize {
                    Some(Color::FluroOrange)
                } else if idx == 466usize {
                    Some(Color::PumpkinOrange)
                } else if idx == 467usize {
                    Some(Color::SafetyOrange)
                } else if idx == 468usize {
                    Some(Color::CarrotOrange)
                } else if idx == 469usize {
                    Some(Color::DarkOrange)
                } else if idx == 470usize {
                    Some(Color::ConstructionConeOrange)
                } else if idx == 471usize {
                    Some(Color::IndianSaffron)
                } else if idx == 472usize {
                    Some(Color::SunriseOrange)
                } else if idx == 473usize {
                    Some(Color::MangoOrange)
                } else if idx == 474usize {
                    Some(Color::Coral)
                } else if idx == 475usize {
                    Some(Color::BasketBallOrange)
                } else if idx == 476usize {
                    Some(Color::LightSalmonRose)
                } else if idx == 477usize {
                    Some(Color::LightSalmon)
                } else if idx == 478usize {
                    Some(Color::PinkOrange)
                } else if idx == 479usize {
                    Some(Color::DarkSalmon)
                } else if idx == 480usize {
                    Some(Color::Tangerine)
                } else if idx == 481usize {
                    Some(Color::LightCopper)
                } else if idx == 482usize {
                    Some(Color::SalmonPink)
                } else if idx == 483usize {
                    Some(Color::Salmon)
                } else if idx == 484usize {
                    Some(Color::PeachPink)
                } else if idx == 485usize {
                    Some(Color::LightCoral)
                } else if idx == 486usize {
                    Some(Color::PastelRed)
                } else if idx == 487usize {
                    Some(Color::PinkCoral)
                } else if idx == 488usize {
                    Some(Color::BeanRed)
                } else if idx == 489usize {
                    Some(Color::ValentineRed)
                } else if idx == 490usize {
                    Some(Color::IndianRed)
                } else if idx == 491usize {
                    Some(Color::Tomato)
                } else if idx == 492usize {
                    Some(Color::ShockingOrange)
                } else if idx == 493usize {
                    Some(Color::OrangeRed)
                } else if idx == 494usize {
                    Some(Color::Red)
                } else if idx == 495usize {
                    Some(Color::NeonRed)
                } else if idx == 496usize {
                    Some(Color::ScarletRed)
                } else if idx == 497usize {
                    Some(Color::RubyRed)
                } else if idx == 498usize {
                    Some(Color::FerrariRed)
                } else if idx == 499usize {
                    Some(Color::FireEngineRed)
                } else if idx == 500usize {
                    Some(Color::LavaRed)
                } else if idx == 501usize {
                    Some(Color::LoveRed)
                } else if idx == 502usize {
                    Some(Color::Grapefruit)
                } else if idx == 503usize {
                    Some(Color::StrawberryRed)
                } else if idx == 504usize {
                    Some(Color::CherryRed)
                } else if idx == 505usize {
                    Some(Color::ChilliPepper)
                } else if idx == 506usize {
                    Some(Color::FireBrick)
                } else if idx == 507usize {
                    Some(Color::TomatoSauceRed)
                } else if idx == 508usize {
                    Some(Color::Brown)
                } else if idx == 509usize {
                    Some(Color::CarbonRed)
                } else if idx == 510usize {
                    Some(Color::Cranberry)
                } else if idx == 511usize {
                    Some(Color::SaffronRed)
                } else if idx == 512usize {
                    Some(Color::CrimsonRed)
                } else if idx == 513usize {
                    Some(Color::RedWine)
                } else if idx == 514usize {
                    Some(Color::WineRed)
                } else if idx == 515usize {
                    Some(Color::DarkRed)
                } else if idx == 516usize {
                    Some(Color::MaroonRed)
                } else if idx == 517usize {
                    Some(Color::Maroon)
                } else if idx == 518usize {
                    Some(Color::Burgundy)
                } else if idx == 519usize {
                    Some(Color::Vermilion)
                } else if idx == 520usize {
                    Some(Color::DeepRed)
                } else if idx == 521usize {
                    Some(Color::GarnetRed)
                } else if idx == 522usize {
                    Some(Color::RedBlood)
                } else if idx == 523usize {
                    Some(Color::BloodNight)
                } else if idx == 524usize {
                    Some(Color::DarkScarlet)
                } else if idx == 525usize {
                    Some(Color::ChocolateBrown)
                } else if idx == 526usize {
                    Some(Color::BlackBean)
                } else if idx == 527usize {
                    Some(Color::DarkMaroon)
                } else if idx == 528usize {
                    Some(Color::Midnight)
                } else if idx == 529usize {
                    Some(Color::PurpleLily)
                } else if idx == 530usize {
                    Some(Color::PurpleMaroon)
                } else if idx == 531usize {
                    Some(Color::PlumPie)
                } else if idx == 532usize {
                    Some(Color::PlumVelvet)
                } else if idx == 533usize {
                    Some(Color::DarkRaspberry)
                } else if idx == 534usize {
                    Some(Color::VelvetMaroon)
                } else if idx == 535usize {
                    Some(Color::RosyFinch)
                } else if idx == 536usize {
                    Some(Color::DullPurple)
                } else if idx == 537usize {
                    Some(Color::Puce)
                } else if idx == 538usize {
                    Some(Color::RoseDust)
                } else if idx == 539usize {
                    Some(Color::PastelBrown)
                } else if idx == 540usize {
                    Some(Color::RosyPink)
                } else if idx == 541usize {
                    Some(Color::RosyBrown)
                } else if idx == 542usize {
                    Some(Color::KhakiRose)
                } else if idx == 543usize {
                    Some(Color::LipstickPink)
                } else if idx == 544usize {
                    Some(Color::DuskyPink)
                } else if idx == 545usize {
                    Some(Color::PinkBrown)
                } else if idx == 546usize {
                    Some(Color::OldRose)
                } else if idx == 547usize {
                    Some(Color::DustyPink)
                } else if idx == 548usize {
                    Some(Color::PinkDaisy)
                } else if idx == 549usize {
                    Some(Color::Rose)
                } else if idx == 550usize {
                    Some(Color::DustyRose)
                } else if idx == 551usize {
                    Some(Color::SilverPink)
                } else if idx == 552usize {
                    Some(Color::GoldPink)
                } else if idx == 553usize {
                    Some(Color::RoseGold)
                } else if idx == 554usize {
                    Some(Color::DeepPeach)
                } else if idx == 555usize {
                    Some(Color::PastelOrange)
                } else if idx == 556usize {
                    Some(Color::DesertSand)
                } else if idx == 557usize {
                    Some(Color::UnbleachedSilk)
                } else if idx == 558usize {
                    Some(Color::PigPink)
                } else if idx == 559usize {
                    Some(Color::PalePink)
                } else if idx == 560usize {
                    Some(Color::Blush)
                } else if idx == 561usize {
                    Some(Color::MistyRose)
                } else if idx == 562usize {
                    Some(Color::PinkBubbleGum)
                } else if idx == 563usize {
                    Some(Color::LightRose)
                } else if idx == 564usize {
                    Some(Color::LightRed)
                } else if idx == 565usize {
                    Some(Color::RoseQuartz)
                } else if idx == 566usize {
                    Some(Color::WarmPink)
                } else if idx == 567usize {
                    Some(Color::DeepRose)
                } else if idx == 568usize {
                    Some(Color::Pink)
                } else if idx == 569usize {
                    Some(Color::LightPink)
                } else if idx == 570usize {
                    Some(Color::SoftPink)
                } else if idx == 571usize {
                    Some(Color::PowderPink)
                } else if idx == 572usize {
                    Some(Color::DonutPink)
                } else if idx == 573usize {
                    Some(Color::BabyPink)
                } else if idx == 574usize {
                    Some(Color::FlamingoPink)
                } else if idx == 575usize {
                    Some(Color::PastelPink)
                } else if idx == 576usize {
                    Some(Color::RosePink)
                } else if idx == 577usize {
                    Some(Color::CadillacPink)
                } else if idx == 578usize {
                    Some(Color::CarnationPink)
                } else if idx == 579usize {
                    Some(Color::PastelRose)
                } else if idx == 580usize {
                    Some(Color::BlushRed)
                } else if idx == 581usize {
                    Some(Color::PaleVioletRed)
                } else if idx == 582usize {
                    Some(Color::PurplePink)
                } else if idx == 583usize {
                    Some(Color::TulipPink)
                } else if idx == 584usize {
                    Some(Color::BashfulPink)
                } else if idx == 585usize {
                    Some(Color::DarkPink)
                } else if idx == 586usize {
                    Some(Color::DarkHotPink)
                } else if idx == 587usize {
                    Some(Color::HotPink)
                } else if idx == 588usize {
                    Some(Color::WatermelonPink)
                } else if idx == 589usize {
                    Some(Color::VioletRed)
                } else if idx == 590usize {
                    Some(Color::HotDeepPink)
                } else if idx == 591usize {
                    Some(Color::BrightPink)
                } else if idx == 592usize {
                    Some(Color::RedMagenta)
                } else if idx == 593usize {
                    Some(Color::DeepPink)
                } else if idx == 594usize {
                    Some(Color::NeonPink)
                } else if idx == 595usize {
                    Some(Color::ChromePink)
                } else if idx == 596usize {
                    Some(Color::NeonHotPink)
                } else if idx == 597usize {
                    Some(Color::PinkCupcake)
                } else if idx == 598usize {
                    Some(Color::RoyalPink)
                } else if idx == 599usize {
                    Some(Color::DimorphothecaMagenta)
                } else if idx == 600usize {
                    Some(Color::BarbiePink)
                } else if idx == 601usize {
                    Some(Color::PinkLemonade)
                } else if idx == 602usize {
                    Some(Color::RedPink)
                } else if idx == 603usize {
                    Some(Color::Raspberry)
                } else if idx == 604usize {
                    Some(Color::Crimson)
                } else if idx == 605usize {
                    Some(Color::BrightMaroon)
                } else if idx == 606usize {
                    Some(Color::RoseRed)
                } else if idx == 607usize {
                    Some(Color::RoguePink)
                } else if idx == 608usize {
                    Some(Color::BurntPink)
                } else if idx == 609usize {
                    Some(Color::PinkViolet)
                } else if idx == 610usize {
                    Some(Color::MagentaPink)
                } else if idx == 611usize {
                    Some(Color::MediumVioletRed)
                } else if idx == 612usize {
                    Some(Color::DarkCarnationPink)
                } else if idx == 613usize {
                    Some(Color::RaspberryPurple)
                } else if idx == 614usize {
                    Some(Color::PinkPlum)
                } else if idx == 615usize {
                    Some(Color::Orchid)
                } else if idx == 616usize {
                    Some(Color::DeepMauve)
                } else if idx == 617usize {
                    Some(Color::Violet)
                } else if idx == 618usize {
                    Some(Color::FuchsiaPink)
                } else if idx == 619usize {
                    Some(Color::BrightNeonPink)
                } else if idx == 620usize {
                    Some(Color::Magenta)
                } else if idx == 621usize {
                    Some(Color::Fuchsia)
                } else if idx == 622usize {
                    Some(Color::CrimsonPurple)
                } else if idx == 623usize {
                    Some(Color::HeliotropePurple)
                } else if idx == 624usize {
                    Some(Color::TyrianPurple)
                } else if idx == 625usize {
                    Some(Color::MediumOrchid)
                } else if idx == 626usize {
                    Some(Color::PurpleFlower)
                } else if idx == 627usize {
                    Some(Color::OrchidPurple)
                } else if idx == 628usize {
                    Some(Color::RichLilac)
                } else if idx == 629usize {
                    Some(Color::PastelViolet)
                } else if idx == 630usize {
                    Some(Color::Rosy)
                } else if idx == 631usize {
                    Some(Color::MauveTaupe)
                } else if idx == 632usize {
                    Some(Color::ViolaPurple)
                } else if idx == 633usize {
                    Some(Color::Eggplant)
                } else if idx == 634usize {
                    Some(Color::PlumPurple)
                } else if idx == 635usize {
                    Some(Color::Grape)
                } else if idx == 636usize {
                    Some(Color::PurpleNavy)
                } else if idx == 637usize {
                    Some(Color::SlateBlue)
                } else if idx == 638usize {
                    Some(Color::BlueLotus)
                } else if idx == 639usize {
                    Some(Color::Blurple)
                } else if idx == 640usize {
                    Some(Color::LightSlateBlue)
                } else if idx == 641usize {
                    Some(Color::MediumSlateBlue)
                } else if idx == 642usize {
                    Some(Color::PeriwinklePurple)
                } else if idx == 643usize {
                    Some(Color::VeryPeri)
                } else if idx == 644usize {
                    Some(Color::BrightGrape)
                } else if idx == 645usize {
                    Some(Color::BrightPurple)
                } else if idx == 646usize {
                    Some(Color::PurpleAmethyst)
                } else if idx == 647usize {
                    Some(Color::BlueMagenta)
                } else if idx == 648usize {
                    Some(Color::DarkBlurple)
                } else if idx == 649usize {
                    Some(Color::DeepPeriwinkle)
                } else if idx == 650usize {
                    Some(Color::DarkSlateBlue)
                } else if idx == 651usize {
                    Some(Color::PurpleHaze)
                } else if idx == 652usize {
                    Some(Color::PurpleIris)
                } else if idx == 653usize {
                    Some(Color::DarkPurple)
                } else if idx == 654usize {
                    Some(Color::DeepPurple)
                } else if idx == 655usize {
                    Some(Color::MidnightPurple)
                } else if idx == 656usize {
                    Some(Color::PurpleMonster)
                } else if idx == 657usize {
                    Some(Color::Indigo)
                } else if idx == 658usize {
                    Some(Color::BlueWhale)
                } else if idx == 659usize {
                    Some(Color::RebeccaPurple)
                } else if idx == 660usize {
                    Some(Color::PurpleJam)
                } else if idx == 661usize {
                    Some(Color::DarkMagenta)
                } else if idx == 662usize {
                    Some(Color::Purple)
                } else if idx == 663usize {
                    Some(Color::FrenchLilac)
                } else if idx == 664usize {
                    Some(Color::DarkOrchid)
                } else if idx == 665usize {
                    Some(Color::DarkViolet)
                } else if idx == 666usize {
                    Some(Color::PurpleViolet)
                } else if idx == 667usize {
                    Some(Color::JasminePurple)
                } else if idx == 668usize {
                    Some(Color::PurpleDaffodil)
                } else if idx == 669usize {
                    Some(Color::ClematisViolet)
                } else if idx == 670usize {
                    Some(Color::BlueViolet)
                } else if idx == 671usize {
                    Some(Color::PurpleSageBush)
                } else if idx == 672usize {
                    Some(Color::LovelyPurple)
                } else if idx == 673usize {
                    Some(Color::NeonPurple)
                } else if idx == 674usize {
                    Some(Color::PurplePlum)
                } else if idx == 675usize {
                    Some(Color::AztechPurple)
                } else if idx == 676usize {
                    Some(Color::MediumPurple)
                } else if idx == 677usize {
                    Some(Color::LightPurple)
                } else if idx == 678usize {
                    Some(Color::CrocusPurple)
                } else if idx == 679usize {
                    Some(Color::PurpleMimosa)
                } else if idx == 680usize {
                    Some(Color::PastelIndigo)
                } else if idx == 681usize {
                    Some(Color::LavenderPurple)
                } else if idx == 682usize {
                    Some(Color::RosePurple)
                } else if idx == 683usize {
                    Some(Color::Viola)
                } else if idx == 684usize {
                    Some(Color::Periwinkle)
                } else if idx == 685usize {
                    Some(Color::PaleLilac)
                } else if idx == 686usize {
                    Some(Color::Lilac)
                } else if idx == 687usize {
                    Some(Color::Mauve)
                } else if idx == 688usize {
                    Some(Color::BrightLilac)
                } else if idx == 689usize {
                    Some(Color::PurpleDragon)
                } else if idx == 690usize {
                    Some(Color::Plum)
                } else if idx == 691usize {
                    Some(Color::BlushPink)
                } else if idx == 692usize {
                    Some(Color::PastelPurple)
                } else if idx == 693usize {
                    Some(Color::BlossomPink)
                } else if idx == 694usize {
                    Some(Color::WisteriaPurple)
                } else if idx == 695usize {
                    Some(Color::PurpleThistle)
                } else if idx == 696usize {
                    Some(Color::Thistle)
                } else if idx == 697usize {
                    Some(Color::PurpleWhite)
                } else if idx == 698usize {
                    Some(Color::PeriwinklePink)
                } else if idx == 699usize {
                    Some(Color::CottonCandy)
                } else if idx == 700usize {
                    Some(Color::LavenderPinocchio)
                } else if idx == 701usize {
                    Some(Color::DarkWhite)
                } else if idx == 702usize {
                    Some(Color::AshWhite)
                } else if idx == 703usize {
                    Some(Color::WarmWhite)
                } else if idx == 704usize {
                    Some(Color::WhiteChocolate)
                } else if idx == 705usize {
                    Some(Color::CreamyWhite)
                } else if idx == 706usize {
                    Some(Color::OffWhite)
                } else if idx == 707usize {
                    Some(Color::SoftIvory)
                } else if idx == 708usize {
                    Some(Color::CosmicLatte)
                } else if idx == 709usize {
                    Some(Color::PearlWhite)
                } else if idx == 710usize {
                    Some(Color::RedWhite)
                } else if idx == 711usize {
                    Some(Color::LavenderBlush)
                } else if idx == 712usize {
                    Some(Color::Pearl)
                } else if idx == 713usize {
                    Some(Color::EggShell)
                } else if idx == 714usize {
                    Some(Color::OldLace)
                } else if idx == 715usize {
                    Some(Color::WhiteIce)
                } else if idx == 716usize {
                    Some(Color::Linen)
                } else if idx == 717usize {
                    Some(Color::SeaShell)
                } else if idx == 718usize {
                    Some(Color::BoneWhite)
                } else if idx == 719usize {
                    Some(Color::Rice)
                } else if idx == 720usize {
                    Some(Color::FloralWhite)
                } else if idx == 721usize {
                    Some(Color::Ivory)
                } else if idx == 722usize {
                    Some(Color::WhiteGold)
                } else if idx == 723usize {
                    Some(Color::LightWhite)
                } else if idx == 724usize {
                    Some(Color::Cotton)
                } else if idx == 725usize {
                    Some(Color::Snow)
                } else if idx == 726usize {
                    Some(Color::MilkWhite)
                } else if idx == 727usize {
                    Some(Color::HalfWhite)
                } else if idx == 728usize {
                    Some(Color::White)
                } else {
                    None
                }
            })
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
