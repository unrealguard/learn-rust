use rand::{self, Rng};

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add_random(x: i32) -> i32 {
    let mut rng = rand::thread_rng();
    x + rng.gen_range(1..10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_should_return_input_plus_one() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
