use tetra::{
    graphics::{self, Color, Texture},
    math::Vec2,
    Context, ContextBuilder, State
};

struct GameState {
    texture1: Texture,
    texture2: Texture,
}

fn main() -> tetra::Result {
    ContextBuilder::new("Test", 640, 480)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(GameState {
            texture1: Texture::new(ctx, "assets/texture1.png")?,
            texture2: Texture::new(ctx, "assets/texture2.png")?,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.769, 0.812, 0.631));

        self.texture1.draw(ctx, Vec2::new(32.0, 32.0));
        self.texture2.draw(ctx, Vec2::new(64.0, 64.0));

        Ok(())
    }
}
