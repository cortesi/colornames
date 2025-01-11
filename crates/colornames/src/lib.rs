mod color_data;
mod colors;

pub use color_data::COLORS;
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

        // Color variants can be converted to names, hex codes, and RGB values
        let c: Color = Color::PinkLemonade;
        assert_eq!(c.name(), "Pink Lemonade");
        assert_eq!(c.rgb_hex(), "#E4287C".to_string());
        assert_eq!(c.rgb(), (228, 40, 124));

        Ok(())
    }
}
