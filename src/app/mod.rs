use ggez::graphics::*;
use ggez::*;
use rand::random;

mod physball;
use crate::app::physball::PhysBall;

pub struct App {
    objects: Vec<PhysBall>,
}

impl App {
    pub fn new() -> Self {
        let mut objects = Vec::new();

        for i in 0..5 {
            objects.push(PhysBall::new(
                [random::<f64>() * 800.0, random::<f64>() * 600.0],
                random::<f64>() * 50.0 + 5.0,
            ));
        }
        App { objects }
    }
}

impl event::EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let delta = timer::duration_to_f64(timer::delta(ctx));

        // Apply gravity to every object
        for i in 0..self.objects.len() {
            for j in 0..self.objects.len() {
                if i == j {
                    continue;
                }

                let object = self.objects.get(i).unwrap();
                let other = self.objects.get(j).unwrap();
                let force = object.get_force(&other);
                let angle = object.get_angle(&other);

                let object = self.objects.get_mut(i).unwrap();
                object.apply_force(force, angle);
            }
        }

        // Update every object's position
        for i in 0..self.objects.len() {
            let object = self.objects.get_mut(i).unwrap();
            object.update(delta);
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        for object in &mut self.objects {
            object.draw(ctx);
        }

        graphics::present(ctx)?;
        Ok(())
    }
}
