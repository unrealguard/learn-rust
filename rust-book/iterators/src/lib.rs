#[derive(PartialEq, Debug, Clone, Copy)]
struct Shoe<'a> {
    size: u32,
    style: &'a str,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use crate::{shoes_in_size, Shoe};

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map() {
        let v1 = vec![1, 2, 3];

        let result: Vec<_> = v1.iter().map(|x| x * 2).collect();

        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: "sneaker",
            },
            Shoe {
                size: 13,
                style: "sandal",
            },
            Shoe {
                size: 10,
                style: "boot",
            },
        ];

        let in_my_size = shoes_in_size(shoes.clone(), 10);

        assert_eq!(in_my_size, vec![shoes[0], shoes[2]])
    }
}
