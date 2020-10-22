use crate::components::{CursorPosition, DrawingObject, Geometry};
use euclid::Point2D;
use specs::prelude::*;

/// A system which snaps CursorPosition to the grid
#[derive(Debug)]
pub struct Snapper;

impl Snapper {
    pub const NAME: &'static str = module_path!();

    pub fn new(_world: &World) -> Snapper {
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
        for drawing_object in (drawing_objects).join() {
            if let Geometry::Grid(grid) = drawing_object.geometry {
                let spacing = grid.grid_spacing.0;
                if grid.snap {
                    let mx = cursor_position.location.x
                        - cursor_position.location.x % spacing;

                    let my = cursor_position.location.y
                        - cursor_position.location.y % spacing;

                    let effective_cursor_location = Point2D::new(
                        mx + ((cursor_position.location.x % spacing) / spacing)
                            .round()
                            * spacing,
                        my + ((cursor_position.location.y % spacing) / spacing)
                            .round()
                            * spacing,
                    );

                    cursor_position.location = effective_cursor_location;
                }
            }
        }
    }
}
