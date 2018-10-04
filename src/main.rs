extern crate gl;
extern crate glfw;
extern crate cgmath;

mod lang;
mod logic;
mod engine;

use lang::{ObjectPar, ObjectMethods};
use logic::{Level};
use engine::{Game};
use engine::camera::Camera;
use engine::window::Window;
use engine::input::{InputControl, MouseEvent, KeyEvent};

struct Player {
    camera: Camera,
}

impl InputControl for Player {
    fn on_mouse(&mut self, mouse: MouseEvent, delta_time: f32) {
    }

    fn on_keyboard(&mut self, key: KeyEvent, delta_time: f32) {
    }
}

struct Interplanetarers {
    window: Window,
    player: ObjectPar<Player>,
    level: Level,
}

impl Interplanetarers {
    fn new() -> Self {
        let mut window = Window::new("Interplanetarers");
        let player = ObjectPar::construct(
            Player {
                camera: Camera::default(),
            }
        );
        window.controls.push(player.clone());

        let level = Level;

        Interplanetarers {
            window,
            player,
            level
        }
    }
}

impl Game for Interplanetarers {
    fn main_loop(&mut self) {
        self.window.events_loop();
    }
}

fn main() {
    let mut game = Interplanetarers::new();
    game.main_loop();
}
