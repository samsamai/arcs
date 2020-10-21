use specs::prelude::*;
use specs_derive::Component;

/// An empty [`Component`] used to give a [`Add Line`] command to
/// the system.
#[derive(Debug, Copy, Clone, PartialEq, Component)]
pub struct AddLine {
    pub layer: Entity,
}
