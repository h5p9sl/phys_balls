use crate::app::components::*;
use specs::prelude::*;

pub struct GravitySystem;

#[derive(SystemData)]
pub struct GravityData<'a> {
    delta: ReadStorage<'a, DeltaTime>,
    entities: Entities<'a>,
    pos: ReadStorage<'a, Position>,
    radius: ReadStorage<'a, Radius>,
    vel: WriteStorage<'a, Velocity>,
}

impl<'a> System<'a> for GravitySystem {
    type SystemData = GravityData<'a>;

    fn run(&mut self, mut data: GravityData<'a>) {
        // Get delta-time entity
        let delta: f32 = data.delta.join().last().unwrap().0 as f32;

        // A vector to store all of the calculated forces to apply
        let mut forces: Vec<(Entity, [f32; 2])> = Vec::new();

        // Iterate through all entities and calculate their forces on others
        let iter = (&data.entities, &data.pos, &data.radius, &data.vel).join();
        for (ent1, pos1, radius1, vel1) in iter.clone() {
            // Create a vector to store our total gravity pull & direction
            let mut force: [f32; 2] = [0.0; 2];

            // Iterate through all *other* objects with mass and
            // calculate the pull towards them.
            // This is then added to our `force` vector.
            for (ent2, pos2, radius2, vel2) in iter.clone() {
                if ent1.id() == ent2.id() {
                    continue;
                }
                // F = G(m1)(m2)/R^2
                let g = 5.0;
                let a = [pos2.0 - pos1.0, pos2.1 - pos1.1];
                let r = f32::hypot(a[0], a[1]);
                if r > 0.0 {
                    let angle = f32::atan2(a[1], a[0]);
                    let f = g * radius1.0 * radius2.0 / (r * r);
                    force[0] += f32::cos(angle) * f;
                    force[1] += f32::sin(angle) * f;
                }
            }


            // Push our `force` vector to the array
            forces.push((ent1, force));
        }

        // Iterate through entities and apply the gravitational forces
        'foo: for (ent, mut vel) in (&data.entities, &mut data.vel).join() {
            for i in 0..forces.len() {
                if let Some(force) = forces.get(i) {
                    if force.0.id() == ent.id() {
                        let vector = force.1;
                        vel.0 += vector[0];
                        vel.1 += vector[1];
                        forces.remove(i);
                        continue 'foo;
                    }
                }
            }
        }
    }
}

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
