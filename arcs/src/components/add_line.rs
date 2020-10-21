use crate::{euclid::Point2D, DrawingSpace};
use specs::prelude::*;
use specs_derive::Component;

/// An empty [`Component`] used to give a [`Add Line`] command to
/// the system.
#[derive(Debug, Copy, Clone, PartialEq, Component)]
pub struct AddLine {
    pub location: Point2D<f64, DrawingSpace>,
    pub layer: Entity,
}
