#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
}

#[derive(Debug, Clone, Copy)]
pub struct Touch {
    pub id: i32,
    pub x: f32,
    pub y: f32,
    pub phase: TouchPhase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    W, A, S, D,
    Space,
    Escape,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    Touch(Touch),
    KeyDown(Key),
    KeyUp(Key),
}