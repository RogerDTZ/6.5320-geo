pub mod adaptive;
mod notification;
pub use notification::*;
pub mod shape;

use shape::*;

pub struct Frame {
    shapes: Vec<shape::Shape>,
    lasting: Option<f32>,
}

pub struct FrameManager {
    arena: shape::ShapeArena,
    pub frames: Vec<Frame>,
}

impl FrameManager {
    pub fn with_arena_capacity(arena_cap: usize) -> Self {
        Self {
            arena: ShapeArena::with_capacity(arena_cap),
            frames: Vec::new(),
        }
    }
    
    pub fn add(&mut self, shape: Shape) -> ShapeHandle {
        self.arena.add(shape)
    }

    pub fn remove(&mut self, handle: ShapeHandle) {
        self.arena.remove(handle);
    }

    pub fn next_frame(&mut self, lasting: Option<f32>) {
        let shapes = self.arena.compile();
        self.frames.push(Frame { shapes, lasting });
    }
}
