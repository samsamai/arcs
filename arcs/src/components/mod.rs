//! Common components used by the `arcs` CAD library.

mod add_line;
mod add_point;
mod cursor_position;
mod delete;
mod dimension;
mod drawing_object;
pub mod layer;
mod name;
mod selected;
mod styles;
mod viewport;
mod vtable;

// FIXME: I'm not 100% sure this was the right approach for a quadtree...
// mod spatial_entity;
// pub use spatial_entity::{Space, SpatialEntity};

pub use add_line::AddLine;
pub use add_point::AddPoint;
pub use cursor_position::CursorPosition;
pub use delete::Delete;
pub use dimension::Dimension;
pub use drawing_object::{DrawingObject, Geometry};
pub use layer::Layer;
pub use name::{Name, NameTable};
pub use selected::Selected;
pub use styles::{GridStyle, LineStyle, PointStyle, WindowStyle};
pub use viewport::Viewport;
pub(crate) use vtable::ComponentVtable;

use crate::DrawingSpace;
use specs::World;

/// Get an iterator over the [`ComponentVtable`] for all known
/// [`specs::Component`] types.
pub(crate) fn known_components(
) -> impl Iterator<Item = &'static ComponentVtable> + 'static {
    lazy_static::lazy_static! {
        static ref VTABLES: Vec<ComponentVtable> = vec![
            ComponentVtable::for_type::<AddLine>(),
            ComponentVtable::for_type::<AddPoint>(),
            ComponentVtable::for_type::<arcs_core::BoundingBox<DrawingSpace>>(),
            ComponentVtable::for_type::<Delete>(),
            ComponentVtable::for_type::<DrawingObject>(),
            ComponentVtable::for_type::<Layer>(),
            ComponentVtable::for_type::<Name>(),
            ComponentVtable::for_type::<GridStyle>(),
            ComponentVtable::for_type::<LineStyle>(),
            ComponentVtable::for_type::<PointStyle>(),
            ComponentVtable::for_type::<Selected>(),
            ComponentVtable::for_type::<WindowStyle>(),
            ComponentVtable::for_type::<Viewport>(),
            ComponentVtable::for_type::<CursorPosition>(),
        ];
    }

    VTABLES.iter()
}

/// Register all [`specs::Component`]s.
pub fn register(world: &mut World) {
    log::debug!("Registering all components");

    for component in known_components() {
        log::debug!("Registering {}", component.name());
        component.register(world);
    }
}
