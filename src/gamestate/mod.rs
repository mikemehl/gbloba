pub mod init;
pub mod startup;

pub trait GameState {
    fn new() -> Self;
    fn init(&mut self, gba: &mut agb::Gba);
    fn draw(&mut self, gba: &mut agb::Gba);
    fn update(&mut self, gba: &mut agb::Gba) -> bool;
    fn exit(&mut self, gba: &mut agb::Gba);
}

pub enum State {
    Startup(startup::Startup),
    Init(init::Init),
}

impl State {
    pub fn new() -> Self {
        Self::Startup(startup::Startup::new())
    }

    pub fn cycle(&mut self, gba: &mut agb::Gba) -> Option<Self> {
        self.init(gba);
        loop {
            self.draw(gba);
            if !self.update(gba) {
                break;
            }
        }
        self.exit(gba);

        match self {
            Self::Startup(_) => Some(Self::Init(init::Init::new())),
            _ => None,
        }
    }
}

impl GameState for State {
    fn new() -> Self {
        State::Startup(startup::Startup::new())
    }

    fn init(&mut self, gba: &mut agb::Gba) {
        match self {
            Self::Startup(s) => s.init(gba),
            Self::Init(s) => s.init(gba),
        }
    }

    fn draw(&mut self, gba: &mut agb::Gba) {
        match self {
            Self::Startup(s) => s.draw(gba),
            Self::Init(s) => s.draw(gba),
        }
    }

    fn update(&mut self, gba: &mut agb::Gba) -> bool {
        match self {
            Self::Startup(s) => s.update(gba),
            Self::Init(s) => s.update(gba),
        }
    }

    fn exit(&mut self, gba: &mut agb::Gba) {
        match self {
            Self::Startup(s) => s.exit(gba),
            Self::Init(s) => s.exit(gba),
        }
    }
}
