use crate::datatype::Total;
use crate::point::{self, Point};

#[derive(Clone, Copy, Debug)]
pub struct ClosestPairResult {
    pub p1: Point,
    pub p2: Point,
}

impl ClosestPairResult {
    pub fn dist(&self) -> f64 {
        let dx = self.p1.x - self.p2.x;
        let dy = self.p1.y - self.p2.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn merge(a: &Option<Self>, b: &Option<Self>) -> Option<Self> {
        match (a, b) {
            (None, None) => None,
            (Some(_), None) => *a,
            (None, Some(_)) => *b,
            (Some(pa), Some(pb)) => {
                if Total::new(pa.dist()) <= Total::new(pb.dist()) {
                    *a
                } else {
                    *b
                }
            }
        }
    }

    pub fn sym(&self) -> Self {
        Self { p1: self.p2, p2: self.p1 }
    }
}

impl Into<(Point, Point)> for ClosestPairResult {
    fn into(self) -> (Point, Point) {
        (self.p1, self.p2)
    }
}

pub fn closest_pair(mut points: Vec<Point>) -> Result<Option<ClosestPairResult>, String> {
    point::sort_x(&mut points);
    if let Some(w) = points.windows(2).find(|w| Total::new(w[0].x) == Total::new(w[1].x)) {
        return Err(format!("Duplicate X coordinate: {} and {}", w[0], w[1]));
    }

    Ok(closest_pair_rec(points).0)
}

fn closest_pair_rec(mut points: Vec<Point>) -> (Option<ClosestPairResult>, Vec<Point>) {
    if points.is_empty() {
        panic!("No points provided");
    }
    if points.len() == 1 {
        return (None, points);
    }

    let n = points.len();
    let mid = n / 2;
    let mid_x = points[mid - 1].x;
    let points_r = points.split_off(mid);
    let points_l = points;
    let (res_l, points_l) = closest_pair_rec(points_l);
    let (res_r, points_r) = closest_pair_rec(points_r);
    let mut res = ClosestPairResult::merge(&res_l, &res_r);

    let mut points = Vec::with_capacity(points_l.len() + points_r.len());
    let mut l = points_l.into_iter().peekable();
    let mut r = points_r.into_iter().peekable();
    while let (Some(pl), Some(pr)) = (l.peek(), r.peek()) {
        if Total::new(pl.y) <= Total::new(pr.y) {
            points.push(l.next().unwrap());
        } else {
            points.push(r.next().unwrap());
        }
    }
    points.extend(l);
    points.extend(r);

    let d = res.map(|r| Total::new(r.dist()));
    let vips: Vec<&Point> = points.iter().filter(|p| d.map_or(true, |d| Total::new((p.x - mid_x).abs()) <= d)).collect();
    for i in 0..vips.len() {
        let mut j = i + 1;
        while j < vips.len() {
            if d.map_or(false, |d| Total::new(vips[j].y - vips[i].y) > d) {
                break;
            }
            res = ClosestPairResult::merge(&res, &Some(ClosestPairResult { p1: *vips[i], p2: *vips[j] }));
            j += 1;
        }
    }
    (res, points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closest_pair1() {
        let points = vec![
            Point { x: -10.0, y: -10.0 },
            Point { x: 1.0, y: 1.0 },
            Point { x: 2.0, y: 2.0 },
            Point { x: 0.5, y: 0.5 },
        ];
        let res: (Point, Point) = closest_pair(points.clone()).unwrap().unwrap().into();
        assert!(res == (points[1], points[3]) || res == (points[3], points[1]));
    }

    #[test]
    #[ignore]
    fn closest_pair2() {
        use rand::prelude::*;

        const N: usize = 50000;
        const VERIFY_THRESHOLD: usize = 50000;
        let mut points = Vec::<Point>::new();
        points.reserve(N);
        let mut rng = rand::rng();
        for _ in 0..N {
            let x = rng.random_range(-10000.0..10000.0);
            let y = rng.random_range(-10000.0..10000.0);
            points.push(Point { x, y });
        }

        let result = closest_pair(points.clone()).unwrap().unwrap();
        println!("Main algorithm reports: {:?}", result);
        if N <= VERIFY_THRESHOLD {
            let mut answer: Option<ClosestPairResult> = None;
            for i in 0..(N-1) {
                for j in (i+1)..N {
                    let curr = ClosestPairResult { p1: points[i], p2: points[j] };
                    answer = ClosestPairResult::merge(&answer, &Some(curr));
                }
            }
            println!("Expected answer is: {:?}", answer);

            let result_tuple: (Point, Point) = result.into();
            let answer_tuple: (Point, Point) = answer.clone().unwrap().into();
            let answer_sym_tuple = answer.unwrap().sym().into();
            assert!(result_tuple == answer_tuple || result_tuple == answer_sym_tuple);
        } else {
            println!("Size too large, skipping verification");
        }
    }
}