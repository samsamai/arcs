use crate::{euclid::Point2D, DrawingSpace};
use specs::prelude::*;
use specs_derive::Component;

/// An empty [`Component`] used to mark an [`Entity`] as selected.
#[derive(Debug, Copy, Clone, PartialEq, Component)]
pub struct AddPoint {
    pub location: Point2D<f64, DrawingSpace>,
    pub layer: Entity,
}
