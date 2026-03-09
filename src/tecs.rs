use std::{
    sync::mpsc::{Sender, channel},
    thread::JoinHandle,
};

use crate::{
    aliases::{EntityID, System},
    message::Message,
    world::World,
};

pub struct TECS {
    thread: JoinHandle<()>,
    tx: Sender<Message>,
}
impl TECS {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let thread = std::thread::spawn(move || {
            let mut ecs = World::new();
            while let Ok(cmd) = rx.recv() {
                match cmd {
                    Message::Tick => {
                        ecs.run_systems();
                    }
                    Message::Create { spawn_fn, response } => {
                        let e = spawn_fn(&mut ecs);
                        response.send(e).unwrap();
                    }
                    Message::Delete { entity } => {
                        ecs.despawn(entity).expect("Could not despawn entity.")
                    }
                    Message::Systems { systems } => {
                        ecs.add_systems(systems);
                    }
                }
            }
        });
        Self { thread, tx }
    }
    fn send(&self, message: Message) {
        self.tx.send(message).expect("Could not send message.");
    }
    pub fn tick(&self) {
        self.send(Message::Tick);
    }
    pub fn create_entity(&self, components: impl hecs::DynamicBundle + Send + 'static) -> EntityID {
        let (tx, rx) = std::sync::mpsc::channel();
        let boxed_components = Box::new(components);
        self.send(Message::Create {
            spawn_fn: Box::new(|world| {
                return world.spawn(*boxed_components);
            }),
            response: tx,
        });
        rx.recv()
            .expect("Could not receive EntityID object after creation.")
    }
    pub fn remove_entity(&self, entity: EntityID) {
        self.send(Message::Delete { entity });
    }
    pub fn add_systems(&self, systems: Vec<impl System + Sync + 'static>) {
        let boxed_systems = systems
            .into_iter()
            .map(|s| Box::new(s) as Box<dyn System>)
            .collect();
        self.send(Message::Systems {
            systems: boxed_systems,
        });
    }
}
