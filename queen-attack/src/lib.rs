#[derive(Debug)]
pub struct ChessPosition {
    rank: i32, 
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn is_valid_position(rank: i32, file: i32) -> bool {
        matches!((rank, file), (0..=7, 0..=7))
    }
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !Self::is_valid_position(rank, file) { return None; }
        Some(Self { rank, file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }
    pub fn can_go(cur: &(i32, i32), coord: &(i32, i32)) -> bool {
        ChessPosition::is_valid_position(cur.0 + coord.0, cur.1 + coord.1)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (x1, y1) = (self.position.rank, self.position.file);
        let (x2, y2) = (other.position.rank, other.position.file);
        
        if x1 == x2 || y1 == y2 { return true; }

        let coords = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
        for coord in coords {
            let (mut cur_x, mut cur_y ) = (x1, y1);
            while Self::can_go(&(cur_x, cur_y), &coord) {
                cur_x += coord.0;
                cur_y += coord.1;
                if cur_x == x2 && cur_y == y2 { return true; }
            }
        }
        false
    }
}
