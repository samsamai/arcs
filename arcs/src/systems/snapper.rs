use crate::components::{CursorPosition, DrawingObject, Geometry};
use euclid::Point2D;
use specs::prelude::*;

/// A system which snaps CursorPosition to the grid
#[derive(Debug)]
pub struct Snapper;

impl Snapper {
    pub const NAME: &'static str = module_path!();

    pub fn new(world: &World) -> Snapper {
        Snapper {}
    }
}

impl<'world> System<'world> for Snapper {
    type SystemData = (
        WriteStorage<'world, DrawingObject>,
        Write<'world, CursorPosition>,
    );

    fn run(
        &mut self,
        (drawing_objects, mut cursor_position): Self::SystemData,
    ) {
        use specs::Join;
        for drawing_object in (drawing_objects).join() {
            if let Geometry::Grid(grid) = drawing_object.geometry {
                let effective_cursor_location = Point2D::new(
                    cursor_position.location.x
                        - cursor_position.location.x % grid.grid_spacing.0,
                    cursor_position.location.y
                        - cursor_position.location.y % grid.grid_spacing.0,
                );

                cursor_position.location = effective_cursor_location;
            }
        }
    }
}
