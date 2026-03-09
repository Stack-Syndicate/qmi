use crate::world::World;

pub type EntityID = hecs::Entity;
pub trait System = FnMut(&mut World) + Send;
pub trait Query = hecs::Query;
