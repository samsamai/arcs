use crate::components::{Delete, DrawingObject, Selected};
use specs::prelude::*;

/// A system which looks for [`Delete`] command and
/// deletes all selected drawingobjects
#[derive(Debug)]
pub struct Deleter;

impl Deleter {
    pub const NAME: &'static str = module_path!();

    pub fn new(world: &World) -> Deleter {
        Deleter {}
    }
}

impl<'world> System<'world> for Deleter {
    type SystemData = (
        Entities<'world>,
        WriteStorage<'world, Selected>,
        WriteStorage<'world, DrawingObject>,
        WriteStorage<'world, Delete>,
        Read<'world, LazyUpdate>,
    );

    fn run(
        &mut self,
        (entities, mut selecteds, mut drawing_objects, mut deletes, updater): Self::SystemData,
    ) {
        for (entity, delete) in (&entities, &mut deletes).join() {
            for (entity, selected, drawing_object) in
                (&entities, &selecteds, &drawing_objects).join()
            {
                entities.delete(entity);
            }
            updater.remove::<Delete>(entity);
        }
    }
}
