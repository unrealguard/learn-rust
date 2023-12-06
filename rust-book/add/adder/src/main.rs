fn main() {
    let mut result = add_one::add_one(2);
    println!("Result {}", result);

    result = add_one::add_random(5);
    println!("Random Result {}", result);
}
