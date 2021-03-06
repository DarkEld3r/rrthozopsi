use screen::*;
use context::Context;
use ui;
use piston_window::{self, PistonWindow};
use conrod::{self, Canvas, Button, Widget, Positionable, Sizeable, Labelable};
use conrod::color::Colorable;

pub struct Options {
    ui: ui::Ui,
    back: bool,
}

impl Options {
    pub fn new(window: &PistonWindow, context: &mut Context) -> Self {
        Options {
            ui: ui::create_ui(&window, &context.resource_manager.get_font()),
            back: false,
        }
    }
}

impl Screen for Options {
    fn on_input(&mut self, _: &piston_window::Input, window: &PistonWindow, _: &mut Context) -> InputResults {
        self.ui.handle_event(window);
        let mut result = Vec::new();

        if self.back {
            result.push(InputResult::PopScreen);
        }

        result
    }

    fn on_draw(&mut self, _: &piston_window::RenderArgs, window: &PistonWindow) {
        self.ui.handle_event(window);
        window.draw_2d(|context, graphics| self.ui.draw_if_changed(context, graphics));
    }

    fn on_update(&mut self, _: &piston_window::UpdateArgs) {
        let button_color = conrod::color::rgb(0.4, 0.75, 0.6);

        let back = &mut self.back;

        self.ui.set_widgets(|mut ui| {
            Canvas::new()
                .pad(30.)
                .color(conrod::color::rgb(0.2, 0.35, 0.45))
                .scroll_kids()
                .set(CANVAS, &mut ui);

            Button::new()
                .w_h(200.0, 50.0)
                .mid_left_of(CANVAS)
                .color(button_color)
                .label("Back")
                .react(|| *back = true)
                .set(BACK_BUTTON, &mut ui);
        });
    }
}

widget_ids! {
    CANVAS,
    BACK_BUTTON,
}
