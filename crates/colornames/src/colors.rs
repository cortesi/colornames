use crate::COLORS;
use once_cell::sync::Lazy;
use std::collections::HashMap;
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
        if let Some(name) = name.strip_prefix('#') {
            let hex = &name[1..];
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
            match name.replace(" ", "").to_lowercase() {
                x if x == stringify!(Black).to_lowercase() => Some(Color::Black),
                x if x == stringify!(BlackBlue).to_lowercase() => Some(Color::BlackBlue),
                x if x == stringify!(Night).to_lowercase() => Some(Color::Night),
                x if x == stringify!(Charcoal).to_lowercase() => Some(Color::Charcoal),
                x if x == stringify!(Oil).to_lowercase() => Some(Color::Oil),
                x if x == stringify!(StormyGray).to_lowercase() => Some(Color::StormyGray),
                x if x == stringify!(LightBlack).to_lowercase() => Some(Color::LightBlack),
                x if x == stringify!(DarkSteampunk).to_lowercase() => Some(Color::DarkSteampunk),
                x if x == stringify!(BlackCat).to_lowercase() => Some(Color::BlackCat),
                x if x == stringify!(Iridium).to_lowercase() => Some(Color::Iridium),
                x if x == stringify!(BlackEel).to_lowercase() => Some(Color::BlackEel),
                x if x == stringify!(BlackCow).to_lowercase() => Some(Color::BlackCow),
                x if x == stringify!(GrayWolf).to_lowercase() => Some(Color::GrayWolf),
                x if x == stringify!(VampireGray).to_lowercase() => Some(Color::VampireGray),
                x if x == stringify!(IronGray).to_lowercase() => Some(Color::IronGray),
                x if x == stringify!(GrayDolphin).to_lowercase() => Some(Color::GrayDolphin),
                x if x == stringify!(CarbonGray).to_lowercase() => Some(Color::CarbonGray),
                x if x == stringify!(AshGray).to_lowercase() => Some(Color::AshGray),
                x if x == stringify!(DimGray).to_lowercase() => Some(Color::DimGray),
                x if x == stringify!(DimGrey).to_lowercase() => Some(Color::DimGrey),
                x if x == stringify!(NardoGray).to_lowercase() => Some(Color::NardoGray),
                x if x == stringify!(CloudyGray).to_lowercase() => Some(Color::CloudyGray),
                x if x == stringify!(SmokeyGray).to_lowercase() => Some(Color::SmokeyGray),
                x if x == stringify!(AlienGray).to_lowercase() => Some(Color::AlienGray),
                x if x == stringify!(SonicSilver).to_lowercase() => Some(Color::SonicSilver),
                x if x == stringify!(PlatinumGray).to_lowercase() => Some(Color::PlatinumGray),
                x if x == stringify!(Granite).to_lowercase() => Some(Color::Granite),
                x if x == stringify!(Gray).to_lowercase() => Some(Color::Gray),
                x if x == stringify!(Grey).to_lowercase() => Some(Color::Grey),
                x if x == stringify!(BattleshipGray).to_lowercase() => Some(Color::BattleshipGray),
                x if x == stringify!(SheetMetal).to_lowercase() => Some(Color::SheetMetal),
                x if x == stringify!(DarkGainsboro).to_lowercase() => Some(Color::DarkGainsboro),
                x if x == stringify!(GunmetalGray).to_lowercase() => Some(Color::GunmetalGray),
                x if x == stringify!(ColdMetal).to_lowercase() => Some(Color::ColdMetal),
                x if x == stringify!(StainlessSteelGray).to_lowercase() => {
                    Some(Color::StainlessSteelGray)
                }
                x if x == stringify!(DarkGrey).to_lowercase() => Some(Color::DarkGrey),
                x if x == stringify!(DarkGray).to_lowercase() => Some(Color::DarkGray),
                x if x == stringify!(ChromeAluminum).to_lowercase() => Some(Color::ChromeAluminum),
                x if x == stringify!(GrayCloud).to_lowercase() => Some(Color::GrayCloud),
                x if x == stringify!(Metal).to_lowercase() => Some(Color::Metal),
                x if x == stringify!(Silver).to_lowercase() => Some(Color::Silver),
                x if x == stringify!(Steampunk).to_lowercase() => Some(Color::Steampunk),
                x if x == stringify!(PaleSilver).to_lowercase() => Some(Color::PaleSilver),
                x if x == stringify!(GearSteelGray).to_lowercase() => Some(Color::GearSteelGray),
                x if x == stringify!(GrayGoose).to_lowercase() => Some(Color::GrayGoose),
                x if x == stringify!(PlatinumSilver).to_lowercase() => Some(Color::PlatinumSilver),
                x if x == stringify!(LightGrey).to_lowercase() => Some(Color::LightGrey),
                x if x == stringify!(LightGray).to_lowercase() => Some(Color::LightGray),
                x if x == stringify!(SilverWhite).to_lowercase() => Some(Color::SilverWhite),
                x if x == stringify!(Gainsboro).to_lowercase() => Some(Color::Gainsboro),
                x if x == stringify!(LightSteelGray).to_lowercase() => Some(Color::LightSteelGray),
                x if x == stringify!(WhiteSmoke).to_lowercase() => Some(Color::WhiteSmoke),
                x if x == stringify!(WhiteGray).to_lowercase() => Some(Color::WhiteGray),
                x if x == stringify!(Platinum).to_lowercase() => Some(Color::Platinum),
                x if x == stringify!(MetallicSilver).to_lowercase() => Some(Color::MetallicSilver),
                x if x == stringify!(BlueGray).to_lowercase() => Some(Color::BlueGray),
                x if x == stringify!(RomanSilver).to_lowercase() => Some(Color::RomanSilver),
                x if x == stringify!(LightSlateGrey).to_lowercase() => Some(Color::LightSlateGrey),
                x if x == stringify!(LightSlateGray).to_lowercase() => Some(Color::LightSlateGray),
                x if x == stringify!(SlateGrey).to_lowercase() => Some(Color::SlateGrey),
                x if x == stringify!(SlateGray).to_lowercase() => Some(Color::SlateGray),
                x if x == stringify!(RatGray).to_lowercase() => Some(Color::RatGray),
                x if x == stringify!(SlateGraniteGray).to_lowercase() => {
                    Some(Color::SlateGraniteGray)
                }
                x if x == stringify!(JetGray).to_lowercase() => Some(Color::JetGray),
                x if x == stringify!(MistBlue).to_lowercase() => Some(Color::MistBlue),
                x if x == stringify!(SteelGray).to_lowercase() => Some(Color::SteelGray),
                x if x == stringify!(MarbleBlue).to_lowercase() => Some(Color::MarbleBlue),
                x if x == stringify!(SlateBlueGray).to_lowercase() => Some(Color::SlateBlueGray),
                x if x == stringify!(LightPurpleBlue).to_lowercase() => {
                    Some(Color::LightPurpleBlue)
                }
                x if x == stringify!(AzureBlue).to_lowercase() => Some(Color::AzureBlue),
                x if x == stringify!(EstorilBlue).to_lowercase() => Some(Color::EstorilBlue),
                x if x == stringify!(BlueJay).to_lowercase() => Some(Color::BlueJay),
                x if x == stringify!(CharcoalBlue).to_lowercase() => Some(Color::CharcoalBlue),
                x if x == stringify!(DarkBlueGray).to_lowercase() => Some(Color::DarkBlueGray),
                x if x == stringify!(DarkSlate).to_lowercase() => Some(Color::DarkSlate),
                x if x == stringify!(DeepSeaBlue).to_lowercase() => Some(Color::DeepSeaBlue),
                x if x == stringify!(NightBlue).to_lowercase() => Some(Color::NightBlue),
                x if x == stringify!(MidnightBlue).to_lowercase() => Some(Color::MidnightBlue),
                x if x == stringify!(Navy).to_lowercase() => Some(Color::Navy),
                x if x == stringify!(DenimDarkBlue).to_lowercase() => Some(Color::DenimDarkBlue),
                x if x == stringify!(DarkBlue).to_lowercase() => Some(Color::DarkBlue),
                x if x == stringify!(LapisBlue).to_lowercase() => Some(Color::LapisBlue),
                x if x == stringify!(NewMidnightBlue).to_lowercase() => {
                    Some(Color::NewMidnightBlue)
                }
                x if x == stringify!(EarthBlue).to_lowercase() => Some(Color::EarthBlue),
                x if x == stringify!(CobaltBlue).to_lowercase() => Some(Color::CobaltBlue),
                x if x == stringify!(MediumBlue).to_lowercase() => Some(Color::MediumBlue),
                x if x == stringify!(BlueberryBlue).to_lowercase() => Some(Color::BlueberryBlue),
                x if x == stringify!(CanaryBlue).to_lowercase() => Some(Color::CanaryBlue),
                x if x == stringify!(Blue).to_lowercase() => Some(Color::Blue),
                x if x == stringify!(SamcoBlue).to_lowercase() => Some(Color::SamcoBlue),
                x if x == stringify!(BrightBlue).to_lowercase() => Some(Color::BrightBlue),
                x if x == stringify!(BlueOrchid).to_lowercase() => Some(Color::BlueOrchid),
                x if x == stringify!(SapphireBlue).to_lowercase() => Some(Color::SapphireBlue),
                x if x == stringify!(BlueEyes).to_lowercase() => Some(Color::BlueEyes),
                x if x == stringify!(BrightNavyBlue).to_lowercase() => Some(Color::BrightNavyBlue),
                x if x == stringify!(BalloonBlue).to_lowercase() => Some(Color::BalloonBlue),
                x if x == stringify!(RoyalBlue).to_lowercase() => Some(Color::RoyalBlue),
                x if x == stringify!(OceanBlue).to_lowercase() => Some(Color::OceanBlue),
                x if x == stringify!(DarkSkyBlue).to_lowercase() => Some(Color::DarkSkyBlue),
                x if x == stringify!(BlueRibbon).to_lowercase() => Some(Color::BlueRibbon),
                x if x == stringify!(BlueDress).to_lowercase() => Some(Color::BlueDress),
                x if x == stringify!(NeonBlue).to_lowercase() => Some(Color::NeonBlue),
                x if x == stringify!(DodgerBlue).to_lowercase() => Some(Color::DodgerBlue),
                x if x == stringify!(GlacialBlueIce).to_lowercase() => Some(Color::GlacialBlueIce),
                x if x == stringify!(SteelBlue).to_lowercase() => Some(Color::SteelBlue),
                x if x == stringify!(SilkBlue).to_lowercase() => Some(Color::SilkBlue),
                x if x == stringify!(WindowsBlue).to_lowercase() => Some(Color::WindowsBlue),
                x if x == stringify!(BlueIvy).to_lowercase() => Some(Color::BlueIvy),
                x if x == stringify!(CyanBlue).to_lowercase() => Some(Color::CyanBlue),
                x if x == stringify!(BlueKoi).to_lowercase() => Some(Color::BlueKoi),
                x if x == stringify!(ColumbiaBlue).to_lowercase() => Some(Color::ColumbiaBlue),
                x if x == stringify!(BabyBlue).to_lowercase() => Some(Color::BabyBlue),
                x if x == stringify!(CornflowerBlue).to_lowercase() => Some(Color::CornflowerBlue),
                x if x == stringify!(SkyBlueDress).to_lowercase() => Some(Color::SkyBlueDress),
                x if x == stringify!(Iceberg).to_lowercase() => Some(Color::Iceberg),
                x if x == stringify!(ButterflyBlue).to_lowercase() => Some(Color::ButterflyBlue),
                x if x == stringify!(DeepSkyBlue).to_lowercase() => Some(Color::DeepSkyBlue),
                x if x == stringify!(MiddayBlue).to_lowercase() => Some(Color::MiddayBlue),
                x if x == stringify!(CrystalBlue).to_lowercase() => Some(Color::CrystalBlue),
                x if x == stringify!(DenimBlue).to_lowercase() => Some(Color::DenimBlue),
                x if x == stringify!(DaySkyBlue).to_lowercase() => Some(Color::DaySkyBlue),
                x if x == stringify!(LightSkyBlue).to_lowercase() => Some(Color::LightSkyBlue),
                x if x == stringify!(SkyBlue).to_lowercase() => Some(Color::SkyBlue),
                x if x == stringify!(JeansBlue).to_lowercase() => Some(Color::JeansBlue),
                x if x == stringify!(BlueAngel).to_lowercase() => Some(Color::BlueAngel),
                x if x == stringify!(PastelBlue).to_lowercase() => Some(Color::PastelBlue),
                x if x == stringify!(LightDayBlue).to_lowercase() => Some(Color::LightDayBlue),
                x if x == stringify!(SeaBlue).to_lowercase() => Some(Color::SeaBlue),
                x if x == stringify!(HeavenlyBlue).to_lowercase() => Some(Color::HeavenlyBlue),
                x if x == stringify!(RobinEggBlue).to_lowercase() => Some(Color::RobinEggBlue),
                x if x == stringify!(PowderBlue).to_lowercase() => Some(Color::PowderBlue),
                x if x == stringify!(CoralBlue).to_lowercase() => Some(Color::CoralBlue),
                x if x == stringify!(LightBlue).to_lowercase() => Some(Color::LightBlue),
                x if x == stringify!(LightSteelBlue).to_lowercase() => Some(Color::LightSteelBlue),
                x if x == stringify!(GulfBlue).to_lowercase() => Some(Color::GulfBlue),
                x if x == stringify!(PastelLightBlue).to_lowercase() => {
                    Some(Color::PastelLightBlue)
                }
                x if x == stringify!(LavenderBlue).to_lowercase() => Some(Color::LavenderBlue),
                x if x == stringify!(WhiteBlue).to_lowercase() => Some(Color::WhiteBlue),
                x if x == stringify!(Lavender).to_lowercase() => Some(Color::Lavender),
                x if x == stringify!(Water).to_lowercase() => Some(Color::Water),
                x if x == stringify!(AliceBlue).to_lowercase() => Some(Color::AliceBlue),
                x if x == stringify!(GhostWhite).to_lowercase() => Some(Color::GhostWhite),
                x if x == stringify!(Azure).to_lowercase() => Some(Color::Azure),
                x if x == stringify!(LightCyan).to_lowercase() => Some(Color::LightCyan),
                x if x == stringify!(LightSlate).to_lowercase() => Some(Color::LightSlate),
                x if x == stringify!(ElectricBlue).to_lowercase() => Some(Color::ElectricBlue),
                x if x == stringify!(TronBlue).to_lowercase() => Some(Color::TronBlue),
                x if x == stringify!(BlueZircon).to_lowercase() => Some(Color::BlueZircon),
                x if x == stringify!(Cyan).to_lowercase() => Some(Color::Cyan),
                x if x == stringify!(Aqua).to_lowercase() => Some(Color::Aqua),
                x if x == stringify!(BrightCyan).to_lowercase() => Some(Color::BrightCyan),
                x if x == stringify!(Celeste).to_lowercase() => Some(Color::Celeste),
                x if x == stringify!(BlueDiamond).to_lowercase() => Some(Color::BlueDiamond),
                x if x == stringify!(BrightTurquoise).to_lowercase() => {
                    Some(Color::BrightTurquoise)
                }
                x if x == stringify!(BlueLagoon).to_lowercase() => Some(Color::BlueLagoon),
                x if x == stringify!(PaleTurquoise).to_lowercase() => Some(Color::PaleTurquoise),
                x if x == stringify!(PaleBlueLily).to_lowercase() => Some(Color::PaleBlueLily),
                x if x == stringify!(LightTeal).to_lowercase() => Some(Color::LightTeal),
                x if x == stringify!(TiffanyBlue).to_lowercase() => Some(Color::TiffanyBlue),
                x if x == stringify!(BlueHosta).to_lowercase() => Some(Color::BlueHosta),
                x if x == stringify!(CyanOpaque).to_lowercase() => Some(Color::CyanOpaque),
                x if x == stringify!(NorthernLightsBlue).to_lowercase() => {
                    Some(Color::NorthernLightsBlue)
                }
                x if x == stringify!(BlueGreen).to_lowercase() => Some(Color::BlueGreen),
                x if x == stringify!(MediumAquaMarine).to_lowercase() => {
                    Some(Color::MediumAquaMarine)
                }
                x if x == stringify!(AquaSeafoamGreen).to_lowercase() => {
                    Some(Color::AquaSeafoamGreen)
                }
                x if x == stringify!(MagicMint).to_lowercase() => Some(Color::MagicMint),
                x if x == stringify!(LightAquamarine).to_lowercase() => {
                    Some(Color::LightAquamarine)
                }
                x if x == stringify!(Aquamarine).to_lowercase() => Some(Color::Aquamarine),
                x if x == stringify!(BrightTeal).to_lowercase() => Some(Color::BrightTeal),
                x if x == stringify!(Turquoise).to_lowercase() => Some(Color::Turquoise),
                x if x == stringify!(MediumTurquoise).to_lowercase() => {
                    Some(Color::MediumTurquoise)
                }
                x if x == stringify!(DeepTurquoise).to_lowercase() => Some(Color::DeepTurquoise),
                x if x == stringify!(Jellyfish).to_lowercase() => Some(Color::Jellyfish),
                x if x == stringify!(BlueTurquoise).to_lowercase() => Some(Color::BlueTurquoise),
                x if x == stringify!(DarkTurquoise).to_lowercase() => Some(Color::DarkTurquoise),
                x if x == stringify!(MacawBlueGreen).to_lowercase() => Some(Color::MacawBlueGreen),
                x if x == stringify!(LightSeaGreen).to_lowercase() => Some(Color::LightSeaGreen),
                x if x == stringify!(SeafoamGreen).to_lowercase() => Some(Color::SeafoamGreen),
                x if x == stringify!(CadetBlue).to_lowercase() => Some(Color::CadetBlue),
                x if x == stringify!(DeepSea).to_lowercase() => Some(Color::DeepSea),
                x if x == stringify!(DarkCyan).to_lowercase() => Some(Color::DarkCyan),
                x if x == stringify!(TealGreen).to_lowercase() => Some(Color::TealGreen),
                x if x == stringify!(Teal).to_lowercase() => Some(Color::Teal),
                x if x == stringify!(TealBlue).to_lowercase() => Some(Color::TealBlue),
                x if x == stringify!(MediumTeal).to_lowercase() => Some(Color::MediumTeal),
                x if x == stringify!(DarkTeal).to_lowercase() => Some(Color::DarkTeal),
                x if x == stringify!(DeepTeal).to_lowercase() => Some(Color::DeepTeal),
                x if x == stringify!(DarkSlateGray).to_lowercase() => Some(Color::DarkSlateGray),
                x if x == stringify!(DarkSlateGrey).to_lowercase() => Some(Color::DarkSlateGrey),
                x if x == stringify!(Gunmetal).to_lowercase() => Some(Color::Gunmetal),
                x if x == stringify!(BlueMossGreen).to_lowercase() => Some(Color::BlueMossGreen),
                x if x == stringify!(BeetleGreen).to_lowercase() => Some(Color::BeetleGreen),
                x if x == stringify!(GrayishTurquoise).to_lowercase() => {
                    Some(Color::GrayishTurquoise)
                }
                x if x == stringify!(GreenishBlue).to_lowercase() => Some(Color::GreenishBlue),
                x if x == stringify!(AquamarineStone).to_lowercase() => {
                    Some(Color::AquamarineStone)
                }
                x if x == stringify!(SeaTurtleGreen).to_lowercase() => Some(Color::SeaTurtleGreen),
                x if x == stringify!(DullSeaGreen).to_lowercase() => Some(Color::DullSeaGreen),
                x if x == stringify!(DarkGreenBlue).to_lowercase() => Some(Color::DarkGreenBlue),
                x if x == stringify!(DeepSeaGreen).to_lowercase() => Some(Color::DeepSeaGreen),
                x if x == stringify!(BottleGreen).to_lowercase() => Some(Color::BottleGreen),
                x if x == stringify!(SeaGreen).to_lowercase() => Some(Color::SeaGreen),
                x if x == stringify!(ElfGreen).to_lowercase() => Some(Color::ElfGreen),
                x if x == stringify!(DarkMint).to_lowercase() => Some(Color::DarkMint),
                x if x == stringify!(Jade).to_lowercase() => Some(Color::Jade),
                x if x == stringify!(EarthGreen).to_lowercase() => Some(Color::EarthGreen),
                x if x == stringify!(ChromeGreen).to_lowercase() => Some(Color::ChromeGreen),
                x if x == stringify!(Mint).to_lowercase() => Some(Color::Mint),
                x if x == stringify!(Emerald).to_lowercase() => Some(Color::Emerald),
                x if x == stringify!(IsleOfManGreen).to_lowercase() => Some(Color::IsleOfManGreen),
                x if x == stringify!(MediumSeaGreen).to_lowercase() => Some(Color::MediumSeaGreen),
                x if x == stringify!(MetallicGreen).to_lowercase() => Some(Color::MetallicGreen),
                x if x == stringify!(CamouflageGreen).to_lowercase() => {
                    Some(Color::CamouflageGreen)
                }
                x if x == stringify!(SageGreen).to_lowercase() => Some(Color::SageGreen),
                x if x == stringify!(HazelGreen).to_lowercase() => Some(Color::HazelGreen),
                x if x == stringify!(VenomGreen).to_lowercase() => Some(Color::VenomGreen),
                x if x == stringify!(OliveDrab).to_lowercase() => Some(Color::OliveDrab),
                x if x == stringify!(Olive).to_lowercase() => Some(Color::Olive),
                x if x == stringify!(Ebony).to_lowercase() => Some(Color::Ebony),
                x if x == stringify!(DarkOliveGreen).to_lowercase() => Some(Color::DarkOliveGreen),
                x if x == stringify!(MilitaryGreen).to_lowercase() => Some(Color::MilitaryGreen),
                x if x == stringify!(GreenLeaves).to_lowercase() => Some(Color::GreenLeaves),
                x if x == stringify!(ArmyGreen).to_lowercase() => Some(Color::ArmyGreen),
                x if x == stringify!(FernGreen).to_lowercase() => Some(Color::FernGreen),
                x if x == stringify!(FallForestGreen).to_lowercase() => {
                    Some(Color::FallForestGreen)
                }
                x if x == stringify!(IrishGreen).to_lowercase() => Some(Color::IrishGreen),
                x if x == stringify!(PineGreen).to_lowercase() => Some(Color::PineGreen),
                x if x == stringify!(MediumForestGreen).to_lowercase() => {
                    Some(Color::MediumForestGreen)
                }
                x if x == stringify!(RacingGreen).to_lowercase() => Some(Color::RacingGreen),
                x if x == stringify!(JungleGreen).to_lowercase() => Some(Color::JungleGreen),
                x if x == stringify!(CactusGreen).to_lowercase() => Some(Color::CactusGreen),
                x if x == stringify!(ForestGreen).to_lowercase() => Some(Color::ForestGreen),
                x if x == stringify!(Green).to_lowercase() => Some(Color::Green),
                x if x == stringify!(DarkGreen).to_lowercase() => Some(Color::DarkGreen),
                x if x == stringify!(DeepGreen).to_lowercase() => Some(Color::DeepGreen),
                x if x == stringify!(DeepEmeraldGreen).to_lowercase() => {
                    Some(Color::DeepEmeraldGreen)
                }
                x if x == stringify!(HunterGreen).to_lowercase() => Some(Color::HunterGreen),
                x if x == stringify!(DarkForestGreen).to_lowercase() => {
                    Some(Color::DarkForestGreen)
                }
                x if x == stringify!(LotusGreen).to_lowercase() => Some(Color::LotusGreen),
                x if x == stringify!(BroccoliGreen).to_lowercase() => Some(Color::BroccoliGreen),
                x if x == stringify!(SeaweedGreen).to_lowercase() => Some(Color::SeaweedGreen),
                x if x == stringify!(ShamrockGreen).to_lowercase() => Some(Color::ShamrockGreen),
                x if x == stringify!(GreenOnion).to_lowercase() => Some(Color::GreenOnion),
                x if x == stringify!(MossGreen).to_lowercase() => Some(Color::MossGreen),
                x if x == stringify!(GrassGreen).to_lowercase() => Some(Color::GrassGreen),
                x if x == stringify!(GreenPepper).to_lowercase() => Some(Color::GreenPepper),
                x if x == stringify!(DarkLimeGreen).to_lowercase() => Some(Color::DarkLimeGreen),
                x if x == stringify!(ParrotGreen).to_lowercase() => Some(Color::ParrotGreen),
                x if x == stringify!(CloverGreen).to_lowercase() => Some(Color::CloverGreen),
                x if x == stringify!(DinosaurGreen).to_lowercase() => Some(Color::DinosaurGreen),
                x if x == stringify!(GreenSnake).to_lowercase() => Some(Color::GreenSnake),
                x if x == stringify!(AlienGreen).to_lowercase() => Some(Color::AlienGreen),
                x if x == stringify!(GreenApple).to_lowercase() => Some(Color::GreenApple),
                x if x == stringify!(LimeGreen).to_lowercase() => Some(Color::LimeGreen),
                x if x == stringify!(PeaGreen).to_lowercase() => Some(Color::PeaGreen),
                x if x == stringify!(KellyGreen).to_lowercase() => Some(Color::KellyGreen),
                x if x == stringify!(ZombieGreen).to_lowercase() => Some(Color::ZombieGreen),
                x if x == stringify!(GreenPeas).to_lowercase() => Some(Color::GreenPeas),
                x if x == stringify!(DollarBillGreen).to_lowercase() => {
                    Some(Color::DollarBillGreen)
                }
                x if x == stringify!(FrogGreen).to_lowercase() => Some(Color::FrogGreen),
                x if x == stringify!(TurquoiseGreen).to_lowercase() => Some(Color::TurquoiseGreen),
                x if x == stringify!(DarkSeaGreen).to_lowercase() => Some(Color::DarkSeaGreen),
                x if x == stringify!(BasilGreen).to_lowercase() => Some(Color::BasilGreen),
                x if x == stringify!(GrayGreen).to_lowercase() => Some(Color::GrayGreen),
                x if x == stringify!(LightOliveGreen).to_lowercase() => {
                    Some(Color::LightOliveGreen)
                }
                x if x == stringify!(IguanaGreen).to_lowercase() => Some(Color::IguanaGreen),
                x if x == stringify!(CitronGreen).to_lowercase() => Some(Color::CitronGreen),
                x if x == stringify!(AcidGreen).to_lowercase() => Some(Color::AcidGreen),
                x if x == stringify!(AvocadoGreen).to_lowercase() => Some(Color::AvocadoGreen),
                x if x == stringify!(PistachioGreen).to_lowercase() => Some(Color::PistachioGreen),
                x if x == stringify!(SaladGreen).to_lowercase() => Some(Color::SaladGreen),
                x if x == stringify!(YellowGreen).to_lowercase() => Some(Color::YellowGreen),
                x if x == stringify!(PastelGreen).to_lowercase() => Some(Color::PastelGreen),
                x if x == stringify!(HummingbirdGreen).to_lowercase() => {
                    Some(Color::HummingbirdGreen)
                }
                x if x == stringify!(NebulaGreen).to_lowercase() => Some(Color::NebulaGreen),
                x if x == stringify!(StoplightGoGreen).to_lowercase() => {
                    Some(Color::StoplightGoGreen)
                }
                x if x == stringify!(NeonGreen).to_lowercase() => Some(Color::NeonGreen),
                x if x == stringify!(JadeGreen).to_lowercase() => Some(Color::JadeGreen),
                x if x == stringify!(SpringGreen).to_lowercase() => Some(Color::SpringGreen),
                x if x == stringify!(OceanGreen).to_lowercase() => Some(Color::OceanGreen),
                x if x == stringify!(LimeMintGreen).to_lowercase() => Some(Color::LimeMintGreen),
                x if x == stringify!(MediumSpringGreen).to_lowercase() => {
                    Some(Color::MediumSpringGreen)
                }
                x if x == stringify!(AquaGreen).to_lowercase() => Some(Color::AquaGreen),
                x if x == stringify!(EmeraldGreen).to_lowercase() => Some(Color::EmeraldGreen),
                x if x == stringify!(Lime).to_lowercase() => Some(Color::Lime),
                x if x == stringify!(LawnGreen).to_lowercase() => Some(Color::LawnGreen),
                x if x == stringify!(BrightGreen).to_lowercase() => Some(Color::BrightGreen),
                x if x == stringify!(Chartreuse).to_lowercase() => Some(Color::Chartreuse),
                x if x == stringify!(YellowLawnGreen).to_lowercase() => {
                    Some(Color::YellowLawnGreen)
                }
                x if x == stringify!(AloeVeraGreen).to_lowercase() => Some(Color::AloeVeraGreen),
                x if x == stringify!(DullGreenYellow).to_lowercase() => {
                    Some(Color::DullGreenYellow)
                }
                x if x == stringify!(LemonGreen).to_lowercase() => Some(Color::LemonGreen),
                x if x == stringify!(GreenYellow).to_lowercase() => Some(Color::GreenYellow),
                x if x == stringify!(ChameleonGreen).to_lowercase() => Some(Color::ChameleonGreen),
                x if x == stringify!(NeonYellowGreen).to_lowercase() => {
                    Some(Color::NeonYellowGreen)
                }
                x if x == stringify!(YellowGreenGrosbeak).to_lowercase() => {
                    Some(Color::YellowGreenGrosbeak)
                }
                x if x == stringify!(TeaGreen).to_lowercase() => Some(Color::TeaGreen),
                x if x == stringify!(SlimeGreen).to_lowercase() => Some(Color::SlimeGreen),
                x if x == stringify!(AlgaeGreen).to_lowercase() => Some(Color::AlgaeGreen),
                x if x == stringify!(LightGreen).to_lowercase() => Some(Color::LightGreen),
                x if x == stringify!(DragonGreen).to_lowercase() => Some(Color::DragonGreen),
                x if x == stringify!(PaleGreen).to_lowercase() => Some(Color::PaleGreen),
                x if x == stringify!(MintGreen).to_lowercase() => Some(Color::MintGreen),
                x if x == stringify!(GreenThumb).to_lowercase() => Some(Color::GreenThumb),
                x if x == stringify!(OrganicBrown).to_lowercase() => Some(Color::OrganicBrown),
                x if x == stringify!(LightJade).to_lowercase() => Some(Color::LightJade),
                x if x == stringify!(LightMintGreen).to_lowercase() => Some(Color::LightMintGreen),
                x if x == stringify!(LightRoseGreen).to_lowercase() => Some(Color::LightRoseGreen),
                x if x == stringify!(ChromeWhite).to_lowercase() => Some(Color::ChromeWhite),
                x if x == stringify!(HoneyDew).to_lowercase() => Some(Color::HoneyDew),
                x if x == stringify!(MintCream).to_lowercase() => Some(Color::MintCream),
                x if x == stringify!(LemonChiffon).to_lowercase() => Some(Color::LemonChiffon),
                x if x == stringify!(Parchment).to_lowercase() => Some(Color::Parchment),
                x if x == stringify!(Cream).to_lowercase() => Some(Color::Cream),
                x if x == stringify!(CreamWhite).to_lowercase() => Some(Color::CreamWhite),
                x if x == stringify!(LightGoldenRodYellow).to_lowercase() => {
                    Some(Color::LightGoldenRodYellow)
                }
                x if x == stringify!(LightYellow).to_lowercase() => Some(Color::LightYellow),
                x if x == stringify!(Beige).to_lowercase() => Some(Color::Beige),
                x if x == stringify!(WhiteYellow).to_lowercase() => Some(Color::WhiteYellow),
                x if x == stringify!(Cornsilk).to_lowercase() => Some(Color::Cornsilk),
                x if x == stringify!(Blonde).to_lowercase() => Some(Color::Blonde),
                x if x == stringify!(AntiqueWhite).to_lowercase() => Some(Color::AntiqueWhite),
                x if x == stringify!(LightBeige).to_lowercase() => Some(Color::LightBeige),
                x if x == stringify!(PapayaWhip).to_lowercase() => Some(Color::PapayaWhip),
                x if x == stringify!(Champagne).to_lowercase() => Some(Color::Champagne),
                x if x == stringify!(BlanchedAlmond).to_lowercase() => Some(Color::BlanchedAlmond),
                x if x == stringify!(Bisque).to_lowercase() => Some(Color::Bisque),
                x if x == stringify!(Wheat).to_lowercase() => Some(Color::Wheat),
                x if x == stringify!(Moccasin).to_lowercase() => Some(Color::Moccasin),
                x if x == stringify!(Peach).to_lowercase() => Some(Color::Peach),
                x if x == stringify!(LightOrange).to_lowercase() => Some(Color::LightOrange),
                x if x == stringify!(PeachPuff).to_lowercase() => Some(Color::PeachPuff),
                x if x == stringify!(CoralPeach).to_lowercase() => Some(Color::CoralPeach),
                x if x == stringify!(NavajoWhite).to_lowercase() => Some(Color::NavajoWhite),
                x if x == stringify!(GoldenBlonde).to_lowercase() => Some(Color::GoldenBlonde),
                x if x == stringify!(GoldenSilk).to_lowercase() => Some(Color::GoldenSilk),
                x if x == stringify!(DarkBlonde).to_lowercase() => Some(Color::DarkBlonde),
                x if x == stringify!(LightGold).to_lowercase() => Some(Color::LightGold),
                x if x == stringify!(Vanilla).to_lowercase() => Some(Color::Vanilla),
                x if x == stringify!(TanBrown).to_lowercase() => Some(Color::TanBrown),
                x if x == stringify!(DirtyWhite).to_lowercase() => Some(Color::DirtyWhite),
                x if x == stringify!(PaleGoldenRod).to_lowercase() => Some(Color::PaleGoldenRod),
                x if x == stringify!(Khaki).to_lowercase() => Some(Color::Khaki),
                x if x == stringify!(CardboardBrown).to_lowercase() => Some(Color::CardboardBrown),
                x if x == stringify!(HarvestGold).to_lowercase() => Some(Color::HarvestGold),
                x if x == stringify!(SunYellow).to_lowercase() => Some(Color::SunYellow),
                x if x == stringify!(CornYellow).to_lowercase() => Some(Color::CornYellow),
                x if x == stringify!(PastelYellow).to_lowercase() => Some(Color::PastelYellow),
                x if x == stringify!(NeonYellow).to_lowercase() => Some(Color::NeonYellow),
                x if x == stringify!(Yellow).to_lowercase() => Some(Color::Yellow),
                x if x == stringify!(LemonYellow).to_lowercase() => Some(Color::LemonYellow),
                x if x == stringify!(CanaryYellow).to_lowercase() => Some(Color::CanaryYellow),
                x if x == stringify!(BananaYellow).to_lowercase() => Some(Color::BananaYellow),
                x if x == stringify!(MustardYellow).to_lowercase() => Some(Color::MustardYellow),
                x if x == stringify!(GoldenYellow).to_lowercase() => Some(Color::GoldenYellow),
                x if x == stringify!(BoldYellow).to_lowercase() => Some(Color::BoldYellow),
                x if x == stringify!(SafetyYellow).to_lowercase() => Some(Color::SafetyYellow),
                x if x == stringify!(RubberDuckyYellow).to_lowercase() => {
                    Some(Color::RubberDuckyYellow)
                }
                x if x == stringify!(Gold).to_lowercase() => Some(Color::Gold),
                x if x == stringify!(BrightGold).to_lowercase() => Some(Color::BrightGold),
                x if x == stringify!(ChromeGold).to_lowercase() => Some(Color::ChromeGold),
                x if x == stringify!(GoldenBrown).to_lowercase() => Some(Color::GoldenBrown),
                x if x == stringify!(DeepYellow).to_lowercase() => Some(Color::DeepYellow),
                x if x == stringify!(MacaroniandCheese).to_lowercase() => {
                    Some(Color::MacaroniandCheese)
                }
                x if x == stringify!(Amber).to_lowercase() => Some(Color::Amber),
                x if x == stringify!(Saffron).to_lowercase() => Some(Color::Saffron),
                x if x == stringify!(NeonGold).to_lowercase() => Some(Color::NeonGold),
                x if x == stringify!(Beer).to_lowercase() => Some(Color::Beer),
                x if x == stringify!(YellowOrange).to_lowercase() => Some(Color::YellowOrange),
                x if x == stringify!(OrangeYellow).to_lowercase() => Some(Color::OrangeYellow),
                x if x == stringify!(Cantaloupe).to_lowercase() => Some(Color::Cantaloupe),
                x if x == stringify!(CheeseOrange).to_lowercase() => Some(Color::CheeseOrange),
                x if x == stringify!(Orange).to_lowercase() => Some(Color::Orange),
                x if x == stringify!(BrownSand).to_lowercase() => Some(Color::BrownSand),
                x if x == stringify!(SandyBrown).to_lowercase() => Some(Color::SandyBrown),
                x if x == stringify!(BrownSugar).to_lowercase() => Some(Color::BrownSugar),
                x if x == stringify!(CamelBrown).to_lowercase() => Some(Color::CamelBrown),
                x if x == stringify!(DeerBrown).to_lowercase() => Some(Color::DeerBrown),
                x if x == stringify!(BurlyWood).to_lowercase() => Some(Color::BurlyWood),
                x if x == stringify!(Tan).to_lowercase() => Some(Color::Tan),
                x if x == stringify!(LightFrenchBeige).to_lowercase() => {
                    Some(Color::LightFrenchBeige)
                }
                x if x == stringify!(Sand).to_lowercase() => Some(Color::Sand),
                x if x == stringify!(SoftHazel).to_lowercase() => Some(Color::SoftHazel),
                x if x == stringify!(Sage).to_lowercase() => Some(Color::Sage),
                x if x == stringify!(FallLeafBrown).to_lowercase() => Some(Color::FallLeafBrown),
                x if x == stringify!(GingerBrown).to_lowercase() => Some(Color::GingerBrown),
                x if x == stringify!(BronzeGold).to_lowercase() => Some(Color::BronzeGold),
                x if x == stringify!(DarkKhaki).to_lowercase() => Some(Color::DarkKhaki),
                x if x == stringify!(OliveGreen).to_lowercase() => Some(Color::OliveGreen),
                x if x == stringify!(Brass).to_lowercase() => Some(Color::Brass),
                x if x == stringify!(CookieBrown).to_lowercase() => Some(Color::CookieBrown),
                x if x == stringify!(MetallicGold).to_lowercase() => Some(Color::MetallicGold),
                x if x == stringify!(Mustard).to_lowercase() => Some(Color::Mustard),
                x if x == stringify!(BeeYellow).to_lowercase() => Some(Color::BeeYellow),
                x if x == stringify!(SchoolBusYellow).to_lowercase() => {
                    Some(Color::SchoolBusYellow)
                }
                x if x == stringify!(GoldenRod).to_lowercase() => Some(Color::GoldenRod),
                x if x == stringify!(OrangeGold).to_lowercase() => Some(Color::OrangeGold),
                x if x == stringify!(Caramel).to_lowercase() => Some(Color::Caramel),
                x if x == stringify!(DarkGoldenRod).to_lowercase() => Some(Color::DarkGoldenRod),
                x if x == stringify!(Cinnamon).to_lowercase() => Some(Color::Cinnamon),
                x if x == stringify!(Peru).to_lowercase() => Some(Color::Peru),
                x if x == stringify!(Bronze).to_lowercase() => Some(Color::Bronze),
                x if x == stringify!(PumpkinPie).to_lowercase() => Some(Color::PumpkinPie),
                x if x == stringify!(TigerOrange).to_lowercase() => Some(Color::TigerOrange),
                x if x == stringify!(Copper).to_lowercase() => Some(Color::Copper),
                x if x == stringify!(DarkGold).to_lowercase() => Some(Color::DarkGold),
                x if x == stringify!(MetallicBronze).to_lowercase() => Some(Color::MetallicBronze),
                x if x == stringify!(DarkAlmond).to_lowercase() => Some(Color::DarkAlmond),
                x if x == stringify!(Wood).to_lowercase() => Some(Color::Wood),
                x if x == stringify!(KhakiBrown).to_lowercase() => Some(Color::KhakiBrown),
                x if x == stringify!(OakBrown).to_lowercase() => Some(Color::OakBrown),
                x if x == stringify!(AntiqueBronze).to_lowercase() => Some(Color::AntiqueBronze),
                x if x == stringify!(Hazel).to_lowercase() => Some(Color::Hazel),
                x if x == stringify!(DarkYellow).to_lowercase() => Some(Color::DarkYellow),
                x if x == stringify!(DarkMoccasin).to_lowercase() => Some(Color::DarkMoccasin),
                x if x == stringify!(KhakiGreen).to_lowercase() => Some(Color::KhakiGreen),
                x if x == stringify!(MillenniumJade).to_lowercase() => Some(Color::MillenniumJade),
                x if x == stringify!(DarkBeige).to_lowercase() => Some(Color::DarkBeige),
                x if x == stringify!(BulletShell).to_lowercase() => Some(Color::BulletShell),
                x if x == stringify!(ArmyBrown).to_lowercase() => Some(Color::ArmyBrown),
                x if x == stringify!(Sandstone).to_lowercase() => Some(Color::Sandstone),
                x if x == stringify!(Taupe).to_lowercase() => Some(Color::Taupe),
                x if x == stringify!(DarkGrayishOlive).to_lowercase() => {
                    Some(Color::DarkGrayishOlive)
                }
                x if x == stringify!(DarkHazelBrown).to_lowercase() => Some(Color::DarkHazelBrown),
                x if x == stringify!(Mocha).to_lowercase() => Some(Color::Mocha),
                x if x == stringify!(MilkChocolate).to_lowercase() => Some(Color::MilkChocolate),
                x if x == stringify!(GrayBrown).to_lowercase() => Some(Color::GrayBrown),
                x if x == stringify!(DarkCoffee).to_lowercase() => Some(Color::DarkCoffee),
                x if x == stringify!(WesternCharcoal).to_lowercase() => {
                    Some(Color::WesternCharcoal)
                }
                x if x == stringify!(OldBurgundy).to_lowercase() => Some(Color::OldBurgundy),
                x if x == stringify!(RedBrown).to_lowercase() => Some(Color::RedBrown),
                x if x == stringify!(BakersBrown).to_lowercase() => Some(Color::BakersBrown),
                x if x == stringify!(PullmanBrown).to_lowercase() => Some(Color::PullmanBrown),
                x if x == stringify!(DarkBrown).to_lowercase() => Some(Color::DarkBrown),
                x if x == stringify!(SepiaBrown).to_lowercase() => Some(Color::SepiaBrown),
                x if x == stringify!(DarkBronze).to_lowercase() => Some(Color::DarkBronze),
                x if x == stringify!(Coffee).to_lowercase() => Some(Color::Coffee),
                x if x == stringify!(BrownBear).to_lowercase() => Some(Color::BrownBear),
                x if x == stringify!(RedDirt).to_lowercase() => Some(Color::RedDirt),
                x if x == stringify!(Sepia).to_lowercase() => Some(Color::Sepia),
                x if x == stringify!(Sienna).to_lowercase() => Some(Color::Sienna),
                x if x == stringify!(SaddleBrown).to_lowercase() => Some(Color::SaddleBrown),
                x if x == stringify!(DarkSienna).to_lowercase() => Some(Color::DarkSienna),
                x if x == stringify!(Sangria).to_lowercase() => Some(Color::Sangria),
                x if x == stringify!(BloodRed).to_lowercase() => Some(Color::BloodRed),
                x if x == stringify!(Chestnut).to_lowercase() => Some(Color::Chestnut),
                x if x == stringify!(CoralBrown).to_lowercase() => Some(Color::CoralBrown),
                x if x == stringify!(DeepAmber).to_lowercase() => Some(Color::DeepAmber),
                x if x == stringify!(ChestnutRed).to_lowercase() => Some(Color::ChestnutRed),
                x if x == stringify!(GingerRed).to_lowercase() => Some(Color::GingerRed),
                x if x == stringify!(Mahogany).to_lowercase() => Some(Color::Mahogany),
                x if x == stringify!(RedGold).to_lowercase() => Some(Color::RedGold),
                x if x == stringify!(RedFox).to_lowercase() => Some(Color::RedFox),
                x if x == stringify!(DarkBisque).to_lowercase() => Some(Color::DarkBisque),
                x if x == stringify!(LightBrown).to_lowercase() => Some(Color::LightBrown),
                x if x == stringify!(PetraGold).to_lowercase() => Some(Color::PetraGold),
                x if x == stringify!(BrownRust).to_lowercase() => Some(Color::BrownRust),
                x if x == stringify!(Rust).to_lowercase() => Some(Color::Rust),
                x if x == stringify!(CopperRed).to_lowercase() => Some(Color::CopperRed),
                x if x == stringify!(OrangeSalmon).to_lowercase() => Some(Color::OrangeSalmon),
                x if x == stringify!(Chocolate).to_lowercase() => Some(Color::Chocolate),
                x if x == stringify!(Sedona).to_lowercase() => Some(Color::Sedona),
                x if x == stringify!(PapayaOrange).to_lowercase() => Some(Color::PapayaOrange),
                x if x == stringify!(HalloweenOrange).to_lowercase() => {
                    Some(Color::HalloweenOrange)
                }
                x if x == stringify!(NeonOrange).to_lowercase() => Some(Color::NeonOrange),
                x if x == stringify!(BrightOrange).to_lowercase() => Some(Color::BrightOrange),
                x if x == stringify!(FluroOrange).to_lowercase() => Some(Color::FluroOrange),
                x if x == stringify!(PumpkinOrange).to_lowercase() => Some(Color::PumpkinOrange),
                x if x == stringify!(SafetyOrange).to_lowercase() => Some(Color::SafetyOrange),
                x if x == stringify!(CarrotOrange).to_lowercase() => Some(Color::CarrotOrange),
                x if x == stringify!(DarkOrange).to_lowercase() => Some(Color::DarkOrange),
                x if x == stringify!(ConstructionConeOrange).to_lowercase() => {
                    Some(Color::ConstructionConeOrange)
                }
                x if x == stringify!(IndianSaffron).to_lowercase() => Some(Color::IndianSaffron),
                x if x == stringify!(SunriseOrange).to_lowercase() => Some(Color::SunriseOrange),
                x if x == stringify!(MangoOrange).to_lowercase() => Some(Color::MangoOrange),
                x if x == stringify!(Coral).to_lowercase() => Some(Color::Coral),
                x if x == stringify!(BasketBallOrange).to_lowercase() => {
                    Some(Color::BasketBallOrange)
                }
                x if x == stringify!(LightSalmonRose).to_lowercase() => {
                    Some(Color::LightSalmonRose)
                }
                x if x == stringify!(LightSalmon).to_lowercase() => Some(Color::LightSalmon),
                x if x == stringify!(PinkOrange).to_lowercase() => Some(Color::PinkOrange),
                x if x == stringify!(DarkSalmon).to_lowercase() => Some(Color::DarkSalmon),
                x if x == stringify!(Tangerine).to_lowercase() => Some(Color::Tangerine),
                x if x == stringify!(LightCopper).to_lowercase() => Some(Color::LightCopper),
                x if x == stringify!(SalmonPink).to_lowercase() => Some(Color::SalmonPink),
                x if x == stringify!(Salmon).to_lowercase() => Some(Color::Salmon),
                x if x == stringify!(PeachPink).to_lowercase() => Some(Color::PeachPink),
                x if x == stringify!(LightCoral).to_lowercase() => Some(Color::LightCoral),
                x if x == stringify!(PastelRed).to_lowercase() => Some(Color::PastelRed),
                x if x == stringify!(PinkCoral).to_lowercase() => Some(Color::PinkCoral),
                x if x == stringify!(BeanRed).to_lowercase() => Some(Color::BeanRed),
                x if x == stringify!(ValentineRed).to_lowercase() => Some(Color::ValentineRed),
                x if x == stringify!(IndianRed).to_lowercase() => Some(Color::IndianRed),
                x if x == stringify!(Tomato).to_lowercase() => Some(Color::Tomato),
                x if x == stringify!(ShockingOrange).to_lowercase() => Some(Color::ShockingOrange),
                x if x == stringify!(OrangeRed).to_lowercase() => Some(Color::OrangeRed),
                x if x == stringify!(Red).to_lowercase() => Some(Color::Red),
                x if x == stringify!(NeonRed).to_lowercase() => Some(Color::NeonRed),
                x if x == stringify!(ScarletRed).to_lowercase() => Some(Color::ScarletRed),
                x if x == stringify!(RubyRed).to_lowercase() => Some(Color::RubyRed),
                x if x == stringify!(FerrariRed).to_lowercase() => Some(Color::FerrariRed),
                x if x == stringify!(FireEngineRed).to_lowercase() => Some(Color::FireEngineRed),
                x if x == stringify!(LavaRed).to_lowercase() => Some(Color::LavaRed),
                x if x == stringify!(LoveRed).to_lowercase() => Some(Color::LoveRed),
                x if x == stringify!(Grapefruit).to_lowercase() => Some(Color::Grapefruit),
                x if x == stringify!(StrawberryRed).to_lowercase() => Some(Color::StrawberryRed),
                x if x == stringify!(CherryRed).to_lowercase() => Some(Color::CherryRed),
                x if x == stringify!(ChilliPepper).to_lowercase() => Some(Color::ChilliPepper),
                x if x == stringify!(FireBrick).to_lowercase() => Some(Color::FireBrick),
                x if x == stringify!(TomatoSauceRed).to_lowercase() => Some(Color::TomatoSauceRed),
                x if x == stringify!(Brown).to_lowercase() => Some(Color::Brown),
                x if x == stringify!(CarbonRed).to_lowercase() => Some(Color::CarbonRed),
                x if x == stringify!(Cranberry).to_lowercase() => Some(Color::Cranberry),
                x if x == stringify!(SaffronRed).to_lowercase() => Some(Color::SaffronRed),
                x if x == stringify!(CrimsonRed).to_lowercase() => Some(Color::CrimsonRed),
                x if x == stringify!(RedWine).to_lowercase() => Some(Color::RedWine),
                x if x == stringify!(WineRed).to_lowercase() => Some(Color::WineRed),
                x if x == stringify!(DarkRed).to_lowercase() => Some(Color::DarkRed),
                x if x == stringify!(MaroonRed).to_lowercase() => Some(Color::MaroonRed),
                x if x == stringify!(Maroon).to_lowercase() => Some(Color::Maroon),
                x if x == stringify!(Burgundy).to_lowercase() => Some(Color::Burgundy),
                x if x == stringify!(Vermilion).to_lowercase() => Some(Color::Vermilion),
                x if x == stringify!(DeepRed).to_lowercase() => Some(Color::DeepRed),
                x if x == stringify!(GarnetRed).to_lowercase() => Some(Color::GarnetRed),
                x if x == stringify!(RedBlood).to_lowercase() => Some(Color::RedBlood),
                x if x == stringify!(BloodNight).to_lowercase() => Some(Color::BloodNight),
                x if x == stringify!(DarkScarlet).to_lowercase() => Some(Color::DarkScarlet),
                x if x == stringify!(ChocolateBrown).to_lowercase() => Some(Color::ChocolateBrown),
                x if x == stringify!(BlackBean).to_lowercase() => Some(Color::BlackBean),
                x if x == stringify!(DarkMaroon).to_lowercase() => Some(Color::DarkMaroon),
                x if x == stringify!(Midnight).to_lowercase() => Some(Color::Midnight),
                x if x == stringify!(PurpleLily).to_lowercase() => Some(Color::PurpleLily),
                x if x == stringify!(PurpleMaroon).to_lowercase() => Some(Color::PurpleMaroon),
                x if x == stringify!(PlumPie).to_lowercase() => Some(Color::PlumPie),
                x if x == stringify!(PlumVelvet).to_lowercase() => Some(Color::PlumVelvet),
                x if x == stringify!(DarkRaspberry).to_lowercase() => Some(Color::DarkRaspberry),
                x if x == stringify!(VelvetMaroon).to_lowercase() => Some(Color::VelvetMaroon),
                x if x == stringify!(RosyFinch).to_lowercase() => Some(Color::RosyFinch),
                x if x == stringify!(DullPurple).to_lowercase() => Some(Color::DullPurple),
                x if x == stringify!(Puce).to_lowercase() => Some(Color::Puce),
                x if x == stringify!(RoseDust).to_lowercase() => Some(Color::RoseDust),
                x if x == stringify!(PastelBrown).to_lowercase() => Some(Color::PastelBrown),
                x if x == stringify!(RosyPink).to_lowercase() => Some(Color::RosyPink),
                x if x == stringify!(RosyBrown).to_lowercase() => Some(Color::RosyBrown),
                x if x == stringify!(KhakiRose).to_lowercase() => Some(Color::KhakiRose),
                x if x == stringify!(LipstickPink).to_lowercase() => Some(Color::LipstickPink),
                x if x == stringify!(DuskyPink).to_lowercase() => Some(Color::DuskyPink),
                x if x == stringify!(PinkBrown).to_lowercase() => Some(Color::PinkBrown),
                x if x == stringify!(OldRose).to_lowercase() => Some(Color::OldRose),
                x if x == stringify!(DustyPink).to_lowercase() => Some(Color::DustyPink),
                x if x == stringify!(PinkDaisy).to_lowercase() => Some(Color::PinkDaisy),
                x if x == stringify!(Rose).to_lowercase() => Some(Color::Rose),
                x if x == stringify!(DustyRose).to_lowercase() => Some(Color::DustyRose),
                x if x == stringify!(SilverPink).to_lowercase() => Some(Color::SilverPink),
                x if x == stringify!(GoldPink).to_lowercase() => Some(Color::GoldPink),
                x if x == stringify!(RoseGold).to_lowercase() => Some(Color::RoseGold),
                x if x == stringify!(DeepPeach).to_lowercase() => Some(Color::DeepPeach),
                x if x == stringify!(PastelOrange).to_lowercase() => Some(Color::PastelOrange),
                x if x == stringify!(DesertSand).to_lowercase() => Some(Color::DesertSand),
                x if x == stringify!(UnbleachedSilk).to_lowercase() => Some(Color::UnbleachedSilk),
                x if x == stringify!(PigPink).to_lowercase() => Some(Color::PigPink),
                x if x == stringify!(PalePink).to_lowercase() => Some(Color::PalePink),
                x if x == stringify!(Blush).to_lowercase() => Some(Color::Blush),
                x if x == stringify!(MistyRose).to_lowercase() => Some(Color::MistyRose),
                x if x == stringify!(PinkBubbleGum).to_lowercase() => Some(Color::PinkBubbleGum),
                x if x == stringify!(LightRose).to_lowercase() => Some(Color::LightRose),
                x if x == stringify!(LightRed).to_lowercase() => Some(Color::LightRed),
                x if x == stringify!(RoseQuartz).to_lowercase() => Some(Color::RoseQuartz),
                x if x == stringify!(WarmPink).to_lowercase() => Some(Color::WarmPink),
                x if x == stringify!(DeepRose).to_lowercase() => Some(Color::DeepRose),
                x if x == stringify!(Pink).to_lowercase() => Some(Color::Pink),
                x if x == stringify!(LightPink).to_lowercase() => Some(Color::LightPink),
                x if x == stringify!(SoftPink).to_lowercase() => Some(Color::SoftPink),
                x if x == stringify!(PowderPink).to_lowercase() => Some(Color::PowderPink),
                x if x == stringify!(DonutPink).to_lowercase() => Some(Color::DonutPink),
                x if x == stringify!(BabyPink).to_lowercase() => Some(Color::BabyPink),
                x if x == stringify!(FlamingoPink).to_lowercase() => Some(Color::FlamingoPink),
                x if x == stringify!(PastelPink).to_lowercase() => Some(Color::PastelPink),
                x if x == stringify!(RosePink).to_lowercase() => Some(Color::RosePink),
                x if x == stringify!(CadillacPink).to_lowercase() => Some(Color::CadillacPink),
                x if x == stringify!(CarnationPink).to_lowercase() => Some(Color::CarnationPink),
                x if x == stringify!(PastelRose).to_lowercase() => Some(Color::PastelRose),
                x if x == stringify!(BlushRed).to_lowercase() => Some(Color::BlushRed),
                x if x == stringify!(PaleVioletRed).to_lowercase() => Some(Color::PaleVioletRed),
                x if x == stringify!(PurplePink).to_lowercase() => Some(Color::PurplePink),
                x if x == stringify!(TulipPink).to_lowercase() => Some(Color::TulipPink),
                x if x == stringify!(BashfulPink).to_lowercase() => Some(Color::BashfulPink),
                x if x == stringify!(DarkPink).to_lowercase() => Some(Color::DarkPink),
                x if x == stringify!(DarkHotPink).to_lowercase() => Some(Color::DarkHotPink),
                x if x == stringify!(HotPink).to_lowercase() => Some(Color::HotPink),
                x if x == stringify!(WatermelonPink).to_lowercase() => Some(Color::WatermelonPink),
                x if x == stringify!(VioletRed).to_lowercase() => Some(Color::VioletRed),
                x if x == stringify!(HotDeepPink).to_lowercase() => Some(Color::HotDeepPink),
                x if x == stringify!(BrightPink).to_lowercase() => Some(Color::BrightPink),
                x if x == stringify!(RedMagenta).to_lowercase() => Some(Color::RedMagenta),
                x if x == stringify!(DeepPink).to_lowercase() => Some(Color::DeepPink),
                x if x == stringify!(NeonPink).to_lowercase() => Some(Color::NeonPink),
                x if x == stringify!(ChromePink).to_lowercase() => Some(Color::ChromePink),
                x if x == stringify!(NeonHotPink).to_lowercase() => Some(Color::NeonHotPink),
                x if x == stringify!(PinkCupcake).to_lowercase() => Some(Color::PinkCupcake),
                x if x == stringify!(RoyalPink).to_lowercase() => Some(Color::RoyalPink),
                x if x == stringify!(DimorphothecaMagenta).to_lowercase() => {
                    Some(Color::DimorphothecaMagenta)
                }
                x if x == stringify!(BarbiePink).to_lowercase() => Some(Color::BarbiePink),
                x if x == stringify!(PinkLemonade).to_lowercase() => Some(Color::PinkLemonade),
                x if x == stringify!(RedPink).to_lowercase() => Some(Color::RedPink),
                x if x == stringify!(Raspberry).to_lowercase() => Some(Color::Raspberry),
                x if x == stringify!(Crimson).to_lowercase() => Some(Color::Crimson),
                x if x == stringify!(BrightMaroon).to_lowercase() => Some(Color::BrightMaroon),
                x if x == stringify!(RoseRed).to_lowercase() => Some(Color::RoseRed),
                x if x == stringify!(RoguePink).to_lowercase() => Some(Color::RoguePink),
                x if x == stringify!(BurntPink).to_lowercase() => Some(Color::BurntPink),
                x if x == stringify!(PinkViolet).to_lowercase() => Some(Color::PinkViolet),
                x if x == stringify!(MagentaPink).to_lowercase() => Some(Color::MagentaPink),
                x if x == stringify!(MediumVioletRed).to_lowercase() => {
                    Some(Color::MediumVioletRed)
                }
                x if x == stringify!(DarkCarnationPink).to_lowercase() => {
                    Some(Color::DarkCarnationPink)
                }
                x if x == stringify!(RaspberryPurple).to_lowercase() => {
                    Some(Color::RaspberryPurple)
                }
                x if x == stringify!(PinkPlum).to_lowercase() => Some(Color::PinkPlum),
                x if x == stringify!(Orchid).to_lowercase() => Some(Color::Orchid),
                x if x == stringify!(DeepMauve).to_lowercase() => Some(Color::DeepMauve),
                x if x == stringify!(Violet).to_lowercase() => Some(Color::Violet),
                x if x == stringify!(FuchsiaPink).to_lowercase() => Some(Color::FuchsiaPink),
                x if x == stringify!(BrightNeonPink).to_lowercase() => Some(Color::BrightNeonPink),
                x if x == stringify!(Magenta).to_lowercase() => Some(Color::Magenta),
                x if x == stringify!(Fuchsia).to_lowercase() => Some(Color::Fuchsia),
                x if x == stringify!(CrimsonPurple).to_lowercase() => Some(Color::CrimsonPurple),
                x if x == stringify!(HeliotropePurple).to_lowercase() => {
                    Some(Color::HeliotropePurple)
                }
                x if x == stringify!(TyrianPurple).to_lowercase() => Some(Color::TyrianPurple),
                x if x == stringify!(MediumOrchid).to_lowercase() => Some(Color::MediumOrchid),
                x if x == stringify!(PurpleFlower).to_lowercase() => Some(Color::PurpleFlower),
                x if x == stringify!(OrchidPurple).to_lowercase() => Some(Color::OrchidPurple),
                x if x == stringify!(RichLilac).to_lowercase() => Some(Color::RichLilac),
                x if x == stringify!(PastelViolet).to_lowercase() => Some(Color::PastelViolet),
                x if x == stringify!(Rosy).to_lowercase() => Some(Color::Rosy),
                x if x == stringify!(MauveTaupe).to_lowercase() => Some(Color::MauveTaupe),
                x if x == stringify!(ViolaPurple).to_lowercase() => Some(Color::ViolaPurple),
                x if x == stringify!(Eggplant).to_lowercase() => Some(Color::Eggplant),
                x if x == stringify!(PlumPurple).to_lowercase() => Some(Color::PlumPurple),
                x if x == stringify!(Grape).to_lowercase() => Some(Color::Grape),
                x if x == stringify!(PurpleNavy).to_lowercase() => Some(Color::PurpleNavy),
                x if x == stringify!(SlateBlue).to_lowercase() => Some(Color::SlateBlue),
                x if x == stringify!(BlueLotus).to_lowercase() => Some(Color::BlueLotus),
                x if x == stringify!(Blurple).to_lowercase() => Some(Color::Blurple),
                x if x == stringify!(LightSlateBlue).to_lowercase() => Some(Color::LightSlateBlue),
                x if x == stringify!(MediumSlateBlue).to_lowercase() => {
                    Some(Color::MediumSlateBlue)
                }
                x if x == stringify!(PeriwinklePurple).to_lowercase() => {
                    Some(Color::PeriwinklePurple)
                }
                x if x == stringify!(VeryPeri).to_lowercase() => Some(Color::VeryPeri),
                x if x == stringify!(BrightGrape).to_lowercase() => Some(Color::BrightGrape),
                x if x == stringify!(BrightPurple).to_lowercase() => Some(Color::BrightPurple),
                x if x == stringify!(PurpleAmethyst).to_lowercase() => Some(Color::PurpleAmethyst),
                x if x == stringify!(BlueMagenta).to_lowercase() => Some(Color::BlueMagenta),
                x if x == stringify!(DarkBlurple).to_lowercase() => Some(Color::DarkBlurple),
                x if x == stringify!(DeepPeriwinkle).to_lowercase() => Some(Color::DeepPeriwinkle),
                x if x == stringify!(DarkSlateBlue).to_lowercase() => Some(Color::DarkSlateBlue),
                x if x == stringify!(PurpleHaze).to_lowercase() => Some(Color::PurpleHaze),
                x if x == stringify!(PurpleIris).to_lowercase() => Some(Color::PurpleIris),
                x if x == stringify!(DarkPurple).to_lowercase() => Some(Color::DarkPurple),
                x if x == stringify!(DeepPurple).to_lowercase() => Some(Color::DeepPurple),
                x if x == stringify!(MidnightPurple).to_lowercase() => Some(Color::MidnightPurple),
                x if x == stringify!(PurpleMonster).to_lowercase() => Some(Color::PurpleMonster),
                x if x == stringify!(Indigo).to_lowercase() => Some(Color::Indigo),
                x if x == stringify!(BlueWhale).to_lowercase() => Some(Color::BlueWhale),
                x if x == stringify!(RebeccaPurple).to_lowercase() => Some(Color::RebeccaPurple),
                x if x == stringify!(PurpleJam).to_lowercase() => Some(Color::PurpleJam),
                x if x == stringify!(DarkMagenta).to_lowercase() => Some(Color::DarkMagenta),
                x if x == stringify!(Purple).to_lowercase() => Some(Color::Purple),
                x if x == stringify!(FrenchLilac).to_lowercase() => Some(Color::FrenchLilac),
                x if x == stringify!(DarkOrchid).to_lowercase() => Some(Color::DarkOrchid),
                x if x == stringify!(DarkViolet).to_lowercase() => Some(Color::DarkViolet),
                x if x == stringify!(PurpleViolet).to_lowercase() => Some(Color::PurpleViolet),
                x if x == stringify!(JasminePurple).to_lowercase() => Some(Color::JasminePurple),
                x if x == stringify!(PurpleDaffodil).to_lowercase() => Some(Color::PurpleDaffodil),
                x if x == stringify!(ClematisViolet).to_lowercase() => Some(Color::ClematisViolet),
                x if x == stringify!(BlueViolet).to_lowercase() => Some(Color::BlueViolet),
                x if x == stringify!(PurpleSageBush).to_lowercase() => Some(Color::PurpleSageBush),
                x if x == stringify!(LovelyPurple).to_lowercase() => Some(Color::LovelyPurple),
                x if x == stringify!(NeonPurple).to_lowercase() => Some(Color::NeonPurple),
                x if x == stringify!(PurplePlum).to_lowercase() => Some(Color::PurplePlum),
                x if x == stringify!(AztechPurple).to_lowercase() => Some(Color::AztechPurple),
                x if x == stringify!(MediumPurple).to_lowercase() => Some(Color::MediumPurple),
                x if x == stringify!(LightPurple).to_lowercase() => Some(Color::LightPurple),
                x if x == stringify!(CrocusPurple).to_lowercase() => Some(Color::CrocusPurple),
                x if x == stringify!(PurpleMimosa).to_lowercase() => Some(Color::PurpleMimosa),
                x if x == stringify!(PastelIndigo).to_lowercase() => Some(Color::PastelIndigo),
                x if x == stringify!(LavenderPurple).to_lowercase() => Some(Color::LavenderPurple),
                x if x == stringify!(RosePurple).to_lowercase() => Some(Color::RosePurple),
                x if x == stringify!(Viola).to_lowercase() => Some(Color::Viola),
                x if x == stringify!(Periwinkle).to_lowercase() => Some(Color::Periwinkle),
                x if x == stringify!(PaleLilac).to_lowercase() => Some(Color::PaleLilac),
                x if x == stringify!(Lilac).to_lowercase() => Some(Color::Lilac),
                x if x == stringify!(Mauve).to_lowercase() => Some(Color::Mauve),
                x if x == stringify!(BrightLilac).to_lowercase() => Some(Color::BrightLilac),
                x if x == stringify!(PurpleDragon).to_lowercase() => Some(Color::PurpleDragon),
                x if x == stringify!(Plum).to_lowercase() => Some(Color::Plum),
                x if x == stringify!(BlushPink).to_lowercase() => Some(Color::BlushPink),
                x if x == stringify!(PastelPurple).to_lowercase() => Some(Color::PastelPurple),
                x if x == stringify!(BlossomPink).to_lowercase() => Some(Color::BlossomPink),
                x if x == stringify!(WisteriaPurple).to_lowercase() => Some(Color::WisteriaPurple),
                x if x == stringify!(PurpleThistle).to_lowercase() => Some(Color::PurpleThistle),
                x if x == stringify!(Thistle).to_lowercase() => Some(Color::Thistle),
                x if x == stringify!(PurpleWhite).to_lowercase() => Some(Color::PurpleWhite),
                x if x == stringify!(PeriwinklePink).to_lowercase() => Some(Color::PeriwinklePink),
                x if x == stringify!(CottonCandy).to_lowercase() => Some(Color::CottonCandy),
                x if x == stringify!(LavenderPinocchio).to_lowercase() => {
                    Some(Color::LavenderPinocchio)
                }
                x if x == stringify!(DarkWhite).to_lowercase() => Some(Color::DarkWhite),
                x if x == stringify!(AshWhite).to_lowercase() => Some(Color::AshWhite),
                x if x == stringify!(WarmWhite).to_lowercase() => Some(Color::WarmWhite),
                x if x == stringify!(WhiteChocolate).to_lowercase() => Some(Color::WhiteChocolate),
                x if x == stringify!(CreamyWhite).to_lowercase() => Some(Color::CreamyWhite),
                x if x == stringify!(OffWhite).to_lowercase() => Some(Color::OffWhite),
                x if x == stringify!(SoftIvory).to_lowercase() => Some(Color::SoftIvory),
                x if x == stringify!(CosmicLatte).to_lowercase() => Some(Color::CosmicLatte),
                x if x == stringify!(PearlWhite).to_lowercase() => Some(Color::PearlWhite),
                x if x == stringify!(RedWhite).to_lowercase() => Some(Color::RedWhite),
                x if x == stringify!(LavenderBlush).to_lowercase() => Some(Color::LavenderBlush),
                x if x == stringify!(Pearl).to_lowercase() => Some(Color::Pearl),
                x if x == stringify!(EggShell).to_lowercase() => Some(Color::EggShell),
                x if x == stringify!(OldLace).to_lowercase() => Some(Color::OldLace),
                x if x == stringify!(WhiteIce).to_lowercase() => Some(Color::WhiteIce),
                x if x == stringify!(Linen).to_lowercase() => Some(Color::Linen),
                x if x == stringify!(SeaShell).to_lowercase() => Some(Color::SeaShell),
                x if x == stringify!(BoneWhite).to_lowercase() => Some(Color::BoneWhite),
                x if x == stringify!(Rice).to_lowercase() => Some(Color::Rice),
                x if x == stringify!(FloralWhite).to_lowercase() => Some(Color::FloralWhite),
                x if x == stringify!(Ivory).to_lowercase() => Some(Color::Ivory),
                x if x == stringify!(WhiteGold).to_lowercase() => Some(Color::WhiteGold),
                x if x == stringify!(LightWhite).to_lowercase() => Some(Color::LightWhite),
                x if x == stringify!(Cotton).to_lowercase() => Some(Color::Cotton),
                x if x == stringify!(Snow).to_lowercase() => Some(Color::Snow),
                x if x == stringify!(MilkWhite).to_lowercase() => Some(Color::MilkWhite),
                x if x == stringify!(HalfWhite).to_lowercase() => Some(Color::HalfWhite),
                x if x == stringify!(White).to_lowercase() => Some(Color::White),
                _ => None,
            }
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
