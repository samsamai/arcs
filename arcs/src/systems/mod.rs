//! Background tasks and useful [`specs::System`]s.

mod bounds;
mod name_table_bookkeeping;
// mod spatial_relation;
pub mod draw;
pub mod mover;

pub use bounds::SyncBounds;
pub use name_table_bookkeeping::NameTableBookkeeping;
// pub use spatial_relation::SpatialRelation;
pub use draw::Draw;
pub use mover::Mover;

use specs::{DispatcherBuilder, World};

/// Register any necessary background tasks with a [`DispatcherBuilder`].
pub fn register_background_tasks<'a, 'b>(
    builder: DispatcherBuilder<'a, 'b>,
    world: &World,
) -> DispatcherBuilder<'a, 'b> {
    builder
        .with(
            NameTableBookkeeping::new(world),
            NameTableBookkeeping::NAME,
            &[],
        )
        .with(SyncBounds::new(world), SyncBounds::NAME, &[])
}
