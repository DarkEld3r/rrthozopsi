use ::screen;
use ::screen::Screen;

use piston_window;

pub struct Game {
    window: piston_window::PistonWindow,
    screens: Vec<Box<screen::Screen>>,
}

impl Game {
    pub fn new() -> Game {
        let window: piston_window::PistonWindow = piston_window::WindowSettings::new("rrthozopsi", [600, 600]).exit_on_esc(true).build().unwrap();
        let screens: Vec<Box<screen::Screen>> = vec![Box::new(screen::MainMenuScreen::new())];
        Game { 
            window: window,
            screens: screens, 
        }
    }

    pub fn run(&mut self) {
        use piston_window::Event;

	// ???
	// TODO: Use drop to unborrow?
        //let mut screen = self.screens.last_mut().unwrap();

        // TODO: Remove clone?
        for window in &mut self.window {
            match window.event {
                Some(Event::Update(args)) => { self.screens.last_mut().unwrap().on_update(&args); }
                Some(Event::Render(args)) => { self.screens.last_mut().unwrap().on_draw(&args, &window); }
                Some(Event::Input(ref input)) => {
            	    for action in self.screens.last_mut().unwrap().on_input(&input, &window) {
            		match action {
            		    screen::InputResult::PushScreen(_) => {}
            		    screen::InputResult::PopScreen => {
            			self.screens.pop();
            		    }
            		}
            	    }
            	    //let result = screen.on_input(&input, &window);
            	}
                _ => {}
            }
        }
    }
}
