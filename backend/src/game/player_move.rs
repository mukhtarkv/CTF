#[derive(Debug, Clone, Copy)]
pub enum Move {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Stay,
}

impl Move {
    pub fn new(dx: i32, dy: i32) -> Self {
        match (dx, dy) {
            (0, -1) => Move::Up,
            (1, -1) => Move::UpRight,
            (1, 0) => Move::Right,
            (1, 1) => Move::DownRight,
            (0, 1) => Move::Down,
            (-1, 1) => Move::DownLeft,
            (-1, 0) => Move::Left,
            (-1, -1) => Move::UpLeft,
            (0, 0) => Move::Stay,
            _ => {
                // Optionally, provide fallback or panic for invalid inputs
                Move::Stay
            }
        }
    }

    pub fn to_coords(self) -> (i32, i32) {
        match self {
            Move::Up => (0, -1),
            Move::UpRight => (1, -1),
            Move::Right => (1, 0),
            Move::DownRight => (1, 1),
            Move::Down => (0, 1),
            Move::DownLeft => (-1, 1),
            Move::Left => (-1, 0),
            Move::UpLeft => (-1, -1),
            Move::Stay => (0, 0),
        }
    }
}
