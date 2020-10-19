use crate::{euclid::Point2D, DrawingSpace};
use specs::prelude::*;
use specs_derive::Component;

/// Something which can be drawn on the screen.
#[derive(Debug, Copy, Clone, PartialEq, Component)]
pub struct CursorPosition {
    pub location: Point2D<f64, DrawingSpace>,
}

impl Default for CursorPosition {
    fn default() -> CursorPosition {
        CursorPosition {
            location: Point2D::new(0., 0.),
        }
    }
}
