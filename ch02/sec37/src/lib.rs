//! 2.3.7 Traits and Trait Bounds
pub fn add_3times<T>(v: T) -> T
where
    // This following trait bound is wrong.
    //T: std::ops::Add<T> + Copy,
    // You should do this.
    T: std::ops::Add<Output = T> + Copy,
{
    v + v + v
}

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self { x, y }
    }
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
