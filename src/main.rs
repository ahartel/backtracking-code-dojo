fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    column: u32,
    row: u32,
}

impl Position {
    pub fn new(column: u32, row: u32) -> Position {
        Position { column, row }
    }
}

const MOVES: [(i32, i32); 8] = [
    (-2, -1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, 1),
    (2, 1),
    (-1, 2),
    (1, 2),
];

pub fn allowed_positions(
    current_position: Position,
    board_size: u32,
) -> impl Iterator<Item = Position> {
    MOVES
        .iter()
        .map(move |(x, y)| {
            let new_x = current_position.column as i32 + x;
            let new_y = current_position.row as i32 + y;
            (new_x, new_y)
        })
        .filter(|&(x, y)| x >= 0 && y >= 0)
        .map(|(x, y)| (x as u32, y as u32))
        .filter(move |&(x, y)| x < board_size && y < board_size)
        .map(|(x, y)| Position::new(x as u32, y as u32))
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
            .map(|pos| {
                char::from_u32(pos.column + 'a' as u32).unwrap().to_string()
                    + &(pos.row + 1).to_string()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

pub fn find_solution_to_knights_tour_by_backtracking(
    _starting_position: Position,
    _board_size: u32,
) -> Option<VisitedPositions> {
    None
}

#[cfg(test)]
mod tests {
    use crate::{allowed_positions, Position, VisitedPositions};

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
        let allowed_positions = allowed_positions(current, 5);
        assert_eq!(allowed_positions.count(), 8);
    }

    #[test]
    fn two_allowed_positions_from_top_left() {
        let current = Position::new(0, 0);
        let allowed_positions = allowed_positions(current, 5);
        assert_eq!(allowed_positions.count(), 2);
    }

    #[test]
    fn two_allowed_positions_from_top_right() {
        let current = Position::new(4, 0);
        let allowed_positions = allowed_positions(current, 5);
        assert_eq!(allowed_positions.count(), 2);
    }
}
