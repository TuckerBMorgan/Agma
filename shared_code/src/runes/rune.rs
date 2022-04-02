// A rune is a transaction over the world that modifies it
pub trait Rune {
    fn execute(mut world: World) -> World;
}


//TODO: add a "moveType" Ex: ClickMove, Push, Forced
pub struct Move {
    entity_id: EntityId,
    desination: Vector3<f32>
}

impl Rune for Move {
    fn execute(mut world: World) -> World {
        //find the entity
        //check that they can be moved by the way the Move wants it to
        //find the place on the map that is cloest to the destination the character can move towards
        //then set the entities direction, and their desired destination on their "MovementComponent"
        //A seperate system down the line will actually move the entity
    }
}