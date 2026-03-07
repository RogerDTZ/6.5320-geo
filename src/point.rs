use crate::datatype::Total;
use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        if x.is_nan() || y.is_nan() {
            panic!("Coordinates cannot be NaN");
        }
        Self { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        Total::new(self.x) == Total::new(other.x) && Total::new(self.y) == Total::new(other.y)
    }
}

impl Eq for Point {}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

pub fn sort_x(points: &mut [Point]) {
    points.sort_by_key(|p| Total::new(p.x));
}

pub fn sort_y(points: &mut [Point]) {
    points.sort_by_key(|p| Total::new(p.y));
}

#[cfg(test)]
mod tests {
    use crate::datatype::*;
    use super::{Point, sort_x, sort_y};

    #[test]
    fn test_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        let d1 = p1.distance(&p2);
        let d2 = p2.distance(&p1);
        assert_eq!(Total::new(d1), Total::new(d2));
        assert_eq!(Total::new(d1), Total::new(5.0));
    }

    #[test]
    fn test_pointvec_sort() {
        let pv = vec![Point::new(1.0, 1.0), Point::new(2.0, 1.1), Point::new(1.9, 2.5)];
        let sx = vec![Point::new(1.0, 1.0), Point::new(1.9, 2.5), Point::new(2.0, 1.1)];
        let sy = vec![Point::new(1.0, 1.0), Point::new(2.0, 1.1), Point::new(1.9, 2.5)];
        let mut pv1 = pv.clone();
        sort_x(&mut pv1);
        pv1.iter().zip(sx.iter()).for_each(|(p1, p2)| {
            assert_eq!(p1, p2);
        });
        let mut pv2 = pv.clone();
        sort_y(&mut pv2);
        pv2.iter().zip(sy.iter()).for_each(|(p1, p2)| {
            assert_eq!(p1, p2);
        });
    }
    
    #[test]
    fn test_eps() {
        assert_eq!(Point::new(1.0, 1.0), Point::new(1.0 + EPS_F64 / 2.0, 1.0                ));
        assert_ne!(Point::new(1.0, 1.0), Point::new(1.0,                 1.0 + EPS_F64 * 1.1));
    }
}
