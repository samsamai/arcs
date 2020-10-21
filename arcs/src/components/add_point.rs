use specs::prelude::*;
use specs_derive::Component;

/// An empty [`Component`] used to give a [`Add Point`] command to
/// the system.
#[derive(Debug, Copy, Clone, PartialEq, Component)]
pub struct AddPoint {
    pub layer: Entity,
}
