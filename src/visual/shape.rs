use egui_macroquad::macroquad;
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum Shape {
    DivLine(f32),
    ShadedRect(Rect),
    Band { x: f32, d: f32 },
    EmpPoint { x: f32, y: f32, style: i32 },
    EmpLine { x1: f32, y1: f32, x2: f32, y2: f32, style: i32 },
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

    pub fn remove(&mut self, handle: ShapeHandle) {
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
