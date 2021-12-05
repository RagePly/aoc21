#[derive(Hash, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl<T> std::ops::Add for Point<T> where T: std::ops::Add {
    type Output = Point<T::Output>;

    fn add(self, other: Self) -> Self::Output {
        point(self.x + other.x, self.y + other.y)
    }
}

impl<T> std::ops::AddAssign for Point<T> where T: std::ops::AddAssign {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> std::ops::Sub for Point<T> where T: std::ops::Sub {
    type Output = Point<T::Output>;

    fn sub(self, other: Self) -> Self::Output {
        point(self.x - other.x, self.y - other.y)
    }
}

impl<T> PartialEq for Point<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Clone for Point<T> where T: Clone {
    fn clone(&self) -> Self {
        point(self.x.clone(), self.y.clone())
    }
}

#[inline]
pub fn point<T>(x: T, y: T) -> Point<T> {
    Point {
        x: x,
        y: y
    }
}

impl<T> Point<T> where T: Clone + super::num_traits::Signed + std::str::FromStr {
    pub fn manhattan(&self) -> T {
        return self.x.abs() + self.y.abs()
    }

    pub fn rotate(&self, rotation: i32) -> Self {
        match rotation % 4 {
            0 => self.clone(),
            1 => point(-self.y.clone(), self.x.clone()),
            2 => point(-self.x.clone(), self.y.clone()),
            3 => point(self.y.clone(), -self.x.clone()),
            _ => unreachable!()
        }
    }

    pub fn scale(&self, scalar: T) -> Self {
        point(self.x.clone() * scalar.clone(), self.x.clone() * scalar)
    }
}


pub type IPoint = Point<usize>;
pub type Coord = Point<i32>;