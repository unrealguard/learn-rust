//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(PartialEq, Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(PartialEq, Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    ///
    /// # Examples:
    /// ```
    /// use art::kinds::*;
    /// use art::utils::mix;
    ///
    /// let mixed_color = mix(&PrimaryColor::Red, &PrimaryColor::Blue);
    /// assert_eq!(mixed_color, Ok(SecondaryColor::Purple));
    ///
    /// ```
    pub fn mix(
        c1: &PrimaryColor,
        c2: &PrimaryColor,
    ) -> Result<SecondaryColor, &'static str> {
        const INVALID_COMBINATION: Result<SecondaryColor, &str> =
            Err("Invalid color combination");
        match c1 {
            PrimaryColor::Red => match c2 {
                PrimaryColor::Red => INVALID_COMBINATION,
                PrimaryColor::Yellow => Ok(SecondaryColor::Green),
                PrimaryColor::Blue => Ok(SecondaryColor::Purple),
            },
            PrimaryColor::Yellow => match c2 {
                PrimaryColor::Yellow => INVALID_COMBINATION,
                PrimaryColor::Red => Ok(SecondaryColor::Green),
                PrimaryColor::Blue => Ok(SecondaryColor::Orange),
            },
            PrimaryColor::Blue => match c2 {
                PrimaryColor::Blue => INVALID_COMBINATION,
                PrimaryColor::Red => Ok(SecondaryColor::Purple),
                PrimaryColor::Yellow => Ok(SecondaryColor::Orange),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::kinds::PrimaryColor;

    use super::*;

    #[rstest]
    #[case(PrimaryColor::Red, PrimaryColor::Red)]
    #[case(PrimaryColor::Red, PrimaryColor::Blue)]
    #[case(PrimaryColor::Red, PrimaryColor::Yellow)]
    fn mix_should_return_same_result_when_input_order_is_swaped(
        #[case] c1: PrimaryColor,
        #[case] c2: PrimaryColor,
    ) {
        let result1 = utils::mix(&c1, &c2);
        let result2 = utils::mix(&c2, &c1);

        assert_eq!(result1, result2);
    }
}
