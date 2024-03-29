use std::thread::{self};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference
            .is_some_and(|color| self.has_stock(color))
            .then(|| user_preference.unwrap())
            .unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            return ShirtColor::Red;
        } else {
            return ShirtColor::Blue;
        }
    }

    fn has_stock(&self, shirt_color: ShirtColor) -> bool {
        for color in &self.shirts {
            if *color == shirt_color {
                return true;
            }
        }
        false
    }
}

pub trait ThreadedGiveaway {
    fn spawn_giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor;
}

impl ThreadedGiveaway for Inventory {
    fn spawn_giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        let mut shirt_won: Option<ShirtColor> = None;

        thread::scope(|s| {
            s.spawn(|| {
                shirt_won = Some(self.giveaway(user_preference));
            });
        });

        shirt_won.unwrap_or(ShirtColor::Red)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn giveaway_should_return_most_stocked_when_no_preference() {
        let inventory = Inventory {
            shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue],
        };

        let shirt_won = inventory.giveaway(Option::None);
        assert_eq!(shirt_won, ShirtColor::Red);
    }

    #[test]
    fn giveaway_should_return_preference_if_available() {
        let inventory = Inventory {
            shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue],
        };

        let shirt_won = inventory.giveaway(Option::Some(ShirtColor::Blue));
        assert_eq!(shirt_won, ShirtColor::Blue);
    }

    #[test]
    fn giveaway_should_return_available_if_preference_out_of_stock() {
        let inventory = Inventory {
            shirts: vec![ShirtColor::Red, ShirtColor::Red],
        };

        let shirt_won = inventory.giveaway(Option::Some(ShirtColor::Blue));
        assert_eq!(shirt_won, ShirtColor::Red);
    }

    #[test]
    fn threaded_giveaway_should_return_preference_if_available() {
        let inventory = Inventory {
            shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue],
        };

        let shirt_won = inventory.spawn_giveaway(Option::Some(ShirtColor::Blue));
        assert_eq!(shirt_won, ShirtColor::Blue);
    }
}
