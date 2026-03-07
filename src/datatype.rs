pub const EPS_F32: f32 = 1e-6;
pub const EPS_F64: f64 = 1e-9;

pub trait ApproxEq: Copy {
    fn approx_eq(self, other: Self) -> bool;
    fn approx_lt(self, other: Self) -> bool;
}

impl ApproxEq for f32 {
    fn approx_eq(self, other: f32) -> bool { (self - other).abs() < EPS_F32 }
    fn approx_lt(self, other: f32) -> bool { other - self > EPS_F32 }
}

impl ApproxEq for f64 {
    fn approx_eq(self, other: f64) -> bool { (self - other).abs() < EPS_F64 }
    fn approx_lt(self, other: f64) -> bool { other - self > EPS_F64 }
}

#[derive(Debug)]
pub struct Total<T: ApproxEq>(pub T);

impl<T: ApproxEq> PartialEq for Total<T> {
    fn eq(&self, other: &Self) -> bool { self.0.approx_eq(other.0) }
}

impl<T: ApproxEq> Eq for Total<T> {}

impl<T: ApproxEq> PartialOrd for Total<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: ApproxEq> Ord for Total<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.0.approx_eq(other.0) {
            std::cmp::Ordering::Equal
        } else if self.0.approx_lt(other.0) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}
