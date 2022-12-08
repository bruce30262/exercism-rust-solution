use Direction::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self{ x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        match self.d {
            North => Self{ d: East, ..self },
            East => Self{ d: South, ..self },
            South => Self{ d: West, ..self }, 
            West => Self{ d: North, ..self }, 
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        match self.d {
            North => Self{ d: West, ..self },
            East => Self{ d: North, ..self },
            South => Self{ d: East, ..self }, 
            West => Self{ d: South, ..self }, 
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (mut x, mut y) = (self.x, self.y);
        match self.d {
            North => y+=1,
            East => x+=1,
            South => y-=1,
            West => x-=1,
        }
        Self{ x, y, ..self }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, inst| {
            match inst {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                 _ => robot
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
