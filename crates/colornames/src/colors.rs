use once_cell::sync::Lazy;
use std::collections::HashMap;
#[doc = r" A list of named colors"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    AcidGreen,
    AlgaeGreen,
    AliceBlue,
    AlienGray,
    AlienGreen,
    AloeVeraGreen,
    Amber,
    AntiqueBronze,
    AntiqueWhite,
    Aqua,
    AquaGreen,
    AquaSeafoamGreen,
    Aquamarine,
    AquamarineStone,
    ArmyBrown,
    ArmyGreen,
    AshGray,
    AshWhite,
    AvocadoGreen,
    AztechPurple,
    Azure,
    AzureBlue,
    BabyBlue,
    BabyPink,
    BakersBrown,
    BalloonBlue,
    BananaYellow,
    BarbiePink,
    BashfulPink,
    BasilGreen,
    BasketBallOrange,
    BattleshipGray,
    BeanRed,
    BeeYellow,
    Beer,
    BeetleGreen,
    Beige,
    Bisque,
    Black,
    BlackBean,
    BlackBlue,
    BlackCat,
    BlackCow,
    BlackEel,
    BlanchedAlmond,
    Blonde,
    BloodNight,
    BloodRed,
    BlossomPink,
    Blue,
    BlueAngel,
    BlueDiamond,
    BlueDress,
    BlueEyes,
    BlueGray,
    BlueGreen,
    BlueHosta,
    BlueIvy,
    BlueJay,
    BlueKoi,
    BlueLagoon,
    BlueLotus,
    BlueMagenta,
    BlueMossGreen,
    BlueOrchid,
    BlueRibbon,
    BlueTurquoise,
    BlueWhale,
    BlueZircon,
    BlueViolet,
    BlueberryBlue,
    Blurple,
    Blush,
    BlushPink,
    BlushRed,
    BoldYellow,
    BoneWhite,
    BottleGreen,
    Brass,
    BrightBlue,
    BrightCyan,
    BrightGold,
    BrightGrape,
    BrightGreen,
    BrightLilac,
    BrightMaroon,
    BrightNavyBlue,
    BrightNeonPink,
    BrightOrange,
    BrightPink,
    BrightPurple,
    BrightTeal,
    BrightTurquoise,
    BroccoliGreen,
    Bronze,
    BronzeGold,
    Brown,
    BrownBear,
    BrownRust,
    BrownSand,
    BrownSugar,
    BulletShell,
    Burgundy,
    BurlyWood,
    BurntPink,
    ButterflyBlue,
    CactusGreen,
    CadetBlue,
    CadillacPink,
    CamelBrown,
    CamouflageGreen,
    CanaryBlue,
    CanaryYellow,
    Cantaloupe,
    Caramel,
    CarbonGray,
    CarbonRed,
    CardboardBrown,
    CarnationPink,
    CarrotOrange,
    Celeste,
    ChameleonGreen,
    Champagne,
    Charcoal,
    CharcoalBlue,
    Chartreuse,
    CheeseOrange,
    CherryRed,
    Chestnut,
    ChestnutRed,
    ChilliPepper,
    Chocolate,
    ChocolateBrown,
    ChromeAluminum,
    ChromeGold,
    ChromeGreen,
    ChromePink,
    ChromeWhite,
    Cinnamon,
    CitronGreen,
    ClematisViolet,
    CloudyGray,
    CloverGreen,
    CobaltBlue,
    Coffee,
    ColdMetal,
    ColumbiaBlue,
    ConstructionConeOrange,
    CookieBrown,
    Copper,
    CopperRed,
    Coral,
    CoralBlue,
    CoralBrown,
    CoralPeach,
    CornYellow,
    CornflowerBlue,
    Cornsilk,
    CosmicLatte,
    Cotton,
    CottonCandy,
    Cranberry,
    Cream,
    CreamWhite,
    CreamyWhite,
    Crimson,
    CrimsonPurple,
    CrimsonRed,
    CrocusPurple,
    CrystalBlue,
    Cyan,
    CyanBlue,
    CyanOpaque,
    DarkAlmond,
    DarkBeige,
    DarkBisque,
    DarkBlonde,
    DarkBlueGray,
    DarkBlurple,
    DarkBronze,
    DarkBrown,
    DarkCarnationPink,
    DarkCoffee,
    DarkForestGreen,
    DarkGainsboro,
    DarkGold,
    DarkGrayishOlive,
    DarkGreenBlue,
    DarkHazelBrown,
    DarkHotPink,
    DarkLimeGreen,
    DarkMaroon,
    DarkMint,
    DarkMoccasin,
    DarkPink,
    DarkPurple,
    DarkRaspberry,
    DarkScarlet,
    DarkSienna,
    DarkSkyBlue,
    DarkSlate,
    DarkSteampunk,
    DarkTeal,
    DarkWhite,
    DarkYellow,
    DarkBlue,
    DarkCyan,
    DarkGoldenRod,
    DarkGray,
    DarkGreen,
    DarkGrey,
    DarkKhaki,
    DarkMagenta,
    DarkOliveGreen,
    DarkOrange,
    DarkOrchid,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSlateBlue,
    DarkSlateGray,
    DarkSlateGrey,
    DarkTurquoise,
    DarkViolet,
    DaySkyBlue,
    DeepAmber,
    DeepEmeraldGreen,
    DeepGreen,
    DeepMauve,
    DeepPeach,
    DeepPeriwinkle,
    DeepPurple,
    DeepRed,
    DeepRose,
    DeepSea,
    DeepSeaBlue,
    DeepSeaGreen,
    DeepTeal,
    DeepTurquoise,
    DeepYellow,
    DeepPink,
    DeepSkyBlue,
    DeerBrown,
    DenimBlue,
    DenimDarkBlue,
    DesertSand,
    DimGray,
    DimGrey,
    DimorphothecaMagenta,
    DinosaurGreen,
    DirtyWhite,
    DodgerBlue,
    DollarBillGreen,
    DonutPink,
    DragonGreen,
    DullGreenYellow,
    DullPurple,
    DullSeaGreen,
    DuskyPink,
    DustyPink,
    DustyRose,
    EarthBlue,
    EarthGreen,
    Ebony,
    EggShell,
    Eggplant,
    ElectricBlue,
    ElfGreen,
    Emerald,
    EmeraldGreen,
    EstorilBlue,
    FallForestGreen,
    FallLeafBrown,
    FernGreen,
    FerrariRed,
    FireEngineRed,
    FireBrick,
    FlamingoPink,
    FloralWhite,
    FluroOrange,
    ForestGreen,
    FrenchLilac,
    FrogGreen,
    Fuchsia,
    FuchsiaPink,
    Gainsboro,
    GarnetRed,
    GearSteelGray,
    GhostWhite,
    GingerBrown,
    GingerRed,
    GlacialBlueIce,
    Gold,
    GoldPink,
    GoldenBlonde,
    GoldenBrown,
    GoldenSilk,
    GoldenYellow,
    GoldenRod,
    Granite,
    Grape,
    Grapefruit,
    GrassGreen,
    Gray,
    GrayBrown,
    GrayCloud,
    GrayDolphin,
    GrayGoose,
    GrayGreen,
    GrayWolf,
    GrayishTurquoise,
    Green,
    GreenApple,
    GreenLeaves,
    GreenOnion,
    GreenPeas,
    GreenPepper,
    GreenSnake,
    GreenThumb,
    GreenYellow,
    GreenishBlue,
    Grey,
    GulfBlue,
    Gunmetal,
    GunmetalGray,
    HalfWhite,
    HalloweenOrange,
    HarvestGold,
    Hazel,
    HazelGreen,
    HeavenlyBlue,
    HeliotropePurple,
    HoneyDew,
    HotDeepPink,
    HotPink,
    HummingbirdGreen,
    HunterGreen,
    Iceberg,
    IguanaGreen,
    IndianSaffron,
    IndianRed,
    Indigo,
    Iridium,
    IrishGreen,
    IronGray,
    IsleOfManGreen,
    Ivory,
    Jade,
    JadeGreen,
    JasminePurple,
    JeansBlue,
    Jellyfish,
    JetGray,
    JungleGreen,
    KellyGreen,
    Khaki,
    KhakiBrown,
    KhakiGreen,
    KhakiRose,
    LapisBlue,
    LavaRed,
    Lavender,
    LavenderBlue,
    LavenderPinocchio,
    LavenderPurple,
    LavenderBlush,
    LawnGreen,
    LemonGreen,
    LemonYellow,
    LemonChiffon,
    LightAquamarine,
    LightBeige,
    LightBlack,
    LightBrown,
    LightCopper,
    LightDayBlue,
    LightFrenchBeige,
    LightGold,
    LightJade,
    LightMintGreen,
    LightOliveGreen,
    LightOrange,
    LightPurple,
    LightPurpleBlue,
    LightRed,
    LightRose,
    LightRoseGreen,
    LightSalmonRose,
    LightSlate,
    LightSlateBlue,
    LightSteelGray,
    LightTeal,
    LightWhite,
    LightBlue,
    LightCoral,
    LightCyan,
    LightGoldenRodYellow,
    LightGray,
    LightGreen,
    LightGrey,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSlateGrey,
    LightSteelBlue,
    LightYellow,
    Lilac,
    Lime,
    LimeMintGreen,
    LimeGreen,
    Linen,
    LipstickPink,
    LotusGreen,
    LoveRed,
    LovelyPurple,
    MacaroniandCheese,
    MacawBlueGreen,
    Magenta,
    MagentaPink,
    MagicMint,
    Mahogany,
    MangoOrange,
    MarbleBlue,
    Maroon,
    MaroonRed,
    Mauve,
    MauveTaupe,
    MediumForestGreen,
    MediumTeal,
    MediumAquaMarine,
    MediumBlue,
    MediumOrchid,
    MediumPurple,
    MediumSeaGreen,
    MediumSlateBlue,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    Metal,
    MetallicBronze,
    MetallicGold,
    MetallicGreen,
    MetallicSilver,
    MiddayBlue,
    Midnight,
    MidnightPurple,
    MidnightBlue,
    MilitaryGreen,
    MilkChocolate,
    MilkWhite,
    MillenniumJade,
    Mint,
    MintGreen,
    MintCream,
    MistBlue,
    MistyRose,
    Moccasin,
    Mocha,
    MossGreen,
    Mustard,
    MustardYellow,
    NardoGray,
    NavajoWhite,
    Navy,
    NebulaGreen,
    NeonBlue,
    NeonGold,
    NeonGreen,
    NeonHotPink,
    NeonOrange,
    NeonPink,
    NeonPurple,
    NeonRed,
    NeonYellow,
    NeonYellowGreen,
    NewMidnightBlue,
    Night,
    NightBlue,
    NorthernLightsBlue,
    OakBrown,
    OceanBlue,
    OceanGreen,
    OffWhite,
    Oil,
    OldBurgundy,
    OldRose,
    OldLace,
    Olive,
    OliveGreen,
    OliveDrab,
    Orange,
    OrangeGold,
    OrangeSalmon,
    OrangeYellow,
    OrangeRed,
    Orchid,
    OrchidPurple,
    OrganicBrown,
    PaleBlueLily,
    PaleLilac,
    PalePink,
    PaleSilver,
    PaleGoldenRod,
    PaleGreen,
    PaleTurquoise,
    PaleVioletRed,
    PapayaOrange,
    PapayaWhip,
    Parchment,
    ParrotGreen,
    PastelBlue,
    PastelBrown,
    PastelGreen,
    PastelIndigo,
    PastelLightBlue,
    PastelOrange,
    PastelPink,
    PastelPurple,
    PastelRed,
    PastelRose,
    PastelViolet,
    PastelYellow,
    PeaGreen,
    Peach,
    PeachPink,
    PeachPuff,
    Pearl,
    PearlWhite,
    Periwinkle,
    PeriwinklePink,
    PeriwinklePurple,
    Peru,
    PetraGold,
    PigPink,
    PineGreen,
    Pink,
    PinkBrown,
    PinkBubbleGum,
    PinkCoral,
    PinkCupcake,
    PinkDaisy,
    PinkLemonade,
    PinkOrange,
    PinkPlum,
    PinkViolet,
    PistachioGreen,
    Platinum,
    PlatinumGray,
    PlatinumSilver,
    Plum,
    PlumPie,
    PlumPurple,
    PlumVelvet,
    PowderPink,
    PowderBlue,
    Puce,
    PullmanBrown,
    PumpkinOrange,
    PumpkinPie,
    Purple,
    PurpleAmethyst,
    PurpleDaffodil,
    PurpleDragon,
    PurpleFlower,
    PurpleHaze,
    PurpleIris,
    PurpleJam,
    PurpleLily,
    PurpleMaroon,
    PurpleMimosa,
    PurpleMonster,
    PurpleNavy,
    PurplePink,
    PurplePlum,
    PurpleSageBush,
    PurpleThistle,
    PurpleViolet,
    PurpleWhite,
    RacingGreen,
    Raspberry,
    RaspberryPurple,
    RatGray,
    RebeccaPurple,
    Red,
    RedBlood,
    RedBrown,
    RedDirt,
    RedFox,
    RedGold,
    RedMagenta,
    RedPink,
    RedWhite,
    RedWine,
    Rice,
    RichLilac,
    RobinEggBlue,
    RoguePink,
    RomanSilver,
    Rose,
    RoseDust,
    RoseGold,
    RosePink,
    RosePurple,
    RoseQuartz,
    RoseRed,
    Rosy,
    RosyFinch,
    RosyPink,
    RosyBrown,
    RoyalPink,
    RoyalBlue,
    RubberDuckyYellow,
    RubyRed,
    Rust,
    SaddleBrown,
    SafetyOrange,
    SafetyYellow,
    Saffron,
    SaffronRed,
    Sage,
    SageGreen,
    SaladGreen,
    Salmon,
    SalmonPink,
    SamcoBlue,
    Sand,
    Sandstone,
    SandyBrown,
    Sangria,
    SapphireBlue,
    ScarletRed,
    SchoolBusYellow,
    SeaBlue,
    SeaTurtleGreen,
    SeaGreen,
    SeaShell,
    SeafoamGreen,
    SeaweedGreen,
    Sedona,
    Sepia,
    SepiaBrown,
    ShamrockGreen,
    SheetMetal,
    ShockingOrange,
    Sienna,
    SilkBlue,
    Silver,
    SilverPink,
    SilverWhite,
    SkyBlueDress,
    SkyBlue,
    SlateBlueGray,
    SlateGraniteGray,
    SlateBlue,
    SlateGray,
    SlateGrey,
    SlimeGreen,
    SmokeyGray,
    Snow,
    SoftHazel,
    SoftIvory,
    SoftPink,
    SonicSilver,
    SpringGreen,
    StainlessSteelGray,
    Steampunk,
    SteelGray,
    SteelBlue,
    StoplightGoGreen,
    StormyGray,
    StrawberryRed,
    SunYellow,
    SunriseOrange,
    Tan,
    TanBrown,
    Tangerine,
    Taupe,
    TeaGreen,
    Teal,
    TealBlue,
    TealGreen,
    Thistle,
    TiffanyBlue,
    TigerOrange,
    Tomato,
    TomatoSauceRed,
    TronBlue,
    TulipPink,
    Turquoise,
    TurquoiseGreen,
    TyrianPurple,
    UnbleachedSilk,
    ValentineRed,
    VampireGray,
    Vanilla,
    VelvetMaroon,
    VenomGreen,
    Vermilion,
    VeryPeri,
    Viola,
    ViolaPurple,
    Violet,
    VioletRed,
    WarmPink,
    WarmWhite,
    Water,
    WatermelonPink,
    WesternCharcoal,
    Wheat,
    White,
    WhiteBlue,
    WhiteChocolate,
    WhiteGold,
    WhiteGray,
    WhiteIce,
    WhiteYellow,
    WhiteSmoke,
    WindowsBlue,
    WineRed,
    WisteriaPurple,
    Wood,
    Yellow,
    YellowGreenGrosbeak,
    YellowLawnGreen,
    YellowOrange,
    YellowGreen,
    ZombieGreen,
    Rgb(u8, u8, u8),
}
#[doc = r" Convert a hex color string to a `Color` variant"]
static RGB_MAP: Lazy<HashMap<&'static str, Color>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("#B0BF1A", Color::AcidGreen);
    m.insert("#64E986", Color::AlgaeGreen);
    m.insert("#F0F8FF", Color::AliceBlue);
    m.insert("#736F6E", Color::AlienGray);
    m.insert("#6CC417", Color::AlienGreen);
    m.insert("#98F516", Color::AloeVeraGreen);
    m.insert("#FFBF00", Color::Amber);
    m.insert("#665D1E", Color::AntiqueBronze);
    m.insert("#FAEBD7", Color::AntiqueWhite);
    m.insert("#00FFFF", Color::Aqua);
    m.insert("#12E193", Color::AquaGreen);
    m.insert("#93E9BE", Color::AquaSeafoamGreen);
    m.insert("#7FFFD4", Color::Aquamarine);
    m.insert("#348781", Color::AquamarineStone);
    m.insert("#827B60", Color::ArmyBrown);
    m.insert("#4B5320", Color::ArmyGreen);
    m.insert("#666362", Color::AshGray);
    m.insert("#E9E4D4", Color::AshWhite);
    m.insert("#B2C248", Color::AvocadoGreen);
    m.insert("#893BFF", Color::AztechPurple);
    m.insert("#F0FFFF", Color::Azure);
    m.insert("#4863A0", Color::AzureBlue);
    m.insert("#95B9C7", Color::BabyBlue);
    m.insert("#FAAFBA", Color::BabyPink);
    m.insert("#5C3317", Color::BakersBrown);
    m.insert("#2B60DE", Color::BalloonBlue);
    m.insert("#F5E216", Color::BananaYellow);
    m.insert("#DA1884", Color::BarbiePink);
    m.insert("#C25283", Color::BashfulPink);
    m.insert("#829F82", Color::BasilGreen);
    m.insert("#F88158", Color::BasketBallOrange);
    m.insert("#848482", Color::BattleshipGray);
    m.insert("#F75D59", Color::BeanRed);
    m.insert("#E9AB17", Color::BeeYellow);
    m.insert("#FBB117", Color::Beer);
    m.insert("#4C787E", Color::BeetleGreen);
    m.insert("#F5F5DC", Color::Beige);
    m.insert("#FFE4C4", Color::Bisque);
    m.insert("#000000", Color::Black);
    m.insert("#3D0C02", Color::BlackBean);
    m.insert("#040720", Color::BlackBlue);
    m.insert("#413839", Color::BlackCat);
    m.insert("#4C4646", Color::BlackCow);
    m.insert("#463E3F", Color::BlackEel);
    m.insert("#FFEBCD", Color::BlanchedAlmond);
    m.insert("#FBF6D9", Color::Blonde);
    m.insert("#551606", Color::BloodNight);
    m.insert("#7E3517", Color::BloodRed);
    m.insert("#F9B7FF", Color::BlossomPink);
    m.insert("#0000FF", Color::Blue);
    m.insert("#B7CEEC", Color::BlueAngel);
    m.insert("#4EE2EC", Color::BlueDiamond);
    m.insert("#157DEC", Color::BlueDress);
    m.insert("#1569C7", Color::BlueEyes);
    m.insert("#98AFC7", Color::BlueGray);
    m.insert("#7BCCB5", Color::BlueGreen);
    m.insert("#77BFC7", Color::BlueHosta);
    m.insert("#3090C7", Color::BlueIvy);
    m.insert("#2B547E", Color::BlueJay);
    m.insert("#659EC7", Color::BlueKoi);
    m.insert("#8EEBEC", Color::BlueLagoon);
    m.insert("#6960EC", Color::BlueLotus);
    m.insert("#822EFF", Color::BlueMagenta);
    m.insert("#3C565B", Color::BlueMossGreen);
    m.insert("#1F45FC", Color::BlueOrchid);
    m.insert("#306EFF", Color::BlueRibbon);
    m.insert("#43C6DB", Color::BlueTurquoise);
    m.insert("#342D7E", Color::BlueWhale);
    m.insert("#57FEFF", Color::BlueZircon);
    m.insert("#8A2BE2", Color::BlueViolet);
    m.insert("#0041C2", Color::BlueberryBlue);
    m.insert("#5865F2", Color::Blurple);
    m.insert("#FFE6E8", Color::Blush);
    m.insert("#E6A9EC", Color::BlushPink);
    m.insert("#E56E94", Color::BlushRed);
    m.insert("#F9DB24", Color::BoldYellow);
    m.insert("#F9F6EE", Color::BoneWhite);
    m.insert("#006A4E", Color::BottleGreen);
    m.insert("#B5A642", Color::Brass);
    m.insert("#0909FF", Color::BrightBlue);
    m.insert("#0AFFFF", Color::BrightCyan);
    m.insert("#FDD017", Color::BrightGold);
    m.insert("#6F2DA8", Color::BrightGrape);
    m.insert("#66FF00", Color::BrightGreen);
    m.insert("#D891EF", Color::BrightLilac);
    m.insert("#C32148", Color::BrightMaroon);
    m.insert("#1974D2", Color::BrightNavyBlue);
    m.insert("#F433FF", Color::BrightNeonPink);
    m.insert("#FF5F1F", Color::BrightOrange);
    m.insert("#FF007F", Color::BrightPink);
    m.insert("#6A0DAD", Color::BrightPurple);
    m.insert("#01F9C6", Color::BrightTeal);
    m.insert("#16E2F5", Color::BrightTurquoise);
    m.insert("#026C3D", Color::BroccoliGreen);
    m.insert("#CD7F32", Color::Bronze);
    m.insert("#C9AE5D", Color::BronzeGold);
    m.insert("#A52A2A", Color::Brown);
    m.insert("#835C3B", Color::BrownBear);
    m.insert("#A55D35", Color::BrownRust);
    m.insert("#EE9A4D", Color::BrownSand);
    m.insert("#E2A76F", Color::BrownSugar);
    m.insert("#AF9B60", Color::BulletShell);
    m.insert("#8C001A", Color::Burgundy);
    m.insert("#DEB887", Color::BurlyWood);
    m.insert("#C12267", Color::BurntPink);
    m.insert("#38ACEC", Color::ButterflyBlue);
    m.insert("#227442", Color::CactusGreen);
    m.insert("#5F9EA0", Color::CadetBlue);
    m.insert("#E38AAE", Color::CadillacPink);
    m.insert("#C19A6B", Color::CamelBrown);
    m.insert("#78866B", Color::CamouflageGreen);
    m.insert("#2916F5", Color::CanaryBlue);
    m.insert("#FFEF00", Color::CanaryYellow);
    m.insert("#FFA62F", Color::Cantaloupe);
    m.insert("#C68E17", Color::Caramel);
    m.insert("#625D5D", Color::CarbonGray);
    m.insert("#A70D2A", Color::CarbonRed);
    m.insert("#EDDA74", Color::CardboardBrown);
    m.insert("#F778A1", Color::CarnationPink);
    m.insert("#F88017", Color::CarrotOrange);
    m.insert("#50EBEC", Color::Celeste);
    m.insert("#BDF516", Color::ChameleonGreen);
    m.insert("#F7E7CE", Color::Champagne);
    m.insert("#34282C", Color::Charcoal);
    m.insert("#36454F", Color::CharcoalBlue);
    m.insert("#7FFF00", Color::Chartreuse);
    m.insert("#FFA600", Color::CheeseOrange);
    m.insert("#C24641", Color::CherryRed);
    m.insert("#954535", Color::Chestnut);
    m.insert("#C34A2C", Color::ChestnutRed);
    m.insert("#C11B17", Color::ChilliPepper);
    m.insert("#D2691E", Color::Chocolate);
    m.insert("#3F000F", Color::ChocolateBrown);
    m.insert("#A8A9AD", Color::ChromeAluminum);
    m.insert("#FFCE44", Color::ChromeGold);
    m.insert("#1AA260", Color::ChromeGreen);
    m.insert("#FF33AA", Color::ChromePink);
    m.insert("#E8F1D4", Color::ChromeWhite);
    m.insert("#C58917", Color::Cinnamon);
    m.insert("#8FB31D", Color::CitronGreen);
    m.insert("#842DCE", Color::ClematisViolet);
    m.insert("#6D6968", Color::CloudyGray);
    m.insert("#3EA055", Color::CloverGreen);
    m.insert("#0020C2", Color::CobaltBlue);
    m.insert("#6F4E37", Color::Coffee);
    m.insert("#9B9A96", Color::ColdMetal);
    m.insert("#87AFC7", Color::ColumbiaBlue);
    m.insert("#F87431", Color::ConstructionConeOrange);
    m.insert("#C7A317", Color::CookieBrown);
    m.insert("#B87333", Color::Copper);
    m.insert("#CB6D51", Color::CopperRed);
    m.insert("#FF7F50", Color::Coral);
    m.insert("#AFDCEC", Color::CoralBlue);
    m.insert("#9E4638", Color::CoralBrown);
    m.insert("#FBD5AB", Color::CoralPeach);
    m.insert("#FFF380", Color::CornYellow);
    m.insert("#6495ED", Color::CornflowerBlue);
    m.insert("#FFF8DC", Color::Cornsilk);
    m.insert("#FFF8E7", Color::CosmicLatte);
    m.insert("#FBFBF9", Color::Cotton);
    m.insert("#FCDFFF", Color::CottonCandy);
    m.insert("#9F000F", Color::Cranberry);
    m.insert("#FFFFCC", Color::Cream);
    m.insert("#FFFDD0", Color::CreamWhite);
    m.insert("#F0E9D6", Color::CreamyWhite);
    m.insert("#DC143C", Color::Crimson);
    m.insert("#E238EC", Color::CrimsonPurple);
    m.insert("#990000", Color::CrimsonRed);
    m.insert("#9172EC", Color::CrocusPurple);
    m.insert("#5CB3FF", Color::CrystalBlue);
    m.insert("#00FFFF", Color::Cyan);
    m.insert("#14A3C7", Color::CyanBlue);
    m.insert("#92C7C7", Color::CyanOpaque);
    m.insert("#AB784E", Color::DarkAlmond);
    m.insert("#9F8C76", Color::DarkBeige);
    m.insert("#B86500", Color::DarkBisque);
    m.insert("#F0E2B6", Color::DarkBlonde);
    m.insert("#29465B", Color::DarkBlueGray);
    m.insert("#5539CC", Color::DarkBlurple);
    m.insert("#804A00", Color::DarkBronze);
    m.insert("#654321", Color::DarkBrown);
    m.insert("#C12283", Color::DarkCarnationPink);
    m.insert("#3B2F2F", Color::DarkCoffee);
    m.insert("#254117", Color::DarkForestGreen);
    m.insert("#8C8C8C", Color::DarkGainsboro);
    m.insert("#AA6C39", Color::DarkGold);
    m.insert("#4A412A", Color::DarkGrayishOlive);
    m.insert("#1F6357", Color::DarkGreenBlue);
    m.insert("#473810", Color::DarkHazelBrown);
    m.insert("#F660AB", Color::DarkHotPink);
    m.insert("#41A317", Color::DarkLimeGreen);
    m.insert("#2F0909", Color::DarkMaroon);
    m.insert("#31906E", Color::DarkMint);
    m.insert("#827839", Color::DarkMoccasin);
    m.insert("#E75480", Color::DarkPink);
    m.insert("#4B0150", Color::DarkPurple);
    m.insert("#872657", Color::DarkRaspberry);
    m.insert("#560319", Color::DarkScarlet);
    m.insert("#8A4117", Color::DarkSienna);
    m.insert("#0059FF", Color::DarkSkyBlue);
    m.insert("#2B3856", Color::DarkSlate);
    m.insert("#4D4D4F", Color::DarkSteampunk);
    m.insert("#045D5D", Color::DarkTeal);
    m.insert("#E1D9D1", Color::DarkWhite);
    m.insert("#8B8000", Color::DarkYellow);
    m.insert("#00008B", Color::DarkBlue);
    m.insert("#008B8B", Color::DarkCyan);
    m.insert("#B8860B", Color::DarkGoldenRod);
    m.insert("#A9A9A9", Color::DarkGray);
    m.insert("#006400", Color::DarkGreen);
    m.insert("#A9A9A9", Color::DarkGrey);
    m.insert("#BDB76B", Color::DarkKhaki);
    m.insert("#8B008B", Color::DarkMagenta);
    m.insert("#556B2F", Color::DarkOliveGreen);
    m.insert("#FF8C00", Color::DarkOrange);
    m.insert("#9932CC", Color::DarkOrchid);
    m.insert("#8B0000", Color::DarkRed);
    m.insert("#E9967A", Color::DarkSalmon);
    m.insert("#8FBC8F", Color::DarkSeaGreen);
    m.insert("#483D8B", Color::DarkSlateBlue);
    m.insert("#25383C", Color::DarkSlateGray);
    m.insert("#25383C", Color::DarkSlateGrey);
    m.insert("#00CED1", Color::DarkTurquoise);
    m.insert("#9400D3", Color::DarkViolet);
    m.insert("#82CAFF", Color::DaySkyBlue);
    m.insert("#A05544", Color::DeepAmber);
    m.insert("#046307", Color::DeepEmeraldGreen);
    m.insert("#056608", Color::DeepGreen);
    m.insert("#DF73D4", Color::DeepMauve);
    m.insert("#FFCBA4", Color::DeepPeach);
    m.insert("#5453A6", Color::DeepPeriwinkle);
    m.insert("#36013F", Color::DeepPurple);
    m.insert("#800517", Color::DeepRed);
    m.insert("#FBBBB9", Color::DeepRose);
    m.insert("#3B9C9C", Color::DeepSea);
    m.insert("#123456", Color::DeepSeaBlue);
    m.insert("#306754", Color::DeepSeaGreen);
    m.insert("#033E3E", Color::DeepTeal);
    m.insert("#48CCCD", Color::DeepTurquoise);
    m.insert("#F6BE00", Color::DeepYellow);
    m.insert("#FF1493", Color::DeepPink);
    m.insert("#00BFFF", Color::DeepSkyBlue);
    m.insert("#E6BF83", Color::DeerBrown);
    m.insert("#79BAEC", Color::DenimBlue);
    m.insert("#151B8D", Color::DenimDarkBlue);
    m.insert("#EDC9AF", Color::DesertSand);
    m.insert("#696969", Color::DimGray);
    m.insert("#696969", Color::DimGrey);
    m.insert("#E3319D", Color::DimorphothecaMagenta);
    m.insert("#73A16C", Color::DinosaurGreen);
    m.insert("#E8E4C9", Color::DirtyWhite);
    m.insert("#1E90FF", Color::DodgerBlue);
    m.insert("#85BB65", Color::DollarBillGreen);
    m.insert("#FAAFBE", Color::DonutPink);
    m.insert("#6AFB92", Color::DragonGreen);
    m.insert("#B1FB17", Color::DullGreenYellow);
    m.insert("#7F525D", Color::DullPurple);
    m.insert("#4E8975", Color::DullSeaGreen);
    m.insert("#CC7A8B", Color::DuskyPink);
    m.insert("#D58A94", Color::DustyPink);
    m.insert("#C9A9A6", Color::DustyRose);
    m.insert("#0000A5", Color::EarthBlue);
    m.insert("#34A56F", Color::EarthGreen);
    m.insert("#555D50", Color::Ebony);
    m.insert("#FFF9E3", Color::EggShell);
    m.insert("#614051", Color::Eggplant);
    m.insert("#9AFEFF", Color::ElectricBlue);
    m.insert("#1B8A6B", Color::ElfGreen);
    m.insert("#50C878", Color::Emerald);
    m.insert("#5FFB17", Color::EmeraldGreen);
    m.insert("#2F539B", Color::EstorilBlue);
    m.insert("#4E9258", Color::FallForestGreen);
    m.insert("#C8B560", Color::FallLeafBrown);
    m.insert("#667C26", Color::FernGreen);
    m.insert("#F70D1A", Color::FerrariRed);
    m.insert("#F62817", Color::FireEngineRed);
    m.insert("#B22222", Color::FireBrick);
    m.insert("#F9A7B0", Color::FlamingoPink);
    m.insert("#FFFAF0", Color::FloralWhite);
    m.insert("#FE632A", Color::FluroOrange);
    m.insert("#228B22", Color::ForestGreen);
    m.insert("#86608E", Color::FrenchLilac);
    m.insert("#99C68E", Color::FrogGreen);
    m.insert("#FF00FF", Color::Fuchsia);
    m.insert("#FF77FF", Color::FuchsiaPink);
    m.insert("#DCDCDC", Color::Gainsboro);
    m.insert("#733635", Color::GarnetRed);
    m.insert("#C0C6C7", Color::GearSteelGray);
    m.insert("#F8F8FF", Color::GhostWhite);
    m.insert("#C9BE62", Color::GingerBrown);
    m.insert("#B83C08", Color::GingerRed);
    m.insert("#368BC1", Color::GlacialBlueIce);
    m.insert("#FFD700", Color::Gold);
    m.insert("#E6C7C2", Color::GoldPink);
    m.insert("#FBE7A1", Color::GoldenBlonde);
    m.insert("#EAC117", Color::GoldenBrown);
    m.insert("#F3E3C3", Color::GoldenSilk);
    m.insert("#FFDF00", Color::GoldenYellow);
    m.insert("#DAA520", Color::GoldenRod);
    m.insert("#837E7C", Color::Granite);
    m.insert("#5E5A80", Color::Grape);
    m.insert("#DC381F", Color::Grapefruit);
    m.insert("#3F9B0B", Color::GrassGreen);
    m.insert("#808080", Color::Gray);
    m.insert("#3D3635", Color::GrayBrown);
    m.insert("#B6B6B4", Color::GrayCloud);
    m.insert("#5C5858", Color::GrayDolphin);
    m.insert("#D1D0CE", Color::GrayGoose);
    m.insert("#A2AD9C", Color::GrayGreen);
    m.insert("#504A4B", Color::GrayWolf);
    m.insert("#5E7D7E", Color::GrayishTurquoise);
    m.insert("#008000", Color::Green);
    m.insert("#4CC417", Color::GreenApple);
    m.insert("#3A5F0B", Color::GreenLeaves);
    m.insert("#6AA121", Color::GreenOnion);
    m.insert("#89C35C", Color::GreenPeas);
    m.insert("#4AA02C", Color::GreenPepper);
    m.insert("#6CBB3C", Color::GreenSnake);
    m.insert("#B5EAAA", Color::GreenThumb);
    m.insert("#ADFF2F", Color::GreenYellow);
    m.insert("#307D7E", Color::GreenishBlue);
    m.insert("#808080", Color::Grey);
    m.insert("#C9DFEC", Color::GulfBlue);
    m.insert("#2C3539", Color::Gunmetal);
    m.insert("#8D918D", Color::GunmetalGray);
    m.insert("#FFFEFA", Color::HalfWhite);
    m.insert("#E66C2C", Color::HalloweenOrange);
    m.insert("#EDE275", Color::HarvestGold);
    m.insert("#8E7618", Color::Hazel);
    m.insert("#617C58", Color::HazelGreen);
    m.insert("#C6DEFF", Color::HeavenlyBlue);
    m.insert("#D462FF", Color::HeliotropePurple);
    m.insert("#F0FFF0", Color::HoneyDew);
    m.insert("#F52887", Color::HotDeepPink);
    m.insert("#FF69B4", Color::HotPink);
    m.insert("#7FE817", Color::HummingbirdGreen);
    m.insert("#355E3B", Color::HunterGreen);
    m.insert("#56A5EC", Color::Iceberg);
    m.insert("#9CB071", Color::IguanaGreen);
    m.insert("#FF7722", Color::IndianSaffron);
    m.insert("#CD5C5C", Color::IndianRed);
    m.insert("#4B0082", Color::Indigo);
    m.insert("#3D3C3A", Color::Iridium);
    m.insert("#08A04B", Color::IrishGreen);
    m.insert("#52595D", Color::IronGray);
    m.insert("#22CE83", Color::IsleOfManGreen);
    m.insert("#FFFFF0", Color::Ivory);
    m.insert("#00A36C", Color::Jade);
    m.insert("#5EFB6E", Color::JadeGreen);
    m.insert("#A23BEC", Color::JasminePurple);
    m.insert("#A0CFEC", Color::JeansBlue);
    m.insert("#46C7C7", Color::Jellyfish);
    m.insert("#616D7E", Color::JetGray);
    m.insert("#347C2C", Color::JungleGreen);
    m.insert("#4CC552", Color::KellyGreen);
    m.insert("#F0E68C", Color::Khaki);
    m.insert("#906E3E", Color::KhakiBrown);
    m.insert("#8A865D", Color::KhakiGreen);
    m.insert("#C5908E", Color::KhakiRose);
    m.insert("#15317E", Color::LapisBlue);
    m.insert("#E42217", Color::LavaRed);
    m.insert("#E6E6FA", Color::Lavender);
    m.insert("#E3E4FA", Color::LavenderBlue);
    m.insert("#EBDDE2", Color::LavenderPinocchio);
    m.insert("#967BB6", Color::LavenderPurple);
    m.insert("#FFF0F5", Color::LavenderBlush);
    m.insert("#7CFC00", Color::LawnGreen);
    m.insert("#ADF802", Color::LemonGreen);
    m.insert("#FEF250", Color::LemonYellow);
    m.insert("#FFFACD", Color::LemonChiffon);
    m.insert("#93FFE8", Color::LightAquamarine);
    m.insert("#FFF0DB", Color::LightBeige);
    m.insert("#454545", Color::LightBlack);
    m.insert("#B5651D", Color::LightBrown);
    m.insert("#DA8A67", Color::LightCopper);
    m.insert("#ADDFFF", Color::LightDayBlue);
    m.insert("#C8AD7F", Color::LightFrenchBeige);
    m.insert("#F1E5AC", Color::LightGold);
    m.insert("#C3FDB8", Color::LightJade);
    m.insert("#C2E5D3", Color::LightMintGreen);
    m.insert("#B8BC86", Color::LightOliveGreen);
    m.insert("#FED8B1", Color::LightOrange);
    m.insert("#8467D7", Color::LightPurple);
    m.insert("#728FCE", Color::LightPurpleBlue);
    m.insert("#FFCCCB", Color::LightRed);
    m.insert("#FBCFCD", Color::LightRose);
    m.insert("#DBF9DB", Color::LightRoseGreen);
    m.insert("#F9966B", Color::LightSalmonRose);
    m.insert("#CCFFFF", Color::LightSlate);
    m.insert("#736AFF", Color::LightSlateBlue);
    m.insert("#E0E5E5", Color::LightSteelGray);
    m.insert("#B3D9D9", Color::LightTeal);
    m.insert("#FFFFF7", Color::LightWhite);
    m.insert("#ADD8E6", Color::LightBlue);
    m.insert("#F08080", Color::LightCoral);
    m.insert("#E0FFFF", Color::LightCyan);
    m.insert("#FAFAD2", Color::LightGoldenRodYellow);
    m.insert("#D3D3D3", Color::LightGray);
    m.insert("#90EE90", Color::LightGreen);
    m.insert("#D3D3D3", Color::LightGrey);
    m.insert("#FFB6C1", Color::LightPink);
    m.insert("#FFA07A", Color::LightSalmon);
    m.insert("#20B2AA", Color::LightSeaGreen);
    m.insert("#87CEFA", Color::LightSkyBlue);
    m.insert("#778899", Color::LightSlateGray);
    m.insert("#778899", Color::LightSlateGrey);
    m.insert("#B0CFDE", Color::LightSteelBlue);
    m.insert("#FFFFE0", Color::LightYellow);
    m.insert("#C8A2C8", Color::Lilac);
    m.insert("#00FF00", Color::Lime);
    m.insert("#36F57F", Color::LimeMintGreen);
    m.insert("#32CD32", Color::LimeGreen);
    m.insert("#FAF0E6", Color::Linen);
    m.insert("#C48793", Color::LipstickPink);
    m.insert("#004225", Color::LotusGreen);
    m.insert("#E41B17", Color::LoveRed);
    m.insert("#7F38EC", Color::LovelyPurple);
    m.insert("#F2BB66", Color::MacaroniandCheese);
    m.insert("#43BFC7", Color::MacawBlueGreen);
    m.insert("#FF00FF", Color::Magenta);
    m.insert("#CC338B", Color::MagentaPink);
    m.insert("#AAF0D1", Color::MagicMint);
    m.insert("#C04000", Color::Mahogany);
    m.insert("#FF8040", Color::MangoOrange);
    m.insert("#566D7E", Color::MarbleBlue);
    m.insert("#800000", Color::Maroon);
    m.insert("#8F0B0B", Color::MaroonRed);
    m.insert("#E0B0FF", Color::Mauve);
    m.insert("#915F6D", Color::MauveTaupe);
    m.insert("#347235", Color::MediumForestGreen);
    m.insert("#045F5F", Color::MediumTeal);
    m.insert("#66CDAA", Color::MediumAquaMarine);
    m.insert("#0000CD", Color::MediumBlue);
    m.insert("#BA55D3", Color::MediumOrchid);
    m.insert("#9370DB", Color::MediumPurple);
    m.insert("#3CB371", Color::MediumSeaGreen);
    m.insert("#7B68EE", Color::MediumSlateBlue);
    m.insert("#00FA9A", Color::MediumSpringGreen);
    m.insert("#48D1CC", Color::MediumTurquoise);
    m.insert("#C71585", Color::MediumVioletRed);
    m.insert("#B6B6B6", Color::Metal);
    m.insert("#A97142", Color::MetallicBronze);
    m.insert("#D4AF37", Color::MetallicGold);
    m.insert("#7C9D8E", Color::MetallicGreen);
    m.insert("#BCC6CC", Color::MetallicSilver);
    m.insert("#3BB9FF", Color::MiddayBlue);
    m.insert("#2B1B17", Color::Midnight);
    m.insert("#2E1A47", Color::MidnightPurple);
    m.insert("#191970", Color::MidnightBlue);
    m.insert("#4E5B31", Color::MilitaryGreen);
    m.insert("#513B1C", Color::MilkChocolate);
    m.insert("#FEFCFF", Color::MilkWhite);
    m.insert("#93917C", Color::MillenniumJade);
    m.insert("#3EB489", Color::Mint);
    m.insert("#98FF98", Color::MintGreen);
    m.insert("#F5FFFA", Color::MintCream);
    m.insert("#646D7E", Color::MistBlue);
    m.insert("#FFE4E1", Color::MistyRose);
    m.insert("#FFE4B5", Color::Moccasin);
    m.insert("#493D26", Color::Mocha);
    m.insert("#8A9A5B", Color::MossGreen);
    m.insert("#E1AD01", Color::Mustard);
    m.insert("#FFDB58", Color::MustardYellow);
    m.insert("#686A6C", Color::NardoGray);
    m.insert("#FFDEAD", Color::NavajoWhite);
    m.insert("#000080", Color::Navy);
    m.insert("#59E817", Color::NebulaGreen);
    m.insert("#1589FF", Color::NeonBlue);
    m.insert("#FDBD01", Color::NeonGold);
    m.insert("#16F529", Color::NeonGreen);
    m.insert("#FD349C", Color::NeonHotPink);
    m.insert("#FF6700", Color::NeonOrange);
    m.insert("#F535AA", Color::NeonPink);
    m.insert("#9D00FF", Color::NeonPurple);
    m.insert("#FD1C03", Color::NeonRed);
    m.insert("#FFFF33", Color::NeonYellow);
    m.insert("#DAEE01", Color::NeonYellowGreen);
    m.insert("#0000A0", Color::NewMidnightBlue);
    m.insert("#0C090A", Color::Night);
    m.insert("#151B54", Color::NightBlue);
    m.insert("#78C7C7", Color::NorthernLightsBlue);
    m.insert("#806517", Color::OakBrown);
    m.insert("#2B65EC", Color::OceanBlue);
    m.insert("#00FF80", Color::OceanGreen);
    m.insert("#F8F0E3", Color::OffWhite);
    m.insert("#3B3131", Color::Oil);
    m.insert("#43302E", Color::OldBurgundy);
    m.insert("#C08081", Color::OldRose);
    m.insert("#FEF0E3", Color::OldLace);
    m.insert("#808000", Color::Olive);
    m.insert("#BAB86C", Color::OliveGreen);
    m.insert("#6B8E23", Color::OliveDrab);
    m.insert("#FFA500", Color::Orange);
    m.insert("#D4A017", Color::OrangeGold);
    m.insert("#C47451", Color::OrangeSalmon);
    m.insert("#FFAE42", Color::OrangeYellow);
    m.insert("#FF4500", Color::OrangeRed);
    m.insert("#DA70D6", Color::Orchid);
    m.insert("#B048B5", Color::OrchidPurple);
    m.insert("#E3F9A6", Color::OrganicBrown);
    m.insert("#CFECEC", Color::PaleBlueLily);
    m.insert("#DCD0FF", Color::PaleLilac);
    m.insert("#F2D4D7", Color::PalePink);
    m.insert("#C9C0BB", Color::PaleSilver);
    m.insert("#EEE8AA", Color::PaleGoldenRod);
    m.insert("#98FB98", Color::PaleGreen);
    m.insert("#AFEEEE", Color::PaleTurquoise);
    m.insert("#DB7093", Color::PaleVioletRed);
    m.insert("#E56717", Color::PapayaOrange);
    m.insert("#FFEFD5", Color::PapayaWhip);
    m.insert("#FFFFC2", Color::Parchment);
    m.insert("#12AD2B", Color::ParrotGreen);
    m.insert("#B4CFEC", Color::PastelBlue);
    m.insert("#B1907F", Color::PastelBrown);
    m.insert("#77DD77", Color::PastelGreen);
    m.insert("#8686AF", Color::PastelIndigo);
    m.insert("#D5D6EA", Color::PastelLightBlue);
    m.insert("#F8B88B", Color::PastelOrange);
    m.insert("#FEA3AA", Color::PastelPink);
    m.insert("#F2A2E8", Color::PastelPurple);
    m.insert("#F67280", Color::PastelRed);
    m.insert("#E5788F", Color::PastelRose);
    m.insert("#D291BC", Color::PastelViolet);
    m.insert("#FAF884", Color::PastelYellow);
    m.insert("#52D017", Color::PeaGreen);
    m.insert("#FFE5B4", Color::Peach);
    m.insert("#F98B88", Color::PeachPink);
    m.insert("#FFDAB9", Color::PeachPuff);
    m.insert("#FDEEF4", Color::Pearl);
    m.insert("#F8F6F0", Color::PearlWhite);
    m.insert("#CCCCFF", Color::Periwinkle);
    m.insert("#E9CFEC", Color::PeriwinklePink);
    m.insert("#7575CF", Color::PeriwinklePurple);
    m.insert("#CD853F", Color::Peru);
    m.insert("#B76734", Color::PetraGold);
    m.insert("#FDD7E4", Color::PigPink);
    m.insert("#387C44", Color::PineGreen);
    m.insert("#FFC0CB", Color::Pink);
    m.insert("#C48189", Color::PinkBrown);
    m.insert("#FFDFDD", Color::PinkBubbleGum);
    m.insert("#E77471", Color::PinkCoral);
    m.insert("#E45E9D", Color::PinkCupcake);
    m.insert("#E799A3", Color::PinkDaisy);
    m.insert("#E4287C", Color::PinkLemonade);
    m.insert("#F89880", Color::PinkOrange);
    m.insert("#B93B8F", Color::PinkPlum);
    m.insert("#CA226B", Color::PinkViolet);
    m.insert("#9DC209", Color::PistachioGreen);
    m.insert("#E5E4E2", Color::Platinum);
    m.insert("#797979", Color::PlatinumGray);
    m.insert("#CECECE", Color::PlatinumSilver);
    m.insert("#DDA0DD", Color::Plum);
    m.insert("#7D0541", Color::PlumPie);
    m.insert("#583759", Color::PlumPurple);
    m.insert("#7D0552", Color::PlumVelvet);
    m.insert("#FFB2D0", Color::PowderPink);
    m.insert("#B0E0E6", Color::PowderBlue);
    m.insert("#7F5A58", Color::Puce);
    m.insert("#644117", Color::PullmanBrown);
    m.insert("#F87217", Color::PumpkinOrange);
    m.insert("#CA762B", Color::PumpkinPie);
    m.insert("#800080", Color::Purple);
    m.insert("#6C2DC7", Color::PurpleAmethyst);
    m.insert("#B041FF", Color::PurpleDaffodil);
    m.insert("#C38EC7", Color::PurpleDragon);
    m.insert("#A74AC7", Color::PurpleFlower);
    m.insert("#4E387E", Color::PurpleHaze);
    m.insert("#571B7E", Color::PurpleIris);
    m.insert("#6A287E", Color::PurpleJam);
    m.insert("#550A35", Color::PurpleLily);
    m.insert("#810541", Color::PurpleMaroon);
    m.insert("#9E7BFF", Color::PurpleMimosa);
    m.insert("#461B7E", Color::PurpleMonster);
    m.insert("#4E5180", Color::PurpleNavy);
    m.insert("#D16587", Color::PurplePink);
    m.insert("#8E35EF", Color::PurplePlum);
    m.insert("#7A5DC7", Color::PurpleSageBush);
    m.insert("#D2B9D3", Color::PurpleThistle);
    m.insert("#8D38C9", Color::PurpleViolet);
    m.insert("#DFD3E3", Color::PurpleWhite);
    m.insert("#27742C", Color::RacingGreen);
    m.insert("#E30B5D", Color::Raspberry);
    m.insert("#B3446C", Color::RaspberryPurple);
    m.insert("#6D7B8D", Color::RatGray);
    m.insert("#663399", Color::RebeccaPurple);
    m.insert("#FF0000", Color::Red);
    m.insert("#660000", Color::RedBlood);
    m.insert("#622F22", Color::RedBrown);
    m.insert("#7F5217", Color::RedDirt);
    m.insert("#C35817", Color::RedFox);
    m.insert("#EB5406", Color::RedGold);
    m.insert("#FF0080", Color::RedMagenta);
    m.insert("#FA2A55", Color::RedPink);
    m.insert("#F3E8EA", Color::RedWhite);
    m.insert("#990012", Color::RedWine);
    m.insert("#FAF5EF", Color::Rice);
    m.insert("#B666D2", Color::RichLilac);
    m.insert("#BDEDFF", Color::RobinEggBlue);
    m.insert("#C12869", Color::RoguePink);
    m.insert("#838996", Color::RomanSilver);
    m.insert("#E8ADAA", Color::Rose);
    m.insert("#997070", Color::RoseDust);
    m.insert("#ECC5C0", Color::RoseGold);
    m.insert("#E7A1B0", Color::RosePink);
    m.insert("#B09FCA", Color::RosePurple);
    m.insert("#F7CAC9", Color::RoseQuartz);
    m.insert("#C21E56", Color::RoseRed);
    m.insert("#A17188", Color::Rosy);
    m.insert("#7F4E52", Color::RosyFinch);
    m.insert("#B38481", Color::RosyPink);
    m.insert("#BC8F8F", Color::RosyBrown);
    m.insert("#E759AC", Color::RoyalPink);
    m.insert("#4169E1", Color::RoyalBlue);
    m.insert("#FFD801", Color::RubberDuckyYellow);
    m.insert("#F62217", Color::RubyRed);
    m.insert("#C36241", Color::Rust);
    m.insert("#8B4513", Color::SaddleBrown);
    m.insert("#FF7900", Color::SafetyOrange);
    m.insert("#EED202", Color::SafetyYellow);
    m.insert("#FBB917", Color::Saffron);
    m.insert("#931314", Color::SaffronRed);
    m.insert("#BCB88A", Color::Sage);
    m.insert("#848B79", Color::SageGreen);
    m.insert("#A1C935", Color::SaladGreen);
    m.insert("#FA8072", Color::Salmon);
    m.insert("#FF8674", Color::SalmonPink);
    m.insert("#0002FF", Color::SamcoBlue);
    m.insert("#C2B280", Color::Sand);
    m.insert("#786D5F", Color::Sandstone);
    m.insert("#F4A460", Color::SandyBrown);
    m.insert("#7E3817", Color::Sangria);
    m.insert("#2554C7", Color::SapphireBlue);
    m.insert("#FF2400", Color::ScarletRed);
    m.insert("#E8A317", Color::SchoolBusYellow);
    m.insert("#C2DFFF", Color::SeaBlue);
    m.insert("#438D80", Color::SeaTurtleGreen);
    m.insert("#2E8B57", Color::SeaGreen);
    m.insert("#FFF5EE", Color::SeaShell);
    m.insert("#3EA99F", Color::SeafoamGreen);
    m.insert("#437C17", Color::SeaweedGreen);
    m.insert("#CC6600", Color::Sedona);
    m.insert("#7F462C", Color::Sepia);
    m.insert("#704214", Color::SepiaBrown);
    m.insert("#347C17", Color::ShamrockGreen);
    m.insert("#888B90", Color::SheetMetal);
    m.insert("#E55B3C", Color::ShockingOrange);
    m.insert("#A0522D", Color::Sienna);
    m.insert("#488AC7", Color::SilkBlue);
    m.insert("#C0C0C0", Color::Silver);
    m.insert("#C4AEAD", Color::SilverPink);
    m.insert("#DADBDD", Color::SilverWhite);
    m.insert("#6698FF", Color::SkyBlueDress);
    m.insert("#87CEEB", Color::SkyBlue);
    m.insert("#737CA1", Color::SlateBlueGray);
    m.insert("#657383", Color::SlateGraniteGray);
    m.insert("#6A5ACD", Color::SlateBlue);
    m.insert("#708090", Color::SlateGray);
    m.insert("#708090", Color::SlateGrey);
    m.insert("#BCE954", Color::SlimeGreen);
    m.insert("#726E6D", Color::SmokeyGray);
    m.insert("#FFFAFA", Color::Snow);
    m.insert("#C6BA8B", Color::SoftHazel);
    m.insert("#FAF0DD", Color::SoftIvory);
    m.insert("#FFB8BF", Color::SoftPink);
    m.insert("#757575", Color::SonicSilver);
    m.insert("#00FF7F", Color::SpringGreen);
    m.insert("#99A3A3", Color::StainlessSteelGray);
    m.insert("#C9C1C1", Color::Steampunk);
    m.insert("#71797E", Color::SteelGray);
    m.insert("#4682B4", Color::SteelBlue);
    m.insert("#57E964", Color::StoplightGoGreen);
    m.insert("#3A3B3C", Color::StormyGray);
    m.insert("#C83F49", Color::StrawberryRed);
    m.insert("#FFE87C", Color::SunYellow);
    m.insert("#E67451", Color::SunriseOrange);
    m.insert("#D2B48C", Color::Tan);
    m.insert("#ECE5B6", Color::TanBrown);
    m.insert("#E78A61", Color::Tangerine);
    m.insert("#483C32", Color::Taupe);
    m.insert("#CCFB5D", Color::TeaGreen);
    m.insert("#008080", Color::Teal);
    m.insert("#007C80", Color::TealBlue);
    m.insert("#00827F", Color::TealGreen);
    m.insert("#D8BFD8", Color::Thistle);
    m.insert("#81D8D0", Color::TiffanyBlue);
    m.insert("#C88141", Color::TigerOrange);
    m.insert("#FF6347", Color::Tomato);
    m.insert("#B21807", Color::TomatoSauceRed);
    m.insert("#7DFDFE", Color::TronBlue);
    m.insert("#C25A7C", Color::TulipPink);
    m.insert("#40E0D0", Color::Turquoise);
    m.insert("#A0D6B4", Color::TurquoiseGreen);
    m.insert("#C45AEC", Color::TyrianPurple);
    m.insert("#FFDDCA", Color::UnbleachedSilk);
    m.insert("#E55451", Color::ValentineRed);
    m.insert("#565051", Color::VampireGray);
    m.insert("#F3E5AB", Color::Vanilla);
    m.insert("#7E354D", Color::VelvetMaroon);
    m.insert("#728C00", Color::VenomGreen);
    m.insert("#7E191B", Color::Vermilion);
    m.insert("#6667AB", Color::VeryPeri);
    m.insert("#C8C4DF", Color::Viola);
    m.insert("#7E587E", Color::ViolaPurple);
    m.insert("#EE82EE", Color::Violet);
    m.insert("#F6358A", Color::VioletRed);
    m.insert("#F6C6BD", Color::WarmPink);
    m.insert("#EFEBD8", Color::WarmWhite);
    m.insert("#EBF4FA", Color::Water);
    m.insert("#FC6C85", Color::WatermelonPink);
    m.insert("#49413F", Color::WesternCharcoal);
    m.insert("#F5DEB3", Color::Wheat);
    m.insert("#FFFFFF", Color::White);
    m.insert("#DBE9FA", Color::WhiteBlue);
    m.insert("#EDE6D6", Color::WhiteChocolate);
    m.insert("#FFFFF4", Color::WhiteGold);
    m.insert("#EEEEEE", Color::WhiteGray);
    m.insert("#EAEEE9", Color::WhiteIce);
    m.insert("#F2F0DF", Color::WhiteYellow);
    m.insert("#F5F5F5", Color::WhiteSmoke);
    m.insert("#357EC7", Color::WindowsBlue);
    m.insert("#990012", Color::WineRed);
    m.insert("#C6AEC7", Color::WisteriaPurple);
    m.insert("#966F33", Color::Wood);
    m.insert("#FFFF00", Color::Yellow);
    m.insert("#E2F516", Color::YellowGreenGrosbeak);
    m.insert("#87F717", Color::YellowLawnGreen);
    m.insert("#FFAE42", Color::YellowOrange);
    m.insert("#9ACD32", Color::YellowGreen);
    m.insert("#54C571", Color::ZombieGreen);
    m
});
impl Color {
    pub fn convert_str(name: &str) -> Option<Self> {
        if name.starts_with('#') {
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
                x if x == stringify!(AcidGreen).to_lowercase() => Some(Color::AcidGreen),
                x if x == stringify!(AlgaeGreen).to_lowercase() => Some(Color::AlgaeGreen),
                x if x == stringify!(AliceBlue).to_lowercase() => Some(Color::AliceBlue),
                x if x == stringify!(AlienGray).to_lowercase() => Some(Color::AlienGray),
                x if x == stringify!(AlienGreen).to_lowercase() => Some(Color::AlienGreen),
                x if x == stringify!(AloeVeraGreen).to_lowercase() => Some(Color::AloeVeraGreen),
                x if x == stringify!(Amber).to_lowercase() => Some(Color::Amber),
                x if x == stringify!(AntiqueBronze).to_lowercase() => Some(Color::AntiqueBronze),
                x if x == stringify!(AntiqueWhite).to_lowercase() => Some(Color::AntiqueWhite),
                x if x == stringify!(Aqua).to_lowercase() => Some(Color::Aqua),
                x if x == stringify!(AquaGreen).to_lowercase() => Some(Color::AquaGreen),
                x if x == stringify!(AquaSeafoamGreen).to_lowercase() => {
                    Some(Color::AquaSeafoamGreen)
                }
                x if x == stringify!(Aquamarine).to_lowercase() => Some(Color::Aquamarine),
                x if x == stringify!(AquamarineStone).to_lowercase() => {
                    Some(Color::AquamarineStone)
                }
                x if x == stringify!(ArmyBrown).to_lowercase() => Some(Color::ArmyBrown),
                x if x == stringify!(ArmyGreen).to_lowercase() => Some(Color::ArmyGreen),
                x if x == stringify!(AshGray).to_lowercase() => Some(Color::AshGray),
                x if x == stringify!(AshWhite).to_lowercase() => Some(Color::AshWhite),
                x if x == stringify!(AvocadoGreen).to_lowercase() => Some(Color::AvocadoGreen),
                x if x == stringify!(AztechPurple).to_lowercase() => Some(Color::AztechPurple),
                x if x == stringify!(Azure).to_lowercase() => Some(Color::Azure),
                x if x == stringify!(AzureBlue).to_lowercase() => Some(Color::AzureBlue),
                x if x == stringify!(BabyBlue).to_lowercase() => Some(Color::BabyBlue),
                x if x == stringify!(BabyPink).to_lowercase() => Some(Color::BabyPink),
                x if x == stringify!(BakersBrown).to_lowercase() => Some(Color::BakersBrown),
                x if x == stringify!(BalloonBlue).to_lowercase() => Some(Color::BalloonBlue),
                x if x == stringify!(BananaYellow).to_lowercase() => Some(Color::BananaYellow),
                x if x == stringify!(BarbiePink).to_lowercase() => Some(Color::BarbiePink),
                x if x == stringify!(BashfulPink).to_lowercase() => Some(Color::BashfulPink),
                x if x == stringify!(BasilGreen).to_lowercase() => Some(Color::BasilGreen),
                x if x == stringify!(BasketBallOrange).to_lowercase() => {
                    Some(Color::BasketBallOrange)
                }
                x if x == stringify!(BattleshipGray).to_lowercase() => Some(Color::BattleshipGray),
                x if x == stringify!(BeanRed).to_lowercase() => Some(Color::BeanRed),
                x if x == stringify!(BeeYellow).to_lowercase() => Some(Color::BeeYellow),
                x if x == stringify!(Beer).to_lowercase() => Some(Color::Beer),
                x if x == stringify!(BeetleGreen).to_lowercase() => Some(Color::BeetleGreen),
                x if x == stringify!(Beige).to_lowercase() => Some(Color::Beige),
                x if x == stringify!(Bisque).to_lowercase() => Some(Color::Bisque),
                x if x == stringify!(Black).to_lowercase() => Some(Color::Black),
                x if x == stringify!(BlackBean).to_lowercase() => Some(Color::BlackBean),
                x if x == stringify!(BlackBlue).to_lowercase() => Some(Color::BlackBlue),
                x if x == stringify!(BlackCat).to_lowercase() => Some(Color::BlackCat),
                x if x == stringify!(BlackCow).to_lowercase() => Some(Color::BlackCow),
                x if x == stringify!(BlackEel).to_lowercase() => Some(Color::BlackEel),
                x if x == stringify!(BlanchedAlmond).to_lowercase() => Some(Color::BlanchedAlmond),
                x if x == stringify!(Blonde).to_lowercase() => Some(Color::Blonde),
                x if x == stringify!(BloodNight).to_lowercase() => Some(Color::BloodNight),
                x if x == stringify!(BloodRed).to_lowercase() => Some(Color::BloodRed),
                x if x == stringify!(BlossomPink).to_lowercase() => Some(Color::BlossomPink),
                x if x == stringify!(Blue).to_lowercase() => Some(Color::Blue),
                x if x == stringify!(BlueAngel).to_lowercase() => Some(Color::BlueAngel),
                x if x == stringify!(BlueDiamond).to_lowercase() => Some(Color::BlueDiamond),
                x if x == stringify!(BlueDress).to_lowercase() => Some(Color::BlueDress),
                x if x == stringify!(BlueEyes).to_lowercase() => Some(Color::BlueEyes),
                x if x == stringify!(BlueGray).to_lowercase() => Some(Color::BlueGray),
                x if x == stringify!(BlueGreen).to_lowercase() => Some(Color::BlueGreen),
                x if x == stringify!(BlueHosta).to_lowercase() => Some(Color::BlueHosta),
                x if x == stringify!(BlueIvy).to_lowercase() => Some(Color::BlueIvy),
                x if x == stringify!(BlueJay).to_lowercase() => Some(Color::BlueJay),
                x if x == stringify!(BlueKoi).to_lowercase() => Some(Color::BlueKoi),
                x if x == stringify!(BlueLagoon).to_lowercase() => Some(Color::BlueLagoon),
                x if x == stringify!(BlueLotus).to_lowercase() => Some(Color::BlueLotus),
                x if x == stringify!(BlueMagenta).to_lowercase() => Some(Color::BlueMagenta),
                x if x == stringify!(BlueMossGreen).to_lowercase() => Some(Color::BlueMossGreen),
                x if x == stringify!(BlueOrchid).to_lowercase() => Some(Color::BlueOrchid),
                x if x == stringify!(BlueRibbon).to_lowercase() => Some(Color::BlueRibbon),
                x if x == stringify!(BlueTurquoise).to_lowercase() => Some(Color::BlueTurquoise),
                x if x == stringify!(BlueWhale).to_lowercase() => Some(Color::BlueWhale),
                x if x == stringify!(BlueZircon).to_lowercase() => Some(Color::BlueZircon),
                x if x == stringify!(BlueViolet).to_lowercase() => Some(Color::BlueViolet),
                x if x == stringify!(BlueberryBlue).to_lowercase() => Some(Color::BlueberryBlue),
                x if x == stringify!(Blurple).to_lowercase() => Some(Color::Blurple),
                x if x == stringify!(Blush).to_lowercase() => Some(Color::Blush),
                x if x == stringify!(BlushPink).to_lowercase() => Some(Color::BlushPink),
                x if x == stringify!(BlushRed).to_lowercase() => Some(Color::BlushRed),
                x if x == stringify!(BoldYellow).to_lowercase() => Some(Color::BoldYellow),
                x if x == stringify!(BoneWhite).to_lowercase() => Some(Color::BoneWhite),
                x if x == stringify!(BottleGreen).to_lowercase() => Some(Color::BottleGreen),
                x if x == stringify!(Brass).to_lowercase() => Some(Color::Brass),
                x if x == stringify!(BrightBlue).to_lowercase() => Some(Color::BrightBlue),
                x if x == stringify!(BrightCyan).to_lowercase() => Some(Color::BrightCyan),
                x if x == stringify!(BrightGold).to_lowercase() => Some(Color::BrightGold),
                x if x == stringify!(BrightGrape).to_lowercase() => Some(Color::BrightGrape),
                x if x == stringify!(BrightGreen).to_lowercase() => Some(Color::BrightGreen),
                x if x == stringify!(BrightLilac).to_lowercase() => Some(Color::BrightLilac),
                x if x == stringify!(BrightMaroon).to_lowercase() => Some(Color::BrightMaroon),
                x if x == stringify!(BrightNavyBlue).to_lowercase() => Some(Color::BrightNavyBlue),
                x if x == stringify!(BrightNeonPink).to_lowercase() => Some(Color::BrightNeonPink),
                x if x == stringify!(BrightOrange).to_lowercase() => Some(Color::BrightOrange),
                x if x == stringify!(BrightPink).to_lowercase() => Some(Color::BrightPink),
                x if x == stringify!(BrightPurple).to_lowercase() => Some(Color::BrightPurple),
                x if x == stringify!(BrightTeal).to_lowercase() => Some(Color::BrightTeal),
                x if x == stringify!(BrightTurquoise).to_lowercase() => {
                    Some(Color::BrightTurquoise)
                }
                x if x == stringify!(BroccoliGreen).to_lowercase() => Some(Color::BroccoliGreen),
                x if x == stringify!(Bronze).to_lowercase() => Some(Color::Bronze),
                x if x == stringify!(BronzeGold).to_lowercase() => Some(Color::BronzeGold),
                x if x == stringify!(Brown).to_lowercase() => Some(Color::Brown),
                x if x == stringify!(BrownBear).to_lowercase() => Some(Color::BrownBear),
                x if x == stringify!(BrownRust).to_lowercase() => Some(Color::BrownRust),
                x if x == stringify!(BrownSand).to_lowercase() => Some(Color::BrownSand),
                x if x == stringify!(BrownSugar).to_lowercase() => Some(Color::BrownSugar),
                x if x == stringify!(BulletShell).to_lowercase() => Some(Color::BulletShell),
                x if x == stringify!(Burgundy).to_lowercase() => Some(Color::Burgundy),
                x if x == stringify!(BurlyWood).to_lowercase() => Some(Color::BurlyWood),
                x if x == stringify!(BurntPink).to_lowercase() => Some(Color::BurntPink),
                x if x == stringify!(ButterflyBlue).to_lowercase() => Some(Color::ButterflyBlue),
                x if x == stringify!(CactusGreen).to_lowercase() => Some(Color::CactusGreen),
                x if x == stringify!(CadetBlue).to_lowercase() => Some(Color::CadetBlue),
                x if x == stringify!(CadillacPink).to_lowercase() => Some(Color::CadillacPink),
                x if x == stringify!(CamelBrown).to_lowercase() => Some(Color::CamelBrown),
                x if x == stringify!(CamouflageGreen).to_lowercase() => {
                    Some(Color::CamouflageGreen)
                }
                x if x == stringify!(CanaryBlue).to_lowercase() => Some(Color::CanaryBlue),
                x if x == stringify!(CanaryYellow).to_lowercase() => Some(Color::CanaryYellow),
                x if x == stringify!(Cantaloupe).to_lowercase() => Some(Color::Cantaloupe),
                x if x == stringify!(Caramel).to_lowercase() => Some(Color::Caramel),
                x if x == stringify!(CarbonGray).to_lowercase() => Some(Color::CarbonGray),
                x if x == stringify!(CarbonRed).to_lowercase() => Some(Color::CarbonRed),
                x if x == stringify!(CardboardBrown).to_lowercase() => Some(Color::CardboardBrown),
                x if x == stringify!(CarnationPink).to_lowercase() => Some(Color::CarnationPink),
                x if x == stringify!(CarrotOrange).to_lowercase() => Some(Color::CarrotOrange),
                x if x == stringify!(Celeste).to_lowercase() => Some(Color::Celeste),
                x if x == stringify!(ChameleonGreen).to_lowercase() => Some(Color::ChameleonGreen),
                x if x == stringify!(Champagne).to_lowercase() => Some(Color::Champagne),
                x if x == stringify!(Charcoal).to_lowercase() => Some(Color::Charcoal),
                x if x == stringify!(CharcoalBlue).to_lowercase() => Some(Color::CharcoalBlue),
                x if x == stringify!(Chartreuse).to_lowercase() => Some(Color::Chartreuse),
                x if x == stringify!(CheeseOrange).to_lowercase() => Some(Color::CheeseOrange),
                x if x == stringify!(CherryRed).to_lowercase() => Some(Color::CherryRed),
                x if x == stringify!(Chestnut).to_lowercase() => Some(Color::Chestnut),
                x if x == stringify!(ChestnutRed).to_lowercase() => Some(Color::ChestnutRed),
                x if x == stringify!(ChilliPepper).to_lowercase() => Some(Color::ChilliPepper),
                x if x == stringify!(Chocolate).to_lowercase() => Some(Color::Chocolate),
                x if x == stringify!(ChocolateBrown).to_lowercase() => Some(Color::ChocolateBrown),
                x if x == stringify!(ChromeAluminum).to_lowercase() => Some(Color::ChromeAluminum),
                x if x == stringify!(ChromeGold).to_lowercase() => Some(Color::ChromeGold),
                x if x == stringify!(ChromeGreen).to_lowercase() => Some(Color::ChromeGreen),
                x if x == stringify!(ChromePink).to_lowercase() => Some(Color::ChromePink),
                x if x == stringify!(ChromeWhite).to_lowercase() => Some(Color::ChromeWhite),
                x if x == stringify!(Cinnamon).to_lowercase() => Some(Color::Cinnamon),
                x if x == stringify!(CitronGreen).to_lowercase() => Some(Color::CitronGreen),
                x if x == stringify!(ClematisViolet).to_lowercase() => Some(Color::ClematisViolet),
                x if x == stringify!(CloudyGray).to_lowercase() => Some(Color::CloudyGray),
                x if x == stringify!(CloverGreen).to_lowercase() => Some(Color::CloverGreen),
                x if x == stringify!(CobaltBlue).to_lowercase() => Some(Color::CobaltBlue),
                x if x == stringify!(Coffee).to_lowercase() => Some(Color::Coffee),
                x if x == stringify!(ColdMetal).to_lowercase() => Some(Color::ColdMetal),
                x if x == stringify!(ColumbiaBlue).to_lowercase() => Some(Color::ColumbiaBlue),
                x if x == stringify!(ConstructionConeOrange).to_lowercase() => {
                    Some(Color::ConstructionConeOrange)
                }
                x if x == stringify!(CookieBrown).to_lowercase() => Some(Color::CookieBrown),
                x if x == stringify!(Copper).to_lowercase() => Some(Color::Copper),
                x if x == stringify!(CopperRed).to_lowercase() => Some(Color::CopperRed),
                x if x == stringify!(Coral).to_lowercase() => Some(Color::Coral),
                x if x == stringify!(CoralBlue).to_lowercase() => Some(Color::CoralBlue),
                x if x == stringify!(CoralBrown).to_lowercase() => Some(Color::CoralBrown),
                x if x == stringify!(CoralPeach).to_lowercase() => Some(Color::CoralPeach),
                x if x == stringify!(CornYellow).to_lowercase() => Some(Color::CornYellow),
                x if x == stringify!(CornflowerBlue).to_lowercase() => Some(Color::CornflowerBlue),
                x if x == stringify!(Cornsilk).to_lowercase() => Some(Color::Cornsilk),
                x if x == stringify!(CosmicLatte).to_lowercase() => Some(Color::CosmicLatte),
                x if x == stringify!(Cotton).to_lowercase() => Some(Color::Cotton),
                x if x == stringify!(CottonCandy).to_lowercase() => Some(Color::CottonCandy),
                x if x == stringify!(Cranberry).to_lowercase() => Some(Color::Cranberry),
                x if x == stringify!(Cream).to_lowercase() => Some(Color::Cream),
                x if x == stringify!(CreamWhite).to_lowercase() => Some(Color::CreamWhite),
                x if x == stringify!(CreamyWhite).to_lowercase() => Some(Color::CreamyWhite),
                x if x == stringify!(Crimson).to_lowercase() => Some(Color::Crimson),
                x if x == stringify!(CrimsonPurple).to_lowercase() => Some(Color::CrimsonPurple),
                x if x == stringify!(CrimsonRed).to_lowercase() => Some(Color::CrimsonRed),
                x if x == stringify!(CrocusPurple).to_lowercase() => Some(Color::CrocusPurple),
                x if x == stringify!(CrystalBlue).to_lowercase() => Some(Color::CrystalBlue),
                x if x == stringify!(Cyan).to_lowercase() => Some(Color::Cyan),
                x if x == stringify!(CyanBlue).to_lowercase() => Some(Color::CyanBlue),
                x if x == stringify!(CyanOpaque).to_lowercase() => Some(Color::CyanOpaque),
                x if x == stringify!(DarkAlmond).to_lowercase() => Some(Color::DarkAlmond),
                x if x == stringify!(DarkBeige).to_lowercase() => Some(Color::DarkBeige),
                x if x == stringify!(DarkBisque).to_lowercase() => Some(Color::DarkBisque),
                x if x == stringify!(DarkBlonde).to_lowercase() => Some(Color::DarkBlonde),
                x if x == stringify!(DarkBlueGray).to_lowercase() => Some(Color::DarkBlueGray),
                x if x == stringify!(DarkBlurple).to_lowercase() => Some(Color::DarkBlurple),
                x if x == stringify!(DarkBronze).to_lowercase() => Some(Color::DarkBronze),
                x if x == stringify!(DarkBrown).to_lowercase() => Some(Color::DarkBrown),
                x if x == stringify!(DarkCarnationPink).to_lowercase() => {
                    Some(Color::DarkCarnationPink)
                }
                x if x == stringify!(DarkCoffee).to_lowercase() => Some(Color::DarkCoffee),
                x if x == stringify!(DarkForestGreen).to_lowercase() => {
                    Some(Color::DarkForestGreen)
                }
                x if x == stringify!(DarkGainsboro).to_lowercase() => Some(Color::DarkGainsboro),
                x if x == stringify!(DarkGold).to_lowercase() => Some(Color::DarkGold),
                x if x == stringify!(DarkGrayishOlive).to_lowercase() => {
                    Some(Color::DarkGrayishOlive)
                }
                x if x == stringify!(DarkGreenBlue).to_lowercase() => Some(Color::DarkGreenBlue),
                x if x == stringify!(DarkHazelBrown).to_lowercase() => Some(Color::DarkHazelBrown),
                x if x == stringify!(DarkHotPink).to_lowercase() => Some(Color::DarkHotPink),
                x if x == stringify!(DarkLimeGreen).to_lowercase() => Some(Color::DarkLimeGreen),
                x if x == stringify!(DarkMaroon).to_lowercase() => Some(Color::DarkMaroon),
                x if x == stringify!(DarkMint).to_lowercase() => Some(Color::DarkMint),
                x if x == stringify!(DarkMoccasin).to_lowercase() => Some(Color::DarkMoccasin),
                x if x == stringify!(DarkPink).to_lowercase() => Some(Color::DarkPink),
                x if x == stringify!(DarkPurple).to_lowercase() => Some(Color::DarkPurple),
                x if x == stringify!(DarkRaspberry).to_lowercase() => Some(Color::DarkRaspberry),
                x if x == stringify!(DarkScarlet).to_lowercase() => Some(Color::DarkScarlet),
                x if x == stringify!(DarkSienna).to_lowercase() => Some(Color::DarkSienna),
                x if x == stringify!(DarkSkyBlue).to_lowercase() => Some(Color::DarkSkyBlue),
                x if x == stringify!(DarkSlate).to_lowercase() => Some(Color::DarkSlate),
                x if x == stringify!(DarkSteampunk).to_lowercase() => Some(Color::DarkSteampunk),
                x if x == stringify!(DarkTeal).to_lowercase() => Some(Color::DarkTeal),
                x if x == stringify!(DarkWhite).to_lowercase() => Some(Color::DarkWhite),
                x if x == stringify!(DarkYellow).to_lowercase() => Some(Color::DarkYellow),
                x if x == stringify!(DarkBlue).to_lowercase() => Some(Color::DarkBlue),
                x if x == stringify!(DarkCyan).to_lowercase() => Some(Color::DarkCyan),
                x if x == stringify!(DarkGoldenRod).to_lowercase() => Some(Color::DarkGoldenRod),
                x if x == stringify!(DarkGray).to_lowercase() => Some(Color::DarkGray),
                x if x == stringify!(DarkGreen).to_lowercase() => Some(Color::DarkGreen),
                x if x == stringify!(DarkGrey).to_lowercase() => Some(Color::DarkGrey),
                x if x == stringify!(DarkKhaki).to_lowercase() => Some(Color::DarkKhaki),
                x if x == stringify!(DarkMagenta).to_lowercase() => Some(Color::DarkMagenta),
                x if x == stringify!(DarkOliveGreen).to_lowercase() => Some(Color::DarkOliveGreen),
                x if x == stringify!(DarkOrange).to_lowercase() => Some(Color::DarkOrange),
                x if x == stringify!(DarkOrchid).to_lowercase() => Some(Color::DarkOrchid),
                x if x == stringify!(DarkRed).to_lowercase() => Some(Color::DarkRed),
                x if x == stringify!(DarkSalmon).to_lowercase() => Some(Color::DarkSalmon),
                x if x == stringify!(DarkSeaGreen).to_lowercase() => Some(Color::DarkSeaGreen),
                x if x == stringify!(DarkSlateBlue).to_lowercase() => Some(Color::DarkSlateBlue),
                x if x == stringify!(DarkSlateGray).to_lowercase() => Some(Color::DarkSlateGray),
                x if x == stringify!(DarkSlateGrey).to_lowercase() => Some(Color::DarkSlateGrey),
                x if x == stringify!(DarkTurquoise).to_lowercase() => Some(Color::DarkTurquoise),
                x if x == stringify!(DarkViolet).to_lowercase() => Some(Color::DarkViolet),
                x if x == stringify!(DaySkyBlue).to_lowercase() => Some(Color::DaySkyBlue),
                x if x == stringify!(DeepAmber).to_lowercase() => Some(Color::DeepAmber),
                x if x == stringify!(DeepEmeraldGreen).to_lowercase() => {
                    Some(Color::DeepEmeraldGreen)
                }
                x if x == stringify!(DeepGreen).to_lowercase() => Some(Color::DeepGreen),
                x if x == stringify!(DeepMauve).to_lowercase() => Some(Color::DeepMauve),
                x if x == stringify!(DeepPeach).to_lowercase() => Some(Color::DeepPeach),
                x if x == stringify!(DeepPeriwinkle).to_lowercase() => Some(Color::DeepPeriwinkle),
                x if x == stringify!(DeepPurple).to_lowercase() => Some(Color::DeepPurple),
                x if x == stringify!(DeepRed).to_lowercase() => Some(Color::DeepRed),
                x if x == stringify!(DeepRose).to_lowercase() => Some(Color::DeepRose),
                x if x == stringify!(DeepSea).to_lowercase() => Some(Color::DeepSea),
                x if x == stringify!(DeepSeaBlue).to_lowercase() => Some(Color::DeepSeaBlue),
                x if x == stringify!(DeepSeaGreen).to_lowercase() => Some(Color::DeepSeaGreen),
                x if x == stringify!(DeepTeal).to_lowercase() => Some(Color::DeepTeal),
                x if x == stringify!(DeepTurquoise).to_lowercase() => Some(Color::DeepTurquoise),
                x if x == stringify!(DeepYellow).to_lowercase() => Some(Color::DeepYellow),
                x if x == stringify!(DeepPink).to_lowercase() => Some(Color::DeepPink),
                x if x == stringify!(DeepSkyBlue).to_lowercase() => Some(Color::DeepSkyBlue),
                x if x == stringify!(DeerBrown).to_lowercase() => Some(Color::DeerBrown),
                x if x == stringify!(DenimBlue).to_lowercase() => Some(Color::DenimBlue),
                x if x == stringify!(DenimDarkBlue).to_lowercase() => Some(Color::DenimDarkBlue),
                x if x == stringify!(DesertSand).to_lowercase() => Some(Color::DesertSand),
                x if x == stringify!(DimGray).to_lowercase() => Some(Color::DimGray),
                x if x == stringify!(DimGrey).to_lowercase() => Some(Color::DimGrey),
                x if x == stringify!(DimorphothecaMagenta).to_lowercase() => {
                    Some(Color::DimorphothecaMagenta)
                }
                x if x == stringify!(DinosaurGreen).to_lowercase() => Some(Color::DinosaurGreen),
                x if x == stringify!(DirtyWhite).to_lowercase() => Some(Color::DirtyWhite),
                x if x == stringify!(DodgerBlue).to_lowercase() => Some(Color::DodgerBlue),
                x if x == stringify!(DollarBillGreen).to_lowercase() => {
                    Some(Color::DollarBillGreen)
                }
                x if x == stringify!(DonutPink).to_lowercase() => Some(Color::DonutPink),
                x if x == stringify!(DragonGreen).to_lowercase() => Some(Color::DragonGreen),
                x if x == stringify!(DullGreenYellow).to_lowercase() => {
                    Some(Color::DullGreenYellow)
                }
                x if x == stringify!(DullPurple).to_lowercase() => Some(Color::DullPurple),
                x if x == stringify!(DullSeaGreen).to_lowercase() => Some(Color::DullSeaGreen),
                x if x == stringify!(DuskyPink).to_lowercase() => Some(Color::DuskyPink),
                x if x == stringify!(DustyPink).to_lowercase() => Some(Color::DustyPink),
                x if x == stringify!(DustyRose).to_lowercase() => Some(Color::DustyRose),
                x if x == stringify!(EarthBlue).to_lowercase() => Some(Color::EarthBlue),
                x if x == stringify!(EarthGreen).to_lowercase() => Some(Color::EarthGreen),
                x if x == stringify!(Ebony).to_lowercase() => Some(Color::Ebony),
                x if x == stringify!(EggShell).to_lowercase() => Some(Color::EggShell),
                x if x == stringify!(Eggplant).to_lowercase() => Some(Color::Eggplant),
                x if x == stringify!(ElectricBlue).to_lowercase() => Some(Color::ElectricBlue),
                x if x == stringify!(ElfGreen).to_lowercase() => Some(Color::ElfGreen),
                x if x == stringify!(Emerald).to_lowercase() => Some(Color::Emerald),
                x if x == stringify!(EmeraldGreen).to_lowercase() => Some(Color::EmeraldGreen),
                x if x == stringify!(EstorilBlue).to_lowercase() => Some(Color::EstorilBlue),
                x if x == stringify!(FallForestGreen).to_lowercase() => {
                    Some(Color::FallForestGreen)
                }
                x if x == stringify!(FallLeafBrown).to_lowercase() => Some(Color::FallLeafBrown),
                x if x == stringify!(FernGreen).to_lowercase() => Some(Color::FernGreen),
                x if x == stringify!(FerrariRed).to_lowercase() => Some(Color::FerrariRed),
                x if x == stringify!(FireEngineRed).to_lowercase() => Some(Color::FireEngineRed),
                x if x == stringify!(FireBrick).to_lowercase() => Some(Color::FireBrick),
                x if x == stringify!(FlamingoPink).to_lowercase() => Some(Color::FlamingoPink),
                x if x == stringify!(FloralWhite).to_lowercase() => Some(Color::FloralWhite),
                x if x == stringify!(FluroOrange).to_lowercase() => Some(Color::FluroOrange),
                x if x == stringify!(ForestGreen).to_lowercase() => Some(Color::ForestGreen),
                x if x == stringify!(FrenchLilac).to_lowercase() => Some(Color::FrenchLilac),
                x if x == stringify!(FrogGreen).to_lowercase() => Some(Color::FrogGreen),
                x if x == stringify!(Fuchsia).to_lowercase() => Some(Color::Fuchsia),
                x if x == stringify!(FuchsiaPink).to_lowercase() => Some(Color::FuchsiaPink),
                x if x == stringify!(Gainsboro).to_lowercase() => Some(Color::Gainsboro),
                x if x == stringify!(GarnetRed).to_lowercase() => Some(Color::GarnetRed),
                x if x == stringify!(GearSteelGray).to_lowercase() => Some(Color::GearSteelGray),
                x if x == stringify!(GhostWhite).to_lowercase() => Some(Color::GhostWhite),
                x if x == stringify!(GingerBrown).to_lowercase() => Some(Color::GingerBrown),
                x if x == stringify!(GingerRed).to_lowercase() => Some(Color::GingerRed),
                x if x == stringify!(GlacialBlueIce).to_lowercase() => Some(Color::GlacialBlueIce),
                x if x == stringify!(Gold).to_lowercase() => Some(Color::Gold),
                x if x == stringify!(GoldPink).to_lowercase() => Some(Color::GoldPink),
                x if x == stringify!(GoldenBlonde).to_lowercase() => Some(Color::GoldenBlonde),
                x if x == stringify!(GoldenBrown).to_lowercase() => Some(Color::GoldenBrown),
                x if x == stringify!(GoldenSilk).to_lowercase() => Some(Color::GoldenSilk),
                x if x == stringify!(GoldenYellow).to_lowercase() => Some(Color::GoldenYellow),
                x if x == stringify!(GoldenRod).to_lowercase() => Some(Color::GoldenRod),
                x if x == stringify!(Granite).to_lowercase() => Some(Color::Granite),
                x if x == stringify!(Grape).to_lowercase() => Some(Color::Grape),
                x if x == stringify!(Grapefruit).to_lowercase() => Some(Color::Grapefruit),
                x if x == stringify!(GrassGreen).to_lowercase() => Some(Color::GrassGreen),
                x if x == stringify!(Gray).to_lowercase() => Some(Color::Gray),
                x if x == stringify!(GrayBrown).to_lowercase() => Some(Color::GrayBrown),
                x if x == stringify!(GrayCloud).to_lowercase() => Some(Color::GrayCloud),
                x if x == stringify!(GrayDolphin).to_lowercase() => Some(Color::GrayDolphin),
                x if x == stringify!(GrayGoose).to_lowercase() => Some(Color::GrayGoose),
                x if x == stringify!(GrayGreen).to_lowercase() => Some(Color::GrayGreen),
                x if x == stringify!(GrayWolf).to_lowercase() => Some(Color::GrayWolf),
                x if x == stringify!(GrayishTurquoise).to_lowercase() => {
                    Some(Color::GrayishTurquoise)
                }
                x if x == stringify!(Green).to_lowercase() => Some(Color::Green),
                x if x == stringify!(GreenApple).to_lowercase() => Some(Color::GreenApple),
                x if x == stringify!(GreenLeaves).to_lowercase() => Some(Color::GreenLeaves),
                x if x == stringify!(GreenOnion).to_lowercase() => Some(Color::GreenOnion),
                x if x == stringify!(GreenPeas).to_lowercase() => Some(Color::GreenPeas),
                x if x == stringify!(GreenPepper).to_lowercase() => Some(Color::GreenPepper),
                x if x == stringify!(GreenSnake).to_lowercase() => Some(Color::GreenSnake),
                x if x == stringify!(GreenThumb).to_lowercase() => Some(Color::GreenThumb),
                x if x == stringify!(GreenYellow).to_lowercase() => Some(Color::GreenYellow),
                x if x == stringify!(GreenishBlue).to_lowercase() => Some(Color::GreenishBlue),
                x if x == stringify!(Grey).to_lowercase() => Some(Color::Grey),
                x if x == stringify!(GulfBlue).to_lowercase() => Some(Color::GulfBlue),
                x if x == stringify!(Gunmetal).to_lowercase() => Some(Color::Gunmetal),
                x if x == stringify!(GunmetalGray).to_lowercase() => Some(Color::GunmetalGray),
                x if x == stringify!(HalfWhite).to_lowercase() => Some(Color::HalfWhite),
                x if x == stringify!(HalloweenOrange).to_lowercase() => {
                    Some(Color::HalloweenOrange)
                }
                x if x == stringify!(HarvestGold).to_lowercase() => Some(Color::HarvestGold),
                x if x == stringify!(Hazel).to_lowercase() => Some(Color::Hazel),
                x if x == stringify!(HazelGreen).to_lowercase() => Some(Color::HazelGreen),
                x if x == stringify!(HeavenlyBlue).to_lowercase() => Some(Color::HeavenlyBlue),
                x if x == stringify!(HeliotropePurple).to_lowercase() => {
                    Some(Color::HeliotropePurple)
                }
                x if x == stringify!(HoneyDew).to_lowercase() => Some(Color::HoneyDew),
                x if x == stringify!(HotDeepPink).to_lowercase() => Some(Color::HotDeepPink),
                x if x == stringify!(HotPink).to_lowercase() => Some(Color::HotPink),
                x if x == stringify!(HummingbirdGreen).to_lowercase() => {
                    Some(Color::HummingbirdGreen)
                }
                x if x == stringify!(HunterGreen).to_lowercase() => Some(Color::HunterGreen),
                x if x == stringify!(Iceberg).to_lowercase() => Some(Color::Iceberg),
                x if x == stringify!(IguanaGreen).to_lowercase() => Some(Color::IguanaGreen),
                x if x == stringify!(IndianSaffron).to_lowercase() => Some(Color::IndianSaffron),
                x if x == stringify!(IndianRed).to_lowercase() => Some(Color::IndianRed),
                x if x == stringify!(Indigo).to_lowercase() => Some(Color::Indigo),
                x if x == stringify!(Iridium).to_lowercase() => Some(Color::Iridium),
                x if x == stringify!(IrishGreen).to_lowercase() => Some(Color::IrishGreen),
                x if x == stringify!(IronGray).to_lowercase() => Some(Color::IronGray),
                x if x == stringify!(IsleOfManGreen).to_lowercase() => Some(Color::IsleOfManGreen),
                x if x == stringify!(Ivory).to_lowercase() => Some(Color::Ivory),
                x if x == stringify!(Jade).to_lowercase() => Some(Color::Jade),
                x if x == stringify!(JadeGreen).to_lowercase() => Some(Color::JadeGreen),
                x if x == stringify!(JasminePurple).to_lowercase() => Some(Color::JasminePurple),
                x if x == stringify!(JeansBlue).to_lowercase() => Some(Color::JeansBlue),
                x if x == stringify!(Jellyfish).to_lowercase() => Some(Color::Jellyfish),
                x if x == stringify!(JetGray).to_lowercase() => Some(Color::JetGray),
                x if x == stringify!(JungleGreen).to_lowercase() => Some(Color::JungleGreen),
                x if x == stringify!(KellyGreen).to_lowercase() => Some(Color::KellyGreen),
                x if x == stringify!(Khaki).to_lowercase() => Some(Color::Khaki),
                x if x == stringify!(KhakiBrown).to_lowercase() => Some(Color::KhakiBrown),
                x if x == stringify!(KhakiGreen).to_lowercase() => Some(Color::KhakiGreen),
                x if x == stringify!(KhakiRose).to_lowercase() => Some(Color::KhakiRose),
                x if x == stringify!(LapisBlue).to_lowercase() => Some(Color::LapisBlue),
                x if x == stringify!(LavaRed).to_lowercase() => Some(Color::LavaRed),
                x if x == stringify!(Lavender).to_lowercase() => Some(Color::Lavender),
                x if x == stringify!(LavenderBlue).to_lowercase() => Some(Color::LavenderBlue),
                x if x == stringify!(LavenderPinocchio).to_lowercase() => {
                    Some(Color::LavenderPinocchio)
                }
                x if x == stringify!(LavenderPurple).to_lowercase() => Some(Color::LavenderPurple),
                x if x == stringify!(LavenderBlush).to_lowercase() => Some(Color::LavenderBlush),
                x if x == stringify!(LawnGreen).to_lowercase() => Some(Color::LawnGreen),
                x if x == stringify!(LemonGreen).to_lowercase() => Some(Color::LemonGreen),
                x if x == stringify!(LemonYellow).to_lowercase() => Some(Color::LemonYellow),
                x if x == stringify!(LemonChiffon).to_lowercase() => Some(Color::LemonChiffon),
                x if x == stringify!(LightAquamarine).to_lowercase() => {
                    Some(Color::LightAquamarine)
                }
                x if x == stringify!(LightBeige).to_lowercase() => Some(Color::LightBeige),
                x if x == stringify!(LightBlack).to_lowercase() => Some(Color::LightBlack),
                x if x == stringify!(LightBrown).to_lowercase() => Some(Color::LightBrown),
                x if x == stringify!(LightCopper).to_lowercase() => Some(Color::LightCopper),
                x if x == stringify!(LightDayBlue).to_lowercase() => Some(Color::LightDayBlue),
                x if x == stringify!(LightFrenchBeige).to_lowercase() => {
                    Some(Color::LightFrenchBeige)
                }
                x if x == stringify!(LightGold).to_lowercase() => Some(Color::LightGold),
                x if x == stringify!(LightJade).to_lowercase() => Some(Color::LightJade),
                x if x == stringify!(LightMintGreen).to_lowercase() => Some(Color::LightMintGreen),
                x if x == stringify!(LightOliveGreen).to_lowercase() => {
                    Some(Color::LightOliveGreen)
                }
                x if x == stringify!(LightOrange).to_lowercase() => Some(Color::LightOrange),
                x if x == stringify!(LightPurple).to_lowercase() => Some(Color::LightPurple),
                x if x == stringify!(LightPurpleBlue).to_lowercase() => {
                    Some(Color::LightPurpleBlue)
                }
                x if x == stringify!(LightRed).to_lowercase() => Some(Color::LightRed),
                x if x == stringify!(LightRose).to_lowercase() => Some(Color::LightRose),
                x if x == stringify!(LightRoseGreen).to_lowercase() => Some(Color::LightRoseGreen),
                x if x == stringify!(LightSalmonRose).to_lowercase() => {
                    Some(Color::LightSalmonRose)
                }
                x if x == stringify!(LightSlate).to_lowercase() => Some(Color::LightSlate),
                x if x == stringify!(LightSlateBlue).to_lowercase() => Some(Color::LightSlateBlue),
                x if x == stringify!(LightSteelGray).to_lowercase() => Some(Color::LightSteelGray),
                x if x == stringify!(LightTeal).to_lowercase() => Some(Color::LightTeal),
                x if x == stringify!(LightWhite).to_lowercase() => Some(Color::LightWhite),
                x if x == stringify!(LightBlue).to_lowercase() => Some(Color::LightBlue),
                x if x == stringify!(LightCoral).to_lowercase() => Some(Color::LightCoral),
                x if x == stringify!(LightCyan).to_lowercase() => Some(Color::LightCyan),
                x if x == stringify!(LightGoldenRodYellow).to_lowercase() => {
                    Some(Color::LightGoldenRodYellow)
                }
                x if x == stringify!(LightGray).to_lowercase() => Some(Color::LightGray),
                x if x == stringify!(LightGreen).to_lowercase() => Some(Color::LightGreen),
                x if x == stringify!(LightGrey).to_lowercase() => Some(Color::LightGrey),
                x if x == stringify!(LightPink).to_lowercase() => Some(Color::LightPink),
                x if x == stringify!(LightSalmon).to_lowercase() => Some(Color::LightSalmon),
                x if x == stringify!(LightSeaGreen).to_lowercase() => Some(Color::LightSeaGreen),
                x if x == stringify!(LightSkyBlue).to_lowercase() => Some(Color::LightSkyBlue),
                x if x == stringify!(LightSlateGray).to_lowercase() => Some(Color::LightSlateGray),
                x if x == stringify!(LightSlateGrey).to_lowercase() => Some(Color::LightSlateGrey),
                x if x == stringify!(LightSteelBlue).to_lowercase() => Some(Color::LightSteelBlue),
                x if x == stringify!(LightYellow).to_lowercase() => Some(Color::LightYellow),
                x if x == stringify!(Lilac).to_lowercase() => Some(Color::Lilac),
                x if x == stringify!(Lime).to_lowercase() => Some(Color::Lime),
                x if x == stringify!(LimeMintGreen).to_lowercase() => Some(Color::LimeMintGreen),
                x if x == stringify!(LimeGreen).to_lowercase() => Some(Color::LimeGreen),
                x if x == stringify!(Linen).to_lowercase() => Some(Color::Linen),
                x if x == stringify!(LipstickPink).to_lowercase() => Some(Color::LipstickPink),
                x if x == stringify!(LotusGreen).to_lowercase() => Some(Color::LotusGreen),
                x if x == stringify!(LoveRed).to_lowercase() => Some(Color::LoveRed),
                x if x == stringify!(LovelyPurple).to_lowercase() => Some(Color::LovelyPurple),
                x if x == stringify!(MacaroniandCheese).to_lowercase() => {
                    Some(Color::MacaroniandCheese)
                }
                x if x == stringify!(MacawBlueGreen).to_lowercase() => Some(Color::MacawBlueGreen),
                x if x == stringify!(Magenta).to_lowercase() => Some(Color::Magenta),
                x if x == stringify!(MagentaPink).to_lowercase() => Some(Color::MagentaPink),
                x if x == stringify!(MagicMint).to_lowercase() => Some(Color::MagicMint),
                x if x == stringify!(Mahogany).to_lowercase() => Some(Color::Mahogany),
                x if x == stringify!(MangoOrange).to_lowercase() => Some(Color::MangoOrange),
                x if x == stringify!(MarbleBlue).to_lowercase() => Some(Color::MarbleBlue),
                x if x == stringify!(Maroon).to_lowercase() => Some(Color::Maroon),
                x if x == stringify!(MaroonRed).to_lowercase() => Some(Color::MaroonRed),
                x if x == stringify!(Mauve).to_lowercase() => Some(Color::Mauve),
                x if x == stringify!(MauveTaupe).to_lowercase() => Some(Color::MauveTaupe),
                x if x == stringify!(MediumForestGreen).to_lowercase() => {
                    Some(Color::MediumForestGreen)
                }
                x if x == stringify!(MediumTeal).to_lowercase() => Some(Color::MediumTeal),
                x if x == stringify!(MediumAquaMarine).to_lowercase() => {
                    Some(Color::MediumAquaMarine)
                }
                x if x == stringify!(MediumBlue).to_lowercase() => Some(Color::MediumBlue),
                x if x == stringify!(MediumOrchid).to_lowercase() => Some(Color::MediumOrchid),
                x if x == stringify!(MediumPurple).to_lowercase() => Some(Color::MediumPurple),
                x if x == stringify!(MediumSeaGreen).to_lowercase() => Some(Color::MediumSeaGreen),
                x if x == stringify!(MediumSlateBlue).to_lowercase() => {
                    Some(Color::MediumSlateBlue)
                }
                x if x == stringify!(MediumSpringGreen).to_lowercase() => {
                    Some(Color::MediumSpringGreen)
                }
                x if x == stringify!(MediumTurquoise).to_lowercase() => {
                    Some(Color::MediumTurquoise)
                }
                x if x == stringify!(MediumVioletRed).to_lowercase() => {
                    Some(Color::MediumVioletRed)
                }
                x if x == stringify!(Metal).to_lowercase() => Some(Color::Metal),
                x if x == stringify!(MetallicBronze).to_lowercase() => Some(Color::MetallicBronze),
                x if x == stringify!(MetallicGold).to_lowercase() => Some(Color::MetallicGold),
                x if x == stringify!(MetallicGreen).to_lowercase() => Some(Color::MetallicGreen),
                x if x == stringify!(MetallicSilver).to_lowercase() => Some(Color::MetallicSilver),
                x if x == stringify!(MiddayBlue).to_lowercase() => Some(Color::MiddayBlue),
                x if x == stringify!(Midnight).to_lowercase() => Some(Color::Midnight),
                x if x == stringify!(MidnightPurple).to_lowercase() => Some(Color::MidnightPurple),
                x if x == stringify!(MidnightBlue).to_lowercase() => Some(Color::MidnightBlue),
                x if x == stringify!(MilitaryGreen).to_lowercase() => Some(Color::MilitaryGreen),
                x if x == stringify!(MilkChocolate).to_lowercase() => Some(Color::MilkChocolate),
                x if x == stringify!(MilkWhite).to_lowercase() => Some(Color::MilkWhite),
                x if x == stringify!(MillenniumJade).to_lowercase() => Some(Color::MillenniumJade),
                x if x == stringify!(Mint).to_lowercase() => Some(Color::Mint),
                x if x == stringify!(MintGreen).to_lowercase() => Some(Color::MintGreen),
                x if x == stringify!(MintCream).to_lowercase() => Some(Color::MintCream),
                x if x == stringify!(MistBlue).to_lowercase() => Some(Color::MistBlue),
                x if x == stringify!(MistyRose).to_lowercase() => Some(Color::MistyRose),
                x if x == stringify!(Moccasin).to_lowercase() => Some(Color::Moccasin),
                x if x == stringify!(Mocha).to_lowercase() => Some(Color::Mocha),
                x if x == stringify!(MossGreen).to_lowercase() => Some(Color::MossGreen),
                x if x == stringify!(Mustard).to_lowercase() => Some(Color::Mustard),
                x if x == stringify!(MustardYellow).to_lowercase() => Some(Color::MustardYellow),
                x if x == stringify!(NardoGray).to_lowercase() => Some(Color::NardoGray),
                x if x == stringify!(NavajoWhite).to_lowercase() => Some(Color::NavajoWhite),
                x if x == stringify!(Navy).to_lowercase() => Some(Color::Navy),
                x if x == stringify!(NebulaGreen).to_lowercase() => Some(Color::NebulaGreen),
                x if x == stringify!(NeonBlue).to_lowercase() => Some(Color::NeonBlue),
                x if x == stringify!(NeonGold).to_lowercase() => Some(Color::NeonGold),
                x if x == stringify!(NeonGreen).to_lowercase() => Some(Color::NeonGreen),
                x if x == stringify!(NeonHotPink).to_lowercase() => Some(Color::NeonHotPink),
                x if x == stringify!(NeonOrange).to_lowercase() => Some(Color::NeonOrange),
                x if x == stringify!(NeonPink).to_lowercase() => Some(Color::NeonPink),
                x if x == stringify!(NeonPurple).to_lowercase() => Some(Color::NeonPurple),
                x if x == stringify!(NeonRed).to_lowercase() => Some(Color::NeonRed),
                x if x == stringify!(NeonYellow).to_lowercase() => Some(Color::NeonYellow),
                x if x == stringify!(NeonYellowGreen).to_lowercase() => {
                    Some(Color::NeonYellowGreen)
                }
                x if x == stringify!(NewMidnightBlue).to_lowercase() => {
                    Some(Color::NewMidnightBlue)
                }
                x if x == stringify!(Night).to_lowercase() => Some(Color::Night),
                x if x == stringify!(NightBlue).to_lowercase() => Some(Color::NightBlue),
                x if x == stringify!(NorthernLightsBlue).to_lowercase() => {
                    Some(Color::NorthernLightsBlue)
                }
                x if x == stringify!(OakBrown).to_lowercase() => Some(Color::OakBrown),
                x if x == stringify!(OceanBlue).to_lowercase() => Some(Color::OceanBlue),
                x if x == stringify!(OceanGreen).to_lowercase() => Some(Color::OceanGreen),
                x if x == stringify!(OffWhite).to_lowercase() => Some(Color::OffWhite),
                x if x == stringify!(Oil).to_lowercase() => Some(Color::Oil),
                x if x == stringify!(OldBurgundy).to_lowercase() => Some(Color::OldBurgundy),
                x if x == stringify!(OldRose).to_lowercase() => Some(Color::OldRose),
                x if x == stringify!(OldLace).to_lowercase() => Some(Color::OldLace),
                x if x == stringify!(Olive).to_lowercase() => Some(Color::Olive),
                x if x == stringify!(OliveGreen).to_lowercase() => Some(Color::OliveGreen),
                x if x == stringify!(OliveDrab).to_lowercase() => Some(Color::OliveDrab),
                x if x == stringify!(Orange).to_lowercase() => Some(Color::Orange),
                x if x == stringify!(OrangeGold).to_lowercase() => Some(Color::OrangeGold),
                x if x == stringify!(OrangeSalmon).to_lowercase() => Some(Color::OrangeSalmon),
                x if x == stringify!(OrangeYellow).to_lowercase() => Some(Color::OrangeYellow),
                x if x == stringify!(OrangeRed).to_lowercase() => Some(Color::OrangeRed),
                x if x == stringify!(Orchid).to_lowercase() => Some(Color::Orchid),
                x if x == stringify!(OrchidPurple).to_lowercase() => Some(Color::OrchidPurple),
                x if x == stringify!(OrganicBrown).to_lowercase() => Some(Color::OrganicBrown),
                x if x == stringify!(PaleBlueLily).to_lowercase() => Some(Color::PaleBlueLily),
                x if x == stringify!(PaleLilac).to_lowercase() => Some(Color::PaleLilac),
                x if x == stringify!(PalePink).to_lowercase() => Some(Color::PalePink),
                x if x == stringify!(PaleSilver).to_lowercase() => Some(Color::PaleSilver),
                x if x == stringify!(PaleGoldenRod).to_lowercase() => Some(Color::PaleGoldenRod),
                x if x == stringify!(PaleGreen).to_lowercase() => Some(Color::PaleGreen),
                x if x == stringify!(PaleTurquoise).to_lowercase() => Some(Color::PaleTurquoise),
                x if x == stringify!(PaleVioletRed).to_lowercase() => Some(Color::PaleVioletRed),
                x if x == stringify!(PapayaOrange).to_lowercase() => Some(Color::PapayaOrange),
                x if x == stringify!(PapayaWhip).to_lowercase() => Some(Color::PapayaWhip),
                x if x == stringify!(Parchment).to_lowercase() => Some(Color::Parchment),
                x if x == stringify!(ParrotGreen).to_lowercase() => Some(Color::ParrotGreen),
                x if x == stringify!(PastelBlue).to_lowercase() => Some(Color::PastelBlue),
                x if x == stringify!(PastelBrown).to_lowercase() => Some(Color::PastelBrown),
                x if x == stringify!(PastelGreen).to_lowercase() => Some(Color::PastelGreen),
                x if x == stringify!(PastelIndigo).to_lowercase() => Some(Color::PastelIndigo),
                x if x == stringify!(PastelLightBlue).to_lowercase() => {
                    Some(Color::PastelLightBlue)
                }
                x if x == stringify!(PastelOrange).to_lowercase() => Some(Color::PastelOrange),
                x if x == stringify!(PastelPink).to_lowercase() => Some(Color::PastelPink),
                x if x == stringify!(PastelPurple).to_lowercase() => Some(Color::PastelPurple),
                x if x == stringify!(PastelRed).to_lowercase() => Some(Color::PastelRed),
                x if x == stringify!(PastelRose).to_lowercase() => Some(Color::PastelRose),
                x if x == stringify!(PastelViolet).to_lowercase() => Some(Color::PastelViolet),
                x if x == stringify!(PastelYellow).to_lowercase() => Some(Color::PastelYellow),
                x if x == stringify!(PeaGreen).to_lowercase() => Some(Color::PeaGreen),
                x if x == stringify!(Peach).to_lowercase() => Some(Color::Peach),
                x if x == stringify!(PeachPink).to_lowercase() => Some(Color::PeachPink),
                x if x == stringify!(PeachPuff).to_lowercase() => Some(Color::PeachPuff),
                x if x == stringify!(Pearl).to_lowercase() => Some(Color::Pearl),
                x if x == stringify!(PearlWhite).to_lowercase() => Some(Color::PearlWhite),
                x if x == stringify!(Periwinkle).to_lowercase() => Some(Color::Periwinkle),
                x if x == stringify!(PeriwinklePink).to_lowercase() => Some(Color::PeriwinklePink),
                x if x == stringify!(PeriwinklePurple).to_lowercase() => {
                    Some(Color::PeriwinklePurple)
                }
                x if x == stringify!(Peru).to_lowercase() => Some(Color::Peru),
                x if x == stringify!(PetraGold).to_lowercase() => Some(Color::PetraGold),
                x if x == stringify!(PigPink).to_lowercase() => Some(Color::PigPink),
                x if x == stringify!(PineGreen).to_lowercase() => Some(Color::PineGreen),
                x if x == stringify!(Pink).to_lowercase() => Some(Color::Pink),
                x if x == stringify!(PinkBrown).to_lowercase() => Some(Color::PinkBrown),
                x if x == stringify!(PinkBubbleGum).to_lowercase() => Some(Color::PinkBubbleGum),
                x if x == stringify!(PinkCoral).to_lowercase() => Some(Color::PinkCoral),
                x if x == stringify!(PinkCupcake).to_lowercase() => Some(Color::PinkCupcake),
                x if x == stringify!(PinkDaisy).to_lowercase() => Some(Color::PinkDaisy),
                x if x == stringify!(PinkLemonade).to_lowercase() => Some(Color::PinkLemonade),
                x if x == stringify!(PinkOrange).to_lowercase() => Some(Color::PinkOrange),
                x if x == stringify!(PinkPlum).to_lowercase() => Some(Color::PinkPlum),
                x if x == stringify!(PinkViolet).to_lowercase() => Some(Color::PinkViolet),
                x if x == stringify!(PistachioGreen).to_lowercase() => Some(Color::PistachioGreen),
                x if x == stringify!(Platinum).to_lowercase() => Some(Color::Platinum),
                x if x == stringify!(PlatinumGray).to_lowercase() => Some(Color::PlatinumGray),
                x if x == stringify!(PlatinumSilver).to_lowercase() => Some(Color::PlatinumSilver),
                x if x == stringify!(Plum).to_lowercase() => Some(Color::Plum),
                x if x == stringify!(PlumPie).to_lowercase() => Some(Color::PlumPie),
                x if x == stringify!(PlumPurple).to_lowercase() => Some(Color::PlumPurple),
                x if x == stringify!(PlumVelvet).to_lowercase() => Some(Color::PlumVelvet),
                x if x == stringify!(PowderPink).to_lowercase() => Some(Color::PowderPink),
                x if x == stringify!(PowderBlue).to_lowercase() => Some(Color::PowderBlue),
                x if x == stringify!(Puce).to_lowercase() => Some(Color::Puce),
                x if x == stringify!(PullmanBrown).to_lowercase() => Some(Color::PullmanBrown),
                x if x == stringify!(PumpkinOrange).to_lowercase() => Some(Color::PumpkinOrange),
                x if x == stringify!(PumpkinPie).to_lowercase() => Some(Color::PumpkinPie),
                x if x == stringify!(Purple).to_lowercase() => Some(Color::Purple),
                x if x == stringify!(PurpleAmethyst).to_lowercase() => Some(Color::PurpleAmethyst),
                x if x == stringify!(PurpleDaffodil).to_lowercase() => Some(Color::PurpleDaffodil),
                x if x == stringify!(PurpleDragon).to_lowercase() => Some(Color::PurpleDragon),
                x if x == stringify!(PurpleFlower).to_lowercase() => Some(Color::PurpleFlower),
                x if x == stringify!(PurpleHaze).to_lowercase() => Some(Color::PurpleHaze),
                x if x == stringify!(PurpleIris).to_lowercase() => Some(Color::PurpleIris),
                x if x == stringify!(PurpleJam).to_lowercase() => Some(Color::PurpleJam),
                x if x == stringify!(PurpleLily).to_lowercase() => Some(Color::PurpleLily),
                x if x == stringify!(PurpleMaroon).to_lowercase() => Some(Color::PurpleMaroon),
                x if x == stringify!(PurpleMimosa).to_lowercase() => Some(Color::PurpleMimosa),
                x if x == stringify!(PurpleMonster).to_lowercase() => Some(Color::PurpleMonster),
                x if x == stringify!(PurpleNavy).to_lowercase() => Some(Color::PurpleNavy),
                x if x == stringify!(PurplePink).to_lowercase() => Some(Color::PurplePink),
                x if x == stringify!(PurplePlum).to_lowercase() => Some(Color::PurplePlum),
                x if x == stringify!(PurpleSageBush).to_lowercase() => Some(Color::PurpleSageBush),
                x if x == stringify!(PurpleThistle).to_lowercase() => Some(Color::PurpleThistle),
                x if x == stringify!(PurpleViolet).to_lowercase() => Some(Color::PurpleViolet),
                x if x == stringify!(PurpleWhite).to_lowercase() => Some(Color::PurpleWhite),
                x if x == stringify!(RacingGreen).to_lowercase() => Some(Color::RacingGreen),
                x if x == stringify!(Raspberry).to_lowercase() => Some(Color::Raspberry),
                x if x == stringify!(RaspberryPurple).to_lowercase() => {
                    Some(Color::RaspberryPurple)
                }
                x if x == stringify!(RatGray).to_lowercase() => Some(Color::RatGray),
                x if x == stringify!(RebeccaPurple).to_lowercase() => Some(Color::RebeccaPurple),
                x if x == stringify!(Red).to_lowercase() => Some(Color::Red),
                x if x == stringify!(RedBlood).to_lowercase() => Some(Color::RedBlood),
                x if x == stringify!(RedBrown).to_lowercase() => Some(Color::RedBrown),
                x if x == stringify!(RedDirt).to_lowercase() => Some(Color::RedDirt),
                x if x == stringify!(RedFox).to_lowercase() => Some(Color::RedFox),
                x if x == stringify!(RedGold).to_lowercase() => Some(Color::RedGold),
                x if x == stringify!(RedMagenta).to_lowercase() => Some(Color::RedMagenta),
                x if x == stringify!(RedPink).to_lowercase() => Some(Color::RedPink),
                x if x == stringify!(RedWhite).to_lowercase() => Some(Color::RedWhite),
                x if x == stringify!(RedWine).to_lowercase() => Some(Color::RedWine),
                x if x == stringify!(Rice).to_lowercase() => Some(Color::Rice),
                x if x == stringify!(RichLilac).to_lowercase() => Some(Color::RichLilac),
                x if x == stringify!(RobinEggBlue).to_lowercase() => Some(Color::RobinEggBlue),
                x if x == stringify!(RoguePink).to_lowercase() => Some(Color::RoguePink),
                x if x == stringify!(RomanSilver).to_lowercase() => Some(Color::RomanSilver),
                x if x == stringify!(Rose).to_lowercase() => Some(Color::Rose),
                x if x == stringify!(RoseDust).to_lowercase() => Some(Color::RoseDust),
                x if x == stringify!(RoseGold).to_lowercase() => Some(Color::RoseGold),
                x if x == stringify!(RosePink).to_lowercase() => Some(Color::RosePink),
                x if x == stringify!(RosePurple).to_lowercase() => Some(Color::RosePurple),
                x if x == stringify!(RoseQuartz).to_lowercase() => Some(Color::RoseQuartz),
                x if x == stringify!(RoseRed).to_lowercase() => Some(Color::RoseRed),
                x if x == stringify!(Rosy).to_lowercase() => Some(Color::Rosy),
                x if x == stringify!(RosyFinch).to_lowercase() => Some(Color::RosyFinch),
                x if x == stringify!(RosyPink).to_lowercase() => Some(Color::RosyPink),
                x if x == stringify!(RosyBrown).to_lowercase() => Some(Color::RosyBrown),
                x if x == stringify!(RoyalPink).to_lowercase() => Some(Color::RoyalPink),
                x if x == stringify!(RoyalBlue).to_lowercase() => Some(Color::RoyalBlue),
                x if x == stringify!(RubberDuckyYellow).to_lowercase() => {
                    Some(Color::RubberDuckyYellow)
                }
                x if x == stringify!(RubyRed).to_lowercase() => Some(Color::RubyRed),
                x if x == stringify!(Rust).to_lowercase() => Some(Color::Rust),
                x if x == stringify!(SaddleBrown).to_lowercase() => Some(Color::SaddleBrown),
                x if x == stringify!(SafetyOrange).to_lowercase() => Some(Color::SafetyOrange),
                x if x == stringify!(SafetyYellow).to_lowercase() => Some(Color::SafetyYellow),
                x if x == stringify!(Saffron).to_lowercase() => Some(Color::Saffron),
                x if x == stringify!(SaffronRed).to_lowercase() => Some(Color::SaffronRed),
                x if x == stringify!(Sage).to_lowercase() => Some(Color::Sage),
                x if x == stringify!(SageGreen).to_lowercase() => Some(Color::SageGreen),
                x if x == stringify!(SaladGreen).to_lowercase() => Some(Color::SaladGreen),
                x if x == stringify!(Salmon).to_lowercase() => Some(Color::Salmon),
                x if x == stringify!(SalmonPink).to_lowercase() => Some(Color::SalmonPink),
                x if x == stringify!(SamcoBlue).to_lowercase() => Some(Color::SamcoBlue),
                x if x == stringify!(Sand).to_lowercase() => Some(Color::Sand),
                x if x == stringify!(Sandstone).to_lowercase() => Some(Color::Sandstone),
                x if x == stringify!(SandyBrown).to_lowercase() => Some(Color::SandyBrown),
                x if x == stringify!(Sangria).to_lowercase() => Some(Color::Sangria),
                x if x == stringify!(SapphireBlue).to_lowercase() => Some(Color::SapphireBlue),
                x if x == stringify!(ScarletRed).to_lowercase() => Some(Color::ScarletRed),
                x if x == stringify!(SchoolBusYellow).to_lowercase() => {
                    Some(Color::SchoolBusYellow)
                }
                x if x == stringify!(SeaBlue).to_lowercase() => Some(Color::SeaBlue),
                x if x == stringify!(SeaTurtleGreen).to_lowercase() => Some(Color::SeaTurtleGreen),
                x if x == stringify!(SeaGreen).to_lowercase() => Some(Color::SeaGreen),
                x if x == stringify!(SeaShell).to_lowercase() => Some(Color::SeaShell),
                x if x == stringify!(SeafoamGreen).to_lowercase() => Some(Color::SeafoamGreen),
                x if x == stringify!(SeaweedGreen).to_lowercase() => Some(Color::SeaweedGreen),
                x if x == stringify!(Sedona).to_lowercase() => Some(Color::Sedona),
                x if x == stringify!(Sepia).to_lowercase() => Some(Color::Sepia),
                x if x == stringify!(SepiaBrown).to_lowercase() => Some(Color::SepiaBrown),
                x if x == stringify!(ShamrockGreen).to_lowercase() => Some(Color::ShamrockGreen),
                x if x == stringify!(SheetMetal).to_lowercase() => Some(Color::SheetMetal),
                x if x == stringify!(ShockingOrange).to_lowercase() => Some(Color::ShockingOrange),
                x if x == stringify!(Sienna).to_lowercase() => Some(Color::Sienna),
                x if x == stringify!(SilkBlue).to_lowercase() => Some(Color::SilkBlue),
                x if x == stringify!(Silver).to_lowercase() => Some(Color::Silver),
                x if x == stringify!(SilverPink).to_lowercase() => Some(Color::SilverPink),
                x if x == stringify!(SilverWhite).to_lowercase() => Some(Color::SilverWhite),
                x if x == stringify!(SkyBlueDress).to_lowercase() => Some(Color::SkyBlueDress),
                x if x == stringify!(SkyBlue).to_lowercase() => Some(Color::SkyBlue),
                x if x == stringify!(SlateBlueGray).to_lowercase() => Some(Color::SlateBlueGray),
                x if x == stringify!(SlateGraniteGray).to_lowercase() => {
                    Some(Color::SlateGraniteGray)
                }
                x if x == stringify!(SlateBlue).to_lowercase() => Some(Color::SlateBlue),
                x if x == stringify!(SlateGray).to_lowercase() => Some(Color::SlateGray),
                x if x == stringify!(SlateGrey).to_lowercase() => Some(Color::SlateGrey),
                x if x == stringify!(SlimeGreen).to_lowercase() => Some(Color::SlimeGreen),
                x if x == stringify!(SmokeyGray).to_lowercase() => Some(Color::SmokeyGray),
                x if x == stringify!(Snow).to_lowercase() => Some(Color::Snow),
                x if x == stringify!(SoftHazel).to_lowercase() => Some(Color::SoftHazel),
                x if x == stringify!(SoftIvory).to_lowercase() => Some(Color::SoftIvory),
                x if x == stringify!(SoftPink).to_lowercase() => Some(Color::SoftPink),
                x if x == stringify!(SonicSilver).to_lowercase() => Some(Color::SonicSilver),
                x if x == stringify!(SpringGreen).to_lowercase() => Some(Color::SpringGreen),
                x if x == stringify!(StainlessSteelGray).to_lowercase() => {
                    Some(Color::StainlessSteelGray)
                }
                x if x == stringify!(Steampunk).to_lowercase() => Some(Color::Steampunk),
                x if x == stringify!(SteelGray).to_lowercase() => Some(Color::SteelGray),
                x if x == stringify!(SteelBlue).to_lowercase() => Some(Color::SteelBlue),
                x if x == stringify!(StoplightGoGreen).to_lowercase() => {
                    Some(Color::StoplightGoGreen)
                }
                x if x == stringify!(StormyGray).to_lowercase() => Some(Color::StormyGray),
                x if x == stringify!(StrawberryRed).to_lowercase() => Some(Color::StrawberryRed),
                x if x == stringify!(SunYellow).to_lowercase() => Some(Color::SunYellow),
                x if x == stringify!(SunriseOrange).to_lowercase() => Some(Color::SunriseOrange),
                x if x == stringify!(Tan).to_lowercase() => Some(Color::Tan),
                x if x == stringify!(TanBrown).to_lowercase() => Some(Color::TanBrown),
                x if x == stringify!(Tangerine).to_lowercase() => Some(Color::Tangerine),
                x if x == stringify!(Taupe).to_lowercase() => Some(Color::Taupe),
                x if x == stringify!(TeaGreen).to_lowercase() => Some(Color::TeaGreen),
                x if x == stringify!(Teal).to_lowercase() => Some(Color::Teal),
                x if x == stringify!(TealBlue).to_lowercase() => Some(Color::TealBlue),
                x if x == stringify!(TealGreen).to_lowercase() => Some(Color::TealGreen),
                x if x == stringify!(Thistle).to_lowercase() => Some(Color::Thistle),
                x if x == stringify!(TiffanyBlue).to_lowercase() => Some(Color::TiffanyBlue),
                x if x == stringify!(TigerOrange).to_lowercase() => Some(Color::TigerOrange),
                x if x == stringify!(Tomato).to_lowercase() => Some(Color::Tomato),
                x if x == stringify!(TomatoSauceRed).to_lowercase() => Some(Color::TomatoSauceRed),
                x if x == stringify!(TronBlue).to_lowercase() => Some(Color::TronBlue),
                x if x == stringify!(TulipPink).to_lowercase() => Some(Color::TulipPink),
                x if x == stringify!(Turquoise).to_lowercase() => Some(Color::Turquoise),
                x if x == stringify!(TurquoiseGreen).to_lowercase() => Some(Color::TurquoiseGreen),
                x if x == stringify!(TyrianPurple).to_lowercase() => Some(Color::TyrianPurple),
                x if x == stringify!(UnbleachedSilk).to_lowercase() => Some(Color::UnbleachedSilk),
                x if x == stringify!(ValentineRed).to_lowercase() => Some(Color::ValentineRed),
                x if x == stringify!(VampireGray).to_lowercase() => Some(Color::VampireGray),
                x if x == stringify!(Vanilla).to_lowercase() => Some(Color::Vanilla),
                x if x == stringify!(VelvetMaroon).to_lowercase() => Some(Color::VelvetMaroon),
                x if x == stringify!(VenomGreen).to_lowercase() => Some(Color::VenomGreen),
                x if x == stringify!(Vermilion).to_lowercase() => Some(Color::Vermilion),
                x if x == stringify!(VeryPeri).to_lowercase() => Some(Color::VeryPeri),
                x if x == stringify!(Viola).to_lowercase() => Some(Color::Viola),
                x if x == stringify!(ViolaPurple).to_lowercase() => Some(Color::ViolaPurple),
                x if x == stringify!(Violet).to_lowercase() => Some(Color::Violet),
                x if x == stringify!(VioletRed).to_lowercase() => Some(Color::VioletRed),
                x if x == stringify!(WarmPink).to_lowercase() => Some(Color::WarmPink),
                x if x == stringify!(WarmWhite).to_lowercase() => Some(Color::WarmWhite),
                x if x == stringify!(Water).to_lowercase() => Some(Color::Water),
                x if x == stringify!(WatermelonPink).to_lowercase() => Some(Color::WatermelonPink),
                x if x == stringify!(WesternCharcoal).to_lowercase() => {
                    Some(Color::WesternCharcoal)
                }
                x if x == stringify!(Wheat).to_lowercase() => Some(Color::Wheat),
                x if x == stringify!(White).to_lowercase() => Some(Color::White),
                x if x == stringify!(WhiteBlue).to_lowercase() => Some(Color::WhiteBlue),
                x if x == stringify!(WhiteChocolate).to_lowercase() => Some(Color::WhiteChocolate),
                x if x == stringify!(WhiteGold).to_lowercase() => Some(Color::WhiteGold),
                x if x == stringify!(WhiteGray).to_lowercase() => Some(Color::WhiteGray),
                x if x == stringify!(WhiteIce).to_lowercase() => Some(Color::WhiteIce),
                x if x == stringify!(WhiteYellow).to_lowercase() => Some(Color::WhiteYellow),
                x if x == stringify!(WhiteSmoke).to_lowercase() => Some(Color::WhiteSmoke),
                x if x == stringify!(WindowsBlue).to_lowercase() => Some(Color::WindowsBlue),
                x if x == stringify!(WineRed).to_lowercase() => Some(Color::WineRed),
                x if x == stringify!(WisteriaPurple).to_lowercase() => Some(Color::WisteriaPurple),
                x if x == stringify!(Wood).to_lowercase() => Some(Color::Wood),
                x if x == stringify!(Yellow).to_lowercase() => Some(Color::Yellow),
                x if x == stringify!(YellowGreenGrosbeak).to_lowercase() => {
                    Some(Color::YellowGreenGrosbeak)
                }
                x if x == stringify!(YellowLawnGreen).to_lowercase() => {
                    Some(Color::YellowLawnGreen)
                }
                x if x == stringify!(YellowOrange).to_lowercase() => Some(Color::YellowOrange),
                x if x == stringify!(YellowGreen).to_lowercase() => Some(Color::YellowGreen),
                x if x == stringify!(ZombieGreen).to_lowercase() => Some(Color::ZombieGreen),
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
            Self::AcidGreen => "#B0BF1A",
            Self::AlgaeGreen => "#64E986",
            Self::AliceBlue => "#F0F8FF",
            Self::AlienGray => "#736F6E",
            Self::AlienGreen => "#6CC417",
            Self::AloeVeraGreen => "#98F516",
            Self::Amber => "#FFBF00",
            Self::AntiqueBronze => "#665D1E",
            Self::AntiqueWhite => "#FAEBD7",
            Self::Aqua => "#00FFFF",
            Self::AquaGreen => "#12E193",
            Self::AquaSeafoamGreen => "#93E9BE",
            Self::Aquamarine => "#7FFFD4",
            Self::AquamarineStone => "#348781",
            Self::ArmyBrown => "#827B60",
            Self::ArmyGreen => "#4B5320",
            Self::AshGray => "#666362",
            Self::AshWhite => "#E9E4D4",
            Self::AvocadoGreen => "#B2C248",
            Self::AztechPurple => "#893BFF",
            Self::Azure => "#F0FFFF",
            Self::AzureBlue => "#4863A0",
            Self::BabyBlue => "#95B9C7",
            Self::BabyPink => "#FAAFBA",
            Self::BakersBrown => "#5C3317",
            Self::BalloonBlue => "#2B60DE",
            Self::BananaYellow => "#F5E216",
            Self::BarbiePink => "#DA1884",
            Self::BashfulPink => "#C25283",
            Self::BasilGreen => "#829F82",
            Self::BasketBallOrange => "#F88158",
            Self::BattleshipGray => "#848482",
            Self::BeanRed => "#F75D59",
            Self::BeeYellow => "#E9AB17",
            Self::Beer => "#FBB117",
            Self::BeetleGreen => "#4C787E",
            Self::Beige => "#F5F5DC",
            Self::Bisque => "#FFE4C4",
            Self::Black => "#000000",
            Self::BlackBean => "#3D0C02",
            Self::BlackBlue => "#040720",
            Self::BlackCat => "#413839",
            Self::BlackCow => "#4C4646",
            Self::BlackEel => "#463E3F",
            Self::BlanchedAlmond => "#FFEBCD",
            Self::Blonde => "#FBF6D9",
            Self::BloodNight => "#551606",
            Self::BloodRed => "#7E3517",
            Self::BlossomPink => "#F9B7FF",
            Self::Blue => "#0000FF",
            Self::BlueAngel => "#B7CEEC",
            Self::BlueDiamond => "#4EE2EC",
            Self::BlueDress => "#157DEC",
            Self::BlueEyes => "#1569C7",
            Self::BlueGray => "#98AFC7",
            Self::BlueGreen => "#7BCCB5",
            Self::BlueHosta => "#77BFC7",
            Self::BlueIvy => "#3090C7",
            Self::BlueJay => "#2B547E",
            Self::BlueKoi => "#659EC7",
            Self::BlueLagoon => "#8EEBEC",
            Self::BlueLotus => "#6960EC",
            Self::BlueMagenta => "#822EFF",
            Self::BlueMossGreen => "#3C565B",
            Self::BlueOrchid => "#1F45FC",
            Self::BlueRibbon => "#306EFF",
            Self::BlueTurquoise => "#43C6DB",
            Self::BlueWhale => "#342D7E",
            Self::BlueZircon => "#57FEFF",
            Self::BlueViolet => "#8A2BE2",
            Self::BlueberryBlue => "#0041C2",
            Self::Blurple => "#5865F2",
            Self::Blush => "#FFE6E8",
            Self::BlushPink => "#E6A9EC",
            Self::BlushRed => "#E56E94",
            Self::BoldYellow => "#F9DB24",
            Self::BoneWhite => "#F9F6EE",
            Self::BottleGreen => "#006A4E",
            Self::Brass => "#B5A642",
            Self::BrightBlue => "#0909FF",
            Self::BrightCyan => "#0AFFFF",
            Self::BrightGold => "#FDD017",
            Self::BrightGrape => "#6F2DA8",
            Self::BrightGreen => "#66FF00",
            Self::BrightLilac => "#D891EF",
            Self::BrightMaroon => "#C32148",
            Self::BrightNavyBlue => "#1974D2",
            Self::BrightNeonPink => "#F433FF",
            Self::BrightOrange => "#FF5F1F",
            Self::BrightPink => "#FF007F",
            Self::BrightPurple => "#6A0DAD",
            Self::BrightTeal => "#01F9C6",
            Self::BrightTurquoise => "#16E2F5",
            Self::BroccoliGreen => "#026C3D",
            Self::Bronze => "#CD7F32",
            Self::BronzeGold => "#C9AE5D",
            Self::Brown => "#A52A2A",
            Self::BrownBear => "#835C3B",
            Self::BrownRust => "#A55D35",
            Self::BrownSand => "#EE9A4D",
            Self::BrownSugar => "#E2A76F",
            Self::BulletShell => "#AF9B60",
            Self::Burgundy => "#8C001A",
            Self::BurlyWood => "#DEB887",
            Self::BurntPink => "#C12267",
            Self::ButterflyBlue => "#38ACEC",
            Self::CactusGreen => "#227442",
            Self::CadetBlue => "#5F9EA0",
            Self::CadillacPink => "#E38AAE",
            Self::CamelBrown => "#C19A6B",
            Self::CamouflageGreen => "#78866B",
            Self::CanaryBlue => "#2916F5",
            Self::CanaryYellow => "#FFEF00",
            Self::Cantaloupe => "#FFA62F",
            Self::Caramel => "#C68E17",
            Self::CarbonGray => "#625D5D",
            Self::CarbonRed => "#A70D2A",
            Self::CardboardBrown => "#EDDA74",
            Self::CarnationPink => "#F778A1",
            Self::CarrotOrange => "#F88017",
            Self::Celeste => "#50EBEC",
            Self::ChameleonGreen => "#BDF516",
            Self::Champagne => "#F7E7CE",
            Self::Charcoal => "#34282C",
            Self::CharcoalBlue => "#36454F",
            Self::Chartreuse => "#7FFF00",
            Self::CheeseOrange => "#FFA600",
            Self::CherryRed => "#C24641",
            Self::Chestnut => "#954535",
            Self::ChestnutRed => "#C34A2C",
            Self::ChilliPepper => "#C11B17",
            Self::Chocolate => "#D2691E",
            Self::ChocolateBrown => "#3F000F",
            Self::ChromeAluminum => "#A8A9AD",
            Self::ChromeGold => "#FFCE44",
            Self::ChromeGreen => "#1AA260",
            Self::ChromePink => "#FF33AA",
            Self::ChromeWhite => "#E8F1D4",
            Self::Cinnamon => "#C58917",
            Self::CitronGreen => "#8FB31D",
            Self::ClematisViolet => "#842DCE",
            Self::CloudyGray => "#6D6968",
            Self::CloverGreen => "#3EA055",
            Self::CobaltBlue => "#0020C2",
            Self::Coffee => "#6F4E37",
            Self::ColdMetal => "#9B9A96",
            Self::ColumbiaBlue => "#87AFC7",
            Self::ConstructionConeOrange => "#F87431",
            Self::CookieBrown => "#C7A317",
            Self::Copper => "#B87333",
            Self::CopperRed => "#CB6D51",
            Self::Coral => "#FF7F50",
            Self::CoralBlue => "#AFDCEC",
            Self::CoralBrown => "#9E4638",
            Self::CoralPeach => "#FBD5AB",
            Self::CornYellow => "#FFF380",
            Self::CornflowerBlue => "#6495ED",
            Self::Cornsilk => "#FFF8DC",
            Self::CosmicLatte => "#FFF8E7",
            Self::Cotton => "#FBFBF9",
            Self::CottonCandy => "#FCDFFF",
            Self::Cranberry => "#9F000F",
            Self::Cream => "#FFFFCC",
            Self::CreamWhite => "#FFFDD0",
            Self::CreamyWhite => "#F0E9D6",
            Self::Crimson => "#DC143C",
            Self::CrimsonPurple => "#E238EC",
            Self::CrimsonRed => "#990000",
            Self::CrocusPurple => "#9172EC",
            Self::CrystalBlue => "#5CB3FF",
            Self::Cyan => "#00FFFF",
            Self::CyanBlue => "#14A3C7",
            Self::CyanOpaque => "#92C7C7",
            Self::DarkAlmond => "#AB784E",
            Self::DarkBeige => "#9F8C76",
            Self::DarkBisque => "#B86500",
            Self::DarkBlonde => "#F0E2B6",
            Self::DarkBlueGray => "#29465B",
            Self::DarkBlurple => "#5539CC",
            Self::DarkBronze => "#804A00",
            Self::DarkBrown => "#654321",
            Self::DarkCarnationPink => "#C12283",
            Self::DarkCoffee => "#3B2F2F",
            Self::DarkForestGreen => "#254117",
            Self::DarkGainsboro => "#8C8C8C",
            Self::DarkGold => "#AA6C39",
            Self::DarkGrayishOlive => "#4A412A",
            Self::DarkGreenBlue => "#1F6357",
            Self::DarkHazelBrown => "#473810",
            Self::DarkHotPink => "#F660AB",
            Self::DarkLimeGreen => "#41A317",
            Self::DarkMaroon => "#2F0909",
            Self::DarkMint => "#31906E",
            Self::DarkMoccasin => "#827839",
            Self::DarkPink => "#E75480",
            Self::DarkPurple => "#4B0150",
            Self::DarkRaspberry => "#872657",
            Self::DarkScarlet => "#560319",
            Self::DarkSienna => "#8A4117",
            Self::DarkSkyBlue => "#0059FF",
            Self::DarkSlate => "#2B3856",
            Self::DarkSteampunk => "#4D4D4F",
            Self::DarkTeal => "#045D5D",
            Self::DarkWhite => "#E1D9D1",
            Self::DarkYellow => "#8B8000",
            Self::DarkBlue => "#00008B",
            Self::DarkCyan => "#008B8B",
            Self::DarkGoldenRod => "#B8860B",
            Self::DarkGray => "#A9A9A9",
            Self::DarkGreen => "#006400",
            Self::DarkGrey => "#A9A9A9",
            Self::DarkKhaki => "#BDB76B",
            Self::DarkMagenta => "#8B008B",
            Self::DarkOliveGreen => "#556B2F",
            Self::DarkOrange => "#FF8C00",
            Self::DarkOrchid => "#9932CC",
            Self::DarkRed => "#8B0000",
            Self::DarkSalmon => "#E9967A",
            Self::DarkSeaGreen => "#8FBC8F",
            Self::DarkSlateBlue => "#483D8B",
            Self::DarkSlateGray => "#25383C",
            Self::DarkSlateGrey => "#25383C",
            Self::DarkTurquoise => "#00CED1",
            Self::DarkViolet => "#9400D3",
            Self::DaySkyBlue => "#82CAFF",
            Self::DeepAmber => "#A05544",
            Self::DeepEmeraldGreen => "#046307",
            Self::DeepGreen => "#056608",
            Self::DeepMauve => "#DF73D4",
            Self::DeepPeach => "#FFCBA4",
            Self::DeepPeriwinkle => "#5453A6",
            Self::DeepPurple => "#36013F",
            Self::DeepRed => "#800517",
            Self::DeepRose => "#FBBBB9",
            Self::DeepSea => "#3B9C9C",
            Self::DeepSeaBlue => "#123456",
            Self::DeepSeaGreen => "#306754",
            Self::DeepTeal => "#033E3E",
            Self::DeepTurquoise => "#48CCCD",
            Self::DeepYellow => "#F6BE00",
            Self::DeepPink => "#FF1493",
            Self::DeepSkyBlue => "#00BFFF",
            Self::DeerBrown => "#E6BF83",
            Self::DenimBlue => "#79BAEC",
            Self::DenimDarkBlue => "#151B8D",
            Self::DesertSand => "#EDC9AF",
            Self::DimGray => "#696969",
            Self::DimGrey => "#696969",
            Self::DimorphothecaMagenta => "#E3319D",
            Self::DinosaurGreen => "#73A16C",
            Self::DirtyWhite => "#E8E4C9",
            Self::DodgerBlue => "#1E90FF",
            Self::DollarBillGreen => "#85BB65",
            Self::DonutPink => "#FAAFBE",
            Self::DragonGreen => "#6AFB92",
            Self::DullGreenYellow => "#B1FB17",
            Self::DullPurple => "#7F525D",
            Self::DullSeaGreen => "#4E8975",
            Self::DuskyPink => "#CC7A8B",
            Self::DustyPink => "#D58A94",
            Self::DustyRose => "#C9A9A6",
            Self::EarthBlue => "#0000A5",
            Self::EarthGreen => "#34A56F",
            Self::Ebony => "#555D50",
            Self::EggShell => "#FFF9E3",
            Self::Eggplant => "#614051",
            Self::ElectricBlue => "#9AFEFF",
            Self::ElfGreen => "#1B8A6B",
            Self::Emerald => "#50C878",
            Self::EmeraldGreen => "#5FFB17",
            Self::EstorilBlue => "#2F539B",
            Self::FallForestGreen => "#4E9258",
            Self::FallLeafBrown => "#C8B560",
            Self::FernGreen => "#667C26",
            Self::FerrariRed => "#F70D1A",
            Self::FireEngineRed => "#F62817",
            Self::FireBrick => "#B22222",
            Self::FlamingoPink => "#F9A7B0",
            Self::FloralWhite => "#FFFAF0",
            Self::FluroOrange => "#FE632A",
            Self::ForestGreen => "#228B22",
            Self::FrenchLilac => "#86608E",
            Self::FrogGreen => "#99C68E",
            Self::Fuchsia => "#FF00FF",
            Self::FuchsiaPink => "#FF77FF",
            Self::Gainsboro => "#DCDCDC",
            Self::GarnetRed => "#733635",
            Self::GearSteelGray => "#C0C6C7",
            Self::GhostWhite => "#F8F8FF",
            Self::GingerBrown => "#C9BE62",
            Self::GingerRed => "#B83C08",
            Self::GlacialBlueIce => "#368BC1",
            Self::Gold => "#FFD700",
            Self::GoldPink => "#E6C7C2",
            Self::GoldenBlonde => "#FBE7A1",
            Self::GoldenBrown => "#EAC117",
            Self::GoldenSilk => "#F3E3C3",
            Self::GoldenYellow => "#FFDF00",
            Self::GoldenRod => "#DAA520",
            Self::Granite => "#837E7C",
            Self::Grape => "#5E5A80",
            Self::Grapefruit => "#DC381F",
            Self::GrassGreen => "#3F9B0B",
            Self::Gray => "#808080",
            Self::GrayBrown => "#3D3635",
            Self::GrayCloud => "#B6B6B4",
            Self::GrayDolphin => "#5C5858",
            Self::GrayGoose => "#D1D0CE",
            Self::GrayGreen => "#A2AD9C",
            Self::GrayWolf => "#504A4B",
            Self::GrayishTurquoise => "#5E7D7E",
            Self::Green => "#008000",
            Self::GreenApple => "#4CC417",
            Self::GreenLeaves => "#3A5F0B",
            Self::GreenOnion => "#6AA121",
            Self::GreenPeas => "#89C35C",
            Self::GreenPepper => "#4AA02C",
            Self::GreenSnake => "#6CBB3C",
            Self::GreenThumb => "#B5EAAA",
            Self::GreenYellow => "#ADFF2F",
            Self::GreenishBlue => "#307D7E",
            Self::Grey => "#808080",
            Self::GulfBlue => "#C9DFEC",
            Self::Gunmetal => "#2C3539",
            Self::GunmetalGray => "#8D918D",
            Self::HalfWhite => "#FFFEFA",
            Self::HalloweenOrange => "#E66C2C",
            Self::HarvestGold => "#EDE275",
            Self::Hazel => "#8E7618",
            Self::HazelGreen => "#617C58",
            Self::HeavenlyBlue => "#C6DEFF",
            Self::HeliotropePurple => "#D462FF",
            Self::HoneyDew => "#F0FFF0",
            Self::HotDeepPink => "#F52887",
            Self::HotPink => "#FF69B4",
            Self::HummingbirdGreen => "#7FE817",
            Self::HunterGreen => "#355E3B",
            Self::Iceberg => "#56A5EC",
            Self::IguanaGreen => "#9CB071",
            Self::IndianSaffron => "#FF7722",
            Self::IndianRed => "#CD5C5C",
            Self::Indigo => "#4B0082",
            Self::Iridium => "#3D3C3A",
            Self::IrishGreen => "#08A04B",
            Self::IronGray => "#52595D",
            Self::IsleOfManGreen => "#22CE83",
            Self::Ivory => "#FFFFF0",
            Self::Jade => "#00A36C",
            Self::JadeGreen => "#5EFB6E",
            Self::JasminePurple => "#A23BEC",
            Self::JeansBlue => "#A0CFEC",
            Self::Jellyfish => "#46C7C7",
            Self::JetGray => "#616D7E",
            Self::JungleGreen => "#347C2C",
            Self::KellyGreen => "#4CC552",
            Self::Khaki => "#F0E68C",
            Self::KhakiBrown => "#906E3E",
            Self::KhakiGreen => "#8A865D",
            Self::KhakiRose => "#C5908E",
            Self::LapisBlue => "#15317E",
            Self::LavaRed => "#E42217",
            Self::Lavender => "#E6E6FA",
            Self::LavenderBlue => "#E3E4FA",
            Self::LavenderPinocchio => "#EBDDE2",
            Self::LavenderPurple => "#967BB6",
            Self::LavenderBlush => "#FFF0F5",
            Self::LawnGreen => "#7CFC00",
            Self::LemonGreen => "#ADF802",
            Self::LemonYellow => "#FEF250",
            Self::LemonChiffon => "#FFFACD",
            Self::LightAquamarine => "#93FFE8",
            Self::LightBeige => "#FFF0DB",
            Self::LightBlack => "#454545",
            Self::LightBrown => "#B5651D",
            Self::LightCopper => "#DA8A67",
            Self::LightDayBlue => "#ADDFFF",
            Self::LightFrenchBeige => "#C8AD7F",
            Self::LightGold => "#F1E5AC",
            Self::LightJade => "#C3FDB8",
            Self::LightMintGreen => "#C2E5D3",
            Self::LightOliveGreen => "#B8BC86",
            Self::LightOrange => "#FED8B1",
            Self::LightPurple => "#8467D7",
            Self::LightPurpleBlue => "#728FCE",
            Self::LightRed => "#FFCCCB",
            Self::LightRose => "#FBCFCD",
            Self::LightRoseGreen => "#DBF9DB",
            Self::LightSalmonRose => "#F9966B",
            Self::LightSlate => "#CCFFFF",
            Self::LightSlateBlue => "#736AFF",
            Self::LightSteelGray => "#E0E5E5",
            Self::LightTeal => "#B3D9D9",
            Self::LightWhite => "#FFFFF7",
            Self::LightBlue => "#ADD8E6",
            Self::LightCoral => "#F08080",
            Self::LightCyan => "#E0FFFF",
            Self::LightGoldenRodYellow => "#FAFAD2",
            Self::LightGray => "#D3D3D3",
            Self::LightGreen => "#90EE90",
            Self::LightGrey => "#D3D3D3",
            Self::LightPink => "#FFB6C1",
            Self::LightSalmon => "#FFA07A",
            Self::LightSeaGreen => "#20B2AA",
            Self::LightSkyBlue => "#87CEFA",
            Self::LightSlateGray => "#778899",
            Self::LightSlateGrey => "#778899",
            Self::LightSteelBlue => "#B0CFDE",
            Self::LightYellow => "#FFFFE0",
            Self::Lilac => "#C8A2C8",
            Self::Lime => "#00FF00",
            Self::LimeMintGreen => "#36F57F",
            Self::LimeGreen => "#32CD32",
            Self::Linen => "#FAF0E6",
            Self::LipstickPink => "#C48793",
            Self::LotusGreen => "#004225",
            Self::LoveRed => "#E41B17",
            Self::LovelyPurple => "#7F38EC",
            Self::MacaroniandCheese => "#F2BB66",
            Self::MacawBlueGreen => "#43BFC7",
            Self::Magenta => "#FF00FF",
            Self::MagentaPink => "#CC338B",
            Self::MagicMint => "#AAF0D1",
            Self::Mahogany => "#C04000",
            Self::MangoOrange => "#FF8040",
            Self::MarbleBlue => "#566D7E",
            Self::Maroon => "#800000",
            Self::MaroonRed => "#8F0B0B",
            Self::Mauve => "#E0B0FF",
            Self::MauveTaupe => "#915F6D",
            Self::MediumForestGreen => "#347235",
            Self::MediumTeal => "#045F5F",
            Self::MediumAquaMarine => "#66CDAA",
            Self::MediumBlue => "#0000CD",
            Self::MediumOrchid => "#BA55D3",
            Self::MediumPurple => "#9370DB",
            Self::MediumSeaGreen => "#3CB371",
            Self::MediumSlateBlue => "#7B68EE",
            Self::MediumSpringGreen => "#00FA9A",
            Self::MediumTurquoise => "#48D1CC",
            Self::MediumVioletRed => "#C71585",
            Self::Metal => "#B6B6B6",
            Self::MetallicBronze => "#A97142",
            Self::MetallicGold => "#D4AF37",
            Self::MetallicGreen => "#7C9D8E",
            Self::MetallicSilver => "#BCC6CC",
            Self::MiddayBlue => "#3BB9FF",
            Self::Midnight => "#2B1B17",
            Self::MidnightPurple => "#2E1A47",
            Self::MidnightBlue => "#191970",
            Self::MilitaryGreen => "#4E5B31",
            Self::MilkChocolate => "#513B1C",
            Self::MilkWhite => "#FEFCFF",
            Self::MillenniumJade => "#93917C",
            Self::Mint => "#3EB489",
            Self::MintGreen => "#98FF98",
            Self::MintCream => "#F5FFFA",
            Self::MistBlue => "#646D7E",
            Self::MistyRose => "#FFE4E1",
            Self::Moccasin => "#FFE4B5",
            Self::Mocha => "#493D26",
            Self::MossGreen => "#8A9A5B",
            Self::Mustard => "#E1AD01",
            Self::MustardYellow => "#FFDB58",
            Self::NardoGray => "#686A6C",
            Self::NavajoWhite => "#FFDEAD",
            Self::Navy => "#000080",
            Self::NebulaGreen => "#59E817",
            Self::NeonBlue => "#1589FF",
            Self::NeonGold => "#FDBD01",
            Self::NeonGreen => "#16F529",
            Self::NeonHotPink => "#FD349C",
            Self::NeonOrange => "#FF6700",
            Self::NeonPink => "#F535AA",
            Self::NeonPurple => "#9D00FF",
            Self::NeonRed => "#FD1C03",
            Self::NeonYellow => "#FFFF33",
            Self::NeonYellowGreen => "#DAEE01",
            Self::NewMidnightBlue => "#0000A0",
            Self::Night => "#0C090A",
            Self::NightBlue => "#151B54",
            Self::NorthernLightsBlue => "#78C7C7",
            Self::OakBrown => "#806517",
            Self::OceanBlue => "#2B65EC",
            Self::OceanGreen => "#00FF80",
            Self::OffWhite => "#F8F0E3",
            Self::Oil => "#3B3131",
            Self::OldBurgundy => "#43302E",
            Self::OldRose => "#C08081",
            Self::OldLace => "#FEF0E3",
            Self::Olive => "#808000",
            Self::OliveGreen => "#BAB86C",
            Self::OliveDrab => "#6B8E23",
            Self::Orange => "#FFA500",
            Self::OrangeGold => "#D4A017",
            Self::OrangeSalmon => "#C47451",
            Self::OrangeYellow => "#FFAE42",
            Self::OrangeRed => "#FF4500",
            Self::Orchid => "#DA70D6",
            Self::OrchidPurple => "#B048B5",
            Self::OrganicBrown => "#E3F9A6",
            Self::PaleBlueLily => "#CFECEC",
            Self::PaleLilac => "#DCD0FF",
            Self::PalePink => "#F2D4D7",
            Self::PaleSilver => "#C9C0BB",
            Self::PaleGoldenRod => "#EEE8AA",
            Self::PaleGreen => "#98FB98",
            Self::PaleTurquoise => "#AFEEEE",
            Self::PaleVioletRed => "#DB7093",
            Self::PapayaOrange => "#E56717",
            Self::PapayaWhip => "#FFEFD5",
            Self::Parchment => "#FFFFC2",
            Self::ParrotGreen => "#12AD2B",
            Self::PastelBlue => "#B4CFEC",
            Self::PastelBrown => "#B1907F",
            Self::PastelGreen => "#77DD77",
            Self::PastelIndigo => "#8686AF",
            Self::PastelLightBlue => "#D5D6EA",
            Self::PastelOrange => "#F8B88B",
            Self::PastelPink => "#FEA3AA",
            Self::PastelPurple => "#F2A2E8",
            Self::PastelRed => "#F67280",
            Self::PastelRose => "#E5788F",
            Self::PastelViolet => "#D291BC",
            Self::PastelYellow => "#FAF884",
            Self::PeaGreen => "#52D017",
            Self::Peach => "#FFE5B4",
            Self::PeachPink => "#F98B88",
            Self::PeachPuff => "#FFDAB9",
            Self::Pearl => "#FDEEF4",
            Self::PearlWhite => "#F8F6F0",
            Self::Periwinkle => "#CCCCFF",
            Self::PeriwinklePink => "#E9CFEC",
            Self::PeriwinklePurple => "#7575CF",
            Self::Peru => "#CD853F",
            Self::PetraGold => "#B76734",
            Self::PigPink => "#FDD7E4",
            Self::PineGreen => "#387C44",
            Self::Pink => "#FFC0CB",
            Self::PinkBrown => "#C48189",
            Self::PinkBubbleGum => "#FFDFDD",
            Self::PinkCoral => "#E77471",
            Self::PinkCupcake => "#E45E9D",
            Self::PinkDaisy => "#E799A3",
            Self::PinkLemonade => "#E4287C",
            Self::PinkOrange => "#F89880",
            Self::PinkPlum => "#B93B8F",
            Self::PinkViolet => "#CA226B",
            Self::PistachioGreen => "#9DC209",
            Self::Platinum => "#E5E4E2",
            Self::PlatinumGray => "#797979",
            Self::PlatinumSilver => "#CECECE",
            Self::Plum => "#DDA0DD",
            Self::PlumPie => "#7D0541",
            Self::PlumPurple => "#583759",
            Self::PlumVelvet => "#7D0552",
            Self::PowderPink => "#FFB2D0",
            Self::PowderBlue => "#B0E0E6",
            Self::Puce => "#7F5A58",
            Self::PullmanBrown => "#644117",
            Self::PumpkinOrange => "#F87217",
            Self::PumpkinPie => "#CA762B",
            Self::Purple => "#800080",
            Self::PurpleAmethyst => "#6C2DC7",
            Self::PurpleDaffodil => "#B041FF",
            Self::PurpleDragon => "#C38EC7",
            Self::PurpleFlower => "#A74AC7",
            Self::PurpleHaze => "#4E387E",
            Self::PurpleIris => "#571B7E",
            Self::PurpleJam => "#6A287E",
            Self::PurpleLily => "#550A35",
            Self::PurpleMaroon => "#810541",
            Self::PurpleMimosa => "#9E7BFF",
            Self::PurpleMonster => "#461B7E",
            Self::PurpleNavy => "#4E5180",
            Self::PurplePink => "#D16587",
            Self::PurplePlum => "#8E35EF",
            Self::PurpleSageBush => "#7A5DC7",
            Self::PurpleThistle => "#D2B9D3",
            Self::PurpleViolet => "#8D38C9",
            Self::PurpleWhite => "#DFD3E3",
            Self::RacingGreen => "#27742C",
            Self::Raspberry => "#E30B5D",
            Self::RaspberryPurple => "#B3446C",
            Self::RatGray => "#6D7B8D",
            Self::RebeccaPurple => "#663399",
            Self::Red => "#FF0000",
            Self::RedBlood => "#660000",
            Self::RedBrown => "#622F22",
            Self::RedDirt => "#7F5217",
            Self::RedFox => "#C35817",
            Self::RedGold => "#EB5406",
            Self::RedMagenta => "#FF0080",
            Self::RedPink => "#FA2A55",
            Self::RedWhite => "#F3E8EA",
            Self::RedWine => "#990012",
            Self::Rice => "#FAF5EF",
            Self::RichLilac => "#B666D2",
            Self::RobinEggBlue => "#BDEDFF",
            Self::RoguePink => "#C12869",
            Self::RomanSilver => "#838996",
            Self::Rose => "#E8ADAA",
            Self::RoseDust => "#997070",
            Self::RoseGold => "#ECC5C0",
            Self::RosePink => "#E7A1B0",
            Self::RosePurple => "#B09FCA",
            Self::RoseQuartz => "#F7CAC9",
            Self::RoseRed => "#C21E56",
            Self::Rosy => "#A17188",
            Self::RosyFinch => "#7F4E52",
            Self::RosyPink => "#B38481",
            Self::RosyBrown => "#BC8F8F",
            Self::RoyalPink => "#E759AC",
            Self::RoyalBlue => "#4169E1",
            Self::RubberDuckyYellow => "#FFD801",
            Self::RubyRed => "#F62217",
            Self::Rust => "#C36241",
            Self::SaddleBrown => "#8B4513",
            Self::SafetyOrange => "#FF7900",
            Self::SafetyYellow => "#EED202",
            Self::Saffron => "#FBB917",
            Self::SaffronRed => "#931314",
            Self::Sage => "#BCB88A",
            Self::SageGreen => "#848B79",
            Self::SaladGreen => "#A1C935",
            Self::Salmon => "#FA8072",
            Self::SalmonPink => "#FF8674",
            Self::SamcoBlue => "#0002FF",
            Self::Sand => "#C2B280",
            Self::Sandstone => "#786D5F",
            Self::SandyBrown => "#F4A460",
            Self::Sangria => "#7E3817",
            Self::SapphireBlue => "#2554C7",
            Self::ScarletRed => "#FF2400",
            Self::SchoolBusYellow => "#E8A317",
            Self::SeaBlue => "#C2DFFF",
            Self::SeaTurtleGreen => "#438D80",
            Self::SeaGreen => "#2E8B57",
            Self::SeaShell => "#FFF5EE",
            Self::SeafoamGreen => "#3EA99F",
            Self::SeaweedGreen => "#437C17",
            Self::Sedona => "#CC6600",
            Self::Sepia => "#7F462C",
            Self::SepiaBrown => "#704214",
            Self::ShamrockGreen => "#347C17",
            Self::SheetMetal => "#888B90",
            Self::ShockingOrange => "#E55B3C",
            Self::Sienna => "#A0522D",
            Self::SilkBlue => "#488AC7",
            Self::Silver => "#C0C0C0",
            Self::SilverPink => "#C4AEAD",
            Self::SilverWhite => "#DADBDD",
            Self::SkyBlueDress => "#6698FF",
            Self::SkyBlue => "#87CEEB",
            Self::SlateBlueGray => "#737CA1",
            Self::SlateGraniteGray => "#657383",
            Self::SlateBlue => "#6A5ACD",
            Self::SlateGray => "#708090",
            Self::SlateGrey => "#708090",
            Self::SlimeGreen => "#BCE954",
            Self::SmokeyGray => "#726E6D",
            Self::Snow => "#FFFAFA",
            Self::SoftHazel => "#C6BA8B",
            Self::SoftIvory => "#FAF0DD",
            Self::SoftPink => "#FFB8BF",
            Self::SonicSilver => "#757575",
            Self::SpringGreen => "#00FF7F",
            Self::StainlessSteelGray => "#99A3A3",
            Self::Steampunk => "#C9C1C1",
            Self::SteelGray => "#71797E",
            Self::SteelBlue => "#4682B4",
            Self::StoplightGoGreen => "#57E964",
            Self::StormyGray => "#3A3B3C",
            Self::StrawberryRed => "#C83F49",
            Self::SunYellow => "#FFE87C",
            Self::SunriseOrange => "#E67451",
            Self::Tan => "#D2B48C",
            Self::TanBrown => "#ECE5B6",
            Self::Tangerine => "#E78A61",
            Self::Taupe => "#483C32",
            Self::TeaGreen => "#CCFB5D",
            Self::Teal => "#008080",
            Self::TealBlue => "#007C80",
            Self::TealGreen => "#00827F",
            Self::Thistle => "#D8BFD8",
            Self::TiffanyBlue => "#81D8D0",
            Self::TigerOrange => "#C88141",
            Self::Tomato => "#FF6347",
            Self::TomatoSauceRed => "#B21807",
            Self::TronBlue => "#7DFDFE",
            Self::TulipPink => "#C25A7C",
            Self::Turquoise => "#40E0D0",
            Self::TurquoiseGreen => "#A0D6B4",
            Self::TyrianPurple => "#C45AEC",
            Self::UnbleachedSilk => "#FFDDCA",
            Self::ValentineRed => "#E55451",
            Self::VampireGray => "#565051",
            Self::Vanilla => "#F3E5AB",
            Self::VelvetMaroon => "#7E354D",
            Self::VenomGreen => "#728C00",
            Self::Vermilion => "#7E191B",
            Self::VeryPeri => "#6667AB",
            Self::Viola => "#C8C4DF",
            Self::ViolaPurple => "#7E587E",
            Self::Violet => "#EE82EE",
            Self::VioletRed => "#F6358A",
            Self::WarmPink => "#F6C6BD",
            Self::WarmWhite => "#EFEBD8",
            Self::Water => "#EBF4FA",
            Self::WatermelonPink => "#FC6C85",
            Self::WesternCharcoal => "#49413F",
            Self::Wheat => "#F5DEB3",
            Self::White => "#FFFFFF",
            Self::WhiteBlue => "#DBE9FA",
            Self::WhiteChocolate => "#EDE6D6",
            Self::WhiteGold => "#FFFFF4",
            Self::WhiteGray => "#EEEEEE",
            Self::WhiteIce => "#EAEEE9",
            Self::WhiteYellow => "#F2F0DF",
            Self::WhiteSmoke => "#F5F5F5",
            Self::WindowsBlue => "#357EC7",
            Self::WineRed => "#990012",
            Self::WisteriaPurple => "#C6AEC7",
            Self::Wood => "#966F33",
            Self::Yellow => "#FFFF00",
            Self::YellowGreenGrosbeak => "#E2F516",
            Self::YellowLawnGreen => "#87F717",
            Self::YellowOrange => "#FFAE42",
            Self::YellowGreen => "#9ACD32",
            Self::ZombieGreen => "#54C571",
            Self::Rgb(r, g, b) => return format!("#{:02X}{:02X}{:02X}", r, g, b),
        }
        .to_string()
    }
    #[doc = r" Get the name of the color as a string"]
    pub fn name(&self) -> String {
        match self {
            Self::AcidGreen => stringify!(AcidGreen),
            Self::AlgaeGreen => stringify!(AlgaeGreen),
            Self::AliceBlue => stringify!(AliceBlue),
            Self::AlienGray => stringify!(AlienGray),
            Self::AlienGreen => stringify!(AlienGreen),
            Self::AloeVeraGreen => stringify!(AloeVeraGreen),
            Self::Amber => stringify!(Amber),
            Self::AntiqueBronze => stringify!(AntiqueBronze),
            Self::AntiqueWhite => stringify!(AntiqueWhite),
            Self::Aqua => stringify!(Aqua),
            Self::AquaGreen => stringify!(AquaGreen),
            Self::AquaSeafoamGreen => stringify!(AquaSeafoamGreen),
            Self::Aquamarine => stringify!(Aquamarine),
            Self::AquamarineStone => stringify!(AquamarineStone),
            Self::ArmyBrown => stringify!(ArmyBrown),
            Self::ArmyGreen => stringify!(ArmyGreen),
            Self::AshGray => stringify!(AshGray),
            Self::AshWhite => stringify!(AshWhite),
            Self::AvocadoGreen => stringify!(AvocadoGreen),
            Self::AztechPurple => stringify!(AztechPurple),
            Self::Azure => stringify!(Azure),
            Self::AzureBlue => stringify!(AzureBlue),
            Self::BabyBlue => stringify!(BabyBlue),
            Self::BabyPink => stringify!(BabyPink),
            Self::BakersBrown => stringify!(BakersBrown),
            Self::BalloonBlue => stringify!(BalloonBlue),
            Self::BananaYellow => stringify!(BananaYellow),
            Self::BarbiePink => stringify!(BarbiePink),
            Self::BashfulPink => stringify!(BashfulPink),
            Self::BasilGreen => stringify!(BasilGreen),
            Self::BasketBallOrange => stringify!(BasketBallOrange),
            Self::BattleshipGray => stringify!(BattleshipGray),
            Self::BeanRed => stringify!(BeanRed),
            Self::BeeYellow => stringify!(BeeYellow),
            Self::Beer => stringify!(Beer),
            Self::BeetleGreen => stringify!(BeetleGreen),
            Self::Beige => stringify!(Beige),
            Self::Bisque => stringify!(Bisque),
            Self::Black => stringify!(Black),
            Self::BlackBean => stringify!(BlackBean),
            Self::BlackBlue => stringify!(BlackBlue),
            Self::BlackCat => stringify!(BlackCat),
            Self::BlackCow => stringify!(BlackCow),
            Self::BlackEel => stringify!(BlackEel),
            Self::BlanchedAlmond => stringify!(BlanchedAlmond),
            Self::Blonde => stringify!(Blonde),
            Self::BloodNight => stringify!(BloodNight),
            Self::BloodRed => stringify!(BloodRed),
            Self::BlossomPink => stringify!(BlossomPink),
            Self::Blue => stringify!(Blue),
            Self::BlueAngel => stringify!(BlueAngel),
            Self::BlueDiamond => stringify!(BlueDiamond),
            Self::BlueDress => stringify!(BlueDress),
            Self::BlueEyes => stringify!(BlueEyes),
            Self::BlueGray => stringify!(BlueGray),
            Self::BlueGreen => stringify!(BlueGreen),
            Self::BlueHosta => stringify!(BlueHosta),
            Self::BlueIvy => stringify!(BlueIvy),
            Self::BlueJay => stringify!(BlueJay),
            Self::BlueKoi => stringify!(BlueKoi),
            Self::BlueLagoon => stringify!(BlueLagoon),
            Self::BlueLotus => stringify!(BlueLotus),
            Self::BlueMagenta => stringify!(BlueMagenta),
            Self::BlueMossGreen => stringify!(BlueMossGreen),
            Self::BlueOrchid => stringify!(BlueOrchid),
            Self::BlueRibbon => stringify!(BlueRibbon),
            Self::BlueTurquoise => stringify!(BlueTurquoise),
            Self::BlueWhale => stringify!(BlueWhale),
            Self::BlueZircon => stringify!(BlueZircon),
            Self::BlueViolet => stringify!(BlueViolet),
            Self::BlueberryBlue => stringify!(BlueberryBlue),
            Self::Blurple => stringify!(Blurple),
            Self::Blush => stringify!(Blush),
            Self::BlushPink => stringify!(BlushPink),
            Self::BlushRed => stringify!(BlushRed),
            Self::BoldYellow => stringify!(BoldYellow),
            Self::BoneWhite => stringify!(BoneWhite),
            Self::BottleGreen => stringify!(BottleGreen),
            Self::Brass => stringify!(Brass),
            Self::BrightBlue => stringify!(BrightBlue),
            Self::BrightCyan => stringify!(BrightCyan),
            Self::BrightGold => stringify!(BrightGold),
            Self::BrightGrape => stringify!(BrightGrape),
            Self::BrightGreen => stringify!(BrightGreen),
            Self::BrightLilac => stringify!(BrightLilac),
            Self::BrightMaroon => stringify!(BrightMaroon),
            Self::BrightNavyBlue => stringify!(BrightNavyBlue),
            Self::BrightNeonPink => stringify!(BrightNeonPink),
            Self::BrightOrange => stringify!(BrightOrange),
            Self::BrightPink => stringify!(BrightPink),
            Self::BrightPurple => stringify!(BrightPurple),
            Self::BrightTeal => stringify!(BrightTeal),
            Self::BrightTurquoise => stringify!(BrightTurquoise),
            Self::BroccoliGreen => stringify!(BroccoliGreen),
            Self::Bronze => stringify!(Bronze),
            Self::BronzeGold => stringify!(BronzeGold),
            Self::Brown => stringify!(Brown),
            Self::BrownBear => stringify!(BrownBear),
            Self::BrownRust => stringify!(BrownRust),
            Self::BrownSand => stringify!(BrownSand),
            Self::BrownSugar => stringify!(BrownSugar),
            Self::BulletShell => stringify!(BulletShell),
            Self::Burgundy => stringify!(Burgundy),
            Self::BurlyWood => stringify!(BurlyWood),
            Self::BurntPink => stringify!(BurntPink),
            Self::ButterflyBlue => stringify!(ButterflyBlue),
            Self::CactusGreen => stringify!(CactusGreen),
            Self::CadetBlue => stringify!(CadetBlue),
            Self::CadillacPink => stringify!(CadillacPink),
            Self::CamelBrown => stringify!(CamelBrown),
            Self::CamouflageGreen => stringify!(CamouflageGreen),
            Self::CanaryBlue => stringify!(CanaryBlue),
            Self::CanaryYellow => stringify!(CanaryYellow),
            Self::Cantaloupe => stringify!(Cantaloupe),
            Self::Caramel => stringify!(Caramel),
            Self::CarbonGray => stringify!(CarbonGray),
            Self::CarbonRed => stringify!(CarbonRed),
            Self::CardboardBrown => stringify!(CardboardBrown),
            Self::CarnationPink => stringify!(CarnationPink),
            Self::CarrotOrange => stringify!(CarrotOrange),
            Self::Celeste => stringify!(Celeste),
            Self::ChameleonGreen => stringify!(ChameleonGreen),
            Self::Champagne => stringify!(Champagne),
            Self::Charcoal => stringify!(Charcoal),
            Self::CharcoalBlue => stringify!(CharcoalBlue),
            Self::Chartreuse => stringify!(Chartreuse),
            Self::CheeseOrange => stringify!(CheeseOrange),
            Self::CherryRed => stringify!(CherryRed),
            Self::Chestnut => stringify!(Chestnut),
            Self::ChestnutRed => stringify!(ChestnutRed),
            Self::ChilliPepper => stringify!(ChilliPepper),
            Self::Chocolate => stringify!(Chocolate),
            Self::ChocolateBrown => stringify!(ChocolateBrown),
            Self::ChromeAluminum => stringify!(ChromeAluminum),
            Self::ChromeGold => stringify!(ChromeGold),
            Self::ChromeGreen => stringify!(ChromeGreen),
            Self::ChromePink => stringify!(ChromePink),
            Self::ChromeWhite => stringify!(ChromeWhite),
            Self::Cinnamon => stringify!(Cinnamon),
            Self::CitronGreen => stringify!(CitronGreen),
            Self::ClematisViolet => stringify!(ClematisViolet),
            Self::CloudyGray => stringify!(CloudyGray),
            Self::CloverGreen => stringify!(CloverGreen),
            Self::CobaltBlue => stringify!(CobaltBlue),
            Self::Coffee => stringify!(Coffee),
            Self::ColdMetal => stringify!(ColdMetal),
            Self::ColumbiaBlue => stringify!(ColumbiaBlue),
            Self::ConstructionConeOrange => stringify!(ConstructionConeOrange),
            Self::CookieBrown => stringify!(CookieBrown),
            Self::Copper => stringify!(Copper),
            Self::CopperRed => stringify!(CopperRed),
            Self::Coral => stringify!(Coral),
            Self::CoralBlue => stringify!(CoralBlue),
            Self::CoralBrown => stringify!(CoralBrown),
            Self::CoralPeach => stringify!(CoralPeach),
            Self::CornYellow => stringify!(CornYellow),
            Self::CornflowerBlue => stringify!(CornflowerBlue),
            Self::Cornsilk => stringify!(Cornsilk),
            Self::CosmicLatte => stringify!(CosmicLatte),
            Self::Cotton => stringify!(Cotton),
            Self::CottonCandy => stringify!(CottonCandy),
            Self::Cranberry => stringify!(Cranberry),
            Self::Cream => stringify!(Cream),
            Self::CreamWhite => stringify!(CreamWhite),
            Self::CreamyWhite => stringify!(CreamyWhite),
            Self::Crimson => stringify!(Crimson),
            Self::CrimsonPurple => stringify!(CrimsonPurple),
            Self::CrimsonRed => stringify!(CrimsonRed),
            Self::CrocusPurple => stringify!(CrocusPurple),
            Self::CrystalBlue => stringify!(CrystalBlue),
            Self::Cyan => stringify!(Cyan),
            Self::CyanBlue => stringify!(CyanBlue),
            Self::CyanOpaque => stringify!(CyanOpaque),
            Self::DarkAlmond => stringify!(DarkAlmond),
            Self::DarkBeige => stringify!(DarkBeige),
            Self::DarkBisque => stringify!(DarkBisque),
            Self::DarkBlonde => stringify!(DarkBlonde),
            Self::DarkBlueGray => stringify!(DarkBlueGray),
            Self::DarkBlurple => stringify!(DarkBlurple),
            Self::DarkBronze => stringify!(DarkBronze),
            Self::DarkBrown => stringify!(DarkBrown),
            Self::DarkCarnationPink => stringify!(DarkCarnationPink),
            Self::DarkCoffee => stringify!(DarkCoffee),
            Self::DarkForestGreen => stringify!(DarkForestGreen),
            Self::DarkGainsboro => stringify!(DarkGainsboro),
            Self::DarkGold => stringify!(DarkGold),
            Self::DarkGrayishOlive => stringify!(DarkGrayishOlive),
            Self::DarkGreenBlue => stringify!(DarkGreenBlue),
            Self::DarkHazelBrown => stringify!(DarkHazelBrown),
            Self::DarkHotPink => stringify!(DarkHotPink),
            Self::DarkLimeGreen => stringify!(DarkLimeGreen),
            Self::DarkMaroon => stringify!(DarkMaroon),
            Self::DarkMint => stringify!(DarkMint),
            Self::DarkMoccasin => stringify!(DarkMoccasin),
            Self::DarkPink => stringify!(DarkPink),
            Self::DarkPurple => stringify!(DarkPurple),
            Self::DarkRaspberry => stringify!(DarkRaspberry),
            Self::DarkScarlet => stringify!(DarkScarlet),
            Self::DarkSienna => stringify!(DarkSienna),
            Self::DarkSkyBlue => stringify!(DarkSkyBlue),
            Self::DarkSlate => stringify!(DarkSlate),
            Self::DarkSteampunk => stringify!(DarkSteampunk),
            Self::DarkTeal => stringify!(DarkTeal),
            Self::DarkWhite => stringify!(DarkWhite),
            Self::DarkYellow => stringify!(DarkYellow),
            Self::DarkBlue => stringify!(DarkBlue),
            Self::DarkCyan => stringify!(DarkCyan),
            Self::DarkGoldenRod => stringify!(DarkGoldenRod),
            Self::DarkGray => stringify!(DarkGray),
            Self::DarkGreen => stringify!(DarkGreen),
            Self::DarkGrey => stringify!(DarkGrey),
            Self::DarkKhaki => stringify!(DarkKhaki),
            Self::DarkMagenta => stringify!(DarkMagenta),
            Self::DarkOliveGreen => stringify!(DarkOliveGreen),
            Self::DarkOrange => stringify!(DarkOrange),
            Self::DarkOrchid => stringify!(DarkOrchid),
            Self::DarkRed => stringify!(DarkRed),
            Self::DarkSalmon => stringify!(DarkSalmon),
            Self::DarkSeaGreen => stringify!(DarkSeaGreen),
            Self::DarkSlateBlue => stringify!(DarkSlateBlue),
            Self::DarkSlateGray => stringify!(DarkSlateGray),
            Self::DarkSlateGrey => stringify!(DarkSlateGrey),
            Self::DarkTurquoise => stringify!(DarkTurquoise),
            Self::DarkViolet => stringify!(DarkViolet),
            Self::DaySkyBlue => stringify!(DaySkyBlue),
            Self::DeepAmber => stringify!(DeepAmber),
            Self::DeepEmeraldGreen => stringify!(DeepEmeraldGreen),
            Self::DeepGreen => stringify!(DeepGreen),
            Self::DeepMauve => stringify!(DeepMauve),
            Self::DeepPeach => stringify!(DeepPeach),
            Self::DeepPeriwinkle => stringify!(DeepPeriwinkle),
            Self::DeepPurple => stringify!(DeepPurple),
            Self::DeepRed => stringify!(DeepRed),
            Self::DeepRose => stringify!(DeepRose),
            Self::DeepSea => stringify!(DeepSea),
            Self::DeepSeaBlue => stringify!(DeepSeaBlue),
            Self::DeepSeaGreen => stringify!(DeepSeaGreen),
            Self::DeepTeal => stringify!(DeepTeal),
            Self::DeepTurquoise => stringify!(DeepTurquoise),
            Self::DeepYellow => stringify!(DeepYellow),
            Self::DeepPink => stringify!(DeepPink),
            Self::DeepSkyBlue => stringify!(DeepSkyBlue),
            Self::DeerBrown => stringify!(DeerBrown),
            Self::DenimBlue => stringify!(DenimBlue),
            Self::DenimDarkBlue => stringify!(DenimDarkBlue),
            Self::DesertSand => stringify!(DesertSand),
            Self::DimGray => stringify!(DimGray),
            Self::DimGrey => stringify!(DimGrey),
            Self::DimorphothecaMagenta => stringify!(DimorphothecaMagenta),
            Self::DinosaurGreen => stringify!(DinosaurGreen),
            Self::DirtyWhite => stringify!(DirtyWhite),
            Self::DodgerBlue => stringify!(DodgerBlue),
            Self::DollarBillGreen => stringify!(DollarBillGreen),
            Self::DonutPink => stringify!(DonutPink),
            Self::DragonGreen => stringify!(DragonGreen),
            Self::DullGreenYellow => stringify!(DullGreenYellow),
            Self::DullPurple => stringify!(DullPurple),
            Self::DullSeaGreen => stringify!(DullSeaGreen),
            Self::DuskyPink => stringify!(DuskyPink),
            Self::DustyPink => stringify!(DustyPink),
            Self::DustyRose => stringify!(DustyRose),
            Self::EarthBlue => stringify!(EarthBlue),
            Self::EarthGreen => stringify!(EarthGreen),
            Self::Ebony => stringify!(Ebony),
            Self::EggShell => stringify!(EggShell),
            Self::Eggplant => stringify!(Eggplant),
            Self::ElectricBlue => stringify!(ElectricBlue),
            Self::ElfGreen => stringify!(ElfGreen),
            Self::Emerald => stringify!(Emerald),
            Self::EmeraldGreen => stringify!(EmeraldGreen),
            Self::EstorilBlue => stringify!(EstorilBlue),
            Self::FallForestGreen => stringify!(FallForestGreen),
            Self::FallLeafBrown => stringify!(FallLeafBrown),
            Self::FernGreen => stringify!(FernGreen),
            Self::FerrariRed => stringify!(FerrariRed),
            Self::FireEngineRed => stringify!(FireEngineRed),
            Self::FireBrick => stringify!(FireBrick),
            Self::FlamingoPink => stringify!(FlamingoPink),
            Self::FloralWhite => stringify!(FloralWhite),
            Self::FluroOrange => stringify!(FluroOrange),
            Self::ForestGreen => stringify!(ForestGreen),
            Self::FrenchLilac => stringify!(FrenchLilac),
            Self::FrogGreen => stringify!(FrogGreen),
            Self::Fuchsia => stringify!(Fuchsia),
            Self::FuchsiaPink => stringify!(FuchsiaPink),
            Self::Gainsboro => stringify!(Gainsboro),
            Self::GarnetRed => stringify!(GarnetRed),
            Self::GearSteelGray => stringify!(GearSteelGray),
            Self::GhostWhite => stringify!(GhostWhite),
            Self::GingerBrown => stringify!(GingerBrown),
            Self::GingerRed => stringify!(GingerRed),
            Self::GlacialBlueIce => stringify!(GlacialBlueIce),
            Self::Gold => stringify!(Gold),
            Self::GoldPink => stringify!(GoldPink),
            Self::GoldenBlonde => stringify!(GoldenBlonde),
            Self::GoldenBrown => stringify!(GoldenBrown),
            Self::GoldenSilk => stringify!(GoldenSilk),
            Self::GoldenYellow => stringify!(GoldenYellow),
            Self::GoldenRod => stringify!(GoldenRod),
            Self::Granite => stringify!(Granite),
            Self::Grape => stringify!(Grape),
            Self::Grapefruit => stringify!(Grapefruit),
            Self::GrassGreen => stringify!(GrassGreen),
            Self::Gray => stringify!(Gray),
            Self::GrayBrown => stringify!(GrayBrown),
            Self::GrayCloud => stringify!(GrayCloud),
            Self::GrayDolphin => stringify!(GrayDolphin),
            Self::GrayGoose => stringify!(GrayGoose),
            Self::GrayGreen => stringify!(GrayGreen),
            Self::GrayWolf => stringify!(GrayWolf),
            Self::GrayishTurquoise => stringify!(GrayishTurquoise),
            Self::Green => stringify!(Green),
            Self::GreenApple => stringify!(GreenApple),
            Self::GreenLeaves => stringify!(GreenLeaves),
            Self::GreenOnion => stringify!(GreenOnion),
            Self::GreenPeas => stringify!(GreenPeas),
            Self::GreenPepper => stringify!(GreenPepper),
            Self::GreenSnake => stringify!(GreenSnake),
            Self::GreenThumb => stringify!(GreenThumb),
            Self::GreenYellow => stringify!(GreenYellow),
            Self::GreenishBlue => stringify!(GreenishBlue),
            Self::Grey => stringify!(Grey),
            Self::GulfBlue => stringify!(GulfBlue),
            Self::Gunmetal => stringify!(Gunmetal),
            Self::GunmetalGray => stringify!(GunmetalGray),
            Self::HalfWhite => stringify!(HalfWhite),
            Self::HalloweenOrange => stringify!(HalloweenOrange),
            Self::HarvestGold => stringify!(HarvestGold),
            Self::Hazel => stringify!(Hazel),
            Self::HazelGreen => stringify!(HazelGreen),
            Self::HeavenlyBlue => stringify!(HeavenlyBlue),
            Self::HeliotropePurple => stringify!(HeliotropePurple),
            Self::HoneyDew => stringify!(HoneyDew),
            Self::HotDeepPink => stringify!(HotDeepPink),
            Self::HotPink => stringify!(HotPink),
            Self::HummingbirdGreen => stringify!(HummingbirdGreen),
            Self::HunterGreen => stringify!(HunterGreen),
            Self::Iceberg => stringify!(Iceberg),
            Self::IguanaGreen => stringify!(IguanaGreen),
            Self::IndianSaffron => stringify!(IndianSaffron),
            Self::IndianRed => stringify!(IndianRed),
            Self::Indigo => stringify!(Indigo),
            Self::Iridium => stringify!(Iridium),
            Self::IrishGreen => stringify!(IrishGreen),
            Self::IronGray => stringify!(IronGray),
            Self::IsleOfManGreen => stringify!(IsleOfManGreen),
            Self::Ivory => stringify!(Ivory),
            Self::Jade => stringify!(Jade),
            Self::JadeGreen => stringify!(JadeGreen),
            Self::JasminePurple => stringify!(JasminePurple),
            Self::JeansBlue => stringify!(JeansBlue),
            Self::Jellyfish => stringify!(Jellyfish),
            Self::JetGray => stringify!(JetGray),
            Self::JungleGreen => stringify!(JungleGreen),
            Self::KellyGreen => stringify!(KellyGreen),
            Self::Khaki => stringify!(Khaki),
            Self::KhakiBrown => stringify!(KhakiBrown),
            Self::KhakiGreen => stringify!(KhakiGreen),
            Self::KhakiRose => stringify!(KhakiRose),
            Self::LapisBlue => stringify!(LapisBlue),
            Self::LavaRed => stringify!(LavaRed),
            Self::Lavender => stringify!(Lavender),
            Self::LavenderBlue => stringify!(LavenderBlue),
            Self::LavenderPinocchio => stringify!(LavenderPinocchio),
            Self::LavenderPurple => stringify!(LavenderPurple),
            Self::LavenderBlush => stringify!(LavenderBlush),
            Self::LawnGreen => stringify!(LawnGreen),
            Self::LemonGreen => stringify!(LemonGreen),
            Self::LemonYellow => stringify!(LemonYellow),
            Self::LemonChiffon => stringify!(LemonChiffon),
            Self::LightAquamarine => stringify!(LightAquamarine),
            Self::LightBeige => stringify!(LightBeige),
            Self::LightBlack => stringify!(LightBlack),
            Self::LightBrown => stringify!(LightBrown),
            Self::LightCopper => stringify!(LightCopper),
            Self::LightDayBlue => stringify!(LightDayBlue),
            Self::LightFrenchBeige => stringify!(LightFrenchBeige),
            Self::LightGold => stringify!(LightGold),
            Self::LightJade => stringify!(LightJade),
            Self::LightMintGreen => stringify!(LightMintGreen),
            Self::LightOliveGreen => stringify!(LightOliveGreen),
            Self::LightOrange => stringify!(LightOrange),
            Self::LightPurple => stringify!(LightPurple),
            Self::LightPurpleBlue => stringify!(LightPurpleBlue),
            Self::LightRed => stringify!(LightRed),
            Self::LightRose => stringify!(LightRose),
            Self::LightRoseGreen => stringify!(LightRoseGreen),
            Self::LightSalmonRose => stringify!(LightSalmonRose),
            Self::LightSlate => stringify!(LightSlate),
            Self::LightSlateBlue => stringify!(LightSlateBlue),
            Self::LightSteelGray => stringify!(LightSteelGray),
            Self::LightTeal => stringify!(LightTeal),
            Self::LightWhite => stringify!(LightWhite),
            Self::LightBlue => stringify!(LightBlue),
            Self::LightCoral => stringify!(LightCoral),
            Self::LightCyan => stringify!(LightCyan),
            Self::LightGoldenRodYellow => stringify!(LightGoldenRodYellow),
            Self::LightGray => stringify!(LightGray),
            Self::LightGreen => stringify!(LightGreen),
            Self::LightGrey => stringify!(LightGrey),
            Self::LightPink => stringify!(LightPink),
            Self::LightSalmon => stringify!(LightSalmon),
            Self::LightSeaGreen => stringify!(LightSeaGreen),
            Self::LightSkyBlue => stringify!(LightSkyBlue),
            Self::LightSlateGray => stringify!(LightSlateGray),
            Self::LightSlateGrey => stringify!(LightSlateGrey),
            Self::LightSteelBlue => stringify!(LightSteelBlue),
            Self::LightYellow => stringify!(LightYellow),
            Self::Lilac => stringify!(Lilac),
            Self::Lime => stringify!(Lime),
            Self::LimeMintGreen => stringify!(LimeMintGreen),
            Self::LimeGreen => stringify!(LimeGreen),
            Self::Linen => stringify!(Linen),
            Self::LipstickPink => stringify!(LipstickPink),
            Self::LotusGreen => stringify!(LotusGreen),
            Self::LoveRed => stringify!(LoveRed),
            Self::LovelyPurple => stringify!(LovelyPurple),
            Self::MacaroniandCheese => stringify!(MacaroniandCheese),
            Self::MacawBlueGreen => stringify!(MacawBlueGreen),
            Self::Magenta => stringify!(Magenta),
            Self::MagentaPink => stringify!(MagentaPink),
            Self::MagicMint => stringify!(MagicMint),
            Self::Mahogany => stringify!(Mahogany),
            Self::MangoOrange => stringify!(MangoOrange),
            Self::MarbleBlue => stringify!(MarbleBlue),
            Self::Maroon => stringify!(Maroon),
            Self::MaroonRed => stringify!(MaroonRed),
            Self::Mauve => stringify!(Mauve),
            Self::MauveTaupe => stringify!(MauveTaupe),
            Self::MediumForestGreen => stringify!(MediumForestGreen),
            Self::MediumTeal => stringify!(MediumTeal),
            Self::MediumAquaMarine => stringify!(MediumAquaMarine),
            Self::MediumBlue => stringify!(MediumBlue),
            Self::MediumOrchid => stringify!(MediumOrchid),
            Self::MediumPurple => stringify!(MediumPurple),
            Self::MediumSeaGreen => stringify!(MediumSeaGreen),
            Self::MediumSlateBlue => stringify!(MediumSlateBlue),
            Self::MediumSpringGreen => stringify!(MediumSpringGreen),
            Self::MediumTurquoise => stringify!(MediumTurquoise),
            Self::MediumVioletRed => stringify!(MediumVioletRed),
            Self::Metal => stringify!(Metal),
            Self::MetallicBronze => stringify!(MetallicBronze),
            Self::MetallicGold => stringify!(MetallicGold),
            Self::MetallicGreen => stringify!(MetallicGreen),
            Self::MetallicSilver => stringify!(MetallicSilver),
            Self::MiddayBlue => stringify!(MiddayBlue),
            Self::Midnight => stringify!(Midnight),
            Self::MidnightPurple => stringify!(MidnightPurple),
            Self::MidnightBlue => stringify!(MidnightBlue),
            Self::MilitaryGreen => stringify!(MilitaryGreen),
            Self::MilkChocolate => stringify!(MilkChocolate),
            Self::MilkWhite => stringify!(MilkWhite),
            Self::MillenniumJade => stringify!(MillenniumJade),
            Self::Mint => stringify!(Mint),
            Self::MintGreen => stringify!(MintGreen),
            Self::MintCream => stringify!(MintCream),
            Self::MistBlue => stringify!(MistBlue),
            Self::MistyRose => stringify!(MistyRose),
            Self::Moccasin => stringify!(Moccasin),
            Self::Mocha => stringify!(Mocha),
            Self::MossGreen => stringify!(MossGreen),
            Self::Mustard => stringify!(Mustard),
            Self::MustardYellow => stringify!(MustardYellow),
            Self::NardoGray => stringify!(NardoGray),
            Self::NavajoWhite => stringify!(NavajoWhite),
            Self::Navy => stringify!(Navy),
            Self::NebulaGreen => stringify!(NebulaGreen),
            Self::NeonBlue => stringify!(NeonBlue),
            Self::NeonGold => stringify!(NeonGold),
            Self::NeonGreen => stringify!(NeonGreen),
            Self::NeonHotPink => stringify!(NeonHotPink),
            Self::NeonOrange => stringify!(NeonOrange),
            Self::NeonPink => stringify!(NeonPink),
            Self::NeonPurple => stringify!(NeonPurple),
            Self::NeonRed => stringify!(NeonRed),
            Self::NeonYellow => stringify!(NeonYellow),
            Self::NeonYellowGreen => stringify!(NeonYellowGreen),
            Self::NewMidnightBlue => stringify!(NewMidnightBlue),
            Self::Night => stringify!(Night),
            Self::NightBlue => stringify!(NightBlue),
            Self::NorthernLightsBlue => stringify!(NorthernLightsBlue),
            Self::OakBrown => stringify!(OakBrown),
            Self::OceanBlue => stringify!(OceanBlue),
            Self::OceanGreen => stringify!(OceanGreen),
            Self::OffWhite => stringify!(OffWhite),
            Self::Oil => stringify!(Oil),
            Self::OldBurgundy => stringify!(OldBurgundy),
            Self::OldRose => stringify!(OldRose),
            Self::OldLace => stringify!(OldLace),
            Self::Olive => stringify!(Olive),
            Self::OliveGreen => stringify!(OliveGreen),
            Self::OliveDrab => stringify!(OliveDrab),
            Self::Orange => stringify!(Orange),
            Self::OrangeGold => stringify!(OrangeGold),
            Self::OrangeSalmon => stringify!(OrangeSalmon),
            Self::OrangeYellow => stringify!(OrangeYellow),
            Self::OrangeRed => stringify!(OrangeRed),
            Self::Orchid => stringify!(Orchid),
            Self::OrchidPurple => stringify!(OrchidPurple),
            Self::OrganicBrown => stringify!(OrganicBrown),
            Self::PaleBlueLily => stringify!(PaleBlueLily),
            Self::PaleLilac => stringify!(PaleLilac),
            Self::PalePink => stringify!(PalePink),
            Self::PaleSilver => stringify!(PaleSilver),
            Self::PaleGoldenRod => stringify!(PaleGoldenRod),
            Self::PaleGreen => stringify!(PaleGreen),
            Self::PaleTurquoise => stringify!(PaleTurquoise),
            Self::PaleVioletRed => stringify!(PaleVioletRed),
            Self::PapayaOrange => stringify!(PapayaOrange),
            Self::PapayaWhip => stringify!(PapayaWhip),
            Self::Parchment => stringify!(Parchment),
            Self::ParrotGreen => stringify!(ParrotGreen),
            Self::PastelBlue => stringify!(PastelBlue),
            Self::PastelBrown => stringify!(PastelBrown),
            Self::PastelGreen => stringify!(PastelGreen),
            Self::PastelIndigo => stringify!(PastelIndigo),
            Self::PastelLightBlue => stringify!(PastelLightBlue),
            Self::PastelOrange => stringify!(PastelOrange),
            Self::PastelPink => stringify!(PastelPink),
            Self::PastelPurple => stringify!(PastelPurple),
            Self::PastelRed => stringify!(PastelRed),
            Self::PastelRose => stringify!(PastelRose),
            Self::PastelViolet => stringify!(PastelViolet),
            Self::PastelYellow => stringify!(PastelYellow),
            Self::PeaGreen => stringify!(PeaGreen),
            Self::Peach => stringify!(Peach),
            Self::PeachPink => stringify!(PeachPink),
            Self::PeachPuff => stringify!(PeachPuff),
            Self::Pearl => stringify!(Pearl),
            Self::PearlWhite => stringify!(PearlWhite),
            Self::Periwinkle => stringify!(Periwinkle),
            Self::PeriwinklePink => stringify!(PeriwinklePink),
            Self::PeriwinklePurple => stringify!(PeriwinklePurple),
            Self::Peru => stringify!(Peru),
            Self::PetraGold => stringify!(PetraGold),
            Self::PigPink => stringify!(PigPink),
            Self::PineGreen => stringify!(PineGreen),
            Self::Pink => stringify!(Pink),
            Self::PinkBrown => stringify!(PinkBrown),
            Self::PinkBubbleGum => stringify!(PinkBubbleGum),
            Self::PinkCoral => stringify!(PinkCoral),
            Self::PinkCupcake => stringify!(PinkCupcake),
            Self::PinkDaisy => stringify!(PinkDaisy),
            Self::PinkLemonade => stringify!(PinkLemonade),
            Self::PinkOrange => stringify!(PinkOrange),
            Self::PinkPlum => stringify!(PinkPlum),
            Self::PinkViolet => stringify!(PinkViolet),
            Self::PistachioGreen => stringify!(PistachioGreen),
            Self::Platinum => stringify!(Platinum),
            Self::PlatinumGray => stringify!(PlatinumGray),
            Self::PlatinumSilver => stringify!(PlatinumSilver),
            Self::Plum => stringify!(Plum),
            Self::PlumPie => stringify!(PlumPie),
            Self::PlumPurple => stringify!(PlumPurple),
            Self::PlumVelvet => stringify!(PlumVelvet),
            Self::PowderPink => stringify!(PowderPink),
            Self::PowderBlue => stringify!(PowderBlue),
            Self::Puce => stringify!(Puce),
            Self::PullmanBrown => stringify!(PullmanBrown),
            Self::PumpkinOrange => stringify!(PumpkinOrange),
            Self::PumpkinPie => stringify!(PumpkinPie),
            Self::Purple => stringify!(Purple),
            Self::PurpleAmethyst => stringify!(PurpleAmethyst),
            Self::PurpleDaffodil => stringify!(PurpleDaffodil),
            Self::PurpleDragon => stringify!(PurpleDragon),
            Self::PurpleFlower => stringify!(PurpleFlower),
            Self::PurpleHaze => stringify!(PurpleHaze),
            Self::PurpleIris => stringify!(PurpleIris),
            Self::PurpleJam => stringify!(PurpleJam),
            Self::PurpleLily => stringify!(PurpleLily),
            Self::PurpleMaroon => stringify!(PurpleMaroon),
            Self::PurpleMimosa => stringify!(PurpleMimosa),
            Self::PurpleMonster => stringify!(PurpleMonster),
            Self::PurpleNavy => stringify!(PurpleNavy),
            Self::PurplePink => stringify!(PurplePink),
            Self::PurplePlum => stringify!(PurplePlum),
            Self::PurpleSageBush => stringify!(PurpleSageBush),
            Self::PurpleThistle => stringify!(PurpleThistle),
            Self::PurpleViolet => stringify!(PurpleViolet),
            Self::PurpleWhite => stringify!(PurpleWhite),
            Self::RacingGreen => stringify!(RacingGreen),
            Self::Raspberry => stringify!(Raspberry),
            Self::RaspberryPurple => stringify!(RaspberryPurple),
            Self::RatGray => stringify!(RatGray),
            Self::RebeccaPurple => stringify!(RebeccaPurple),
            Self::Red => stringify!(Red),
            Self::RedBlood => stringify!(RedBlood),
            Self::RedBrown => stringify!(RedBrown),
            Self::RedDirt => stringify!(RedDirt),
            Self::RedFox => stringify!(RedFox),
            Self::RedGold => stringify!(RedGold),
            Self::RedMagenta => stringify!(RedMagenta),
            Self::RedPink => stringify!(RedPink),
            Self::RedWhite => stringify!(RedWhite),
            Self::RedWine => stringify!(RedWine),
            Self::Rice => stringify!(Rice),
            Self::RichLilac => stringify!(RichLilac),
            Self::RobinEggBlue => stringify!(RobinEggBlue),
            Self::RoguePink => stringify!(RoguePink),
            Self::RomanSilver => stringify!(RomanSilver),
            Self::Rose => stringify!(Rose),
            Self::RoseDust => stringify!(RoseDust),
            Self::RoseGold => stringify!(RoseGold),
            Self::RosePink => stringify!(RosePink),
            Self::RosePurple => stringify!(RosePurple),
            Self::RoseQuartz => stringify!(RoseQuartz),
            Self::RoseRed => stringify!(RoseRed),
            Self::Rosy => stringify!(Rosy),
            Self::RosyFinch => stringify!(RosyFinch),
            Self::RosyPink => stringify!(RosyPink),
            Self::RosyBrown => stringify!(RosyBrown),
            Self::RoyalPink => stringify!(RoyalPink),
            Self::RoyalBlue => stringify!(RoyalBlue),
            Self::RubberDuckyYellow => stringify!(RubberDuckyYellow),
            Self::RubyRed => stringify!(RubyRed),
            Self::Rust => stringify!(Rust),
            Self::SaddleBrown => stringify!(SaddleBrown),
            Self::SafetyOrange => stringify!(SafetyOrange),
            Self::SafetyYellow => stringify!(SafetyYellow),
            Self::Saffron => stringify!(Saffron),
            Self::SaffronRed => stringify!(SaffronRed),
            Self::Sage => stringify!(Sage),
            Self::SageGreen => stringify!(SageGreen),
            Self::SaladGreen => stringify!(SaladGreen),
            Self::Salmon => stringify!(Salmon),
            Self::SalmonPink => stringify!(SalmonPink),
            Self::SamcoBlue => stringify!(SamcoBlue),
            Self::Sand => stringify!(Sand),
            Self::Sandstone => stringify!(Sandstone),
            Self::SandyBrown => stringify!(SandyBrown),
            Self::Sangria => stringify!(Sangria),
            Self::SapphireBlue => stringify!(SapphireBlue),
            Self::ScarletRed => stringify!(ScarletRed),
            Self::SchoolBusYellow => stringify!(SchoolBusYellow),
            Self::SeaBlue => stringify!(SeaBlue),
            Self::SeaTurtleGreen => stringify!(SeaTurtleGreen),
            Self::SeaGreen => stringify!(SeaGreen),
            Self::SeaShell => stringify!(SeaShell),
            Self::SeafoamGreen => stringify!(SeafoamGreen),
            Self::SeaweedGreen => stringify!(SeaweedGreen),
            Self::Sedona => stringify!(Sedona),
            Self::Sepia => stringify!(Sepia),
            Self::SepiaBrown => stringify!(SepiaBrown),
            Self::ShamrockGreen => stringify!(ShamrockGreen),
            Self::SheetMetal => stringify!(SheetMetal),
            Self::ShockingOrange => stringify!(ShockingOrange),
            Self::Sienna => stringify!(Sienna),
            Self::SilkBlue => stringify!(SilkBlue),
            Self::Silver => stringify!(Silver),
            Self::SilverPink => stringify!(SilverPink),
            Self::SilverWhite => stringify!(SilverWhite),
            Self::SkyBlueDress => stringify!(SkyBlueDress),
            Self::SkyBlue => stringify!(SkyBlue),
            Self::SlateBlueGray => stringify!(SlateBlueGray),
            Self::SlateGraniteGray => stringify!(SlateGraniteGray),
            Self::SlateBlue => stringify!(SlateBlue),
            Self::SlateGray => stringify!(SlateGray),
            Self::SlateGrey => stringify!(SlateGrey),
            Self::SlimeGreen => stringify!(SlimeGreen),
            Self::SmokeyGray => stringify!(SmokeyGray),
            Self::Snow => stringify!(Snow),
            Self::SoftHazel => stringify!(SoftHazel),
            Self::SoftIvory => stringify!(SoftIvory),
            Self::SoftPink => stringify!(SoftPink),
            Self::SonicSilver => stringify!(SonicSilver),
            Self::SpringGreen => stringify!(SpringGreen),
            Self::StainlessSteelGray => stringify!(StainlessSteelGray),
            Self::Steampunk => stringify!(Steampunk),
            Self::SteelGray => stringify!(SteelGray),
            Self::SteelBlue => stringify!(SteelBlue),
            Self::StoplightGoGreen => stringify!(StoplightGoGreen),
            Self::StormyGray => stringify!(StormyGray),
            Self::StrawberryRed => stringify!(StrawberryRed),
            Self::SunYellow => stringify!(SunYellow),
            Self::SunriseOrange => stringify!(SunriseOrange),
            Self::Tan => stringify!(Tan),
            Self::TanBrown => stringify!(TanBrown),
            Self::Tangerine => stringify!(Tangerine),
            Self::Taupe => stringify!(Taupe),
            Self::TeaGreen => stringify!(TeaGreen),
            Self::Teal => stringify!(Teal),
            Self::TealBlue => stringify!(TealBlue),
            Self::TealGreen => stringify!(TealGreen),
            Self::Thistle => stringify!(Thistle),
            Self::TiffanyBlue => stringify!(TiffanyBlue),
            Self::TigerOrange => stringify!(TigerOrange),
            Self::Tomato => stringify!(Tomato),
            Self::TomatoSauceRed => stringify!(TomatoSauceRed),
            Self::TronBlue => stringify!(TronBlue),
            Self::TulipPink => stringify!(TulipPink),
            Self::Turquoise => stringify!(Turquoise),
            Self::TurquoiseGreen => stringify!(TurquoiseGreen),
            Self::TyrianPurple => stringify!(TyrianPurple),
            Self::UnbleachedSilk => stringify!(UnbleachedSilk),
            Self::ValentineRed => stringify!(ValentineRed),
            Self::VampireGray => stringify!(VampireGray),
            Self::Vanilla => stringify!(Vanilla),
            Self::VelvetMaroon => stringify!(VelvetMaroon),
            Self::VenomGreen => stringify!(VenomGreen),
            Self::Vermilion => stringify!(Vermilion),
            Self::VeryPeri => stringify!(VeryPeri),
            Self::Viola => stringify!(Viola),
            Self::ViolaPurple => stringify!(ViolaPurple),
            Self::Violet => stringify!(Violet),
            Self::VioletRed => stringify!(VioletRed),
            Self::WarmPink => stringify!(WarmPink),
            Self::WarmWhite => stringify!(WarmWhite),
            Self::Water => stringify!(Water),
            Self::WatermelonPink => stringify!(WatermelonPink),
            Self::WesternCharcoal => stringify!(WesternCharcoal),
            Self::Wheat => stringify!(Wheat),
            Self::White => stringify!(White),
            Self::WhiteBlue => stringify!(WhiteBlue),
            Self::WhiteChocolate => stringify!(WhiteChocolate),
            Self::WhiteGold => stringify!(WhiteGold),
            Self::WhiteGray => stringify!(WhiteGray),
            Self::WhiteIce => stringify!(WhiteIce),
            Self::WhiteYellow => stringify!(WhiteYellow),
            Self::WhiteSmoke => stringify!(WhiteSmoke),
            Self::WindowsBlue => stringify!(WindowsBlue),
            Self::WineRed => stringify!(WineRed),
            Self::WisteriaPurple => stringify!(WisteriaPurple),
            Self::Wood => stringify!(Wood),
            Self::Yellow => stringify!(Yellow),
            Self::YellowGreenGrosbeak => stringify!(YellowGreenGrosbeak),
            Self::YellowLawnGreen => stringify!(YellowLawnGreen),
            Self::YellowOrange => stringify!(YellowOrange),
            Self::YellowGreen => stringify!(YellowGreen),
            Self::ZombieGreen => stringify!(ZombieGreen),
            Self::Rgb(r, g, b) => return format!("rgb({}, {}, {})", r, g, b),
        }
        .to_string()
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
