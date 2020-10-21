use crate::{
    components::{CursorPosition, DrawingObject, Geometry, Selected},
    primitives::Line,
};
use specs::prelude::*;

/// A system which looks for selected entities and moves them
/// to the cursor position
#[derive(Debug)]
pub struct Mover;

impl Mover {
    pub const NAME: &'static str = module_path!();

    pub fn new(world: &World) -> Mover {
        Mover {}
    }
}

impl<'world> System<'world> for Mover {
    type SystemData = (
        Entities<'world>,
        ReadStorage<'world, Selected>,
        WriteStorage<'world, DrawingObject>,
        Read<'world, CursorPosition>,
        Read<'world, LazyUpdate>,
    );

    fn run(
        &mut self,
        (entities, selecteds, mut drawing_objects, cursor_position, updater): Self::SystemData,
    ) {
        for (entity, selected, drawing_object) in
            (&entities, &selecteds, &mut drawing_objects).join()
        {
            if let Geometry::Point(point) = drawing_object.geometry {
                drawing_object.geometry =
                    Geometry::Point(cursor_position.location);
            };

            match drawing_object.geometry {
                Geometry::Point(point) => {
                    drawing_object.geometry =
                        Geometry::Point(cursor_position.location);
                }
                Geometry::Line(line) => {
                    drawing_object.geometry = Geometry::Line(Line::new(
                        line.start,
                        cursor_position.location,
                    ));
                }
                _ => (),
            }
        }
    }
}
