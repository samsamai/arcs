use crate::{
    algorithms::Bounded,
    components::{AddPoint, DrawingObject, Geometry, Layer, Selected},
};
use specs::prelude::*;

/// Lets us keep track of a [`DrawingObject`]'s rough location in *Drawing
/// Space*.
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
        use specs::Join;
        for (add_point) in (&mut add_points).join() {
            let temp_point = entities.create();

            updater.insert(temp_point, Selected);
            updater.insert(
                temp_point,
                DrawingObject {
                    geometry: Geometry::Point(add_point.location),
                    layer: add_point.layer,
                },
            );
        }
    }

    //     let (mut bounds, drawing_objects, entities) = data;

    //     // find out which items have changed since we were last polled
    //     for event in drawing_objects.channel().read(&mut self.changes) {
    //         match *event {
    //             ComponentEvent::Inserted(id) | ComponentEvent::Modified(id) => {
    //                 self.to_update.add(id);
    //             }
    //             ComponentEvent::Removed(id) => {
    //                 self.removed.add(id);
    //             }
    //         }
    //     }

    //     for (ent, drawing_object, _) in
    //         (&entities, &drawing_objects, &self.to_update).join()
    //     {
    //         bounds
    //             .insert(ent, drawing_object.geometry.bounding_box())
    //             .unwrap();
    //     }

    //     for (ent, _) in (&entities, &self.removed).join() {
    //         bounds.remove(ent);
    //     }
    // }
}
