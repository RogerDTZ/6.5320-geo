#[inline]
pub fn point_radius(n: usize) -> f32 {
    (100.0 / (n as f32).sqrt()).clamp(0.5, 5.0)
}

#[inline]
pub fn div_line_width(n: usize, level: usize) -> f32 {
    (10.0 / ((n as f32).sqrt() * (level as f32 + 1.0))).clamp(0.5, 3.0)
}

#[inline]
pub fn seg_line_width(n: usize) -> f32 {
    (10.0 / ((n as f32).sqrt())).clamp(0.5, 3.0)
}
