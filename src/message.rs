use crate::{
    aliases::{EntityID, System},
    world::World,
};

pub enum Message {
    Tick,
    Create {
        spawn_fn: Box<dyn FnOnce(&mut World) -> EntityID + Send>,
        response: std::sync::mpsc::Sender<EntityID>,
    },
    Delete {
        entity: EntityID,
    },
    Systems {
        systems: Vec<Box<dyn System>>,
    },
}
