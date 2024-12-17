pub type Pos = (usize, usize);
pub trait Coord {
    fn x(&self) -> usize;
    fn y(&self) -> usize;

    fn up(&self) -> Option<Pos> {
        if self.y() == 0 {
            None
        } else {
            Some((self.x(), self.y() - 1))
        }
    }
    fn down(&self, max: usize) -> Option<Pos> {
        if self.y() + 1 < max {
            Some((self.x(), self.y() + 1))
        } else {
            None
        }
    }

    fn left(&self) -> Option<Pos> {
        if self.x() == 0 {
            None
        } else {
            Some((self.x() - 1, self.y()))
        }
    }
    fn right(&self, max: usize) -> Option<Pos> {
        if self.x() + 1 < max {
            Some((self.x() + 1, self.y()))
        } else {
            None
        }
    }
    fn dir(&self, dir: &Dir, maxy: usize, maxx: usize) -> Option<Pos> {
        match dir {
            Dir::Up => self.up(),
            Dir::Down => self.down(maxy),
            Dir::Left => self.left(),
            Dir::Right => self.right(maxx),
        }
    }
    fn new(x: usize, y: usize) -> Pos {
        (x, y)
    }
    fn manhattan(&self, other: &Pos) -> usize {
        ((self.x() as isize - other.x() as isize).abs() + (self.y() as isize - other.y() as isize).abs()) as usize
    }
    #[allow(dead_code)]
    fn neighbors(&self, maxy: usize, maxx: usize) -> Vec<Pos> {
        [self.up(), self.down(maxy), self.left(), self.right(maxx)].iter().filter_map(|x| *x).collect()
    }
    #[allow(dead_code)]
    fn all_neighbors(&self) -> Vec<Pos> {
        vec![self.up(), self.down(usize::MAX), self.left(), self.right(usize::MAX)]
            .iter()
            .filter_map(|x| *x)
            .collect()
    }
}
impl Coord for Pos {
    fn x(&self) -> usize {
        self.0
    }
    fn y(&self) -> usize {
        self.1
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    pub fn dirs() -> Vec<Self> {
        vec![Self::Up, Self::Down, Self::Left, Self::Right]
    }
}
