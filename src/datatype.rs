use std::cmp::Ord;

pub const EPS_F32: f32 = 1e-6;
pub const EPS_F64: f64 = 1e-9;

pub trait FpCmp: Copy + PartialOrd + Into<f64> {
    fn eps() -> Self;
    fn abs_diff(a: Self, b: Self) -> Self;
    fn is_nan(self) -> bool;
}

impl FpCmp for f32 {
    fn eps() -> f32 { EPS_F32 }
    fn abs_diff(a: f32, b: f32) -> f32 { (a - b).abs() }
    fn is_nan(self) -> bool { self.is_nan() }
}

impl FpCmp for f64 {
    fn eps() -> f64 { EPS_F64 }
    fn abs_diff(a: f64, b: f64) -> f64 { (a - b).abs() }
    fn is_nan(self) -> bool { self.is_nan() }
}

#[derive(Copy, Clone, Debug)]
pub struct Total<T>(T);

impl<T: FpCmp> Total<T> {
    pub fn new(val: T) -> Self {
        if val.is_nan() {
            panic!("Value cannot be NaN");
        } else {
            Self(val)
        }
    }

    pub fn value(&self) -> T { self.0 }
}

impl<T: FpCmp> From<Total<T>> for f64 {
    fn from(value: Total<T>) -> Self {
        value.0.into()
    }
}

impl<T: FpCmp> PartialEq for Total<T> {
    fn eq(&self, other: &Self) -> bool {
        T::abs_diff(self.0, other.0) < T::eps()
    }
}

impl<T: FpCmp> Eq for Total<T> {}

impl<T: FpCmp> PartialOrd for Total<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: FpCmp> Ord for Total<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            std::cmp::Ordering::Equal
        } else if self.0 < other.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Total;

    #[test]
    fn test_total() {
        let a = Total::new(1.0 as f32);
        let b = Total::new((1.0 - 1e-7) as f32);
        let c = Total::new((1.0 + 1e-5) as f32);
        assert_eq!(a, b);
        assert_eq!(b, a);
        assert_ne!(a, c);
        assert!(a < c);
        assert!(a <= b);

        let a = Total::new(1.0 as f64);
        let b = Total::new((1.0 - 1e-10) as f64);
        let c = Total::new((1.0 + 1e-8) as f64);
        assert_eq!(a, b);
        assert_eq!(b, a);
        assert_ne!(a, c);
        assert!(a < c);
        assert!(a <= b);
    }
}
