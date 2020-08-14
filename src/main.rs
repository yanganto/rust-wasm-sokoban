use gwg::{conf::Conf, event, Context, GameResult};
use specs::{RunNow, World, WorldExt};

mod render_sys;
use render_sys::RenderingSystem;

#[cfg(not(target_arch = "wasm32"))]
fn conf() -> Conf {
    Conf {
        // This path supposed correct, but segment fail here
        physical_root_dir: Some("./".into()),
        ..Default::default()
    }
}

#[cfg(target_arch = "wasm32")]
fn conf() -> Conf {
    Conf {
        cache: gwg::conf::Cache::Tar(include_bytes!("../resources.tar").to_vec()),
        loading: gwg::conf::Loading::Embedded,
        ..Default::default()
    }
}

struct Game {
    world: World,
}
impl Game {
    fn new(_context: &mut Context) -> Self {
        let mut world = World::new();
        render_sys::register_components(&mut world);
        render_sys::initialize_level(&mut world);
        Self { world }
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        {
            let mut rs = RenderingSystem { context };
            rs.run_now(&self.world);
        }
        Ok(())
    }

    fn resize_event(&mut self, _context: &mut Context, _w: f32, _h: f32) {}

    fn mouse_button_up_event(
        &mut self,
        _context: &mut Context,
        _: gwg::event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn mouse_motion_event(&mut self, _context: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
    }

    fn key_down_event(&mut self, _: &mut Context, _: event::KeyCode, _: event::KeyMods, _: bool) {}
}

fn main() -> gwg::GameResult {
    env_logger::init();
    gwg::start(conf(), |mut context| {
        log::info!("Init good web game");
        let game = Game::new(&mut context);
        Box::new(game)
    })
}
