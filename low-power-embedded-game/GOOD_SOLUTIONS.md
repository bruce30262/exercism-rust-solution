* [steinbro](https://exercism.org/tracks/rust/exercises/low-power-embedded-game/solutions/steinbro)  

```rust
pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.step_by(2)
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        let Position(x, y) = self;
        x.abs() + y.abs()
    }
}

```
