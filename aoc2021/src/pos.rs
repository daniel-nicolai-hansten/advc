type Pos = (usize, usize);
fn get_pos(pos: Pos, dir: u8) -> Pos {
    match dir {
        1 => (pos.0, pos.1 - 1),
        2 => (pos.0, pos.1 + 1),
        3 => (pos.0 - 1, pos.1),
        4 => (pos.0 + 1, pos.1),
        _ => panic!("Invalid direction"),
    }
}
enum Dir {
    Up,
    Down,
    Left,
    Right,
}