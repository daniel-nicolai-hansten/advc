#[allow(dead_code)]
pub type IPos = (isize, isize);
pub trait ICoord {
    fn x(&self) -> isize;
    fn y(&self) -> isize;

    fn up(&self, min: isize) -> Option<IPos> {
        if self.y() == min {
            None
        } else {
            Some((self.x(), self.y() - 1))
        }
    }
    fn down(&self, max: isize) -> Option<IPos> {
        if self.y() + 1 < max {
            Some((self.x(), self.y() + 1))
        } else {
            None
        }
    }

    fn left(&self, min: isize) -> Option<IPos> {
        if self.x() == min {
            None
        } else {
            Some((self.x() - 1, self.y()))
        }
    }
    fn right(&self, max: isize) -> Option<IPos> {
        if self.x() + 1 < max {
            Some((self.x() + 1, self.y()))
        } else {
            None
        }
    }
    #[allow(dead_code)]
    fn dir(&self, dir: Dir) -> Option<IPos> {
        match dir {
            Dir::Up => self.up(-2),
            Dir::Down => self.down(isize::MAX),
            Dir::Left => self.left(-2),
            Dir::Right => self.right(isize::MAX),
        }
    }
    #[allow(dead_code)]
    fn neighbors(&self, miny: isize, maxy: isize, minx: isize, maxx: isize) -> Vec<IPos> {
        [self.up(miny), self.down(maxy), self.left(minx), self.right(maxx)]
            .iter()
            .filter_map(|x| *x)
            .collect()
    }
    fn all_neighbors(&self, _maxy: isize, _maxx: isize) -> Vec<IPos> {
        vec![self.up(-2), self.down(isize::MAX), self.left(-2), self.right(isize::MAX)]
            .iter()
            .filter_map(|x| *x)
            .collect()
    }
}
impl ICoord for IPos {
    fn x(&self) -> isize {
        self.0
    }
    fn y(&self) -> isize {
        self.1
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    #[allow(dead_code)]
    fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
    #[allow(dead_code)]
    pub fn dirs() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }
}
