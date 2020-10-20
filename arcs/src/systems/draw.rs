use crate::components::{
    AddPoint, Delete, DrawingObject, Geometry, Layer, Selected,
};
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
        Read<'world, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut add_points, updater): Self::SystemData) {
        for (entity, add_point) in (&entities, &mut add_points).join() {
            let temp_point = entities.create();

            updater.insert(temp_point, Selected);
            updater.insert(
                temp_point,
                DrawingObject {
                    geometry: Geometry::Point(add_point.location),
                    layer: add_point.layer,
                },
            );
            updater.remove::<AddPoint>(entity);
        }
    }
}
