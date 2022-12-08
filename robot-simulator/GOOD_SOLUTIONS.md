* [CodeGradox](https://exercism.org/tracks/rust/exercises/robot-simulator/solutions/CodeGradox)  

```rust
// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
use Direction::*;
#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
#[derive(Debug)]
pub struct Robot {
    x: isize,
    y: isize,
    direction: Direction
}
impl Robot {
    pub fn new(x: isize, y: isize, direction: Direction) -> Self {
        Self { x, y, direction }
    }
    pub fn turn_right(self) -> Self {
        match self.direction {
            North => Self { direction: East,  ..self },
            East  => Self { direction: South, ..self },
            South => Self { direction: West,  ..self },
            West  => Self { direction: North, ..self }
        }
    }
    pub fn turn_left(self) -> Self {
        match self.direction {
            North => Self { direction: West,  ..self },
            East  => Self { direction: North, ..self },
            South => Self { direction: East,  ..self },
            West  => Self { direction: South, ..self }
        }
    }
    pub fn advance(self) -> Self {
        match self.direction {
            North => Self { y: self.y + 1, ..self },
            East  => Self { x: self.x + 1, ..self },
            South => Self { y: self.y - 1, ..self },
            West  => Self { x: self.x - 1, ..self }
        }
    }
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars()
            .fold(self, |robot, ch|
                match ch {
                    'L' => robot.turn_left(),
                    'R' => robot.turn_right(),
                    'A' => robot.advance(),
                    _   => robot,
                })
    }
    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }
    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
```

