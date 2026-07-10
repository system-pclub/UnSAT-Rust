use bevy_hecs::*;

fn main() {
    let mut world = World::new();

    // Spawn an entity with a component type that is NOT `i32`.
    let e = world.spawn((0u8,));

    // Safe API: ask for an `i32` component that the entity does not have.
    // This returns an error, so no unsafe pointer arithmetic is actually reached.
    assert!(world.get::<i32>(e).is_err());

    println!("No exploit from the provided safe API.");
}
