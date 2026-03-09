use crate::datatype::Total;
use crate::point::{self, Point};
use crate::visual::{Recording, NoRecord, shape::{Shape, ShapeHandle}};

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

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.same_pair(other)
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
        Some(self.cmp(other))
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

pub fn closest_pair<R: Recording>(mut points: Vec<Point>, record: &mut R, animated: bool) -> Result<Pair, String> {
    if points.len() < 2 {
        return Err("At least two points are required".to_string());
    }

    point::sort_x(&mut points);
    let xrange = (f64::NEG_INFINITY, f64::INFINITY);

    let mut aux = vec![Point::new(0.0, 0.0); points.len()];
    let pair = if animated {
        closest_pair_rec(&mut points, &mut aux, xrange, 0, record).0.unwrap()
    } else {
        let pair = closest_pair_rec(&mut points, &mut aux, xrange, 0, &mut NoRecord).0.unwrap();
        record.add(Shape::EmpPoint { x: pair.0.x as f32, y: pair.0.y as f32, style: 0 });
        record.add(Shape::EmpPoint { x: pair.1.x as f32, y: pair.1.y as f32, style: 0 });
        record.add(Shape::EmpLine { x1: pair.0.x as f32, y1: pair.0.y as f32, x2: pair.1.x as f32, y2: pair.1.y as f32, style: 0 });
        record.next_frame(None);
        pair
    };
    Ok(pair)
}

fn closest_pair_rec<R: Recording>(points: &mut [Point], aux: &mut [Point], xrange: (f64, f64), level: usize, record: &mut R) -> (Option<Pair>, Vec<ShapeHandle>) {
    let n = points.len();
    if n == 1 {
        return (None, vec![]);
    }

    let mid = n / 2;
    let mid_x = points[mid - 1].x;
    let hdl_div = record.add(Shape::DivLine { x: mid_x as f32, level });
    record.next_frame(Some(0.2));

    let (pl, pr) = points.split_at_mut(mid);
    let (al, ar) = aux.split_at_mut(mid);

    let hdl_cover_r = record.add(Shape::ShadedRect { xl: mid_x as f32, xr: xrange.1 as f32, style: 0 });
    record.next_frame(Some(0.2));
    let (res_l, hdl_l) = closest_pair_rec(pl, al, (xrange.0, mid_x), level + 1, record);
    record.remove(&hdl_cover_r);
    record.next_frame(Some(0.2));

    let hdl_cover_l = record.add(Shape::ShadedRect { xl: xrange.0 as f32, xr: mid_x as f32, style: 0 });
    record.next_frame(Some(0.2));
    let (res_r, hdl_r) = closest_pair_rec(pr, ar, (mid_x, xrange.1), level + 1, record);
    record.remove(&hdl_cover_l);
    record.next_frame(Some(0.2));

    let mut res = merge(res_l, res_r);
    let mut hdls_best = if res_l == res {
        for hdl in hdl_r {
            record.remove(&Some(hdl));
        }
        hdl_l
    } else {
        for hdl in hdl_l {
            record.remove(&Some(hdl));
        }
        hdl_r
    };

    merge_by_y(points, aux, mid);

    let d = res.map(|r| r.dist());
    let band: Vec<&Point> = points.iter()
        .filter(|p| d.map_or(true, |d| Total((p.x - mid_x).abs()) <= Total(d)))
        .collect();
    let hdl_band = record.add(d.map_or(
        Shape::ShadedRect { xl: xrange.0 as f32, xr: xrange.1 as f32, style: 1 },
        |d| Shape::ShadedRect { xl: (mid_x - d).max(xrange.0) as f32, xr: (mid_x + d).min(xrange.1) as f32, style: 1 }
    ));
    record.next_frame(Some(0.5));

    for i in 0..band.len() {
        let hdl_i = record.add(Shape::EmpPoint { x: band[i].x as f32, y: band[i].y as f32, style: 1 });
        for j in (i + 1)..band.len() {
            if d.is_some_and(|d| Total(band[j].y - band[i].y) > Total(d)) {
                break;
            }
            let hdl_j = record.add(Shape::EmpPoint { x: band[j].x as f32, y: band[j].y as f32, style: 1 });
            let seg = record.add(Shape::EmpLine { x1: band[i].x as f32, y1: band[i].y as f32, x2: band[j].x as f32, y2: band[j].y as f32, style: 1 });
            record.next_frame(Some(0.2));
            record.remove(&hdl_j);
            record.remove(&seg);
            let pr = Some(Pair(*band[i], *band[j]));
            res = merge(res, pr);
            if res == pr {
                for hdl in hdls_best {
                    record.remove(&Some(hdl));
                }
                hdls_best = vec![
                    record.add(Shape::EmpPoint { x: band[i].x as f32, y: band[i].y as f32, style: 0 }),
                    record.add(Shape::EmpPoint { x: band[j].x as f32, y: band[j].y as f32, style: 0 }),
                    record.add(Shape::EmpLine { x1: band[i].x as f32, y1: band[i].y as f32, x2: band[j].x as f32, y2: band[j].y as f32, style: 0 }),
                ].into_iter().flatten().collect();
            }
        }
        record.remove(&hdl_i);
    }

    record.remove(&hdl_div);
    record.remove(&hdl_band);
    record.next_frame(Some(1.0));
    (res, hdls_best)
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
    use crate::visual::NoRecord;

    #[test]
    fn closest_pair1() {
        let points = vec![
            Point::new(-10.0, -10.0),
            Point::new(1.0, 1.0),
            Point::new(2.0, 2.0),
            Point::new(0.2, 0.2),
        ];
        let res = closest_pair(points.clone(), &mut NoRecord, false).unwrap();
        assert!(res.same_pair(&Pair(points[1], points[3])));
    }

    #[test]
    #[ignore]
    fn closest_pair2() {
        use rand::prelude::*;

        const N: usize = 1000000;
        const VERIFY_THRESHOLD: usize = 50000;
        let mut points = Vec::with_capacity(N);
        let mut rng = rand::rng();
        for _ in 0..N {
            let x = rng.random_range(-10000.0..10000.0);
            let y = rng.random_range(-10000.0..10000.0);
            points.push(Point::new(x, y));
        }

        let result = closest_pair(points.clone(), &mut NoRecord, false).unwrap();
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