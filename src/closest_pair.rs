use crate::datatype::Total;
use crate::point::{self, Point};

#[derive(Clone, Copy, Debug)]
pub struct Pair(pub Point, pub Point);

#[derive(Clone, Copy)]
pub struct ByDist(pub Pair);

impl Pair {
    pub fn dist(&self) -> f64 {
        let dx = self.0.x - self.1.x;
        let dy = self.0.y - self.1.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn same_pair(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl PartialEq for ByDist {
    fn eq(&self, other: &Self) -> bool {
        Total(self.0.dist()) == Total(other.0.dist())
    }
}

impl Eq for ByDist {}

impl PartialOrd for ByDist {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for ByDist {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let d1 = self.0.dist();
        let d2 = other.0.dist();
        Total(d1).cmp(&Total(d2))
    }
}

fn merge(a: Option<Pair>, b: Option<Pair>) -> Option<Pair> {
    match (a, b) {
        (None, x) | (x, None) => x,
        (Some(a), Some(b)) => Some(ByDist(a).min(ByDist(b)).0),
    }
}

pub fn closest_pair(mut points: Vec<Point>) -> Result<Option<Pair>, String> {
    if points.len() < 2 {
        return Err("At least two points are required".to_string());
    }

    point::sort_x(&mut points);
    if let Some(w) = points.windows(2).find(|w| Total(w[0].x) == Total(w[1].x)) {
        return Err(format!("Duplicate X coordinate: {} and {}", w[0], w[1]));
    }

    let mut aux = vec![Point::new(0.0, 0.0); points.len()];
    Ok(closest_pair_rec(&mut points, &mut aux))
}

fn closest_pair_rec(mut points: &mut [Point], mut aux: &mut [Point]) -> Option<Pair> {
    let n = points.len();
    if n == 1 {
        return None;
    }

    let mid = n / 2;
    let mid_x = points[mid - 1].x;

    let (pl, pr) = points.split_at_mut(mid);
    let (al, ar) = aux.split_at_mut(mid);
    let res_l = closest_pair_rec(pl, al);
    let res_r = closest_pair_rec(pr, ar);
    let mut res = merge(res_l, res_r);

    merge_by_y(&mut points, &mut aux, mid);

    let d = res.map(|r| r.dist());
    let band: Vec<&Point> = points.iter()
        .filter(|p| d.map_or(true, |d| Total((p.x - mid_x).abs()) <= Total(d)))
        .collect();
    for i in 0..band.len() {
        for j in (i + 1)..band.len() {
            if d.is_some_and(|d| Total(band[j].y - band[i].y) > Total(d)) {
                break;
            }
            res = merge(res, Some(Pair(*band[i], *band[j])));
        }
    }
    res
}

fn merge_by_y(data: &mut [Point], aux: &mut [Point], mid: usize) {
    let mut i = 0;
    let mut j = mid;
    let mut k = 0;
    while i < mid && j < data.len() {
        if Total(data[i].y) <= Total(data[j].y) {
            aux[k] = data[i];
            i += 1;
        } else {
            aux[k] = data[j];
            j += 1;
        }
        k += 1;
    }
    aux[k..(k + mid - i)].copy_from_slice(&data[i..mid]);
    k += mid - i;
    aux[k..(k + data.len() - j)].copy_from_slice(&data[j..]);
    data.copy_from_slice(&aux);
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
        let res = closest_pair(points.clone()).unwrap().unwrap();
        assert!(res.same_pair(&Pair(points[1], points[3])));
    }

    #[test]
    #[ignore]
    fn closest_pair2() {
        use rand::prelude::*;

        const N: usize = 1000000;
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
            let mut answer: Option<Pair> = None;
            for i in 0..(N-1) {
                for j in (i+1)..N {
                    let curr = Pair(points[i], points[j]);
                    answer = merge(answer, Some(curr));
                }
            }
            println!("Expected answer is: {:?}", answer);
            assert!(result.same_pair(&answer.unwrap()));
        } else {
            println!("Size too large, skipping verification");
        }
    }
}