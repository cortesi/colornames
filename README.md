# colornames

![docs.rs](https://img.shields.io/docsrs/colornames)

This crate does one thing only: it provides an enum of color names, with a
catchall RGB variant for un-named colors. Color variants can be constructed by
name or by hex value.

The crate lazily generates lookup tables for color names and hex values. There
are 728 named colors, so this incurs a startup cost on first use.

Name matching is case and whitespace insensitive.

```rust
use colornames::*;

// We implment TryFrom<_> for string types
assert_eq!("PinkLemonade".try_into(), Ok(Color::PinkLemonade));

// Conversion from color names is case and whitespace insensitive
assert_eq!("pink lemonade".try_into(), Ok(Color::PinkLemonade));
assert_eq!("pinklemonade".try_into(), Ok(Color::PinkLemonade));

// Conversion from hex codes maps to the named color if possible
assert_eq!(("#E4287C").try_into(), Ok(Color::PinkLemonade));
assert_eq!(("#e4287c").try_into(), Ok(Color::PinkLemonade));
// ... and a catch-all Rgb variant if not
assert_eq!(("#000011").try_into(), Ok(Color::Rgb(0, 0, 17)));
// We support short hex codes
assert_eq!("#fff".try_into(), Ok(Color::White));

// Colors can be converted to names, hex codes, and RGB values
let c: Color = Color::PinkLemonade;
assert_eq!(c.name(), Some("Pink Lemonade".to_string()));
assert_eq!(c.rgb_hex(), "#E4287C".to_string());
assert_eq!(c.rgb(), (228, 40, 124));

let c: Color = Color::Rgb(0, 0, 17);
assert_eq!(c.name(), None);
assert_eq!(c.rgb_hex(), "#000011".to_string());
assert_eq!(c.rgb(), (0, 0, 17));
```

# Supported Colors

<table>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/black.png" width="50" height="20"></td>
<td>Black</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/black_blue.png" width="50" height="20"></td>
<td>Black Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/night.png" width="50" height="20"></td>
<td>Night</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/charcoal.png" width="50" height="20"></td>
<td>Charcoal</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/oil.png" width="50" height="20"></td>
<td>Oil</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/stormy_gray.png" width="50" height="20"></td>
<td>Stormy Gray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_black.png" width="50" height="20"></td>
<td>Light Black</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_steampunk.png" width="50" height="20"></td>
<td>Dark Steampunk</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/black_cat.png" width="50" height="20"></td>
<td>Black Cat</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/iridium.png" width="50" height="20"></td>
<td>Iridium</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/black_eel.png" width="50" height="20"></td>
<td>Black Eel</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/black_cow.png" width="50" height="20"></td>
<td>Black Cow</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gray_wolf.png" width="50" height="20"></td>
<td>Gray Wolf</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/vampire_gray.png" width="50" height="20"></td>
<td>Vampire Gray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/iron_gray.png" width="50" height="20"></td>
<td>Iron Gray</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gray_dolphin.png" width="50" height="20"></td>
<td>Gray Dolphin</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/carbon_gray.png" width="50" height="20"></td>
<td>Carbon Gray</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ash_gray.png" width="50" height="20"></td>
<td>Ash Gray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dimgray.png" width="50" height="20"></td>
<td>DimGray</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dimgrey.png" width="50" height="20"></td>
<td>DimGrey</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/nardo_gray.png" width="50" height="20"></td>
<td>Nardo Gray</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cloudy_gray.png" width="50" height="20"></td>
<td>Cloudy Gray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/smokey_gray.png" width="50" height="20"></td>
<td>Smokey Gray</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/alien_gray.png" width="50" height="20"></td>
<td>Alien Gray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sonic_silver.png" width="50" height="20"></td>
<td>Sonic Silver</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/platinum_gray.png" width="50" height="20"></td>
<td>Platinum Gray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/granite.png" width="50" height="20"></td>
<td>Granite</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gray.png" width="50" height="20"></td>
<td>Gray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/grey.png" width="50" height="20"></td>
<td>Grey</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/battleship_gray.png" width="50" height="20"></td>
<td>Battleship Gray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sheet_metal.png" width="50" height="20"></td>
<td>Sheet Metal</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_gainsboro.png" width="50" height="20"></td>
<td>Dark Gainsboro</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gunmetal_gray.png" width="50" height="20"></td>
<td>Gunmetal Gray</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cold_metal.png" width="50" height="20"></td>
<td>Cold Metal</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/stainless_steel_gray.png" width="50" height="20"></td>
<td>Stainless Steel Gray</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkgrey.png" width="50" height="20"></td>
<td>DarkGrey</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkgray.png" width="50" height="20"></td>
<td>DarkGray</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chrome_aluminum.png" width="50" height="20"></td>
<td>Chrome Aluminum</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gray_cloud.png" width="50" height="20"></td>
<td>Gray Cloud</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/metal.png" width="50" height="20"></td>
<td>Metal</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/silver.png" width="50" height="20"></td>
<td>Silver</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/steampunk.png" width="50" height="20"></td>
<td>Steampunk</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pale_silver.png" width="50" height="20"></td>
<td>Pale Silver</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gear_steel_gray.png" width="50" height="20"></td>
<td>Gear Steel Gray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gray_goose.png" width="50" height="20"></td>
<td>Gray Goose</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/platinum_silver.png" width="50" height="20"></td>
<td>Platinum Silver</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightgrey.png" width="50" height="20"></td>
<td>LightGrey</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightgray.png" width="50" height="20"></td>
<td>LightGray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/silver_white.png" width="50" height="20"></td>
<td>Silver White</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gainsboro.png" width="50" height="20"></td>
<td>Gainsboro</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_steel_gray.png" width="50" height="20"></td>
<td>Light Steel Gray</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/whitesmoke.png" width="50" height="20"></td>
<td>WhiteSmoke</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/white_gray.png" width="50" height="20"></td>
<td>White Gray</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/platinum.png" width="50" height="20"></td>
<td>Platinum</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/metallic_silver.png" width="50" height="20"></td>
<td>Metallic Silver</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_gray.png" width="50" height="20"></td>
<td>Blue Gray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/roman_silver.png" width="50" height="20"></td>
<td>Roman Silver</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightslategrey.png" width="50" height="20"></td>
<td>LightSlateGrey</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightslategray.png" width="50" height="20"></td>
<td>LightSlateGray</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/slategrey.png" width="50" height="20"></td>
<td>SlateGrey</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/slategray.png" width="50" height="20"></td>
<td>SlateGray</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rat_gray.png" width="50" height="20"></td>
<td>Rat Gray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/slate_granite_gray.png" width="50" height="20"></td>
<td>Slate Granite Gray</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/jet_gray.png" width="50" height="20"></td>
<td>Jet Gray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mist_blue.png" width="50" height="20"></td>
<td>Mist Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/steel_gray.png" width="50" height="20"></td>
<td>Steel Gray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/marble_blue.png" width="50" height="20"></td>
<td>Marble Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/slate_blue_gray.png" width="50" height="20"></td>
<td>Slate Blue Gray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_purple_blue.png" width="50" height="20"></td>
<td>Light Purple Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/azure_blue.png" width="50" height="20"></td>
<td>Azure Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/estoril_blue.png" width="50" height="20"></td>
<td>Estoril Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_jay.png" width="50" height="20"></td>
<td>Blue Jay</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/charcoal_blue.png" width="50" height="20"></td>
<td>Charcoal Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_blue_gray.png" width="50" height="20"></td>
<td>Dark Blue Gray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_slate.png" width="50" height="20"></td>
<td>Dark Slate</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_sea_blue.png" width="50" height="20"></td>
<td>Deep Sea Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/night_blue.png" width="50" height="20"></td>
<td>Night Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/midnightblue.png" width="50" height="20"></td>
<td>MidnightBlue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/navy.png" width="50" height="20"></td>
<td>Navy</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/denim_dark_blue.png" width="50" height="20"></td>
<td>Denim Dark Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkblue.png" width="50" height="20"></td>
<td>DarkBlue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lapis_blue.png" width="50" height="20"></td>
<td>Lapis Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/new_midnight_blue.png" width="50" height="20"></td>
<td>New Midnight Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/earth_blue.png" width="50" height="20"></td>
<td>Earth Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cobalt_blue.png" width="50" height="20"></td>
<td>Cobalt Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumblue.png" width="50" height="20"></td>
<td>MediumBlue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blueberry_blue.png" width="50" height="20"></td>
<td>Blueberry Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/canary_blue.png" width="50" height="20"></td>
<td>Canary Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue.png" width="50" height="20"></td>
<td>Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/samco_blue.png" width="50" height="20"></td>
<td>Samco Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_blue.png" width="50" height="20"></td>
<td>Bright Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_orchid.png" width="50" height="20"></td>
<td>Blue Orchid</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sapphire_blue.png" width="50" height="20"></td>
<td>Sapphire Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_eyes.png" width="50" height="20"></td>
<td>Blue Eyes</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_navy_blue.png" width="50" height="20"></td>
<td>Bright Navy Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/balloon_blue.png" width="50" height="20"></td>
<td>Balloon Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/royalblue.png" width="50" height="20"></td>
<td>RoyalBlue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ocean_blue.png" width="50" height="20"></td>
<td>Ocean Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_sky_blue.png" width="50" height="20"></td>
<td>Dark Sky Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_ribbon.png" width="50" height="20"></td>
<td>Blue Ribbon</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_dress.png" width="50" height="20"></td>
<td>Blue Dress</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_blue.png" width="50" height="20"></td>
<td>Neon Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dodgerblue.png" width="50" height="20"></td>
<td>DodgerBlue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/glacial_blue_ice.png" width="50" height="20"></td>
<td>Glacial Blue Ice</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/steelblue.png" width="50" height="20"></td>
<td>SteelBlue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/silk_blue.png" width="50" height="20"></td>
<td>Silk Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/windows_blue.png" width="50" height="20"></td>
<td>Windows Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_ivy.png" width="50" height="20"></td>
<td>Blue Ivy</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cyan_blue.png" width="50" height="20"></td>
<td>Cyan Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_koi.png" width="50" height="20"></td>
<td>Blue Koi</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/columbia_blue.png" width="50" height="20"></td>
<td>Columbia Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/baby_blue.png" width="50" height="20"></td>
<td>Baby Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cornflowerblue.png" width="50" height="20"></td>
<td>CornflowerBlue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sky_blue_dress.png" width="50" height="20"></td>
<td>Sky Blue Dress</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/iceberg.png" width="50" height="20"></td>
<td>Iceberg</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/butterfly_blue.png" width="50" height="20"></td>
<td>Butterfly Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deepskyblue.png" width="50" height="20"></td>
<td>DeepSkyBlue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/midday_blue.png" width="50" height="20"></td>
<td>Midday Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/crystal_blue.png" width="50" height="20"></td>
<td>Crystal Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/denim_blue.png" width="50" height="20"></td>
<td>Denim Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/day_sky_blue.png" width="50" height="20"></td>
<td>Day Sky Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightskyblue.png" width="50" height="20"></td>
<td>LightSkyBlue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/skyblue.png" width="50" height="20"></td>
<td>SkyBlue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/jeans_blue.png" width="50" height="20"></td>
<td>Jeans Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_angel.png" width="50" height="20"></td>
<td>Blue Angel</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_blue.png" width="50" height="20"></td>
<td>Pastel Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_day_blue.png" width="50" height="20"></td>
<td>Light Day Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sea_blue.png" width="50" height="20"></td>
<td>Sea Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/heavenly_blue.png" width="50" height="20"></td>
<td>Heavenly Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/robin_egg_blue.png" width="50" height="20"></td>
<td>Robin Egg Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/powderblue.png" width="50" height="20"></td>
<td>PowderBlue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/coral_blue.png" width="50" height="20"></td>
<td>Coral Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightblue.png" width="50" height="20"></td>
<td>LightBlue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightsteelblue.png" width="50" height="20"></td>
<td>LightSteelBlue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gulf_blue.png" width="50" height="20"></td>
<td>Gulf Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_light_blue.png" width="50" height="20"></td>
<td>Pastel Light Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lavender_blue.png" width="50" height="20"></td>
<td>Lavender Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/white_blue.png" width="50" height="20"></td>
<td>White Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lavender.png" width="50" height="20"></td>
<td>Lavender</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/water.png" width="50" height="20"></td>
<td>Water</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aliceblue.png" width="50" height="20"></td>
<td>AliceBlue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ghostwhite.png" width="50" height="20"></td>
<td>GhostWhite</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/azure.png" width="50" height="20"></td>
<td>Azure</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightcyan.png" width="50" height="20"></td>
<td>LightCyan</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_slate.png" width="50" height="20"></td>
<td>Light Slate</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/electric_blue.png" width="50" height="20"></td>
<td>Electric Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tron_blue.png" width="50" height="20"></td>
<td>Tron Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_zircon.png" width="50" height="20"></td>
<td>Blue Zircon</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cyan.png" width="50" height="20"></td>
<td>Cyan</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aqua.png" width="50" height="20"></td>
<td>Aqua</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_cyan.png" width="50" height="20"></td>
<td>Bright Cyan</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/celeste.png" width="50" height="20"></td>
<td>Celeste</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_diamond.png" width="50" height="20"></td>
<td>Blue Diamond</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_turquoise.png" width="50" height="20"></td>
<td>Bright Turquoise</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_lagoon.png" width="50" height="20"></td>
<td>Blue Lagoon</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/paleturquoise.png" width="50" height="20"></td>
<td>PaleTurquoise</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pale_blue_lily.png" width="50" height="20"></td>
<td>Pale Blue Lily</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_teal.png" width="50" height="20"></td>
<td>Light Teal</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tiffany_blue.png" width="50" height="20"></td>
<td>Tiffany Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_hosta.png" width="50" height="20"></td>
<td>Blue Hosta</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cyan_opaque.png" width="50" height="20"></td>
<td>Cyan Opaque</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/northern_lights_blue.png" width="50" height="20"></td>
<td>Northern Lights Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_green.png" width="50" height="20"></td>
<td>Blue Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumaquamarine.png" width="50" height="20"></td>
<td>MediumAquaMarine</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aqua_seafoam_green.png" width="50" height="20"></td>
<td>Aqua Seafoam Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/magic_mint.png" width="50" height="20"></td>
<td>Magic Mint</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_aquamarine.png" width="50" height="20"></td>
<td>Light Aquamarine</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aquamarine.png" width="50" height="20"></td>
<td>Aquamarine</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_teal.png" width="50" height="20"></td>
<td>Bright Teal</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/turquoise.png" width="50" height="20"></td>
<td>Turquoise</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumturquoise.png" width="50" height="20"></td>
<td>MediumTurquoise</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_turquoise.png" width="50" height="20"></td>
<td>Deep Turquoise</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/jellyfish.png" width="50" height="20"></td>
<td>Jellyfish</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_turquoise.png" width="50" height="20"></td>
<td>Blue Turquoise</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkturquoise.png" width="50" height="20"></td>
<td>DarkTurquoise</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/macaw_blue_green.png" width="50" height="20"></td>
<td>Macaw Blue Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightseagreen.png" width="50" height="20"></td>
<td>LightSeaGreen</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/seafoam_green.png" width="50" height="20"></td>
<td>Seafoam Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cadetblue.png" width="50" height="20"></td>
<td>CadetBlue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_sea.png" width="50" height="20"></td>
<td>Deep Sea</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkcyan.png" width="50" height="20"></td>
<td>DarkCyan</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/teal_green.png" width="50" height="20"></td>
<td>Teal Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/teal.png" width="50" height="20"></td>
<td>Teal</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/teal_blue.png" width="50" height="20"></td>
<td>Teal Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/medium_teal.png" width="50" height="20"></td>
<td>Medium Teal</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_teal.png" width="50" height="20"></td>
<td>Dark Teal</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_teal.png" width="50" height="20"></td>
<td>Deep Teal</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkslategray.png" width="50" height="20"></td>
<td>DarkSlateGray</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkslategrey.png" width="50" height="20"></td>
<td>DarkSlateGrey</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gunmetal.png" width="50" height="20"></td>
<td>Gunmetal</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_moss_green.png" width="50" height="20"></td>
<td>Blue Moss Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/beetle_green.png" width="50" height="20"></td>
<td>Beetle Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/grayish_turquoise.png" width="50" height="20"></td>
<td>Grayish Turquoise</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/greenish_blue.png" width="50" height="20"></td>
<td>Greenish Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aquamarine_stone.png" width="50" height="20"></td>
<td>Aquamarine Stone</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sea_turtle_green.png" width="50" height="20"></td>
<td>Sea Turtle Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dull_sea_green.png" width="50" height="20"></td>
<td>Dull Sea Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_green_blue.png" width="50" height="20"></td>
<td>Dark Green Blue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_sea_green.png" width="50" height="20"></td>
<td>Deep Sea Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bottle_green.png" width="50" height="20"></td>
<td>Bottle Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/seagreen.png" width="50" height="20"></td>
<td>SeaGreen</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/elf_green.png" width="50" height="20"></td>
<td>Elf Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_mint.png" width="50" height="20"></td>
<td>Dark Mint</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/jade.png" width="50" height="20"></td>
<td>Jade</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/earth_green.png" width="50" height="20"></td>
<td>Earth Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chrome_green.png" width="50" height="20"></td>
<td>Chrome Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mint.png" width="50" height="20"></td>
<td>Mint</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/emerald.png" width="50" height="20"></td>
<td>Emerald</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/isle_of_man_green.png" width="50" height="20"></td>
<td>Isle Of Man Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumseagreen.png" width="50" height="20"></td>
<td>MediumSeaGreen</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/metallic_green.png" width="50" height="20"></td>
<td>Metallic Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/camouflage_green.png" width="50" height="20"></td>
<td>Camouflage Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sage_green.png" width="50" height="20"></td>
<td>Sage Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/hazel_green.png" width="50" height="20"></td>
<td>Hazel Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/venom_green.png" width="50" height="20"></td>
<td>Venom Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/olivedrab.png" width="50" height="20"></td>
<td>OliveDrab</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/olive.png" width="50" height="20"></td>
<td>Olive</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ebony.png" width="50" height="20"></td>
<td>Ebony</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkolivegreen.png" width="50" height="20"></td>
<td>DarkOliveGreen</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/military_green.png" width="50" height="20"></td>
<td>Military Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green_leaves.png" width="50" height="20"></td>
<td>Green Leaves</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/army_green.png" width="50" height="20"></td>
<td>Army Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/fern_green.png" width="50" height="20"></td>
<td>Fern Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/fall_forest_green.png" width="50" height="20"></td>
<td>Fall Forest Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/irish_green.png" width="50" height="20"></td>
<td>Irish Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pine_green.png" width="50" height="20"></td>
<td>Pine Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/medium_forest_green.png" width="50" height="20"></td>
<td>Medium Forest Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/racing_green.png" width="50" height="20"></td>
<td>Racing Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/jungle_green.png" width="50" height="20"></td>
<td>Jungle Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cactus_green.png" width="50" height="20"></td>
<td>Cactus Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/forestgreen.png" width="50" height="20"></td>
<td>ForestGreen</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green.png" width="50" height="20"></td>
<td>Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkgreen.png" width="50" height="20"></td>
<td>DarkGreen</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_green.png" width="50" height="20"></td>
<td>Deep Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_emerald_green.png" width="50" height="20"></td>
<td>Deep Emerald Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/hunter_green.png" width="50" height="20"></td>
<td>Hunter Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_forest_green.png" width="50" height="20"></td>
<td>Dark Forest Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lotus_green.png" width="50" height="20"></td>
<td>Lotus Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/broccoli_green.png" width="50" height="20"></td>
<td>Broccoli Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/seaweed_green.png" width="50" height="20"></td>
<td>Seaweed Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/shamrock_green.png" width="50" height="20"></td>
<td>Shamrock Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green_onion.png" width="50" height="20"></td>
<td>Green Onion</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/moss_green.png" width="50" height="20"></td>
<td>Moss Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/grass_green.png" width="50" height="20"></td>
<td>Grass Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green_pepper.png" width="50" height="20"></td>
<td>Green Pepper</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_lime_green.png" width="50" height="20"></td>
<td>Dark Lime Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/parrot_green.png" width="50" height="20"></td>
<td>Parrot Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/clover_green.png" width="50" height="20"></td>
<td>Clover Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dinosaur_green.png" width="50" height="20"></td>
<td>Dinosaur Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green_snake.png" width="50" height="20"></td>
<td>Green Snake</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/alien_green.png" width="50" height="20"></td>
<td>Alien Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green_apple.png" width="50" height="20"></td>
<td>Green Apple</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/limegreen.png" width="50" height="20"></td>
<td>LimeGreen</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pea_green.png" width="50" height="20"></td>
<td>Pea Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/kelly_green.png" width="50" height="20"></td>
<td>Kelly Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/zombie_green.png" width="50" height="20"></td>
<td>Zombie Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green_peas.png" width="50" height="20"></td>
<td>Green Peas</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dollar_bill_green.png" width="50" height="20"></td>
<td>Dollar Bill Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/frog_green.png" width="50" height="20"></td>
<td>Frog Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/turquoise_green.png" width="50" height="20"></td>
<td>Turquoise Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkseagreen.png" width="50" height="20"></td>
<td>DarkSeaGreen</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/basil_green.png" width="50" height="20"></td>
<td>Basil Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gray_green.png" width="50" height="20"></td>
<td>Gray Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_olive_green.png" width="50" height="20"></td>
<td>Light Olive Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/iguana_green.png" width="50" height="20"></td>
<td>Iguana Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/citron_green.png" width="50" height="20"></td>
<td>Citron Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/acid_green.png" width="50" height="20"></td>
<td>Acid Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/avocado_green.png" width="50" height="20"></td>
<td>Avocado Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pistachio_green.png" width="50" height="20"></td>
<td>Pistachio Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/salad_green.png" width="50" height="20"></td>
<td>Salad Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/yellowgreen.png" width="50" height="20"></td>
<td>YellowGreen</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_green.png" width="50" height="20"></td>
<td>Pastel Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/hummingbird_green.png" width="50" height="20"></td>
<td>Hummingbird Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/nebula_green.png" width="50" height="20"></td>
<td>Nebula Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/stoplight_go_green.png" width="50" height="20"></td>
<td>Stoplight Go Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_green.png" width="50" height="20"></td>
<td>Neon Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/jade_green.png" width="50" height="20"></td>
<td>Jade Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/springgreen.png" width="50" height="20"></td>
<td>SpringGreen</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ocean_green.png" width="50" height="20"></td>
<td>Ocean Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lime_mint_green.png" width="50" height="20"></td>
<td>Lime Mint Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumspringgreen.png" width="50" height="20"></td>
<td>MediumSpringGreen</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aqua_green.png" width="50" height="20"></td>
<td>Aqua Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/emerald_green.png" width="50" height="20"></td>
<td>Emerald Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lime.png" width="50" height="20"></td>
<td>Lime</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lawngreen.png" width="50" height="20"></td>
<td>LawnGreen</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_green.png" width="50" height="20"></td>
<td>Bright Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chartreuse.png" width="50" height="20"></td>
<td>Chartreuse</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/yellow_lawn_green.png" width="50" height="20"></td>
<td>Yellow Lawn Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aloe_vera_green.png" width="50" height="20"></td>
<td>Aloe Vera Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dull_green_yellow.png" width="50" height="20"></td>
<td>Dull Green Yellow</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lemon_green.png" width="50" height="20"></td>
<td>Lemon Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/greenyellow.png" width="50" height="20"></td>
<td>GreenYellow</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chameleon_green.png" width="50" height="20"></td>
<td>Chameleon Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_yellow_green.png" width="50" height="20"></td>
<td>Neon Yellow Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/yellow_green_grosbeak.png" width="50" height="20"></td>
<td>Yellow Green Grosbeak</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tea_green.png" width="50" height="20"></td>
<td>Tea Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/slime_green.png" width="50" height="20"></td>
<td>Slime Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/algae_green.png" width="50" height="20"></td>
<td>Algae Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightgreen.png" width="50" height="20"></td>
<td>LightGreen</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dragon_green.png" width="50" height="20"></td>
<td>Dragon Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/palegreen.png" width="50" height="20"></td>
<td>PaleGreen</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mint_green.png" width="50" height="20"></td>
<td>Mint Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green_thumb.png" width="50" height="20"></td>
<td>Green Thumb</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/organic_brown.png" width="50" height="20"></td>
<td>Organic Brown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_jade.png" width="50" height="20"></td>
<td>Light Jade</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_mint_green.png" width="50" height="20"></td>
<td>Light Mint Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_rose_green.png" width="50" height="20"></td>
<td>Light Rose Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chrome_white.png" width="50" height="20"></td>
<td>Chrome White</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/honeydew.png" width="50" height="20"></td>
<td>HoneyDew</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mintcream.png" width="50" height="20"></td>
<td>MintCream</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lemonchiffon.png" width="50" height="20"></td>
<td>LemonChiffon</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/parchment.png" width="50" height="20"></td>
<td>Parchment</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cream.png" width="50" height="20"></td>
<td>Cream</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cream_white.png" width="50" height="20"></td>
<td>Cream White</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightgoldenrodyellow.png" width="50" height="20"></td>
<td>LightGoldenRodYellow</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightyellow.png" width="50" height="20"></td>
<td>LightYellow</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/beige.png" width="50" height="20"></td>
<td>Beige</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/white_yellow.png" width="50" height="20"></td>
<td>White Yellow</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cornsilk.png" width="50" height="20"></td>
<td>Cornsilk</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blonde.png" width="50" height="20"></td>
<td>Blonde</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/antiquewhite.png" width="50" height="20"></td>
<td>AntiqueWhite</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_beige.png" width="50" height="20"></td>
<td>Light Beige</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/papayawhip.png" width="50" height="20"></td>
<td>PapayaWhip</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/champagne.png" width="50" height="20"></td>
<td>Champagne</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blanchedalmond.png" width="50" height="20"></td>
<td>BlanchedAlmond</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bisque.png" width="50" height="20"></td>
<td>Bisque</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/wheat.png" width="50" height="20"></td>
<td>Wheat</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/moccasin.png" width="50" height="20"></td>
<td>Moccasin</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/peach.png" width="50" height="20"></td>
<td>Peach</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_orange.png" width="50" height="20"></td>
<td>Light Orange</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/peachpuff.png" width="50" height="20"></td>
<td>PeachPuff</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/coral_peach.png" width="50" height="20"></td>
<td>Coral Peach</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/navajowhite.png" width="50" height="20"></td>
<td>NavajoWhite</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/golden_blonde.png" width="50" height="20"></td>
<td>Golden Blonde</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/golden_silk.png" width="50" height="20"></td>
<td>Golden Silk</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_blonde.png" width="50" height="20"></td>
<td>Dark Blonde</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_gold.png" width="50" height="20"></td>
<td>Light Gold</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/vanilla.png" width="50" height="20"></td>
<td>Vanilla</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tan_brown.png" width="50" height="20"></td>
<td>Tan Brown</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dirty_white.png" width="50" height="20"></td>
<td>Dirty White</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/palegoldenrod.png" width="50" height="20"></td>
<td>PaleGoldenRod</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/khaki.png" width="50" height="20"></td>
<td>Khaki</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cardboard_brown.png" width="50" height="20"></td>
<td>Cardboard Brown</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/harvest_gold.png" width="50" height="20"></td>
<td>Harvest Gold</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sun_yellow.png" width="50" height="20"></td>
<td>Sun Yellow</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/corn_yellow.png" width="50" height="20"></td>
<td>Corn Yellow</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_yellow.png" width="50" height="20"></td>
<td>Pastel Yellow</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_yellow.png" width="50" height="20"></td>
<td>Neon Yellow</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/yellow.png" width="50" height="20"></td>
<td>Yellow</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lemon_yellow.png" width="50" height="20"></td>
<td>Lemon Yellow</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/canary_yellow.png" width="50" height="20"></td>
<td>Canary Yellow</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/banana_yellow.png" width="50" height="20"></td>
<td>Banana Yellow</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mustard_yellow.png" width="50" height="20"></td>
<td>Mustard Yellow</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/golden_yellow.png" width="50" height="20"></td>
<td>Golden Yellow</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bold_yellow.png" width="50" height="20"></td>
<td>Bold Yellow</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/safety_yellow.png" width="50" height="20"></td>
<td>Safety Yellow</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rubber_ducky_yellow.png" width="50" height="20"></td>
<td>Rubber Ducky Yellow</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gold.png" width="50" height="20"></td>
<td>Gold</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_gold.png" width="50" height="20"></td>
<td>Bright Gold</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chrome_gold.png" width="50" height="20"></td>
<td>Chrome Gold</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/golden_brown.png" width="50" height="20"></td>
<td>Golden Brown</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_yellow.png" width="50" height="20"></td>
<td>Deep Yellow</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/macaroni_and_cheese.png" width="50" height="20"></td>
<td>Macaroni and Cheese</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/amber.png" width="50" height="20"></td>
<td>Amber</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/saffron.png" width="50" height="20"></td>
<td>Saffron</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_gold.png" width="50" height="20"></td>
<td>Neon Gold</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/beer.png" width="50" height="20"></td>
<td>Beer</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/yellow_orange.png" width="50" height="20"></td>
<td>Yellow Orange</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/orange_yellow.png" width="50" height="20"></td>
<td>Orange Yellow</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cantaloupe.png" width="50" height="20"></td>
<td>Cantaloupe</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cheese_orange.png" width="50" height="20"></td>
<td>Cheese Orange</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/orange.png" width="50" height="20"></td>
<td>Orange</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/brown_sand.png" width="50" height="20"></td>
<td>Brown Sand</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sandybrown.png" width="50" height="20"></td>
<td>SandyBrown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/brown_sugar.png" width="50" height="20"></td>
<td>Brown Sugar</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/camel_brown.png" width="50" height="20"></td>
<td>Camel Brown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deer_brown.png" width="50" height="20"></td>
<td>Deer Brown</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/burlywood.png" width="50" height="20"></td>
<td>BurlyWood</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tan.png" width="50" height="20"></td>
<td>Tan</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_french_beige.png" width="50" height="20"></td>
<td>Light French Beige</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sand.png" width="50" height="20"></td>
<td>Sand</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/soft_hazel.png" width="50" height="20"></td>
<td>Soft Hazel</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sage.png" width="50" height="20"></td>
<td>Sage</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/fall_leaf_brown.png" width="50" height="20"></td>
<td>Fall Leaf Brown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ginger_brown.png" width="50" height="20"></td>
<td>Ginger Brown</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bronze_gold.png" width="50" height="20"></td>
<td>Bronze Gold</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkkhaki.png" width="50" height="20"></td>
<td>DarkKhaki</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/olive_green.png" width="50" height="20"></td>
<td>Olive Green</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/brass.png" width="50" height="20"></td>
<td>Brass</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cookie_brown.png" width="50" height="20"></td>
<td>Cookie Brown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/metallic_gold.png" width="50" height="20"></td>
<td>Metallic Gold</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mustard.png" width="50" height="20"></td>
<td>Mustard</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bee_yellow.png" width="50" height="20"></td>
<td>Bee Yellow</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/school_bus_yellow.png" width="50" height="20"></td>
<td>School Bus Yellow</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/goldenrod.png" width="50" height="20"></td>
<td>GoldenRod</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/orange_gold.png" width="50" height="20"></td>
<td>Orange Gold</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/caramel.png" width="50" height="20"></td>
<td>Caramel</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkgoldenrod.png" width="50" height="20"></td>
<td>DarkGoldenRod</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cinnamon.png" width="50" height="20"></td>
<td>Cinnamon</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/peru.png" width="50" height="20"></td>
<td>Peru</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bronze.png" width="50" height="20"></td>
<td>Bronze</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pumpkin_pie.png" width="50" height="20"></td>
<td>Pumpkin Pie</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tiger_orange.png" width="50" height="20"></td>
<td>Tiger Orange</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/copper.png" width="50" height="20"></td>
<td>Copper</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_gold.png" width="50" height="20"></td>
<td>Dark Gold</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/metallic_bronze.png" width="50" height="20"></td>
<td>Metallic Bronze</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_almond.png" width="50" height="20"></td>
<td>Dark Almond</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/wood.png" width="50" height="20"></td>
<td>Wood</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/khaki_brown.png" width="50" height="20"></td>
<td>Khaki Brown</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/oak_brown.png" width="50" height="20"></td>
<td>Oak Brown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/antique_bronze.png" width="50" height="20"></td>
<td>Antique Bronze</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/hazel.png" width="50" height="20"></td>
<td>Hazel</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_yellow.png" width="50" height="20"></td>
<td>Dark Yellow</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_moccasin.png" width="50" height="20"></td>
<td>Dark Moccasin</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/khaki_green.png" width="50" height="20"></td>
<td>Khaki Green</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/millennium_jade.png" width="50" height="20"></td>
<td>Millennium Jade</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_beige.png" width="50" height="20"></td>
<td>Dark Beige</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bullet_shell.png" width="50" height="20"></td>
<td>Bullet Shell</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/army_brown.png" width="50" height="20"></td>
<td>Army Brown</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sandstone.png" width="50" height="20"></td>
<td>Sandstone</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/taupe.png" width="50" height="20"></td>
<td>Taupe</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_grayish_olive.png" width="50" height="20"></td>
<td>Dark Grayish Olive</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_hazel_brown.png" width="50" height="20"></td>
<td>Dark Hazel Brown</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mocha.png" width="50" height="20"></td>
<td>Mocha</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/milk_chocolate.png" width="50" height="20"></td>
<td>Milk Chocolate</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gray_brown.png" width="50" height="20"></td>
<td>Gray Brown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_coffee.png" width="50" height="20"></td>
<td>Dark Coffee</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/western_charcoal.png" width="50" height="20"></td>
<td>Western Charcoal</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/old_burgundy.png" width="50" height="20"></td>
<td>Old Burgundy</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_brown.png" width="50" height="20"></td>
<td>Red Brown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bakers_brown.png" width="50" height="20"></td>
<td>Bakers Brown</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pullman_brown.png" width="50" height="20"></td>
<td>Pullman Brown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_brown.png" width="50" height="20"></td>
<td>Dark Brown</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sepia_brown.png" width="50" height="20"></td>
<td>Sepia Brown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_bronze.png" width="50" height="20"></td>
<td>Dark Bronze</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/coffee.png" width="50" height="20"></td>
<td>Coffee</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/brown_bear.png" width="50" height="20"></td>
<td>Brown Bear</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_dirt.png" width="50" height="20"></td>
<td>Red Dirt</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sepia.png" width="50" height="20"></td>
<td>Sepia</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sienna.png" width="50" height="20"></td>
<td>Sienna</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/saddlebrown.png" width="50" height="20"></td>
<td>SaddleBrown</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_sienna.png" width="50" height="20"></td>
<td>Dark Sienna</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sangria.png" width="50" height="20"></td>
<td>Sangria</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blood_red.png" width="50" height="20"></td>
<td>Blood Red</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chestnut.png" width="50" height="20"></td>
<td>Chestnut</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/coral_brown.png" width="50" height="20"></td>
<td>Coral Brown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_amber.png" width="50" height="20"></td>
<td>Deep Amber</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chestnut_red.png" width="50" height="20"></td>
<td>Chestnut Red</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ginger_red.png" width="50" height="20"></td>
<td>Ginger Red</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mahogany.png" width="50" height="20"></td>
<td>Mahogany</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_gold.png" width="50" height="20"></td>
<td>Red Gold</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_fox.png" width="50" height="20"></td>
<td>Red Fox</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_bisque.png" width="50" height="20"></td>
<td>Dark Bisque</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_brown.png" width="50" height="20"></td>
<td>Light Brown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/petra_gold.png" width="50" height="20"></td>
<td>Petra Gold</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/brown_rust.png" width="50" height="20"></td>
<td>Brown Rust</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rust.png" width="50" height="20"></td>
<td>Rust</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/copper_red.png" width="50" height="20"></td>
<td>Copper Red</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/orange_salmon.png" width="50" height="20"></td>
<td>Orange Salmon</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chocolate.png" width="50" height="20"></td>
<td>Chocolate</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sedona.png" width="50" height="20"></td>
<td>Sedona</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/papaya_orange.png" width="50" height="20"></td>
<td>Papaya Orange</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/halloween_orange.png" width="50" height="20"></td>
<td>Halloween Orange</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_orange.png" width="50" height="20"></td>
<td>Neon Orange</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_orange.png" width="50" height="20"></td>
<td>Bright Orange</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/fluro_orange.png" width="50" height="20"></td>
<td>Fluro Orange</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pumpkin_orange.png" width="50" height="20"></td>
<td>Pumpkin Orange</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/safety_orange.png" width="50" height="20"></td>
<td>Safety Orange</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/carrot_orange.png" width="50" height="20"></td>
<td>Carrot Orange</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkorange.png" width="50" height="20"></td>
<td>DarkOrange</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/construction_cone_orange.png" width="50" height="20"></td>
<td>Construction Cone Orange</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/indian_saffron.png" width="50" height="20"></td>
<td>Indian Saffron</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sunrise_orange.png" width="50" height="20"></td>
<td>Sunrise Orange</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mango_orange.png" width="50" height="20"></td>
<td>Mango Orange</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/coral.png" width="50" height="20"></td>
<td>Coral</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/basket_ball_orange.png" width="50" height="20"></td>
<td>Basket Ball Orange</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_salmon_rose.png" width="50" height="20"></td>
<td>Light Salmon Rose</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightsalmon.png" width="50" height="20"></td>
<td>LightSalmon</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_orange.png" width="50" height="20"></td>
<td>Pink Orange</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darksalmon.png" width="50" height="20"></td>
<td>DarkSalmon</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tangerine.png" width="50" height="20"></td>
<td>Tangerine</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_copper.png" width="50" height="20"></td>
<td>Light Copper</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/salmon_pink.png" width="50" height="20"></td>
<td>Salmon Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/salmon.png" width="50" height="20"></td>
<td>Salmon</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/peach_pink.png" width="50" height="20"></td>
<td>Peach Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightcoral.png" width="50" height="20"></td>
<td>LightCoral</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_red.png" width="50" height="20"></td>
<td>Pastel Red</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_coral.png" width="50" height="20"></td>
<td>Pink Coral</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bean_red.png" width="50" height="20"></td>
<td>Bean Red</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/valentine_red.png" width="50" height="20"></td>
<td>Valentine Red</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/indianred.png" width="50" height="20"></td>
<td>IndianRed</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tomato.png" width="50" height="20"></td>
<td>Tomato</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/shocking_orange.png" width="50" height="20"></td>
<td>Shocking Orange</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/orangered.png" width="50" height="20"></td>
<td>OrangeRed</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red.png" width="50" height="20"></td>
<td>Red</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_red.png" width="50" height="20"></td>
<td>Neon Red</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/scarlet_red.png" width="50" height="20"></td>
<td>Scarlet Red</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ruby_red.png" width="50" height="20"></td>
<td>Ruby Red</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ferrari_red.png" width="50" height="20"></td>
<td>Ferrari Red</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/fire_engine_red.png" width="50" height="20"></td>
<td>Fire Engine Red</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lava_red.png" width="50" height="20"></td>
<td>Lava Red</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/love_red.png" width="50" height="20"></td>
<td>Love Red</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/grapefruit.png" width="50" height="20"></td>
<td>Grapefruit</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/strawberry_red.png" width="50" height="20"></td>
<td>Strawberry Red</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cherry_red.png" width="50" height="20"></td>
<td>Cherry Red</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chilli_pepper.png" width="50" height="20"></td>
<td>Chilli Pepper</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/firebrick.png" width="50" height="20"></td>
<td>FireBrick</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tomato_sauce_red.png" width="50" height="20"></td>
<td>Tomato Sauce Red</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/brown.png" width="50" height="20"></td>
<td>Brown</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/carbon_red.png" width="50" height="20"></td>
<td>Carbon Red</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cranberry.png" width="50" height="20"></td>
<td>Cranberry</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/saffron_red.png" width="50" height="20"></td>
<td>Saffron Red</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/crimson_red.png" width="50" height="20"></td>
<td>Crimson Red</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_wine.png" width="50" height="20"></td>
<td>Red Wine</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/wine_red.png" width="50" height="20"></td>
<td>Wine Red</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkred.png" width="50" height="20"></td>
<td>DarkRed</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/maroon_red.png" width="50" height="20"></td>
<td>Maroon Red</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/maroon.png" width="50" height="20"></td>
<td>Maroon</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/burgundy.png" width="50" height="20"></td>
<td>Burgundy</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/vermilion.png" width="50" height="20"></td>
<td>Vermilion</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_red.png" width="50" height="20"></td>
<td>Deep Red</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/garnet_red.png" width="50" height="20"></td>
<td>Garnet Red</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_blood.png" width="50" height="20"></td>
<td>Red Blood</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blood_night.png" width="50" height="20"></td>
<td>Blood Night</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_scarlet.png" width="50" height="20"></td>
<td>Dark Scarlet</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chocolate_brown.png" width="50" height="20"></td>
<td>Chocolate Brown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/black_bean.png" width="50" height="20"></td>
<td>Black Bean</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_maroon.png" width="50" height="20"></td>
<td>Dark Maroon</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/midnight.png" width="50" height="20"></td>
<td>Midnight</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_lily.png" width="50" height="20"></td>
<td>Purple Lily</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_maroon.png" width="50" height="20"></td>
<td>Purple Maroon</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/plum_pie.png" width="50" height="20"></td>
<td>Plum Pie</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/plum_velvet.png" width="50" height="20"></td>
<td>Plum Velvet</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_raspberry.png" width="50" height="20"></td>
<td>Dark Raspberry</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/velvet_maroon.png" width="50" height="20"></td>
<td>Velvet Maroon</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rosy_finch.png" width="50" height="20"></td>
<td>Rosy Finch</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dull_purple.png" width="50" height="20"></td>
<td>Dull Purple</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/puce.png" width="50" height="20"></td>
<td>Puce</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rose_dust.png" width="50" height="20"></td>
<td>Rose Dust</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_brown.png" width="50" height="20"></td>
<td>Pastel Brown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rosy_pink.png" width="50" height="20"></td>
<td>Rosy Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rosybrown.png" width="50" height="20"></td>
<td>RosyBrown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/khaki_rose.png" width="50" height="20"></td>
<td>Khaki Rose</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lipstick_pink.png" width="50" height="20"></td>
<td>Lipstick Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dusky_pink.png" width="50" height="20"></td>
<td>Dusky Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_brown.png" width="50" height="20"></td>
<td>Pink Brown</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/old_rose.png" width="50" height="20"></td>
<td>Old Rose</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dusty_pink.png" width="50" height="20"></td>
<td>Dusty Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_daisy.png" width="50" height="20"></td>
<td>Pink Daisy</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rose.png" width="50" height="20"></td>
<td>Rose</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dusty_rose.png" width="50" height="20"></td>
<td>Dusty Rose</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/silver_pink.png" width="50" height="20"></td>
<td>Silver Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gold_pink.png" width="50" height="20"></td>
<td>Gold Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rose_gold.png" width="50" height="20"></td>
<td>Rose Gold</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_peach.png" width="50" height="20"></td>
<td>Deep Peach</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_orange.png" width="50" height="20"></td>
<td>Pastel Orange</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/desert_sand.png" width="50" height="20"></td>
<td>Desert Sand</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/unbleached_silk.png" width="50" height="20"></td>
<td>Unbleached Silk</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pig_pink.png" width="50" height="20"></td>
<td>Pig Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pale_pink.png" width="50" height="20"></td>
<td>Pale Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blush.png" width="50" height="20"></td>
<td>Blush</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mistyrose.png" width="50" height="20"></td>
<td>MistyRose</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_bubble_gum.png" width="50" height="20"></td>
<td>Pink Bubble Gum</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_rose.png" width="50" height="20"></td>
<td>Light Rose</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_red.png" width="50" height="20"></td>
<td>Light Red</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rose_quartz.png" width="50" height="20"></td>
<td>Rose Quartz</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/warm_pink.png" width="50" height="20"></td>
<td>Warm Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_rose.png" width="50" height="20"></td>
<td>Deep Rose</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink.png" width="50" height="20"></td>
<td>Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightpink.png" width="50" height="20"></td>
<td>LightPink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/soft_pink.png" width="50" height="20"></td>
<td>Soft Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/powder_pink.png" width="50" height="20"></td>
<td>Powder Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/donut_pink.png" width="50" height="20"></td>
<td>Donut Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/baby_pink.png" width="50" height="20"></td>
<td>Baby Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/flamingo_pink.png" width="50" height="20"></td>
<td>Flamingo Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_pink.png" width="50" height="20"></td>
<td>Pastel Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rose_pink.png" width="50" height="20"></td>
<td>Rose Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cadillac_pink.png" width="50" height="20"></td>
<td>Cadillac Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/carnation_pink.png" width="50" height="20"></td>
<td>Carnation Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_rose.png" width="50" height="20"></td>
<td>Pastel Rose</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blush_red.png" width="50" height="20"></td>
<td>Blush Red</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/palevioletred.png" width="50" height="20"></td>
<td>PaleVioletRed</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_pink.png" width="50" height="20"></td>
<td>Purple Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tulip_pink.png" width="50" height="20"></td>
<td>Tulip Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bashful_pink.png" width="50" height="20"></td>
<td>Bashful Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_pink.png" width="50" height="20"></td>
<td>Dark Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_hot_pink.png" width="50" height="20"></td>
<td>Dark Hot Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/hotpink.png" width="50" height="20"></td>
<td>HotPink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/watermelon_pink.png" width="50" height="20"></td>
<td>Watermelon Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/violet_red.png" width="50" height="20"></td>
<td>Violet Red</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/hot_deep_pink.png" width="50" height="20"></td>
<td>Hot Deep Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_pink.png" width="50" height="20"></td>
<td>Bright Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_magenta.png" width="50" height="20"></td>
<td>Red Magenta</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deeppink.png" width="50" height="20"></td>
<td>DeepPink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_pink.png" width="50" height="20"></td>
<td>Neon Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chrome_pink.png" width="50" height="20"></td>
<td>Chrome Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_hot_pink.png" width="50" height="20"></td>
<td>Neon Hot Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_cupcake.png" width="50" height="20"></td>
<td>Pink Cupcake</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/royal_pink.png" width="50" height="20"></td>
<td>Royal Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dimorphotheca_magenta.png" width="50" height="20"></td>
<td>Dimorphotheca Magenta</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/barbie_pink.png" width="50" height="20"></td>
<td>Barbie Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_lemonade.png" width="50" height="20"></td>
<td>Pink Lemonade</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_pink.png" width="50" height="20"></td>
<td>Red Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/raspberry.png" width="50" height="20"></td>
<td>Raspberry</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/crimson.png" width="50" height="20"></td>
<td>Crimson</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_maroon.png" width="50" height="20"></td>
<td>Bright Maroon</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rose_red.png" width="50" height="20"></td>
<td>Rose Red</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rogue_pink.png" width="50" height="20"></td>
<td>Rogue Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/burnt_pink.png" width="50" height="20"></td>
<td>Burnt Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_violet.png" width="50" height="20"></td>
<td>Pink Violet</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/magenta_pink.png" width="50" height="20"></td>
<td>Magenta Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumvioletred.png" width="50" height="20"></td>
<td>MediumVioletRed</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_carnation_pink.png" width="50" height="20"></td>
<td>Dark Carnation Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/raspberry_purple.png" width="50" height="20"></td>
<td>Raspberry Purple</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_plum.png" width="50" height="20"></td>
<td>Pink Plum</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/orchid.png" width="50" height="20"></td>
<td>Orchid</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_mauve.png" width="50" height="20"></td>
<td>Deep Mauve</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/violet.png" width="50" height="20"></td>
<td>Violet</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/fuchsia_pink.png" width="50" height="20"></td>
<td>Fuchsia Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_neon_pink.png" width="50" height="20"></td>
<td>Bright Neon Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/magenta.png" width="50" height="20"></td>
<td>Magenta</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/fuchsia.png" width="50" height="20"></td>
<td>Fuchsia</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/crimson_purple.png" width="50" height="20"></td>
<td>Crimson Purple</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/heliotrope_purple.png" width="50" height="20"></td>
<td>Heliotrope Purple</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tyrian_purple.png" width="50" height="20"></td>
<td>Tyrian Purple</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumorchid.png" width="50" height="20"></td>
<td>MediumOrchid</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_flower.png" width="50" height="20"></td>
<td>Purple Flower</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/orchid_purple.png" width="50" height="20"></td>
<td>Orchid Purple</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rich_lilac.png" width="50" height="20"></td>
<td>Rich Lilac</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_violet.png" width="50" height="20"></td>
<td>Pastel Violet</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rosy.png" width="50" height="20"></td>
<td>Rosy</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mauve_taupe.png" width="50" height="20"></td>
<td>Mauve Taupe</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/viola_purple.png" width="50" height="20"></td>
<td>Viola Purple</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/eggplant.png" width="50" height="20"></td>
<td>Eggplant</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/plum_purple.png" width="50" height="20"></td>
<td>Plum Purple</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/grape.png" width="50" height="20"></td>
<td>Grape</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_navy.png" width="50" height="20"></td>
<td>Purple Navy</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/slateblue.png" width="50" height="20"></td>
<td>SlateBlue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_lotus.png" width="50" height="20"></td>
<td>Blue Lotus</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blurple.png" width="50" height="20"></td>
<td>Blurple</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_slate_blue.png" width="50" height="20"></td>
<td>Light Slate Blue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumslateblue.png" width="50" height="20"></td>
<td>MediumSlateBlue</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/periwinkle_purple.png" width="50" height="20"></td>
<td>Periwinkle Purple</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/very_peri.png" width="50" height="20"></td>
<td>Very Peri</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_grape.png" width="50" height="20"></td>
<td>Bright Grape</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_purple.png" width="50" height="20"></td>
<td>Bright Purple</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_amethyst.png" width="50" height="20"></td>
<td>Purple Amethyst</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_magenta.png" width="50" height="20"></td>
<td>Blue Magenta</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_blurple.png" width="50" height="20"></td>
<td>Dark Blurple</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_periwinkle.png" width="50" height="20"></td>
<td>Deep Periwinkle</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkslateblue.png" width="50" height="20"></td>
<td>DarkSlateBlue</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_haze.png" width="50" height="20"></td>
<td>Purple Haze</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_iris.png" width="50" height="20"></td>
<td>Purple Iris</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_purple.png" width="50" height="20"></td>
<td>Dark Purple</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_purple.png" width="50" height="20"></td>
<td>Deep Purple</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/midnight_purple.png" width="50" height="20"></td>
<td>Midnight Purple</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_monster.png" width="50" height="20"></td>
<td>Purple Monster</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/indigo.png" width="50" height="20"></td>
<td>Indigo</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_whale.png" width="50" height="20"></td>
<td>Blue Whale</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rebeccapurple.png" width="50" height="20"></td>
<td>RebeccaPurple</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_jam.png" width="50" height="20"></td>
<td>Purple Jam</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkmagenta.png" width="50" height="20"></td>
<td>DarkMagenta</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple.png" width="50" height="20"></td>
<td>Purple</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/french_lilac.png" width="50" height="20"></td>
<td>French Lilac</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkorchid.png" width="50" height="20"></td>
<td>DarkOrchid</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkviolet.png" width="50" height="20"></td>
<td>DarkViolet</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_violet.png" width="50" height="20"></td>
<td>Purple Violet</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/jasmine_purple.png" width="50" height="20"></td>
<td>Jasmine Purple</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_daffodil.png" width="50" height="20"></td>
<td>Purple Daffodil</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/clematis_violet.png" width="50" height="20"></td>
<td>Clematis Violet</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blueviolet.png" width="50" height="20"></td>
<td>BlueViolet</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_sage_bush.png" width="50" height="20"></td>
<td>Purple Sage Bush</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lovely_purple.png" width="50" height="20"></td>
<td>Lovely Purple</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_purple.png" width="50" height="20"></td>
<td>Neon Purple</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_plum.png" width="50" height="20"></td>
<td>Purple Plum</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aztech_purple.png" width="50" height="20"></td>
<td>Aztech Purple</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumpurple.png" width="50" height="20"></td>
<td>MediumPurple</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_purple.png" width="50" height="20"></td>
<td>Light Purple</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/crocus_purple.png" width="50" height="20"></td>
<td>Crocus Purple</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_mimosa.png" width="50" height="20"></td>
<td>Purple Mimosa</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_indigo.png" width="50" height="20"></td>
<td>Pastel Indigo</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lavender_purple.png" width="50" height="20"></td>
<td>Lavender Purple</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rose_purple.png" width="50" height="20"></td>
<td>Rose Purple</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/viola.png" width="50" height="20"></td>
<td>Viola</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/periwinkle.png" width="50" height="20"></td>
<td>Periwinkle</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pale_lilac.png" width="50" height="20"></td>
<td>Pale Lilac</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lilac.png" width="50" height="20"></td>
<td>Lilac</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mauve.png" width="50" height="20"></td>
<td>Mauve</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_lilac.png" width="50" height="20"></td>
<td>Bright Lilac</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_dragon.png" width="50" height="20"></td>
<td>Purple Dragon</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/plum.png" width="50" height="20"></td>
<td>Plum</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blush_pink.png" width="50" height="20"></td>
<td>Blush Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_purple.png" width="50" height="20"></td>
<td>Pastel Purple</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blossom_pink.png" width="50" height="20"></td>
<td>Blossom Pink</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/wisteria_purple.png" width="50" height="20"></td>
<td>Wisteria Purple</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_thistle.png" width="50" height="20"></td>
<td>Purple Thistle</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/thistle.png" width="50" height="20"></td>
<td>Thistle</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_white.png" width="50" height="20"></td>
<td>Purple White</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/periwinkle_pink.png" width="50" height="20"></td>
<td>Periwinkle Pink</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cotton_candy.png" width="50" height="20"></td>
<td>Cotton Candy</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lavender_pinocchio.png" width="50" height="20"></td>
<td>Lavender Pinocchio</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_white.png" width="50" height="20"></td>
<td>Dark White</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ash_white.png" width="50" height="20"></td>
<td>Ash White</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/warm_white.png" width="50" height="20"></td>
<td>Warm White</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/white_chocolate.png" width="50" height="20"></td>
<td>White Chocolate</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/creamy_white.png" width="50" height="20"></td>
<td>Creamy White</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/off_white.png" width="50" height="20"></td>
<td>Off White</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/soft_ivory.png" width="50" height="20"></td>
<td>Soft Ivory</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cosmic_latte.png" width="50" height="20"></td>
<td>Cosmic Latte</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pearl_white.png" width="50" height="20"></td>
<td>Pearl White</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_white.png" width="50" height="20"></td>
<td>Red White</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lavenderblush.png" width="50" height="20"></td>
<td>LavenderBlush</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pearl.png" width="50" height="20"></td>
<td>Pearl</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/egg_shell.png" width="50" height="20"></td>
<td>Egg Shell</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/oldlace.png" width="50" height="20"></td>
<td>OldLace</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/white_ice.png" width="50" height="20"></td>
<td>White Ice</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/linen.png" width="50" height="20"></td>
<td>Linen</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/seashell.png" width="50" height="20"></td>
<td>SeaShell</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bone_white.png" width="50" height="20"></td>
<td>Bone White</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rice.png" width="50" height="20"></td>
<td>Rice</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/floralwhite.png" width="50" height="20"></td>
<td>FloralWhite</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ivory.png" width="50" height="20"></td>
<td>Ivory</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/white_gold.png" width="50" height="20"></td>
<td>White Gold</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_white.png" width="50" height="20"></td>
<td>Light White</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cotton.png" width="50" height="20"></td>
<td>Cotton</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/snow.png" width="50" height="20"></td>
<td>Snow</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/milk_white.png" width="50" height="20"></td>
<td>Milk White</td>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/half_white.png" width="50" height="20"></td>
<td>Half White</td>
</tr>
<tr>
<td width="50"><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/white.png" width="50" height="20"></td>
<td>White</td>
</tr>
</table>
