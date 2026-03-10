use super::adaptive::*;
use egui_macroquad::macroquad;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Shape {
    DivLine {
        x: f32,
        level: usize,
    },
    EmpPoint {
        x: f32,
        y: f32,
        style: i32,
    },
    EmpLine {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        style: i32,
    },
    ShadedRect {
        xl: f32,
        xr: f32,
        style: i32,
    },
}

const fn hex_digit(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => panic!("invalid hex digit in color literal"),
    }
}

const fn hex_byte(hi: u8, lo: u8) -> u8 {
    hex_digit(hi) << 4 | hex_digit(lo)
}

const fn parse_hex_color(s: &str) -> (u8, u8, u8, u8) {
    let b = s.as_bytes();
    let i = if b[0] == b'#' { 1 } else { 0 };
    let r = hex_byte(b[i], b[i + 1]);
    let g = hex_byte(b[i + 2], b[i + 3]);
    let v = hex_byte(b[i + 4], b[i + 5]);
    let a = if b.len() - i == 8 {
        hex_byte(b[i + 6], b[i + 7])
    } else {
        0xFF
    };
    (r, g, v, a)
}

macro_rules! hex_color {
    ($s:literal) => {{
        const RGBA: (u8, u8, u8, u8) = parse_hex_color($s);
        Color::from_rgba(RGBA.0, RGBA.1, RGBA.2, RGBA.3)
    }};
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
                    0 => draw_rectangle(xl, 0.0, xr - xl, space, hex_color!("#0f0f0fd4")),
                    1 => draw_rectangle(xl, 0.0, xr - xl, space, hex_color!("#dad306a8")),
                    _ => unimplemented!(),
                }
            }
            Shape::EmpPoint { x, y, style } => match style {
                0 => draw_circle(*x, *y, point_radius(n) * 1.3, hex_color!("#ff0000ff")),
                1 => draw_circle(*x, *y, point_radius(n), hex_color!("#00851dff")),
                _ => unimplemented!(),
            },
            Shape::EmpLine {
                x1,
                y1,
                x2,
                y2,
                style,
            } => match style {
                0 => draw_line(
                    *x1,
                    *y1,
                    *x2,
                    *y2,
                    seg_line_width(n) * 1.3,
                    hex_color!("#ff0000ff"),
                ),
                1 => draw_line(
                    *x1,
                    *y1,
                    *x2,
                    *y2,
                    seg_line_width(n),
                    hex_color!("#00851dff"),
                ),
                _ => unimplemented!(),
            },
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
        Self {
            shapes: Vec::new(),
            free_indices: Vec::new(),
        }
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
            self.shapes[index] = ShapeSlot {
                shape: Some(shape),
                timestamp,
            };
            ShapeHandle { index, timestamp }
        } else {
            self.shapes.push(ShapeSlot {
                shape: Some(shape),
                timestamp: 0,
            });
            ShapeHandle {
                index: self.shapes.len() - 1,
                timestamp: 0,
            }
        }
    }

    pub fn remove(&mut self, handle: &ShapeHandle) {
        let slot = self.shapes.get_mut(handle.index).unwrap();
        if slot.timestamp == handle.timestamp {
            slot.shape = None;
            self.free_indices.push(handle.index);
        } else {
            panic!(
                "Invalid shape handle: index {}, timestamp {}, but referred timestamp is {}",
                handle.index, handle.timestamp, slot.timestamp
            );
        }
    }

    pub fn compile(&self) -> Vec<Shape> {
        self.shapes.iter().filter_map(|slot| slot.shape).collect()
    }
}
