use specs::prelude::*;

#[derive(Debug)]
pub struct DeltaTime(pub f64);
#[derive(Debug)]
pub struct Position(pub f32, pub f32);
#[derive(Debug)]
pub struct Velocity(pub f32, pub f32);
#[derive(Debug)]
pub struct Radius(pub f32);
#[derive(Debug)]
pub struct Color(pub ggez::graphics::Color);

impl Component for DeltaTime {
    type Storage = VecStorage<Self>;
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Component for Radius {
    type Storage = VecStorage<Self>;
}

impl Component for Color {
    type Storage = VecStorage<Self>;
}
