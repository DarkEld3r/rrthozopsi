use screen::*;
use context::Context;
use graphics::Texture;
use piston_window::{self, PistonWindow};

pub struct Battle {
    a: Texture,
}

impl Battle {
    pub fn new(window: &PistonWindow, context: &mut Context) -> Self {
        Battle { a: context.resource_manager.load_texture(window) }
    }
}

impl Screen for Battle {
    fn on_input(&mut self, input: &piston_window::Input, window: &PistonWindow, context: &mut Context) -> InputResults {
        use piston_window::{Input, Button, Key};

        let mut result = Vec::new();

        match input {
            &Input::Press(button) => {
                match button {
                    Button::Keyboard(Key::Escape) => {
                        result.push(InputResult::PushScreen(Box::new(MainMenu::new(&window, context))));
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        result
    }

    fn on_draw(&mut self, _args: &piston_window::RenderArgs, window: &PistonWindow) {
        window.draw_2d(|context, graphics| {
            piston_window::clear([0.0, 0.0, 0.0, 1.0], graphics);
            piston_window::image(&self.a, context.transform, graphics);
        });
    }

    fn on_update(&mut self, _args: &piston_window::UpdateArgs) {}
}
