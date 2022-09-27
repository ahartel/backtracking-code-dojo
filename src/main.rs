fn main() {
    println!("Hello, world!");
}

pub fn export_board(moves: Vec<u32>, size: u32) -> String {
    let mut exported_moves = String::new();
    for m in moves {
        exported_moves.push(char::from_u32(m % size + 97).unwrap());
        exported_moves.push_str(&(m / size + 1).to_string());
        exported_moves.push(' ');
    }
    exported_moves.remove(exported_moves.len() - 1);
    exported_moves
}

#[cfg(test)]
mod tests {
    use crate::export_board;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn should_return_a1() {
        let moves = vec![0];
        assert_eq!(export_board(moves, 5), "a1")
    }
    #[test]
    fn should_return_a1b1() {
        let moves = vec![0, 1];
        assert_eq!(export_board(moves, 5), "a1 b1")
    }
    #[test]
    fn should_return_a1a2() {
        let moves = vec![0, 5];
        assert_eq!(export_board(moves, 5), "a1 a2")
    }
}
