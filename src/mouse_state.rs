use mandel::{new_vector, tuple_to_vector, Vector};

pub struct MouseState {
    pub pos: Vector,
    prev_pos: Vector,
    pub delta: Vector,
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            pos: new_vector(0.0, 0.0),
            prev_pos: new_vector(0.0, 0.0),
            delta: new_vector(0.0, 0.0),
        }
    }

    pub fn set_pos(&mut self, pos: Option<(f32, f32)>) {
        if let Some(pos) = pos {
            self.pos = tuple_to_vector(pos);
            self.delta = &self.pos - &self.prev_pos;
        }
        self.prev_pos = self.pos.clone();
    }
}
