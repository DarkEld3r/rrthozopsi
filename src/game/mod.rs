use screen::{Screen, MainMenu};
use context::{Context, GameState};
use resource_manager::Manager;
use piston_window::{self, PistonWindow, WindowSettings};

type Screens = Vec<Box<Screen>>;

pub struct Game {
    window: PistonWindow,
    context: Context,
    screens: Screens,
}

impl Game {
    pub fn new() -> Self {
        let window: PistonWindow = WindowSettings::new("rrthozopsi", [600, 600])
                                       .build()
                                       .unwrap();
        let mut context = Context {
            resource_manager: Manager::new(),
            game_state: GameState::NotStarted,
        };
        let screens: Screens = vec![Box::new(MainMenu::new(&window, &mut context))];
        Game {
            window: window,
            screens: screens,
            context: context,
        }
    }

    pub fn run(&mut self) {
        use piston_window::Event;

        let screens = &mut self.screens;

        while let Some(event) = self.window.next() {
            match event {
                Some(Event::Update(ref args)) => {
                    last(screens).on_update(&args);
                }
                Some(Event::Render(ref args)) => {
                    last(screens).on_draw(&args, &self.window);
                }
                Some(Event::Input(ref input)) => {
                    if handle_input(screens, &input, &self.window, &mut self.context) {
                        // Exit game.
                        return;
                    }
                }
                _ => {}
            }
        }
    }
}

fn last<'a>(vec: &'a mut Screens) -> &'a mut Box<Screen> {
    vec.last_mut().unwrap()
}

fn handle_input(screens: &mut Screens,
                input: &piston_window::Input,
                window: &PistonWindow,
                context: &mut Context)
                -> bool {
    use screen::InputResult;

    for action in last(screens).on_input(&input, &window, context) {
        match action {
            InputResult::PushScreen(new_screen) => {
                screens.push(new_screen);
            }
            InputResult::PopScreen => {
                screens.pop();
            }
            InputResult::Exit => return true,
        }
    }

    false
}
