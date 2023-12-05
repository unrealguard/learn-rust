use closures::{Inventory, ShirtColor};

fn main() {
    let inventory = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue],
    };

    let mut user_preference = Some(ShirtColor::Red);
    let mut shirt_won = inventory.giveaway(user_preference);
    println!(
        "User with preference {:?} won shirt with color {:?}",
        user_preference, shirt_won
    );

    user_preference = None;
    shirt_won = inventory.giveaway(user_preference);

    println!(
        "User with preference {:?} won shirt with color {:?}",
        user_preference, shirt_won
    );
}
