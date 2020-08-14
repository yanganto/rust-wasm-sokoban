use crate::components::{Position, Renderable};
use crate::resources::Gameplay;
use gwg::{
    graphics::{self, Color, DrawParam, Image},
    Context,
};
use nalgebra::Point2;
use specs::{
    join::Join, Builder, Component, Read, ReadStorage, RunNow, System, VecStorage, World, WorldExt,
};

const TILE_WIDTH: f32 = 32.0;

// Systems
pub struct RenderingSystem<'a> {
    pub(crate) context: &'a mut Context,
}

// System implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (
        Read<'a, Gameplay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (_game_play, positions, renderables) = data;

        // Clearing the screen (this gives us the backround colour)
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by(|&a, &b| a.0.z.partial_cmp(&b.0.z).expect("expected comparison"));

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            let image = Image::new(self.context, renderable.path.clone()).expect("expected image");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            // draw
            let draw_params = DrawParam::new().dest(Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        // TODO: handle text in wasm
        // Render any text
        // self.draw_text(&game_play.state.to_string(), 525.0, 80.0);
        // self.draw_text(&game_play.moves_count.to_string(), 525.0, 100.0);

        // Finally, present the context, this will actually display everything
        // on the screen.
        graphics::present(self.context).expect("expected to present");
    }
}

// Initialize the level
pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . P . . . . W
    W W . W W W W W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    crate::map::load_map(world, MAP.to_string());
}
