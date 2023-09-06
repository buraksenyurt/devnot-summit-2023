#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("World build error: {0}")]
    World(&'static str),
}
pub type Result<T> = core::result::Result<T, Error>;

fn main() -> Result<()> {
    let gondor = GameBuilder::new()
        .name("Gondor") // Kapatıp derleyelim
        .background("lord_of_the_rings_map_big.jpg") // Kapatıp derleyelim
        .window(1280., 900.)
        .build()?;
    println!("{gondor:#?}");

    Ok(())
}

#[derive(Default, Clone)]
struct NoName;

#[derive(Default, Clone)]
struct Name(String);

#[derive(Default, Clone)]
struct NoBackground;

#[derive(Default, Clone)]
struct Background(String);

#[derive(Debug, Clone)]
struct Window {
    width: f32,
    height: f32,
}

#[derive(Debug, Clone)]
struct WindowSize {
    width: f32,
    height: f32,
}

#[derive(Debug)]
struct Game {
    name: String,
    background: String,
    window: Option<Window>,
}

#[derive(Default, Clone)]
struct GameBuilder<N, B> {
    name: N,
    background: B,
    window: Option<Window>,
}

impl GameBuilder<NoName, NoBackground> {
    fn new() -> Self {
        GameBuilder::default()
    }
}

impl GameBuilder<Name, Background> {
    fn build(self) -> Result<Game> {
        let window = self.window.unwrap_or_else(|| Window {
            width: 640.,
            height: 480.,
        });

        Ok(Game {
            name: self.name.0,
            background: self.background.0,
            window: Some(window),
        })
    }
}

impl<N, B> GameBuilder<N, B> {
    fn name(mut self, name: impl Into<String>) -> GameBuilder<Name, B> {
        GameBuilder {
            name: Name(name.into()),
            background: self.background,
            window: self.window,
        }
    }

    fn background(mut self, background: impl Into<String>) -> GameBuilder<N, Background> {
        GameBuilder {
            name: self.name,
            background: Background(background.into()),
            window: self.window,
        }
    }

    fn window(mut self, width: f32, height: f32) -> Self {
        self.window = Some(Window { width, height });
        self
    }
}
