pub struct Startup {}

impl crate::gamestate::GameState for Startup {
    fn new() -> Self {
        Self {}
    }
    fn init(&mut self, gba: &mut agb::Gba) {}
    fn draw(&mut self, gba: &mut agb::Gba) {}
    fn update(&mut self, gba: &mut agb::Gba) -> bool {
        false
    }
    fn exit(&mut self, gba: &mut agb::Gba) {}
}
