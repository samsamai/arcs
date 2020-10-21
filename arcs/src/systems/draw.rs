use crate::components::{
    AddLine, AddPoint, CursorPosition, Delete, DrawingObject, Geometry, Layer,
    Selected,
};
use crate::primitives::Line;
use specs::prelude::*;

/// A system which looks for [`Add...`] type commands and
/// begins the process of adding the primitive to the world
/// For example AddPoint command adds a point at the given
/// location, in the given layer
#[derive(Debug)]
pub struct Draw;

impl Draw {
    pub const NAME: &'static str = module_path!();

    pub fn new(world: &World) -> Draw {
        Draw {}
    }
}

impl<'world> System<'world> for Draw {
    type SystemData = (
        Entities<'world>,
        WriteStorage<'world, AddPoint>,
        WriteStorage<'world, AddLine>,
        Read<'world, LazyUpdate>,
        Read<'world, CursorPosition>,
    );

    fn run(
        &mut self,
        (entities, mut add_points, mut add_lines, updater, cursor_position): Self::SystemData,
    ) {
        for (entity, add_point) in (&entities, &mut add_points).join() {
            let new_point = entities.create();

            updater.insert(new_point, Selected);
            updater.insert(
                new_point,
                DrawingObject {
                    geometry: Geometry::Point(cursor_position.location),
                    layer: add_point.layer,
                },
            );
            updater.remove::<AddPoint>(entity);
        }
        for (entity, add_line) in (&entities, &mut add_lines).join() {
            let new_line = entities.create();

            updater.insert(new_line, Selected);
            updater.insert(
                new_line,
                DrawingObject {
                    geometry: Geometry::Line(Line::new(
                        cursor_position.location,
                        cursor_position.location,
                    )),
                    layer: add_line.layer,
                },
            );
            updater.remove::<AddLine>(entity);
        }
    }
}
