fn main() {
    println!("Hello, world!");
}

pub struct Moves {
    moves: Vec<u32>,
    board_size: u32,
}

impl Moves {
    pub fn new(moves: Vec<u32>, board_size: u32) -> Moves {
        Moves { moves, board_size }
    }

    pub fn serialize(&self) -> String {
        self.moves
            .iter()
            .map(|pos| {
                char::from_u32(pos % self.board_size + 97)
                    .unwrap()
                    .to_string()
                    + &(pos / self.board_size + 1).to_string()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use crate::Moves;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn should_return_a1() {
        let moves = Moves::new(vec![0], 5);
        assert_eq!(moves.serialize(), "a1")
    }
    #[test]
    fn should_return_a1b1() {
        let moves = Moves::new(vec![0, 1], 5);
        assert_eq!(moves.serialize(), "a1 b1")
    }
    #[test]
    fn should_return_a1a2() {
        let moves = Moves::new(vec![0, 5], 5);
        assert_eq!(moves.serialize(), "a1 a2")
    }
}
