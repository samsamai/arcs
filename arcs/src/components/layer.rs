use crate::components::Name;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub enum LayerType {
    System,
    Geometry,
}
/// A logical grouping of data, assembled as though each [`Layer`] were laid out
/// on transparent acetate overlays.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
#[storage(HashMapStorage)]
pub struct Layer {
    /// The z-coordinate. Lower z-levels will be drawn above higher z-levels.
    pub z_level: usize,
    /// Should entities on this layer be displayed?
    pub visible: bool,
    /// What type of layer this is. A [`System`] layer is used for things such as grids
    /// while [`Geometry`] layers are used for normal user controlled objects
    pub layer_type: LayerType,
}

impl Layer {
    pub fn create(builder: EntityBuilder, name: Name, layer: Layer) -> Entity {
        builder.with(layer).with(name).build()
    }
}

impl Default for Layer {
    fn default() -> Layer {
        Layer {
            z_level: 0,
            visible: true,
            layer_type: LayerType::Geometry,
        }
    }
}
