mod colors;

pub use colors::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_data() -> Result<(), Box<dyn std::error::Error>> {
        let c: Color = "PinkLemonade".try_into()?;
        assert_eq!(c, Color::PinkLemonade);

        let c: Color = "pink lemonade".try_into()?;
        assert_eq!(c, Color::PinkLemonade);

        let c: Color = "pinklemonade".try_into()?;
        assert_eq!(c, Color::PinkLemonade);

        assert_eq!(c.rgb(), (228, 40, 124));
        assert_eq!(c.rgb_hex(), "#E4287C".to_string());
        assert_eq!(c.name(), "PinkLemonade");

        // Hex codes matching named colors return that color
        assert_eq!(Color::convert_str("#E4287C").unwrap(), Color::PinkLemonade);
        // Case is normalized
        assert_eq!(Color::convert_str("#E4287c").unwrap(), Color::PinkLemonade);
        // Short hex codes are expanded
        assert_eq!(Color::convert_str("#fff").unwrap(), Color::White);
        Ok(())
    }
}
