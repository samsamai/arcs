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

    pub fn new(_world: &World) -> Mover {
        Mover {}
    }
}

impl<'world> System<'world> for Mover {
    type SystemData = (
        Entities<'world>,
        ReadStorage<'world, Selected>,
        WriteStorage<'world, DrawingObject>,
        Read<'world, CursorPosition>,
    );

    fn run(
        &mut self,
        (entities, selecteds, mut drawing_objects, cursor_position): Self::SystemData,
    ) {
        for (_entity, _selected, drawing_object) in
            (&entities, &selecteds, &mut drawing_objects).join()
        {
            if let Geometry::Point(_point) = drawing_object.geometry {
                drawing_object.geometry =
                    Geometry::Point(cursor_position.location);
            };

            match drawing_object.geometry {
                Geometry::Point(_point) => {
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
