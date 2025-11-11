// Each player starts with four ships. Each ship takes up a certain number of spaces or coordinates on the grid. Each ship has an associated number of shots.

use common::pos::{Pos, Position};
use itertools::Itertools;
use prompted::input;
use rand::{Rng, seq::IndexedRandom};

// Battleship - 5 spaces - 3 shots
// Cruiser - 4 spaces - 2 shots
// Destroyer - 3 spaces - 1 shot
// Sub - 2 spaces - 1 shot
const BOARD_SIZE: usize = 10;
fn main() {
    let mut game = GameState::new();
    println!("Welcome to Battleship!");
    loop {
        if game.current_shots() == 0 {
            println!(
                "{} has no remaining ships of type {} to shoot with. Skipping turn.",
                game.current_player().name(),
                game.current_ship().name()
            );
            game.next_turn();
            continue;
        }
        if game.current_player() == Player::Human {
            println!("Player Board:");
            game.player_board.display();
            println!("Computer Board:");
            game.computer_board.display();
        }
        println!("Its {}'s {} turn.", game.current_player().name(), game.current_ship().name());
        let shots = match game.current_player() {
            Player::Human => game.get_player_shots(),
            Player::Computer => game.get_computer_shots(),
        };
        for shot in shots {
            let result = game.shoot(shot);
            println!("{} fired at {} and it's a {:?}", game.current_player().name(), shot.to_algebraic(), result);
        }
        if let Some(winner) = game.win() {
            println!("{} wins!", winner.name());
            break;
        }
        game.next_turn();
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Player {
    Human,
    Computer,
}
impl Player {
    fn name(&self) -> &'static str {
        match self {
            Player::Human => "Player",
            Player::Computer => "Computer",
        }
    }
}
struct GameState {
    player_board: Board,
    computer_board: Board,
    turn: usize,
}
impl GameState {
    fn new() -> Self {
        GameState {
            player_board: Board::new(),
            computer_board: Board::new(),
            turn: 0,
        }
    }
    fn current_ship(&self) -> ShipType {
        let ship_types = ShipType::all_types();
        let index = (self.turn / 2) % ship_types.len();
        ship_types[index]
    }

    fn has_current_ship(&self) -> bool {
        let ship = self.current_ship();
        match self.current_player() {
            Player::Human => self.player_board.ships.iter().any(|s| s.ship_type == ship && !s.is_sunk()),
            Player::Computer => self.computer_board.ships.iter().any(|s| s.ship_type == ship && !s.is_sunk()),
        }
    }
    fn current_shots(&self) -> usize {
        match self.has_current_ship() {
            false => 0,
            true => self.current_ship().shots(),
        }
    }
    fn next_turn(&mut self) {
        self.turn += 1;
    }
    fn current_player(&self) -> Player {
        match self.turn % 2 {
            0 => Player::Human,
            _ => Player::Computer,
        }
    }
    fn win(&self) -> Option<Player> {
        match (self.player_board.is_all_sunk(), self.computer_board.is_all_sunk()) {
            (false, false) => None,
            (true, false) => Some(Player::Computer),
            (false, true) => Some(Player::Human),
            (true, true) => Some(Player::Human),
        }
    }

    fn get_player_shots(&self) -> Vec<Position> {
        let shotnum = self.current_shots();
        if shotnum == 0 {
            return Vec::new();
        }
        let input_shots = input!("Enter your shot coordinates (e.g., E9 ). You have {} shots:", shotnum);
        let parsed_shots = parse_input(&input_shots);
        parsed_shots.into_iter().take(shotnum).collect()
    }

    fn get_computer_shots(&self) -> Vec<Position> {
        let shotnum = self.current_shots();
        self.player_board.find_next_shots(shotnum)
    }
    fn shoot(&mut self, pos: Position) -> Cell {
        match self.current_player() {
            Player::Human => self.computer_board.shoot(pos),
            Player::Computer => self.player_board.shoot(pos),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Cell {
    Empty,
    Ship,
    Hit,
    Miss,
}
impl Cell {
    fn has_been_shot(&self) -> bool {
        match self {
            Cell::Hit | Cell::Miss => true,
            _ => false,
        }
    }
}
#[derive(Debug)]
struct Board {
    grid: [[Cell; BOARD_SIZE]; BOARD_SIZE],
    ships: Vec<Ship>,
}
impl Board {
    fn new() -> Self {
        let ships = ShipType::all_types();
        let ships = ships.iter().fold(Vec::new(), |mut acc, &ship_type| {
            let ship = Ship::new_random(ship_type, &acc);
            acc.push(ship);
            acc
        });
        let mut grid = [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE];
        for ship in &ships {
            for &(x, y) in &ship.positions {
                grid[x][y] = Cell::Ship;
            }
        }
        Board { grid, ships }
    }
    fn display(&self) {
        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, &cell) in row.iter().enumerate() {
                let shipsymbol = self.ships.iter().find_map(|ship| ship.pos_symbol((row_index, col_index)));
                let symbol = match (cell, shipsymbol) {
                    (Cell::Empty, _) => '.',
                    (Cell::Ship, Some(ship_symbol)) => ship_symbol,
                    (Cell::Hit, _) => '💥',
                    (Cell::Miss, _) => '💦',
                    _ => '~',
                };
                print!("{} ", symbol);
            }
            println!();
        }
    }
    fn shoot(&mut self, pos: Position) -> Cell {
        let (x, y) = pos;
        match self.grid[x][y] {
            Cell::Ship => {
                self.grid[x][y] = Cell::Hit;
                for ship in &mut self.ships {
                    if ship.register_hit(pos) {
                        ship.display_damage();
                        break;
                    }
                }
            }
            Cell::Empty => {
                self.grid[x][y] = Cell::Miss;
            }
            _ => (), // Already hit or missed
        }
        self.grid[x][y]
    }
    fn is_all_sunk(&self) -> bool {
        self.ships.iter().all(|ship| ship.is_sunk())
    }
    fn is_within_bounds(&self, pos: Position) -> bool {
        let (x, y) = pos;
        x < BOARD_SIZE && y < BOARD_SIZE
    }
    fn find_next_shots(&self, n: usize) -> Vec<Position> {
        let mut shots = Vec::new();
        let hit_neighbors: Vec<Position> = (0..BOARD_SIZE)
            .cartesian_product(0..BOARD_SIZE)
            .filter(|&(x, y)| self.grid[x][y] == Cell::Hit)
            .map(|pos| pos.all_neighbor_positions())
            .flatten()
            .dedup()
            .filter(|&(x, y)| self.is_within_bounds((x, y)) && !self.grid[x][y].has_been_shot())
            .collect();
        shots.extend(hit_neighbors);
        let positions_not_tried: Vec<Position> = (0..BOARD_SIZE)
            .cartesian_product(0..BOARD_SIZE)
            .filter(|&(x, y)| !self.grid[x][y].has_been_shot() || shots.contains(&(x, y)) == false)
            .collect();
        let rand_positions: Vec<Position> = positions_not_tried.choose_multiple(&mut rand::rng(), n).cloned().collect();
        shots.extend(rand_positions);
        shots.truncate(n);
        shots
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum ShipType {
    Battleship,
    Cruiser,
    Destroyer,
    Sub,
}

impl ShipType {
    fn size(&self) -> usize {
        match self {
            ShipType::Battleship => 5,
            ShipType::Cruiser => 4,
            ShipType::Destroyer => 3,
            ShipType::Sub => 2,
        }
    }
    fn shots(&self) -> usize {
        match self {
            ShipType::Battleship => 3,
            ShipType::Cruiser => 2,
            ShipType::Destroyer => 1,
            ShipType::Sub => 1,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            ShipType::Battleship => "Battleship",
            ShipType::Cruiser => "Cruiser",
            ShipType::Destroyer => "Destroyer",
            ShipType::Sub => "Submarine",
        }
    }
    fn all_types() -> Vec<ShipType> {
        vec![ShipType::Battleship, ShipType::Cruiser, ShipType::Destroyer, ShipType::Sub]
    }
}
#[derive(Debug)]
struct Ship {
    positions: Vec<Position>,
    hits: Vec<bool>,
    ship_type: ShipType,
}
impl Ship {
    fn new(positions: Vec<Position>, ship_type: ShipType) -> Self {
        let hits = vec![false; positions.len()];
        Ship { positions, hits, ship_type }
    }
    fn new_random(ship_type: ShipType, other_ships: &[Ship]) -> Self {
        let other_ship_positions: Vec<Position> = other_ships.iter().flat_map(|s| s.positions.clone()).collect();
        loop {
            let positions = Ship::random_positions(ship_type);
            if !positions.iter().any(|pos| other_ship_positions.contains(pos)) {
                return Ship::new(positions, ship_type);
            }
        }
    }
    fn pos_symbol(&self, pos: Position) -> Option<char> {
        if let Some(index) = self.positions.iter().position(|&p| p == pos) {
            if self.hits[index] {
                Some('X') // Hit
            } else {
                match self.ship_type {
                    ShipType::Battleship => Some('B'),
                    ShipType::Cruiser => Some('C'),
                    ShipType::Destroyer => Some('D'),
                    ShipType::Sub => Some('U'),
                }
            }
        } else {
            None
        }
    }

    fn is_sunk(&self) -> bool {
        self.hits.iter().all(|&hit| hit)
    }
    fn register_hit(&mut self, pos: Position) -> bool {
        for (i, &ship_pos) in self.positions.iter().enumerate() {
            if ship_pos == pos {
                self.hits[i] = true;
                return true;
            }
        }
        false
    }

    fn name(&self) -> &'static str {
        self.ship_type.name()
    }

    fn random_positions(ship_type: ShipType) -> Vec<Position> {
        let mut rng = rand::rng();
        let placement = rng.random_range(0..3);
        let mut positions = Vec::new();
        let start = match placement {
            0 => (rng.random_range(0..BOARD_SIZE - ship_type.size()), rng.random_range(0..BOARD_SIZE)), // Horizontal
            1 => (rng.random_range(0..BOARD_SIZE), rng.random_range(0..BOARD_SIZE - ship_type.size())), // Vertical
            2 => (
                rng.random_range(0..BOARD_SIZE - ship_type.size()),
                rng.random_range(0..BOARD_SIZE - ship_type.size()),
            ), // Diagonal
            _ => unreachable!(),
        };
        for i in 0..ship_type.size() {
            let pos = match placement {
                0 => (start.0 + i, start.1),     // Horizontal
                1 => (start.0, start.1 + i),     // Vertical
                2 => (start.0 + i, start.1 + i), // Diagonal
                _ => unreachable!(),
            };
            positions.push(pos);
        }
        positions
    }
    fn display_damage(&self) {
        let damage: usize = self.hits.iter().filter(|&&hit| hit).count();
        let total = self.hits.len();
        match self.is_sunk() {
            true => println!("The {} is sunk!", self.name()),
            false => println!("Hit on {}! (damage: {}/{})", self.name(), damage, total),
        }
    }
}
trait AlgebraicNotation {
    fn to_algebraic(&self) -> String;
    fn from_algebraic(notation: &str) -> Option<Self>
    where
        Self: Sized;
}
impl AlgebraicNotation for Position {
    fn to_algebraic(&self) -> String {
        let (mut x, y) = *self;
        let mut col_arr = vec![];
        while x >= 26 {
            col_arr.push((b'A' + (x % 26) as u8) as char);
            x /= 26;
            x -= 1;
        }
        col_arr.push((b'A' + (x % 26) as u8) as char);
        let column = col_arr.iter().rev().collect::<String>();
        let row = y + 1;
        format!("{}{}", column, row)
    }
    fn from_algebraic(notation: &str) -> Option<Self>
    where
        Self: Sized,
    {
        let mut chars = notation.chars();
        let mut x = 0;
        while let Some(c) = chars.next() {
            if c.is_ascii_alphabetic() {
                x = x * 26 + (c.to_ascii_uppercase() as u8 - b'A' + 1) as usize;
            } else {
                let y_str: String = std::iter::once(c).chain(chars).collect();
                if let Ok(y) = y_str.parse::<usize>() {
                    return Some((x - 1, y - 1));
                } else {
                    return None;
                }
            }
        }
        None
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_shiploop() {
        let mut game = GameState::new();
        let mut result = Vec::new();
        for turn in 0..8 {
            let ship = game.current_ship();
            println!("Turn {}: Current ship is {:?}", turn, ship);
            result.push(ship);
            game.next_turn();
        }
        let expected = vec![
            crate::ShipType::Battleship,
            crate::ShipType::Battleship,
            crate::ShipType::Cruiser,
            crate::ShipType::Cruiser,
            crate::ShipType::Destroyer,
            crate::ShipType::Destroyer,
            crate::ShipType::Sub,
            crate::ShipType::Sub,
        ];
        assert_eq!(result, expected);
    }
    #[test]
    fn test_algebraic_notation() {
        let pos: Position = (27, 4);
        let notation = pos.to_algebraic();
        assert_eq!(notation, "AB5");
    }
    #[test]
    fn test_parse_input() {
        let input1 = "A1 B2 C3 D4 EA55";
        let positions1 = parse_input(input1);
        println!("Parsed positions: {:?}", positions1);
        assert_eq!(positions1.len(), 5);
    }
}
fn parse_input(input: &str) -> Vec<Position> {
    let mut positions = Vec::new();
    for token in input.split_whitespace() {
        if let Some(pos) = Position::from_algebraic(token) {
            positions.push(pos);
        } else {
            println!("Invalid input: {}", token);
        }
    }
    positions
}
