use gwg::{conf::Conf, event, Context, GameResult};

#[cfg(not(target_arch = "wasm32"))]
fn conf() -> Conf {
    Conf {
        // physical_root_dir: Some("assets".into()),
        ..Default::default()
    }
}

#[cfg(target_arch = "wasm32")]
fn conf() -> Conf {
    Conf {
        // cache: gwg::conf::Cache::Tar(include_bytes!("../assets.tar").to_vec()),
        loading: gwg::conf::Loading::Embedded,
        ..Default::default()
    }
}
struct Game {}
impl Game {
    fn new(context: &mut Context) -> Self {
        Self {}
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        Ok(())
    }

    fn resize_event(&mut self, context: &mut Context, w: f32, h: f32) {}

    fn mouse_button_up_event(
        &mut self,
        context: &mut Context,
        _: gwg::event::MouseButton,
        x: f32,
        y: f32,
    ) {
    }

    fn mouse_motion_event(&mut self, context: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {}

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
