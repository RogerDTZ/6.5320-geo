pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point<T>) -> f64
    where
        T: Into<f64> + Copy,
    {
        let dx = self.x.into() - other.x.into();
        let dy = self.y.into() - other.y.into();
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn test_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        let d1 = p1.distance(&p2);
        let d2 = p2.distance(&p1);
        assert!((d1 - d2).abs() < 1e-6);
        assert!((d1 - 5.0).abs() < 1e-6);
    }
}
