pub mod adaptive;
mod notification;
pub use notification::*;
pub mod shape;

use shape::*;

pub struct Frame {
    shapes: Vec<shape::Shape>,
    lasting: Option<f32>,
}

pub trait Recording {
    fn add(&mut self, shape: Shape) -> Option<ShapeHandle>;
    fn remove(&mut self, handle: &Option<ShapeHandle>);
    fn next_frame(&mut self, lasting: Option<f32>);
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
}

pub struct Player {
    fman: FrameManager,
    curr_frame: usize,
    elapsed: f32,
}

impl From<FrameManager> for Player {
    fn from(fman: FrameManager) -> Self {
        Player {
            fman,
            curr_frame: 0,
            elapsed: 0.0,
        }
    }
}

impl Player {
    pub fn update(&mut self, dt: f32) {
        if self.fman.frames.is_empty() || self.curr_frame == self.fman.frames.len() - 1 {
            // Freeze at the last frame
            return;
        }
        self.elapsed += dt;
        while self.curr_frame < self.fman.frames.len() - 1 {
            let frame = &self.fman.frames[self.curr_frame];
            match frame.lasting {
                Some(lasting) => {
                    if self.elapsed > lasting {
                        self.elapsed -= lasting;
                        self.curr_frame += 1;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    pub fn get_shapes(&self) -> &[Shape] {
        &self.fman.frames[self.curr_frame].shapes
    }

    pub fn finished(&self) -> bool {
        self.fman.frames.is_empty() || self.curr_frame >= self.fman.frames.len() - 1
    }
}

impl Recording for FrameManager {
    fn add(&mut self, shape: Shape) -> Option<ShapeHandle> {
        Some(self.arena.add(shape))
    }

    fn remove(&mut self, handle: &Option<ShapeHandle>) {
        if let Some(hdl) = handle {
            self.arena.remove(hdl);
        }
    }

    fn next_frame(&mut self, lasting: Option<f32>) {
        let mut shapes = self.arena.compile();
        shapes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        self.frames.push(Frame { shapes, lasting });
    }
}

pub struct NoRecord;
impl Recording for NoRecord {
    fn add(&mut self, _shape: Shape) -> Option<ShapeHandle> { None }

    fn remove(&mut self, _handle: &Option<ShapeHandle>) {}

    fn next_frame(&mut self, _lasting: Option<f32>) {}
}
