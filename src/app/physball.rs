use ggez::graphics::*;
use ggez::*;
use rand::random;

pub struct PhysBall {
    pub pos: [f64; 2],
    pub velocity: [f64; 2],
    pub radius: f64,
    color: Color,
    collide: [f64; 2],
}

impl PhysBall {
    pub fn new(pos: [f64; 2], radius: f64) -> Self {
        let velocity = [0.0, 0.0];
        let collide = [0.0, 0.0];
        let color = PhysBall::generate_color();
        PhysBall {
            pos,
            velocity,
            radius,
            collide,
            color,
        }
    }

    pub fn with_velocity(pos: [f64; 2], radius: f64, velocity: [f64; 2]) -> Self {
        let collide = [0.0, 0.0];
        let color = PhysBall::generate_color();
        PhysBall {
            pos,
            velocity,
            radius,
            collide,
            color,
        }
    }

    fn generate_color() -> Color {
        let r: f32 = random();
        let g: f32 = random();
        let b: f32 = random();
        let a: f32 = 1.0;
        Color::new(r, g, b, a)
    }

    pub fn apply_force(&mut self, force: f64, direction: f64) {
        let a = [f64::cos(direction) * force, f64::sin(direction) * force];
        self.velocity[0] += a[0] / self.get_mass();
        self.velocity[1] += a[1] / self.get_mass();
    }

    pub fn get_mass(&self) -> f64 {
        self.radius * 50.0
    }

    pub fn get_angle(&self, other: &PhysBall) -> f64 {
        let xy = [other.pos[0] - self.pos[0], other.pos[1] - self.pos[1]];
        f64::atan2(xy[1], xy[0])
    }

    // Gets the gravitational pull between two objects
    pub fn get_force(&self, other: &PhysBall) -> f64 {
        // F = GMm/R^2
        let r = f64::hypot(other.pos[0] - self.pos[0], other.pos[1] - self.pos[1]);
        let m = self.get_mass() * other.get_mass();
        m / f64::powf(r, 2.0)
    }

    pub fn apply_collide(&mut self, collide: [f64; 2]) {
        if f64::hypot(collide[0], collide[1]) > 0.0 {
            for i in 0..2 {
                self.collide[i] = collide[i] / self.get_mass();
            }
        }
    }

    // If object is in collision, return the delta of the collided surfaces
    pub fn get_collide(&self, other: &PhysBall) -> [f64; 2] {
        let r = f64::hypot(other.pos[0] - self.pos[0], other.pos[1] - self.pos[1]);
        if r > self.radius + other.radius {
            return [0.0; 2];
        }
        let r = [other.pos[0] - self.pos[0], other.pos[1] - self.pos[1]];
        r
    }

    // Called once per frame, updates the object's position
    pub fn update(&mut self, dt: f64) {
        for i in 0..2 {
            self.pos[i] += self.velocity[i] * dt as f64;
            //self.pos[i] += self.collide[i];
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let mesh = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [self.pos[0] as f32, self.pos[1] as f32],
            self.radius as f32,
            1.0,
            self.color,
        )
        .unwrap();
        graphics::draw(ctx, &mesh, DrawParam::default());
    }
}
