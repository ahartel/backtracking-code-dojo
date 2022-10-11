fn main() {
    let starting_position = Position::new(0, 0);
    let found_journey =
        find_solution_to_knights_tour_by_backtracking(starting_position, 8).unwrap();
    println!("{}", found_journey.serialize());
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

pub struct Journey {
    visited_positions: Vec<Position>,
}

impl Journey {
    pub fn new(position: Position) -> Journey {
        Journey {
            visited_positions: vec![position],
        }
    }

    pub fn clone_and_push(&self, new_pos: Position) -> Journey {
        let mut new = self.visited_positions.clone();
        new.push(new_pos);
        Journey {
            visited_positions: new,
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

    pub fn is_complete(&self, board_size: u32) -> bool {
        (0..board_size)
            .zip(0..board_size)
            .map(|(x, y)| Position::new(x, y))
            .all(|p| self.visited_positions.contains(&p))
    }

    pub fn allowed_positions(&self, board_size: u32) -> impl Iterator<Item = Position> {
        let current_position = self.visited_positions.last().unwrap().clone();
        let visited_positions = self.visited_positions.clone();
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
            .filter(move |p| !visited_positions.contains(p))
    }
}

pub fn find_solution_to_knights_tour_by_backtracking(
    starting_position: Position,
    board_size: u32,
) -> Option<Journey> {
    let mut open = vec![Journey::new(starting_position)];
    while let Some(journey) = open.pop() {
        if journey.is_complete(board_size) {
            return Some(journey);
        }
        journey.allowed_positions(board_size).for_each(|p| {
            let new = journey.clone_and_push(p);
            open.push(new);
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{Journey, Position};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn should_return_a1() {
        let moves = Journey::new(Position::new(0, 0));
        assert_eq!(moves.serialize(), "a1")
    }
    #[test]
    fn should_return_a1b1() {
        let moves = Journey::new(Position::new(0, 0)).clone_and_push(Position::new(1, 0));
        assert_eq!(moves.serialize(), "a1 b1")
    }
    #[test]
    fn should_return_a1a2() {
        let moves = Journey::new(Position::new(0, 0)).clone_and_push(Position::new(0, 1));
        assert_eq!(moves.serialize(), "a1 a2")
    }

    #[test]
    fn eight_allowed_positions_from_center() {
        let current = Position::new(2, 2);
        let allowed_positions = Journey::new(current).allowed_positions(5);
        assert_eq!(allowed_positions.count(), 8);
    }

    #[test]
    fn two_allowed_positions_from_top_left() {
        let current = Position::new(0, 0);
        let allowed_positions = Journey::new(current).allowed_positions(5);
        assert_eq!(allowed_positions.count(), 2);
    }

    #[test]
    fn two_allowed_positions_from_top_right() {
        let current = Position::new(4, 0);
        let allowed_positions = Journey::new(current).allowed_positions(5);
        assert_eq!(allowed_positions.count(), 2);
    }
}
