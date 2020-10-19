use crate::{euclid::Point2D, CanvasSpace};
use specs::prelude::*;

/// Something which can be drawn on the screen.
#[derive(Debug, Clone, PartialEq)]
pub struct CursorPosition {
    pub position: Point2D<f64, CanvasSpace>,
}

impl Component for CursorPosition {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
