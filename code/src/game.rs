use glium::{Display, Frame, Program};

use crate::input_mgr::InputManager;
use crate::texture::Texture;

pub struct Player {
    pub texture: Texture,
    pub position: [f32; 2],
}

impl Player {
    pub fn new(display: &Display) -> Self {
        let texture = Texture::new("./res/star.png", display);
        Player {
            texture: texture,
            position: [0.0, 0.0],
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program){
        self.texture.draw(target, program);
    }
}

pub struct Game {
    player: Player,
    pub input: InputManager,
}

impl Game {
    pub fn new(display: &Display) -> Self {
        let p = Player::new(display);
        let input = InputManager::new();

        Game {
            player: p,
            input: input,
        }
    }

    pub fn update(&mut self) {
        if self
            .input
            .key_went_up(glium::glutin::event::VirtualKeyCode::Up)
        {
            println!("up is pressed");
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.player.draw(target, program);
    }
}
