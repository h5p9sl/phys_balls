pub mod app;

use crate::app::App;
use ggez::*;

fn main() -> GameResult<()> {
    let window_setup = conf::WindowSetup::default().title("PhysBalls");
    let mut conf = conf::Conf::new();
    conf.window_setup = window_setup;

    let (ref mut ctx, ref mut event_loop) =
        ContextBuilder::new("PhysBalls v2", "h5p9sl@github.com")
            .conf(conf)
            .build()
            .unwrap();

    let mut app = App::new();

    event::run(ctx, event_loop, &mut app)
}
