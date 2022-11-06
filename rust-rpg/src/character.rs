pub struct Character {
    pub name: String,
    pub class: String,
    pub health: i32,
    attack: i32,
    dodge: i32,
    luck: i32,
    xp: i32,
}

pub trait Player {
    fn new(
        name: String,
        class_name: String,
        health: i32,
        attack: i32,
        dodge: i32,
        luck: i32,
    ) -> Character;

    fn select(&self, player_name: String, player_luck: i32) -> Self;

    fn damage(&mut self, damage_amount: i32);

    fn heal(&mut self, heal_amount: i32);

    fn attack(&self) -> i32;

    fn dodge(&self) -> i32;

    fn info(&self) -> String;

    fn stats(&self) -> String;
}


