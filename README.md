# colornames

This crate does one thing only: it provides an enum of color names, with a
catchall RGB variant for un-named colors. Color variants can be constructed by
name or by hex value.

On first use, the crate generates lookup tables for color names and hex values.
There are 728 named colors, so this incurs a small startup cost.

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

<table style='border-collapse: collapse;'>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/black.png" width="50" height="20"></td>
<td style='padding: 5px;'>Black</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/black_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Black Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/night.png" width="50" height="20"></td>
<td style='padding: 5px;'>Night</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/charcoal.png" width="50" height="20"></td>
<td style='padding: 5px;'>Charcoal</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/oil.png" width="50" height="20"></td>
<td style='padding: 5px;'>Oil</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/stormy_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Stormy Gray</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_black.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Black</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_steampunk.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Steampunk</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/black_cat.png" width="50" height="20"></td>
<td style='padding: 5px;'>Black Cat</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/iridium.png" width="50" height="20"></td>
<td style='padding: 5px;'>Iridium</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/black_eel.png" width="50" height="20"></td>
<td style='padding: 5px;'>Black Eel</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/black_cow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Black Cow</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gray_wolf.png" width="50" height="20"></td>
<td style='padding: 5px;'>Gray Wolf</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/vampire_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Vampire Gray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/iron_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Iron Gray</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gray_dolphin.png" width="50" height="20"></td>
<td style='padding: 5px;'>Gray Dolphin</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/carbon_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Carbon Gray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ash_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Ash Gray</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dimgray.png" width="50" height="20"></td>
<td style='padding: 5px;'>DimGray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dimgrey.png" width="50" height="20"></td>
<td style='padding: 5px;'>DimGrey</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/nardo_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Nardo Gray</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cloudy_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cloudy Gray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/smokey_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Smokey Gray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/alien_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Alien Gray</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sonic_silver.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sonic Silver</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/platinum_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Platinum Gray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/granite.png" width="50" height="20"></td>
<td style='padding: 5px;'>Granite</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Gray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/grey.png" width="50" height="20"></td>
<td style='padding: 5px;'>Grey</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/battleship_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Battleship Gray</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sheet_metal.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sheet Metal</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_gainsboro.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Gainsboro</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gunmetal_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Gunmetal Gray</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cold_metal.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cold Metal</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/stainless_steel_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Stainless Steel Gray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkgrey.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkGrey</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkgray.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkGray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chrome_aluminum.png" width="50" height="20"></td>
<td style='padding: 5px;'>Chrome Aluminum</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gray_cloud.png" width="50" height="20"></td>
<td style='padding: 5px;'>Gray Cloud</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/metal.png" width="50" height="20"></td>
<td style='padding: 5px;'>Metal</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/silver.png" width="50" height="20"></td>
<td style='padding: 5px;'>Silver</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/steampunk.png" width="50" height="20"></td>
<td style='padding: 5px;'>Steampunk</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pale_silver.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pale Silver</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gear_steel_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Gear Steel Gray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gray_goose.png" width="50" height="20"></td>
<td style='padding: 5px;'>Gray Goose</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/platinum_silver.png" width="50" height="20"></td>
<td style='padding: 5px;'>Platinum Silver</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightgrey.png" width="50" height="20"></td>
<td style='padding: 5px;'>LightGrey</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightgray.png" width="50" height="20"></td>
<td style='padding: 5px;'>LightGray</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/silver_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Silver White</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gainsboro.png" width="50" height="20"></td>
<td style='padding: 5px;'>Gainsboro</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_steel_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Steel Gray</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/whitesmoke.png" width="50" height="20"></td>
<td style='padding: 5px;'>WhiteSmoke</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/white_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>White Gray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/platinum.png" width="50" height="20"></td>
<td style='padding: 5px;'>Platinum</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/metallic_silver.png" width="50" height="20"></td>
<td style='padding: 5px;'>Metallic Silver</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Gray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/roman_silver.png" width="50" height="20"></td>
<td style='padding: 5px;'>Roman Silver</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightslategrey.png" width="50" height="20"></td>
<td style='padding: 5px;'>LightSlateGrey</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightslategray.png" width="50" height="20"></td>
<td style='padding: 5px;'>LightSlateGray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/slategrey.png" width="50" height="20"></td>
<td style='padding: 5px;'>SlateGrey</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/slategray.png" width="50" height="20"></td>
<td style='padding: 5px;'>SlateGray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rat_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rat Gray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/slate_granite_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Slate Granite Gray</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/jet_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Jet Gray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mist_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Mist Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/steel_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Steel Gray</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/marble_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Marble Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/slate_blue_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Slate Blue Gray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_purple_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Purple Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/azure_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Azure Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/estoril_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Estoril Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_jay.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Jay</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/charcoal_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Charcoal Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_blue_gray.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Blue Gray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_slate.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Slate</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_sea_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deep Sea Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/night_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Night Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/midnightblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>MidnightBlue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/navy.png" width="50" height="20"></td>
<td style='padding: 5px;'>Navy</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/denim_dark_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Denim Dark Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkBlue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lapis_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Lapis Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/new_midnight_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>New Midnight Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/earth_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Earth Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cobalt_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cobalt Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>MediumBlue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blueberry_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blueberry Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/canary_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Canary Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/samco_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Samco Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bright Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_orchid.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Orchid</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sapphire_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sapphire Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_eyes.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Eyes</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_navy_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bright Navy Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/balloon_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Balloon Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/royalblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>RoyalBlue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ocean_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Ocean Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_sky_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Sky Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_ribbon.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Ribbon</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_dress.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Dress</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Neon Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dodgerblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>DodgerBlue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/glacial_blue_ice.png" width="50" height="20"></td>
<td style='padding: 5px;'>Glacial Blue Ice</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/steelblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>SteelBlue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/silk_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Silk Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/windows_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Windows Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_ivy.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Ivy</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cyan_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cyan Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_koi.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Koi</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/columbia_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Columbia Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/baby_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Baby Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cornflowerblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>CornflowerBlue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sky_blue_dress.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sky Blue Dress</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/iceberg.png" width="50" height="20"></td>
<td style='padding: 5px;'>Iceberg</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/butterfly_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Butterfly Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deepskyblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>DeepSkyBlue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/midday_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Midday Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/crystal_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Crystal Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/denim_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Denim Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/day_sky_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Day Sky Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightskyblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>LightSkyBlue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/skyblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>SkyBlue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/jeans_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Jeans Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_angel.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Angel</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pastel Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_day_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Day Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sea_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sea Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/heavenly_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Heavenly Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/robin_egg_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Robin Egg Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/powderblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>PowderBlue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/coral_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Coral Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>LightBlue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightsteelblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>LightSteelBlue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gulf_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Gulf Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_light_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pastel Light Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lavender_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Lavender Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/white_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>White Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lavender.png" width="50" height="20"></td>
<td style='padding: 5px;'>Lavender</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/water.png" width="50" height="20"></td>
<td style='padding: 5px;'>Water</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aliceblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>AliceBlue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ghostwhite.png" width="50" height="20"></td>
<td style='padding: 5px;'>GhostWhite</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/azure.png" width="50" height="20"></td>
<td style='padding: 5px;'>Azure</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightcyan.png" width="50" height="20"></td>
<td style='padding: 5px;'>LightCyan</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_slate.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Slate</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/electric_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Electric Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tron_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Tron Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_zircon.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Zircon</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cyan.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cyan</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aqua.png" width="50" height="20"></td>
<td style='padding: 5px;'>Aqua</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_cyan.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bright Cyan</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/celeste.png" width="50" height="20"></td>
<td style='padding: 5px;'>Celeste</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_diamond.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Diamond</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_turquoise.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bright Turquoise</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_lagoon.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Lagoon</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/paleturquoise.png" width="50" height="20"></td>
<td style='padding: 5px;'>PaleTurquoise</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pale_blue_lily.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pale Blue Lily</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_teal.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Teal</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tiffany_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Tiffany Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_hosta.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Hosta</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cyan_opaque.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cyan Opaque</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/northern_lights_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Northern Lights Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumaquamarine.png" width="50" height="20"></td>
<td style='padding: 5px;'>MediumAquaMarine</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aqua_seafoam_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Aqua Seafoam Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/magic_mint.png" width="50" height="20"></td>
<td style='padding: 5px;'>Magic Mint</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_aquamarine.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Aquamarine</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aquamarine.png" width="50" height="20"></td>
<td style='padding: 5px;'>Aquamarine</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_teal.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bright Teal</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/turquoise.png" width="50" height="20"></td>
<td style='padding: 5px;'>Turquoise</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumturquoise.png" width="50" height="20"></td>
<td style='padding: 5px;'>MediumTurquoise</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_turquoise.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deep Turquoise</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/jellyfish.png" width="50" height="20"></td>
<td style='padding: 5px;'>Jellyfish</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_turquoise.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Turquoise</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkturquoise.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkTurquoise</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/macaw_blue_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Macaw Blue Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightseagreen.png" width="50" height="20"></td>
<td style='padding: 5px;'>LightSeaGreen</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/seafoam_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Seafoam Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cadetblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>CadetBlue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_sea.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deep Sea</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkcyan.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkCyan</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/teal_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Teal Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/teal.png" width="50" height="20"></td>
<td style='padding: 5px;'>Teal</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/teal_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Teal Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/medium_teal.png" width="50" height="20"></td>
<td style='padding: 5px;'>Medium Teal</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_teal.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Teal</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_teal.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deep Teal</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkslategray.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkSlateGray</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkslategrey.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkSlateGrey</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gunmetal.png" width="50" height="20"></td>
<td style='padding: 5px;'>Gunmetal</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_moss_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Moss Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/beetle_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Beetle Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/grayish_turquoise.png" width="50" height="20"></td>
<td style='padding: 5px;'>Grayish Turquoise</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/greenish_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Greenish Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aquamarine_stone.png" width="50" height="20"></td>
<td style='padding: 5px;'>Aquamarine Stone</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sea_turtle_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sea Turtle Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dull_sea_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dull Sea Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_green_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Green Blue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_sea_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deep Sea Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bottle_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bottle Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/seagreen.png" width="50" height="20"></td>
<td style='padding: 5px;'>SeaGreen</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/elf_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Elf Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_mint.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Mint</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/jade.png" width="50" height="20"></td>
<td style='padding: 5px;'>Jade</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/earth_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Earth Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chrome_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Chrome Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mint.png" width="50" height="20"></td>
<td style='padding: 5px;'>Mint</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/emerald.png" width="50" height="20"></td>
<td style='padding: 5px;'>Emerald</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/isle_of_man_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Isle Of Man Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumseagreen.png" width="50" height="20"></td>
<td style='padding: 5px;'>MediumSeaGreen</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/metallic_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Metallic Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/camouflage_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Camouflage Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sage_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sage Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/hazel_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Hazel Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/venom_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Venom Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/olivedrab.png" width="50" height="20"></td>
<td style='padding: 5px;'>OliveDrab</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/olive.png" width="50" height="20"></td>
<td style='padding: 5px;'>Olive</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ebony.png" width="50" height="20"></td>
<td style='padding: 5px;'>Ebony</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkolivegreen.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkOliveGreen</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/military_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Military Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green_leaves.png" width="50" height="20"></td>
<td style='padding: 5px;'>Green Leaves</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/army_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Army Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/fern_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Fern Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/fall_forest_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Fall Forest Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/irish_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Irish Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pine_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pine Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/medium_forest_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Medium Forest Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/racing_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Racing Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/jungle_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Jungle Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cactus_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cactus Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/forestgreen.png" width="50" height="20"></td>
<td style='padding: 5px;'>ForestGreen</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkgreen.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkGreen</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deep Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_emerald_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deep Emerald Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/hunter_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Hunter Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_forest_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Forest Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lotus_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Lotus Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/broccoli_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Broccoli Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/seaweed_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Seaweed Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/shamrock_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Shamrock Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green_onion.png" width="50" height="20"></td>
<td style='padding: 5px;'>Green Onion</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/moss_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Moss Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/grass_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Grass Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green_pepper.png" width="50" height="20"></td>
<td style='padding: 5px;'>Green Pepper</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_lime_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Lime Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/parrot_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Parrot Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/clover_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Clover Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dinosaur_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dinosaur Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green_snake.png" width="50" height="20"></td>
<td style='padding: 5px;'>Green Snake</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/alien_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Alien Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green_apple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Green Apple</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/limegreen.png" width="50" height="20"></td>
<td style='padding: 5px;'>LimeGreen</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pea_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pea Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/kelly_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Kelly Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/zombie_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Zombie Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green_peas.png" width="50" height="20"></td>
<td style='padding: 5px;'>Green Peas</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dollar_bill_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dollar Bill Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/frog_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Frog Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/turquoise_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Turquoise Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkseagreen.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkSeaGreen</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/basil_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Basil Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gray_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Gray Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_olive_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Olive Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/iguana_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Iguana Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/citron_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Citron Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/acid_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Acid Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/avocado_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Avocado Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pistachio_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pistachio Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/salad_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Salad Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/yellowgreen.png" width="50" height="20"></td>
<td style='padding: 5px;'>YellowGreen</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pastel Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/hummingbird_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Hummingbird Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/nebula_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Nebula Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/stoplight_go_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Stoplight Go Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Neon Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/jade_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Jade Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/springgreen.png" width="50" height="20"></td>
<td style='padding: 5px;'>SpringGreen</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ocean_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Ocean Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lime_mint_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Lime Mint Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumspringgreen.png" width="50" height="20"></td>
<td style='padding: 5px;'>MediumSpringGreen</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aqua_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Aqua Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/emerald_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Emerald Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lime.png" width="50" height="20"></td>
<td style='padding: 5px;'>Lime</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lawngreen.png" width="50" height="20"></td>
<td style='padding: 5px;'>LawnGreen</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bright Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chartreuse.png" width="50" height="20"></td>
<td style='padding: 5px;'>Chartreuse</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/yellow_lawn_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Yellow Lawn Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aloe_vera_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Aloe Vera Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dull_green_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dull Green Yellow</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lemon_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Lemon Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/greenyellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>GreenYellow</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chameleon_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Chameleon Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_yellow_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Neon Yellow Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/yellow_green_grosbeak.png" width="50" height="20"></td>
<td style='padding: 5px;'>Yellow Green Grosbeak</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tea_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Tea Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/slime_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Slime Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/algae_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Algae Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightgreen.png" width="50" height="20"></td>
<td style='padding: 5px;'>LightGreen</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dragon_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dragon Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/palegreen.png" width="50" height="20"></td>
<td style='padding: 5px;'>PaleGreen</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mint_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Mint Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/green_thumb.png" width="50" height="20"></td>
<td style='padding: 5px;'>Green Thumb</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/organic_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Organic Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_jade.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Jade</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_mint_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Mint Green</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_rose_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Rose Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chrome_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Chrome White</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/honeydew.png" width="50" height="20"></td>
<td style='padding: 5px;'>HoneyDew</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mintcream.png" width="50" height="20"></td>
<td style='padding: 5px;'>MintCream</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lemonchiffon.png" width="50" height="20"></td>
<td style='padding: 5px;'>LemonChiffon</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/parchment.png" width="50" height="20"></td>
<td style='padding: 5px;'>Parchment</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cream.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cream</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cream_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cream White</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightgoldenrodyellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>LightGoldenRodYellow</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightyellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>LightYellow</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/beige.png" width="50" height="20"></td>
<td style='padding: 5px;'>Beige</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/white_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>White Yellow</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cornsilk.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cornsilk</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blonde.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blonde</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/antiquewhite.png" width="50" height="20"></td>
<td style='padding: 5px;'>AntiqueWhite</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_beige.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Beige</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/papayawhip.png" width="50" height="20"></td>
<td style='padding: 5px;'>PapayaWhip</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/champagne.png" width="50" height="20"></td>
<td style='padding: 5px;'>Champagne</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blanchedalmond.png" width="50" height="20"></td>
<td style='padding: 5px;'>BlanchedAlmond</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bisque.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bisque</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/wheat.png" width="50" height="20"></td>
<td style='padding: 5px;'>Wheat</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/moccasin.png" width="50" height="20"></td>
<td style='padding: 5px;'>Moccasin</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/peach.png" width="50" height="20"></td>
<td style='padding: 5px;'>Peach</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Orange</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/peachpuff.png" width="50" height="20"></td>
<td style='padding: 5px;'>PeachPuff</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/coral_peach.png" width="50" height="20"></td>
<td style='padding: 5px;'>Coral Peach</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/navajowhite.png" width="50" height="20"></td>
<td style='padding: 5px;'>NavajoWhite</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/golden_blonde.png" width="50" height="20"></td>
<td style='padding: 5px;'>Golden Blonde</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/golden_silk.png" width="50" height="20"></td>
<td style='padding: 5px;'>Golden Silk</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_blonde.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Blonde</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_gold.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Gold</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/vanilla.png" width="50" height="20"></td>
<td style='padding: 5px;'>Vanilla</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tan_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Tan Brown</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dirty_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dirty White</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/palegoldenrod.png" width="50" height="20"></td>
<td style='padding: 5px;'>PaleGoldenRod</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/khaki.png" width="50" height="20"></td>
<td style='padding: 5px;'>Khaki</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cardboard_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cardboard Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/harvest_gold.png" width="50" height="20"></td>
<td style='padding: 5px;'>Harvest Gold</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sun_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sun Yellow</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/corn_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Corn Yellow</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pastel Yellow</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Neon Yellow</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Yellow</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lemon_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Lemon Yellow</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/canary_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Canary Yellow</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/banana_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Banana Yellow</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mustard_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Mustard Yellow</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/golden_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Golden Yellow</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bold_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bold Yellow</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/safety_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Safety Yellow</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rubber_ducky_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rubber Ducky Yellow</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gold.png" width="50" height="20"></td>
<td style='padding: 5px;'>Gold</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_gold.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bright Gold</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chrome_gold.png" width="50" height="20"></td>
<td style='padding: 5px;'>Chrome Gold</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/golden_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Golden Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deep Yellow</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/macaroni_and_cheese.png" width="50" height="20"></td>
<td style='padding: 5px;'>Macaroni and Cheese</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/amber.png" width="50" height="20"></td>
<td style='padding: 5px;'>Amber</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/saffron.png" width="50" height="20"></td>
<td style='padding: 5px;'>Saffron</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_gold.png" width="50" height="20"></td>
<td style='padding: 5px;'>Neon Gold</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/beer.png" width="50" height="20"></td>
<td style='padding: 5px;'>Beer</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/yellow_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Yellow Orange</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/orange_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Orange Yellow</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cantaloupe.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cantaloupe</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cheese_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cheese Orange</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Orange</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/brown_sand.png" width="50" height="20"></td>
<td style='padding: 5px;'>Brown Sand</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sandybrown.png" width="50" height="20"></td>
<td style='padding: 5px;'>SandyBrown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/brown_sugar.png" width="50" height="20"></td>
<td style='padding: 5px;'>Brown Sugar</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/camel_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Camel Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deer_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deer Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/burlywood.png" width="50" height="20"></td>
<td style='padding: 5px;'>BurlyWood</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tan.png" width="50" height="20"></td>
<td style='padding: 5px;'>Tan</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_french_beige.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light French Beige</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sand.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sand</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/soft_hazel.png" width="50" height="20"></td>
<td style='padding: 5px;'>Soft Hazel</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sage.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sage</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/fall_leaf_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Fall Leaf Brown</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ginger_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Ginger Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bronze_gold.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bronze Gold</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkkhaki.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkKhaki</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/olive_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Olive Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/brass.png" width="50" height="20"></td>
<td style='padding: 5px;'>Brass</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cookie_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cookie Brown</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/metallic_gold.png" width="50" height="20"></td>
<td style='padding: 5px;'>Metallic Gold</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mustard.png" width="50" height="20"></td>
<td style='padding: 5px;'>Mustard</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bee_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bee Yellow</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/school_bus_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>School Bus Yellow</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/goldenrod.png" width="50" height="20"></td>
<td style='padding: 5px;'>GoldenRod</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/orange_gold.png" width="50" height="20"></td>
<td style='padding: 5px;'>Orange Gold</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/caramel.png" width="50" height="20"></td>
<td style='padding: 5px;'>Caramel</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkgoldenrod.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkGoldenRod</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cinnamon.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cinnamon</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/peru.png" width="50" height="20"></td>
<td style='padding: 5px;'>Peru</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bronze.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bronze</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pumpkin_pie.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pumpkin Pie</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tiger_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Tiger Orange</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/copper.png" width="50" height="20"></td>
<td style='padding: 5px;'>Copper</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_gold.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Gold</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/metallic_bronze.png" width="50" height="20"></td>
<td style='padding: 5px;'>Metallic Bronze</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_almond.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Almond</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/wood.png" width="50" height="20"></td>
<td style='padding: 5px;'>Wood</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/khaki_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Khaki Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/oak_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Oak Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/antique_bronze.png" width="50" height="20"></td>
<td style='padding: 5px;'>Antique Bronze</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/hazel.png" width="50" height="20"></td>
<td style='padding: 5px;'>Hazel</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_yellow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Yellow</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_moccasin.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Moccasin</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/khaki_green.png" width="50" height="20"></td>
<td style='padding: 5px;'>Khaki Green</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/millennium_jade.png" width="50" height="20"></td>
<td style='padding: 5px;'>Millennium Jade</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_beige.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Beige</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bullet_shell.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bullet Shell</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/army_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Army Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sandstone.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sandstone</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/taupe.png" width="50" height="20"></td>
<td style='padding: 5px;'>Taupe</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_grayish_olive.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Grayish Olive</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_hazel_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Hazel Brown</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mocha.png" width="50" height="20"></td>
<td style='padding: 5px;'>Mocha</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/milk_chocolate.png" width="50" height="20"></td>
<td style='padding: 5px;'>Milk Chocolate</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gray_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Gray Brown</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_coffee.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Coffee</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/western_charcoal.png" width="50" height="20"></td>
<td style='padding: 5px;'>Western Charcoal</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/old_burgundy.png" width="50" height="20"></td>
<td style='padding: 5px;'>Old Burgundy</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Red Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bakers_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bakers Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pullman_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pullman Brown</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sepia_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sepia Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_bronze.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Bronze</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/coffee.png" width="50" height="20"></td>
<td style='padding: 5px;'>Coffee</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/brown_bear.png" width="50" height="20"></td>
<td style='padding: 5px;'>Brown Bear</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_dirt.png" width="50" height="20"></td>
<td style='padding: 5px;'>Red Dirt</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sepia.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sepia</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sienna.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sienna</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/saddlebrown.png" width="50" height="20"></td>
<td style='padding: 5px;'>SaddleBrown</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_sienna.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Sienna</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sangria.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sangria</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blood_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blood Red</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chestnut.png" width="50" height="20"></td>
<td style='padding: 5px;'>Chestnut</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/coral_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Coral Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_amber.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deep Amber</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chestnut_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Chestnut Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ginger_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Ginger Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mahogany.png" width="50" height="20"></td>
<td style='padding: 5px;'>Mahogany</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_gold.png" width="50" height="20"></td>
<td style='padding: 5px;'>Red Gold</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_fox.png" width="50" height="20"></td>
<td style='padding: 5px;'>Red Fox</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_bisque.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Bisque</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/petra_gold.png" width="50" height="20"></td>
<td style='padding: 5px;'>Petra Gold</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/brown_rust.png" width="50" height="20"></td>
<td style='padding: 5px;'>Brown Rust</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rust.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rust</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/copper_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Copper Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/orange_salmon.png" width="50" height="20"></td>
<td style='padding: 5px;'>Orange Salmon</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chocolate.png" width="50" height="20"></td>
<td style='padding: 5px;'>Chocolate</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sedona.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sedona</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/papaya_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Papaya Orange</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/halloween_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Halloween Orange</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Neon Orange</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bright Orange</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/fluro_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Fluro Orange</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pumpkin_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pumpkin Orange</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/safety_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Safety Orange</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/carrot_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Carrot Orange</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkorange.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkOrange</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/construction_cone_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Construction Cone Orange</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/indian_saffron.png" width="50" height="20"></td>
<td style='padding: 5px;'>Indian Saffron</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/sunrise_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Sunrise Orange</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mango_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Mango Orange</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/coral.png" width="50" height="20"></td>
<td style='padding: 5px;'>Coral</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/basket_ball_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Basket Ball Orange</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_salmon_rose.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Salmon Rose</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightsalmon.png" width="50" height="20"></td>
<td style='padding: 5px;'>LightSalmon</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pink Orange</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darksalmon.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkSalmon</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tangerine.png" width="50" height="20"></td>
<td style='padding: 5px;'>Tangerine</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_copper.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Copper</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/salmon_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Salmon Pink</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/salmon.png" width="50" height="20"></td>
<td style='padding: 5px;'>Salmon</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/peach_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Peach Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightcoral.png" width="50" height="20"></td>
<td style='padding: 5px;'>LightCoral</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pastel Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_coral.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pink Coral</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bean_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bean Red</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/valentine_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Valentine Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/indianred.png" width="50" height="20"></td>
<td style='padding: 5px;'>IndianRed</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tomato.png" width="50" height="20"></td>
<td style='padding: 5px;'>Tomato</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/shocking_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Shocking Orange</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/orangered.png" width="50" height="20"></td>
<td style='padding: 5px;'>OrangeRed</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Red</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Neon Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/scarlet_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Scarlet Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ruby_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Ruby Red</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ferrari_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Ferrari Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/fire_engine_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Fire Engine Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lava_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Lava Red</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/love_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Love Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/grapefruit.png" width="50" height="20"></td>
<td style='padding: 5px;'>Grapefruit</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/strawberry_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Strawberry Red</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cherry_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cherry Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chilli_pepper.png" width="50" height="20"></td>
<td style='padding: 5px;'>Chilli Pepper</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/firebrick.png" width="50" height="20"></td>
<td style='padding: 5px;'>FireBrick</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tomato_sauce_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Tomato Sauce Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/carbon_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Carbon Red</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cranberry.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cranberry</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/saffron_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Saffron Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/crimson_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Crimson Red</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_wine.png" width="50" height="20"></td>
<td style='padding: 5px;'>Red Wine</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/wine_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Wine Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkred.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkRed</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/maroon_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Maroon Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/maroon.png" width="50" height="20"></td>
<td style='padding: 5px;'>Maroon</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/burgundy.png" width="50" height="20"></td>
<td style='padding: 5px;'>Burgundy</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/vermilion.png" width="50" height="20"></td>
<td style='padding: 5px;'>Vermilion</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deep Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/garnet_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Garnet Red</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_blood.png" width="50" height="20"></td>
<td style='padding: 5px;'>Red Blood</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blood_night.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blood Night</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_scarlet.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Scarlet</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chocolate_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Chocolate Brown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/black_bean.png" width="50" height="20"></td>
<td style='padding: 5px;'>Black Bean</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_maroon.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Maroon</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/midnight.png" width="50" height="20"></td>
<td style='padding: 5px;'>Midnight</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_lily.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Lily</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_maroon.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Maroon</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/plum_pie.png" width="50" height="20"></td>
<td style='padding: 5px;'>Plum Pie</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/plum_velvet.png" width="50" height="20"></td>
<td style='padding: 5px;'>Plum Velvet</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_raspberry.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Raspberry</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/velvet_maroon.png" width="50" height="20"></td>
<td style='padding: 5px;'>Velvet Maroon</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rosy_finch.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rosy Finch</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dull_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dull Purple</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/puce.png" width="50" height="20"></td>
<td style='padding: 5px;'>Puce</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rose_dust.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rose Dust</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pastel Brown</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rosy_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rosy Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rosybrown.png" width="50" height="20"></td>
<td style='padding: 5px;'>RosyBrown</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/khaki_rose.png" width="50" height="20"></td>
<td style='padding: 5px;'>Khaki Rose</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lipstick_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Lipstick Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dusky_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dusky Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_brown.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pink Brown</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/old_rose.png" width="50" height="20"></td>
<td style='padding: 5px;'>Old Rose</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dusty_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dusty Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_daisy.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pink Daisy</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rose.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rose</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dusty_rose.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dusty Rose</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/silver_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Silver Pink</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/gold_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Gold Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rose_gold.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rose Gold</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_peach.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deep Peach</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_orange.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pastel Orange</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/desert_sand.png" width="50" height="20"></td>
<td style='padding: 5px;'>Desert Sand</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/unbleached_silk.png" width="50" height="20"></td>
<td style='padding: 5px;'>Unbleached Silk</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pig_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pig Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pale_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pale Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blush.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blush</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mistyrose.png" width="50" height="20"></td>
<td style='padding: 5px;'>MistyRose</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_bubble_gum.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pink Bubble Gum</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_rose.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Rose</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rose_quartz.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rose Quartz</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/warm_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Warm Pink</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_rose.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deep Rose</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lightpink.png" width="50" height="20"></td>
<td style='padding: 5px;'>LightPink</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/soft_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Soft Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/powder_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Powder Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/donut_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Donut Pink</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/baby_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Baby Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/flamingo_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Flamingo Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pastel Pink</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rose_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rose Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cadillac_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cadillac Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/carnation_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Carnation Pink</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_rose.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pastel Rose</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blush_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blush Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/palevioletred.png" width="50" height="20"></td>
<td style='padding: 5px;'>PaleVioletRed</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tulip_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Tulip Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bashful_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bashful Pink</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_hot_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Hot Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/hotpink.png" width="50" height="20"></td>
<td style='padding: 5px;'>HotPink</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/watermelon_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Watermelon Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/violet_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Violet Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/hot_deep_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Hot Deep Pink</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bright Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_magenta.png" width="50" height="20"></td>
<td style='padding: 5px;'>Red Magenta</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deeppink.png" width="50" height="20"></td>
<td style='padding: 5px;'>DeepPink</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Neon Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/chrome_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Chrome Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_hot_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Neon Hot Pink</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_cupcake.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pink Cupcake</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/royal_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Royal Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dimorphotheca_magenta.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dimorphotheca Magenta</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/barbie_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Barbie Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_lemonade.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pink Lemonade</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Red Pink</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/raspberry.png" width="50" height="20"></td>
<td style='padding: 5px;'>Raspberry</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/crimson.png" width="50" height="20"></td>
<td style='padding: 5px;'>Crimson</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_maroon.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bright Maroon</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rose_red.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rose Red</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rogue_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rogue Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/burnt_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Burnt Pink</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_violet.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pink Violet</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/magenta_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Magenta Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumvioletred.png" width="50" height="20"></td>
<td style='padding: 5px;'>MediumVioletRed</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_carnation_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Carnation Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/raspberry_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Raspberry Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pink_plum.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pink Plum</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/orchid.png" width="50" height="20"></td>
<td style='padding: 5px;'>Orchid</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_mauve.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deep Mauve</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/violet.png" width="50" height="20"></td>
<td style='padding: 5px;'>Violet</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/fuchsia_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Fuchsia Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_neon_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bright Neon Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/magenta.png" width="50" height="20"></td>
<td style='padding: 5px;'>Magenta</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/fuchsia.png" width="50" height="20"></td>
<td style='padding: 5px;'>Fuchsia</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/crimson_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Crimson Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/heliotrope_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Heliotrope Purple</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/tyrian_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Tyrian Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumorchid.png" width="50" height="20"></td>
<td style='padding: 5px;'>MediumOrchid</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_flower.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Flower</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/orchid_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Orchid Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rich_lilac.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rich Lilac</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_violet.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pastel Violet</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rosy.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rosy</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mauve_taupe.png" width="50" height="20"></td>
<td style='padding: 5px;'>Mauve Taupe</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/viola_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Viola Purple</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/eggplant.png" width="50" height="20"></td>
<td style='padding: 5px;'>Eggplant</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/plum_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Plum Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/grape.png" width="50" height="20"></td>
<td style='padding: 5px;'>Grape</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_navy.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Navy</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/slateblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>SlateBlue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_lotus.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Lotus</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blurple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blurple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_slate_blue.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Slate Blue</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumslateblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>MediumSlateBlue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/periwinkle_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Periwinkle Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/very_peri.png" width="50" height="20"></td>
<td style='padding: 5px;'>Very Peri</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_grape.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bright Grape</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bright Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_amethyst.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Amethyst</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_magenta.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Magenta</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_blurple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Blurple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_periwinkle.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deep Periwinkle</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkslateblue.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkSlateBlue</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_haze.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Haze</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_iris.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Iris</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark Purple</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/deep_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Deep Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/midnight_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Midnight Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_monster.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Monster</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/indigo.png" width="50" height="20"></td>
<td style='padding: 5px;'>Indigo</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blue_whale.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blue Whale</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rebeccapurple.png" width="50" height="20"></td>
<td style='padding: 5px;'>RebeccaPurple</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_jam.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Jam</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkmagenta.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkMagenta</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/french_lilac.png" width="50" height="20"></td>
<td style='padding: 5px;'>French Lilac</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkorchid.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkOrchid</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/darkviolet.png" width="50" height="20"></td>
<td style='padding: 5px;'>DarkViolet</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_violet.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Violet</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/jasmine_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Jasmine Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_daffodil.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Daffodil</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/clematis_violet.png" width="50" height="20"></td>
<td style='padding: 5px;'>Clematis Violet</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blueviolet.png" width="50" height="20"></td>
<td style='padding: 5px;'>BlueViolet</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_sage_bush.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Sage Bush</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lovely_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Lovely Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/neon_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Neon Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_plum.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Plum</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/aztech_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Aztech Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mediumpurple.png" width="50" height="20"></td>
<td style='padding: 5px;'>MediumPurple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light Purple</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/crocus_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Crocus Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_mimosa.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Mimosa</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_indigo.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pastel Indigo</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lavender_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Lavender Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rose_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rose Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/viola.png" width="50" height="20"></td>
<td style='padding: 5px;'>Viola</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/periwinkle.png" width="50" height="20"></td>
<td style='padding: 5px;'>Periwinkle</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pale_lilac.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pale Lilac</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lilac.png" width="50" height="20"></td>
<td style='padding: 5px;'>Lilac</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/mauve.png" width="50" height="20"></td>
<td style='padding: 5px;'>Mauve</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bright_lilac.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bright Lilac</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_dragon.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Dragon</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/plum.png" width="50" height="20"></td>
<td style='padding: 5px;'>Plum</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blush_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blush Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pastel_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pastel Purple</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/blossom_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Blossom Pink</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/wisteria_purple.png" width="50" height="20"></td>
<td style='padding: 5px;'>Wisteria Purple</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_thistle.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple Thistle</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/thistle.png" width="50" height="20"></td>
<td style='padding: 5px;'>Thistle</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/purple_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Purple White</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/periwinkle_pink.png" width="50" height="20"></td>
<td style='padding: 5px;'>Periwinkle Pink</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cotton_candy.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cotton Candy</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lavender_pinocchio.png" width="50" height="20"></td>
<td style='padding: 5px;'>Lavender Pinocchio</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/dark_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Dark White</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ash_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Ash White</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/warm_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Warm White</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/white_chocolate.png" width="50" height="20"></td>
<td style='padding: 5px;'>White Chocolate</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/creamy_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Creamy White</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/off_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Off White</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/soft_ivory.png" width="50" height="20"></td>
<td style='padding: 5px;'>Soft Ivory</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cosmic_latte.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cosmic Latte</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pearl_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pearl White</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/red_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Red White</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/lavenderblush.png" width="50" height="20"></td>
<td style='padding: 5px;'>LavenderBlush</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/pearl.png" width="50" height="20"></td>
<td style='padding: 5px;'>Pearl</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/egg_shell.png" width="50" height="20"></td>
<td style='padding: 5px;'>Egg Shell</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/oldlace.png" width="50" height="20"></td>
<td style='padding: 5px;'>OldLace</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/white_ice.png" width="50" height="20"></td>
<td style='padding: 5px;'>White Ice</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/linen.png" width="50" height="20"></td>
<td style='padding: 5px;'>Linen</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/seashell.png" width="50" height="20"></td>
<td style='padding: 5px;'>SeaShell</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/bone_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Bone White</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/rice.png" width="50" height="20"></td>
<td style='padding: 5px;'>Rice</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/floralwhite.png" width="50" height="20"></td>
<td style='padding: 5px;'>FloralWhite</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/ivory.png" width="50" height="20"></td>
<td style='padding: 5px;'>Ivory</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/white_gold.png" width="50" height="20"></td>
<td style='padding: 5px;'>White Gold</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/light_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Light White</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/cotton.png" width="50" height="20"></td>
<td style='padding: 5px;'>Cotton</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/snow.png" width="50" height="20"></td>
<td style='padding: 5px;'>Snow</td>
</tr>
<tr>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/milk_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Milk White</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/half_white.png" width="50" height="20"></td>
<td style='padding: 5px;'>Half White</td>
<td style='padding: 5px;'><img src="https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/white.png" width="50" height="20"></td>
<td style='padding: 5px;'>White</td>
</tr>
</table>
