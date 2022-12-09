fn main() {
    println!("Hello, world!");
}

enum Orientation {
    Up,
    Down,
    Left,
    Right,
}
struct PuzzlePiece {
    sides: [u8; 4],
    orientation: Orientation,
}
