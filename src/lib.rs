use std::collections::HashMap;

pub type ID = usize;
pub type ComponentType = u8;
pub type Signature = u128;

struct IDTracker {
    unused_ids: Vec<ID>,
    entity_count: usize,
    signatures: HashMap<ID, Signature>,
}
impl IDTracker {
    fn new() -> IDTracker {
        let mut ids = Vec::new();
        for i in 0..ID::MAX {
            ids.push(i);
        }
        IDTracker {
            unused_ids: ids,
            entity_count: 0,
            signatures: HashMap::new(),
        }
    }
    fn create_entity(&mut self) -> ID {
        self.entity_count += 1;
        self.unused_ids.pop().unwrap()
    }
    fn destroy_entity(&mut self, entity: ID) {
        self.unused_ids.push(entity);
        self.entity_count -= 1;
    }
    fn set_signature(&mut self, entity: ID, signature: Signature) {
        self.signatures.insert(entity, signature);
    }
    fn get_signature(&self, entity: ID) -> Signature {
        *self.signatures.get(&entity).unwrap()
    }
}
