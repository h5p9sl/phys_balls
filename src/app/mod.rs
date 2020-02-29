use ggez::*;
use rand::random;
use specs::*;

mod components;
use crate::app::components::*;

mod velocitysystem;
use crate::app::velocitysystem::*;

mod gravitysystem;
use crate::app::gravitysystem::*;

pub struct App<'a> {
    world: specs::World,
    dispatcher: specs::Dispatcher<'a, 'a>,
}

impl App<'_> {
    pub fn create_physball(&mut self) {
        self.world
            .create_entity()
            .with(Position(random::<f32>() * 800.0, random::<f32>() * 600.0))
            .with(Velocity(0.0, 0.0))
            .with(Radius(random::<f32>() * 80.0 + 5.0))
            .with(Color(ggez::graphics::Color::new(
                random::<f32>(),
                random::<f32>(),
                random::<f32>(),
                1.0,
            )))
            .build();
    }

    pub fn new() -> Self {
        let mut world = World::new();
        world.register::<DeltaTime>();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Radius>();
        world.register::<Color>();

        let dispatcher = DispatcherBuilder::new()
            .with(GravitySystem, "gravity_sys", &[])
            .with(VelocitySystem, "velocity_sys", &["gravity_sys"])
            .build();

        let mut app = App { world, dispatcher };

        for _i in 0..5 {
            app.create_physball();
        }

        app
    }
}

impl event::EventHandler for App<'_> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let delta = timer::duration_to_f64(timer::delta(ctx));

        // Create an entity storing the delta time
        let dt = self
            .world
            .create_entity()
            .with::<DeltaTime>(DeltaTime(delta))
            .build();

        // Dispatch all of our systems
        self.dispatcher.dispatch(&mut self.world);
        // Delete the delta time entity as it is no longer in use
        self.world.delete_entity(dt).unwrap();
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Iterate through all entites with a Position, Radius, and Color and draw them
        self.world.exec(
            |(pos, radius, color): (
                ReadStorage<Position>,
                ReadStorage<Radius>,
                ReadStorage<Color>,
            )| {
                for (pos, radius, color) in (&pos, &radius, &color).join() {
                    // Create a circle mesh using our components
                    let circle = graphics::Mesh::new_circle(
                        ctx,
                        graphics::DrawMode::fill(),
                        [pos.0, pos.1],
                        radius.0,
                        1.0,
                        color.0,
                    )
                    .unwrap();
                    // Draw the mesh
                    graphics::draw(ctx, &circle, graphics::DrawParam::default()).unwrap();
                }
            },
        );

        graphics::present(ctx)?;
        Ok(())
    }
}
