use hecs::{QueryBorrow, QueryMut};

use crate::aliases::{EntityID, System};

pub struct World {
    hecs_world: hecs::World,
    systems_bundles: Vec<Vec<Box<dyn System>>>,
}
impl World {
    pub(crate) fn new() -> Self {
        Self {
            hecs_world: hecs::World::new(),
            systems_bundles: Vec::new(),
        }
    }
    pub fn spawn(&mut self, components: impl hecs::DynamicBundle) -> EntityID {
        self.hecs_world.spawn(components)
    }
    pub fn despawn(&mut self, entity: EntityID) -> anyhow::Result<()> {
        self.hecs_world.despawn(entity)?;
        Ok(())
    }
    pub(crate) fn add_systems(&mut self, systems: Vec<Box<dyn System>>) {
        self.systems_bundles.push(systems);
    }
    pub(crate) fn run_systems(&mut self) {
        let mut bundles = std::mem::take(&mut self.systems_bundles);
        for bundle in bundles.iter_mut() {
            for system in bundle {
                system(self)
            }
        }
        self.systems_bundles = bundles;
    }
    pub fn query<T: hecs::Query>(&self) -> QueryBorrow<'_, T> {
        self.hecs_world.query::<T>()
    }
    pub fn query_mut<T: hecs::Query>(&mut self) -> QueryMut<'_, T> {
        self.hecs_world.query_mut::<T>()
    }
}
