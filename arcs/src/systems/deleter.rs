use crate::components::{Delete, DrawingObject, Selected};
use specs::prelude::*;

/// A system which looks for [`Delete`] command and
/// deletes all selected drawingobjects
#[derive(Debug)]
pub struct Deleter;

impl Deleter {
    pub const NAME: &'static str = module_path!();

    pub fn new(_world: &World) -> Deleter {
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
        (entities, selecteds, drawing_objects, mut deletes, updater): Self::SystemData,
    ) {
        for (entity, _delete) in (&entities, &mut deletes).join() {
            for (entity, _selected, _drawing_object) in
                (&entities, &selecteds, &drawing_objects).join()
            {
                let _ = entities.delete(entity);
            }
            updater.remove::<Delete>(entity);
        }
    }
}
