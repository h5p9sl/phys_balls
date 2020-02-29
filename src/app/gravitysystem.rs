use crate::app::components::*;
use specs::prelude::*;

pub struct GravitySystem;

#[derive(SystemData)]
pub struct GravityData<'a> {
    entities: Entities<'a>,
    pos: ReadStorage<'a, Position>,
    radius: ReadStorage<'a, Radius>,
    vel: WriteStorage<'a, Velocity>,
}

impl<'a> System<'a> for GravitySystem {
    type SystemData = GravityData<'a>;

    fn run(&mut self, mut data: GravityData<'a>) {
        // A vector to store all of the calculated forces to apply
        let mut forces: Vec<(Entity, [f32; 2])> = Vec::new();

        // Iterate through all entities and calculate their forces on others
        let iter = (&data.entities, &data.pos, &data.radius, &data.vel).join();
        for (ent1, pos1, radius1, _vel1) in iter.clone() {
            // Create a vector to store our total gravity pull & direction
            let mut force: [f32; 2] = [0.0; 2];

            // Iterate through all *other* objects with mass and
            // calculate the pull towards them.
            // This is then added to our `force` vector.
            for (ent2, pos2, radius2, _vel2) in iter.clone() {
                if ent1.id() == ent2.id() {
                    continue;
                }
                // F = G(m1)(m2)/R^2
                let a = [pos2.0 - pos1.0, pos2.1 - pos1.1];

                let g = 1000.0;
                let r = f32::hypot(a[0], a[1]);
                let m1 = radius1.0;
                let m2 = radius2.0;
                if r > 0.0 {
                    let angle = f32::atan2(a[1], a[0]);
                    let f = g * m1 * m2 / (r * r);
                    force[0] += f32::cos(angle) * f / m1;
                    force[1] += f32::sin(angle) * f / m1;
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
