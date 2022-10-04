fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Copy, Clone)]
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
    index: usize,
}

impl AllowedPositions {
    pub fn new(current_position: Position, board_size: u32) -> AllowedPositions {
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
            .map(|(x, y)| (x as u32, y as u32))
            .filter(|&(x, y)| x < board_size && y < board_size)
            .map(|(x, y)| Position::new(x as u32, y as u32))
            .collect();
        AllowedPositions {
            allowed_positions,
            index: 0,
        }
    }
}

impl Iterator for AllowedPositions {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.allowed_positions.get(self.index);
        if result.is_some() {
            self.index += 1;
        }
        result.copied()
    }
}

pub struct VisitedPositions {
    visited_positions: Vec<Position>,
}

impl VisitedPositions {
    pub fn new(positions: Vec<Position>) -> VisitedPositions {
        VisitedPositions {
            visited_positions: positions,
        }
    }

    pub fn serialize(&self) -> String {
        self.visited_positions
            .iter()
            .map(|pos| char::from_u32(pos.x + 97).unwrap().to_string() + &(pos.y + 1).to_string())
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
        let moves = VisitedPositions::new(vec![Position::new(0, 0)]);
        assert_eq!(moves.serialize(), "a1")
    }
    #[test]
    fn should_return_a1b1() {
        let moves = VisitedPositions::new(vec![Position::new(0, 0), Position::new(1, 0)]);
        assert_eq!(moves.serialize(), "a1 b1")
    }
    #[test]
    fn should_return_a1a2() {
        let moves = VisitedPositions::new(vec![Position::new(0, 0), Position::new(0, 1)]);
        assert_eq!(moves.serialize(), "a1 a2")
    }

    #[test]
    fn eight_allowed_positions_from_center() {
        let current = Position::new(2, 2);
        let allowed_positions = AllowedPositions::new(current, 5);
        assert_eq!(allowed_positions.count(), 8);
    }

    #[test]
    fn two_allowed_positions_from_top_left() {
        let current = Position::new(0, 0);
        let allowed_positions = AllowedPositions::new(current, 5);
        assert_eq!(allowed_positions.count(), 2);
    }

    #[test]
    fn two_allowed_positions_from_top_right() {
        let current = Position::new(4, 0);
        let allowed_positions = AllowedPositions::new(current, 5);
        assert_eq!(allowed_positions.count(), 2);
    }
}
