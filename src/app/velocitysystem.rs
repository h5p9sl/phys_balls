use crate::app::components::*;
use specs::prelude::*;

pub struct VelocitySystem;

#[derive(SystemData)]
pub struct VelocityData<'a> {
    delta: ReadStorage<'a, DeltaTime>,
    pos: WriteStorage<'a, Position>,
    vel: ReadStorage<'a, Velocity>,
}

impl<'a> System<'a> for VelocitySystem {
    type SystemData = VelocityData<'a>;

    fn run(&mut self, mut data: VelocityData<'a>) {
        let delta: f32 = data.delta.join().last().unwrap().0 as f32;
        for (mut pos, vel) in (&mut data.pos, &data.vel).join() {
            pos.0 += vel.0 * delta;
            pos.1 += vel.1 * delta;
        }
    }
}
