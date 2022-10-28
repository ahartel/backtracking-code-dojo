fn main() {
    let starting_position = Position::new(0, 0);
    let solutions = Solutions::new(starting_position, 8);
    // println!("{}", solutions.count());
    for journey in solutions.take(10) {
        println!("{}", journey.serialize());
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    column: u32,
    row: u32,
}

impl Position {
    pub fn new(column: u32, row: u32) -> Position {
        Position { column, row }
    }

    pub fn as_idx(&self, board_size: usize) -> usize {
        (self.column as usize) * board_size + (self.row as usize)
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

#[derive(Clone)]
pub struct Journey {
    moves: Vec<Position>,
    visited: Vec<bool>,
    board_size: usize,
}

impl Journey {
    pub fn new(board_size: usize, start: Position) -> Journey {
        Journey {
            moves: vec![start],
            visited: vec![false; board_size * board_size],
            board_size,
        }
    }

    pub fn serialize(&self) -> String {
        self.moves
            .iter()
            .map(|pos| {
                char::from_u32(pos.column + 'a' as u32).unwrap().to_string()
                    + &(pos.row + 1).to_string()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn is_complete(&self) -> bool {
        self.moves.len() == self.visited.len()
    }

    pub fn apply_move(&mut self, pos: Position) {
        self.moves.push(pos);
        *self.visited.get_mut(pos.as_idx(self.board_size)).unwrap() = true;
    }

    pub fn undo(&mut self) {
        if let Some(last_move) = self.moves.pop() {
            *self
                .visited
                .get_mut(last_move.as_idx(self.board_size))
                .unwrap() = false;
        }
    }

    pub fn candidate_positions(&self) -> Vec<Position> {
        let current_position = self.moves.last().unwrap().clone();
        MOVES
            .iter()
            .map(move |(x, y)| {
                let new_x = current_position.column as i32 + x;
                let new_y = current_position.row as i32 + y;
                (new_x, new_y)
            })
            .filter(|&(x, y)| x >= 0 && y >= 0)
            .map(|(x, y)| (x as u32, y as u32))
            .filter(move |&(x, y)| (x as usize) < self.board_size && (y as usize) < self.board_size)
            .map(|(x, y)| Position::new(x as u32, y as u32))
            .filter(move |p| !self.visited[p.as_idx(self.board_size)])
            .collect()
    }
}

struct Solutions {
    journey: Journey,
    open: Vec<(i32, Position)>,
    count: i32,
}

impl Solutions {
    pub fn new(starting_position: Position, board_size: usize) -> Solutions {
        let journey = Journey::new(board_size, starting_position);
        Solutions {
            journey,
            open: vec![(0, starting_position)],
            count: 0,
        }
    }
}

impl Iterator for Solutions {
    type Item = Journey;

    /// Algorithm:
    /// There is only one journey, but there is a stack of iterators for potential next moves.
    fn next(&mut self) -> Option<Self::Item> {
        if self.open.is_empty() {
            return None;
        }

        while let Some((count, next)) = self.open.pop() {
            for _ in 0..(self.count - count + 1) {
                self.journey.undo();
            }

            self.journey.apply_move(next);
            self.count = count;

            if self.journey.is_complete() {
                return Some(self.journey.clone());
            }

            self.journey
                .candidate_positions()
                .into_iter()
                .for_each(|pos| {
                    self.open.push((count + 1, pos));
                });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{Journey, Position, Solutions};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn should_return_a1() {
        let moves = Journey::new(8, Position::new(0, 0));
        assert_eq!(moves.serialize(), "a1")
    }
    #[test]
    fn should_return_a1b1() {
        let mut moves = Journey::new(8, Position::new(0, 0));
        moves.apply_move(Position::new(1, 0));
        assert_eq!(moves.serialize(), "a1 b1")
    }
    #[test]
    fn should_return_a1a2() {
        let mut moves = Journey::new(8, Position::new(0, 0));
        moves.apply_move(Position::new(0, 1));
        assert_eq!(moves.serialize(), "a1 a2")
    }

    #[test]
    fn eight_allowed_positions_from_center() {
        let current = Position::new(2, 2);
        let allowed_positions = Journey::new(5, current).candidate_positions();
        assert_eq!(allowed_positions.len(), 8);
    }

    #[test]
    fn two_allowed_positions_from_top_left() {
        let current = Position::new(0, 0);
        let allowed_positions = Journey::new(5, current).candidate_positions();
        assert_eq!(allowed_positions.len(), 2);
    }

    #[test]
    fn two_allowed_positions_from_top_right() {
        let current = Position::new(4, 0);
        let journey = Journey::new(5, current);
        let allowed_positions = journey.candidate_positions();
        assert_eq!(allowed_positions.len(), 2);
    }

    #[test]
    fn solution_has_25_positions() {
        let solution = Solutions::new(Position { column: 0, row: 0 }, 5)
            .next()
            .unwrap();
        assert_eq!(solution.visited.len(), 25);
    }

    #[test]
    fn can_generate_all_fields() {
        let board_size: u32 = 8;
        let all_pos: Vec<_> = (0..board_size)
            .flat_map(|x| {
                let x = x.clone();
                (0..board_size).map(move |y| Position::new(x, y))
            })
            .collect();
        assert_eq!(all_pos.len(), (board_size * board_size) as usize);
    }
}
