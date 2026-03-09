use qmi::{tecs::TECS, world::World};

#[test]
fn test() {
    let tecs = TECS::new();
    fn test_system(world: &mut World) {
        let mut entities = world.query::<&u32>();
        for e in entities.iter() {
            println!("{:?}", e);
        }
    }
    tecs.tick();
    tecs.create_entity((43u32, "hello!"));
    tecs.add_systems(vec![test_system]);
    tecs.tick();
}
