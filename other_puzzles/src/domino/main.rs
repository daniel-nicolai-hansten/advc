type Domino = (u8, u8);
trait Dominos {
    fn sort(self) -> Self;
}
impl Dominos for Domino {
    fn sort(self) -> Self {
        let (a, b) = self;
        if a <= b { (a, b) } else { (b, a) }
    }
}

fn generate_dominoes() -> Vec<Domino> {
    let mut dominoes = Vec::new();
    for i in 0..=6 {
        for j in i..=6 {
            dominoes.push((i, j).sort());
        }
    }
    dominoes
}
#[derive(Clone)]
struct Puzzle {
    board: Vec<Vec<Option<Domino>>>,
    remaining: Vec<Domino>,
    constraint_x: Vec<Vec<u8>>,
    constraint_y: Vec<(Vec<u8>, Vec<u8>)>,
}
impl Puzzle {
    fn place_domino(&self, &y: &usize, &x: &usize, domino: Domino) -> Option<Self> {
        let mut new_puzzle = self.clone();
        new_puzzle.board[y][x] = Some(domino);
        let piecepos = new_puzzle.remaining.iter().position(|&d| d == domino.sort())?;
        new_puzzle.remaining.swap_remove(piecepos);
        let contstaintx1_pos = new_puzzle.constraint_x[x].iter().position(|&n| n == domino.0)?;
        new_puzzle.constraint_x[x].swap_remove(contstaintx1_pos);
        let contstaintx2_pos = new_puzzle.constraint_x[x].iter().position(|&n| n == domino.1)?;
        new_puzzle.constraint_x[x].swap_remove(contstaintx2_pos);
        let constrainty1_pos = new_puzzle.constraint_y[y].0.iter().position(|&n| n == domino.0)?;
        new_puzzle.constraint_y[y].0.swap_remove(constrainty1_pos);
        let constrainty2_pos = new_puzzle.constraint_y[y].1.iter().position(|&n| n == domino.1)?;
        new_puzzle.constraint_y[y].1.swap_remove(constrainty2_pos);
        Some(new_puzzle)
    }
    fn check_constraints(&self, &y: &usize, &x: &usize, domino: Domino) -> bool {
        self.constraint_x[x].contains(&domino.0)
            && self.constraint_x[x].contains(&domino.1)
            && self.constraint_y[y].0.contains(&domino.0)
            && self.constraint_y[y].1.contains(&domino.1)
    }
}

fn main() {
    let puzzle = Puzzle {
        board: vec![vec![None; 7]; 4],
        remaining: generate_dominoes(),

        constraint_x: vec![
            vec![0, 0, 1, 1, 1, 5, 6, 6],
            vec![1, 1, 2, 2, 3, 4, 5, 6],
            vec![0, 0, 2, 2, 3, 3, 4, 4],
            vec![1, 1, 3, 6, 6, 6, 5, 3],
            vec![0, 0, 0, 1, 3, 4, 4, 5],
            vec![3, 4, 4, 5, 5, 5, 6, 6],
            vec![0, 2, 2, 2, 2, 3, 4, 5],
        ],
        constraint_y: vec![
            (vec![1, 1, 2, 3, 4, 4, 5], vec![0, 1, 1, 2, 2, 4, 6]),
            (vec![0, 0, 1, 1, 3, 3, 6], vec![0, 0, 2, 3, 3, 4, 4]),
            (vec![1, 3, 3, 5, 5, 6, 6], vec![1, 2, 2, 4, 5, 6, 6]),
            (vec![0, 2, 2, 3, 5, 6, 6], vec![0, 0, 4, 4, 5, 5, 5]),
        ],
    };

    if let Some(solved) = solve_puzzle(puzzle, &(0..4).flat_map(|y| (0..7).map(move |x| (y, x))).collect::<Vec<_>>()) {
        for row in solved.board {
            for cell in row {
                match cell {
                    Some((a, b)) => print!("({},{}) ", a, b),
                    None => print!("( ,) "),
                }
            }
            println!();
        }
    } else {
        println!("No solution found.");
    }
}

fn solve_puzzle(puzzle: Puzzle, squares: &[(usize, usize)]) -> Option<Puzzle> {
    match squares {
        [(y, x), rem_squares @ ..] => {
            for puzzle in puzzle
                .remaining
                .iter()
                .flat_map(|&d| [(d.0, d.1), (d.1, d.0)])
                .filter(|d| puzzle.check_constraints(y, x, *d))
                .filter_map(|domino| puzzle.place_domino(y, x, domino))
                .filter_map(|puz| solve_puzzle(puz, rem_squares))
            {
                return Some(puzzle);
            }
            return None;
        }
        [] => return Some(puzzle),
    }
}
