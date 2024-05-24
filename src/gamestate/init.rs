pub struct Init {}

impl crate::gamestate::GameState for Init {
    fn new() -> Self {
        Self {}
    }
    fn init(&mut self, gba: &mut agb::Gba) {}
    fn draw(&mut self, gba: &mut agb::Gba) {
        agb::println!("HELLO WORLD");
    }
    fn update(&mut self, gba: &mut agb::Gba) -> bool {
        let input = agb::input::ButtonController::new();
        if input.is_just_pressed(agb::input::Button::START) {
            true
        } else {
            false
        }
    }
    fn exit(&mut self, gba: &mut agb::Gba) {}
}
