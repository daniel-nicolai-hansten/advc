pub enum Direction {
    North,
    East,
    South,
    West,
    Northeast,
    Southeast,
    Southwest,
    Northwest,
}
#[derive(Debug)]
pub enum PositionError {
    OutOfBounds,
}
impl Direction {
    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::Northeast => Direction::Northwest,
            Direction::Southeast => Direction::Northeast,
            Direction::Southwest => Direction::Southeast,
            Direction::Northwest => Direction::Southwest,
        }
    }
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::Northeast => Direction::Southeast,
            Direction::Southeast => Direction::Southwest,
            Direction::Southwest => Direction::Northwest,
            Direction::Northwest => Direction::Northeast,
        }
    }
}
pub type Position = (usize, usize);
pub type IPosition = (isize, isize);
pub trait Pos<T> {
    fn new() -> T;
    fn move_in_direction(&self, direction: &Direction, steps: usize) -> Result<T, PositionError>;
    fn distance(&self, other: &T) -> usize;
    fn neighbor_positions(&self) -> Vec<T> {
        let mut neighbors = Vec::new();
        for direction in [Direction::North, Direction::East, Direction::South, Direction::West] {
            if let Ok(pos) = self.move_in_direction(&direction, 1) {
                neighbors.push(pos);
            }
        }
        neighbors
    }
    fn all_neighbor_positions(&self) -> Vec<T> {
        let mut neighbors = Vec::new();
        for direction in [
            Direction::North,
            Direction::Northeast,
            Direction::East,
            Direction::Southeast,
            Direction::South,
            Direction::Southwest,
            Direction::West,
            Direction::Northwest,
        ] {
            if let Ok(pos) = self.move_in_direction(&direction, 1) {
                neighbors.push(pos);
            }
        }
        neighbors
    }
}
impl Pos<Position> for Position {
    fn new() -> Position {
        (0, 0)
    }
    
  fn move_in_direction(&self, direction: &Direction, steps: usize) -> Result<Position, PositionError> {
        match direction {
            Direction::North => Ok((self.0, self.1 + steps)),
            Direction::East => Ok((self.0 + steps, self.1)),
            Direction::South => Ok((self.0, self.1.checked_sub(steps).ok_or(PositionError::OutOfBounds)?)),
            Direction::West => Ok((self.0.checked_sub(steps).ok_or(PositionError::OutOfBounds)?, self.1)),
            Direction::Northeast => Ok((self.0 + steps, self.1 + steps)),
            Direction::Southeast => Ok((self.0 + steps, self.1.checked_sub(steps).ok_or(PositionError::OutOfBounds)?)),
            Direction::Southwest => Ok((self.0.checked_sub(steps).ok_or(PositionError::OutOfBounds)?, self.1.checked_sub(steps).ok_or(PositionError::OutOfBounds)?)),
            Direction::Northwest => Ok((self.0.checked_sub(steps).ok_or(PositionError::OutOfBounds)?, self.1 + steps)),
        }
    }
    fn distance(&self, other: &Position) -> usize {
        ((self.0 as isize - other.0 as isize).abs() + (self.1 as isize - other.1 as isize).abs()) as usize
    }
}

impl Pos<IPosition> for IPosition {
    fn new() -> IPosition {
        (0, 0)
    }
  fn move_in_direction(&self, direction: &Direction, steps: usize) -> Result<IPosition, PositionError> {
        let steps = steps as isize;
        match direction {
            Direction::North => Ok((self.0, self.1 + steps)),
            Direction::East => Ok((self.0 + steps, self.1)),
            Direction::South => Ok((self.0, self.1 - steps)),
            Direction::West => Ok((self.0 - steps, self.1)),
            Direction::Northeast => Ok((self.0 + steps, self.1 + steps)),
            Direction::Southeast => Ok((self.0 + steps, self.1 - steps)),
            Direction::Southwest => Ok((self.0 - steps, self.1 - steps)),
            Direction::Northwest => Ok((self.0 - steps, self.1 + steps)),
        }
    }
    fn distance(&self, other: &IPosition) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize
    }
}