#[inline]
pub fn point_radius(n: usize) -> f32 {
    (80.0 / (n as f32).sqrt()).clamp(0.5, 4.0)
}

#[inline]
pub fn div_line_width(n: usize, level: usize) -> f32 {
    (10.0 / ((n as f32).sqrt() * (level as f32 * 0.1 + 1.0))).clamp(1.0, 3.0)
}

#[inline]
pub fn seg_line_width(n: usize) -> f32 {
    (30.0 / ((n as f32).sqrt())).clamp(0.5, 3.0)
}
