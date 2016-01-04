extern crate find_folder;

use ::screen;
use ::screen::Screen;
use ::events;
use ::object;

use piston_window;
use piston_window::Transformed;

use gfx_device_gl;

pub struct Game {
    window: piston_window::PistonWindow,
    screens: Vec<Box<screen::Screen>>,
    events: events::Events,
}

impl Game {
    pub fn new() -> Game {
        let window: piston_window::PistonWindow = piston_window::WindowSettings::new("rrthozopsi", [600, 600]).exit_on_esc(true).build().unwrap();
        let screens: Vec<Box<screen::Screen>> = vec![Box::new(screen::MainMenuScreen::new())];
        Game { 
            window: window,
            screens: screens, 
            events: events::Events::new(), 
        }
    }

    pub fn run(&mut self) {
        use piston_window::Event;

        let mut screen = self.screens.last_mut().unwrap();

        // TODO: Remove clone?
        for window in &mut self.window {
            match window.event {
                Some(Event::Update(args)) => { screen.on_update(&args); }
                Some(Event::Render(args)) => { screen.on_draw(&args, &window); }
                Some(Event::Input(input)) => { screen.on_input(&input); }
                _ => {}
            }
        }
    }

    pub fn on_update(&mut self, args: &piston_window::UpdateArgs) {
        if self.events.is_up()   { self.player.mov(0.0, -150.0 * args.dt); }
        if self.events.is_down() { self.player.mov(0.0, 150.0 * args.dt); }
        if self.events.is_left() { self.player.mov(-150.0 * args.dt, 0.0); }
        if self.events.is_right() { self.player.mov(150.0 * args.dt, 0.0); }
    }

    pub fn on_draw(&mut self, args: &piston_window::RenderArgs, window: &piston_window::PistonWindow) {
        window.draw_2d(|context, graphics| {
            piston_window::clear([0.0, 0.0, 0.0, 1.0], graphics);

            let center = context.transform.trans((args.width / 2) as f64, (args.height / 2) as f64);
            self.player.render(graphics, &center);
        });
    }

    pub fn on_input(&mut self, input: &piston_window::Input) {
        self.events.process_input(&input);
    }
}
