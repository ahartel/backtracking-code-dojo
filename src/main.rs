fn main() {
    println!("Hello, world!");
}

pub fn serialize_moves(moves: Vec<u32>, size: u32) -> String {
    moves
        .into_iter()
        .map(|pos| {
            char::from_u32(pos % size + 97).unwrap().to_string() + &(pos / size + 1).to_string()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::serialize_moves;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn should_return_a1() {
        let moves = vec![0];
        assert_eq!(serialize_moves(moves, 5), "a1")
    }
    #[test]
    fn should_return_a1b1() {
        let moves = vec![0, 1];
        assert_eq!(serialize_moves(moves, 5), "a1 b1")
    }
    #[test]
    fn should_return_a1a2() {
        let moves = vec![0, 5];
        assert_eq!(serialize_moves(moves, 5), "a1 a2")
    }
}
