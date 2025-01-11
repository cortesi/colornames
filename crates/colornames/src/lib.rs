mod color_data;
mod colors;

use color_data::COLORS;
pub use colors::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_data() -> Result<(), Box<dyn std::error::Error>> {
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

        // We implement Display for Color. Variants are displayed as their names.
        assert_eq!(format!("{}", Color::PinkLemonade), "PinkLemonade");
        // ... or as hex codes for the Rgb variant
        assert_eq!(format!("{}", Color::Rgb(0, 0, 17)), "#000011");

        // Colors can be converted to names, hex codes, and RGB values
        let c: Color = Color::PinkLemonade;
        assert_eq!(c.name(), Some("Pink Lemonade".to_string()));
        assert_eq!(c.rgb_hex(), "#E4287C".to_string());
        assert_eq!(c.rgb(), (228, 40, 124));

        let c: Color = Color::Rgb(0, 0, 17);
        assert_eq!(c.name(), None);
        assert_eq!(c.rgb_hex(), "#000011".to_string());
        assert_eq!(c.rgb(), (0, 0, 17));

        Ok(())
    }
}
