mod colors;

pub use colors::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_data() {
        assert_eq!(Color::from_name("Red"), Some(Color::Red));
        assert_eq!(Color::Red.rgb(), Some((255, 0, 0)));
        assert_eq!(Color::Red.rgb_hex(), Some("#FF0000".to_string()));
        assert_eq!(Color::Red.name(), "Red");
    }
}
