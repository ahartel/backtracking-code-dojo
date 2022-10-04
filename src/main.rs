fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Position {
        Position { x, y }
    }
}

/// Returns allowed moves from a given position
pub struct AllowedPositions {
    pub allowed_positions: Vec<Position>,
}

impl AllowedPositions {
    pub fn new(current_position: Position) -> AllowedPositions {
        let possible_moves: Vec<(i32, i32)> = vec![
            (-2, -1),
            (2, -1),
            (1, -2),
            (-1, -2),
            (-2, 1),
            (2, 1),
            (-1, 2),
            (1, 2),
        ];
        let allowed_positions = possible_moves
            .into_iter()
            .map(|(x, y)| {
                let new_x = current_position.x as i32 + x;
                let new_y = current_position.y as i32 + y;
                (new_x, new_y)
            })
            .filter(|&(x, y)| x >= 0 && y >= 0)
            .map(|(x, y)| Position::new(x as u32, y as u32))
            .collect();
        AllowedPositions { allowed_positions }
    }
}

pub struct VisitedPositions {
    visited_positions: Vec<u32>,
    board_size: u32,
}

impl VisitedPositions {
    pub fn new(positions: Vec<u32>, board_size: u32) -> VisitedPositions {
        VisitedPositions {
            visited_positions: positions,
            board_size,
        }
    }

    pub fn serialize(&self) -> String {
        self.visited_positions
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
    use crate::{AllowedPositions, Position, VisitedPositions};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn should_return_a1() {
        let moves = VisitedPositions::new(vec![0], 5);
        assert_eq!(moves.serialize(), "a1")
    }
    #[test]
    fn should_return_a1b1() {
        let moves = VisitedPositions::new(vec![0, 1], 5);
        assert_eq!(moves.serialize(), "a1 b1")
    }
    #[test]
    fn should_return_a1a2() {
        let moves = VisitedPositions::new(vec![0, 5], 5);
        assert_eq!(moves.serialize(), "a1 a2")
    }

    #[test]
    fn eight_allowed_positions_from_center() {
        let current = Position::new(2, 2);
        let allowed_positions = AllowedPositions::new(current);
        assert_eq!(allowed_positions.allowed_positions.len(), 8);
    }

    #[test]
    fn two_allowed_positions_from_top_left() {
        let current = Position::new(0, 0);
        let allowed_positions = AllowedPositions::new(current);
        dbg!(&allowed_positions.allowed_positions);
        assert_eq!(allowed_positions.allowed_positions.len(), 2);
    }
}
