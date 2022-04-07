// A rune is a transaction over the world that modifies it
pub trait Rune {
    fn execute(mut world: World) -> World;
}

pub struct RuneSystem {

}

impl RuneSystem {
    //
}

pub struct FrameRune {}

impl Rune for FrameRune {
    fn execute(mut world: World) -> World {
        world
    }
}

pub struct MoveRune {
    entity: DefaultKey,
    destination_x: f32,
    destination_y: f32
}