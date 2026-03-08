use egui_macroquad::macroquad;
use macroquad::prelude::*;
use super::adaptive::*;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Shape {
    DivLine { x: f32, level: usize },
    EmpPoint { x: f32, y: f32, style: i32 },
    EmpLine { x1: f32, y1: f32, x2: f32, y2: f32, style: i32 },
    ShadedRect { xl: f32, xr: f32, style: i32 },
}

fn rgba_from_str(s: &str) -> Color {
    let hex = s.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    let a = if hex.len() == 8 {
        u8::from_str_radix(&hex[6..8], 16).unwrap()
    } else {
        255
    };
    Color::from_rgba(r, g, b, a)
}

impl Shape {
    pub fn render(&self, n: usize, space: f32) {
        match self {
            Shape::DivLine { x, level } => {
                draw_line(*x, 0.0, *x, space, div_line_width(n, *level), BLACK);
            }
            Shape::ShadedRect { xl, xr, style } => {
                let xl = xl.max(0.0);
                let xr = xr.min(space);
                match style {
                    0 => { draw_rectangle(xl, 0.0, xr - xl, space, rgba_from_str("#0f0f0fd4")) }
                    1 => { draw_rectangle(xl, 0.0, xr - xl, space, rgba_from_str("#dad306a8")) }
                    _ => unimplemented!()
                }
            }
            Shape::EmpPoint { x, y, style } => {
                match style {
                    0 => { draw_circle(*x, *y, 6.0, rgba_from_str("#ff0000")) }
                    1 => { draw_circle(*x, *y, point_radius(n), rgba_from_str("#00851d")) }
                    _ => unimplemented!()
                }
            }
            Shape::EmpLine { x1, y1, x2, y2, style } => {
                match style {
                    0 => { draw_line(*x1, *y1, *x2, *y2, 3.0, rgba_from_str("#ff0000")) }
                    1 => { draw_line(*x1, *y1, *x2, *y2, seg_line_width(n), rgba_from_str("#00851d")) }
                    _ => unimplemented!()
                }
            }
        }
    }
}

pub struct ShapeHandle {
    index: usize,
    timestamp: u32,
}

#[derive(Clone, Copy, Default)]
struct ShapeSlot {
    shape: Option<Shape>,
    timestamp: u32,
}

pub struct ShapeArena {
    shapes: Vec<ShapeSlot>,
    free_indices: Vec<usize>,
}

impl ShapeArena {
    pub fn new() -> Self {
        Self { shapes: Vec::new(), free_indices: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            shapes: vec![ShapeSlot::default(); cap],
            free_indices: (0..cap).collect(),
        }
    }

    pub fn add(&mut self, shape: Shape) -> ShapeHandle {
        if let Some(index) = self.free_indices.pop() {
            let timestamp = self.shapes[index].timestamp.wrapping_add(1);
            self.shapes[index] = ShapeSlot { shape: Some(shape), timestamp };
            ShapeHandle { index, timestamp }
        } else {
            self.shapes.push(ShapeSlot { shape: Some(shape), timestamp: 0 });
            ShapeHandle { index: self.shapes.len() - 1, timestamp: 0 }
        }
    }

    pub fn remove(&mut self, handle: &ShapeHandle) {
        let slot = self.shapes.get_mut(handle.index).unwrap();
        if slot.timestamp == handle.timestamp {
            slot.shape = None;
            self.free_indices.push(handle.index);
        } else {
            panic!("Invalid shape handle: index {}, timestamp {}, but referred timestamp is {}", handle.index, handle.timestamp, slot.timestamp);
        }
    }

    pub fn compile(&self) -> Vec<Shape> {
        self.shapes.iter().filter_map(|slot| slot.shape).collect()
    }
}
