use std::vec;

use glium::{Display, Frame, Program, glutin::event::VirtualKeyCode};
use rand::{rngs::ThreadRng, Rng};

use crate::{
    enemy::{Enemy, Species, SPAWN_DELAY},
    input_mgr::InputManager,
    platform::{Platform, Size, Type},
    player::Player,
    shape::{BOTTOM, LEFT, RIGHT, SCREEN_WIDTH, TOP},
    texture::{Rect, Texture, Transform, Score},
};

fn overlap(a: Rect, b: Rect) -> bool {
    if a.x + a.w / 2.0 <= b.x - b.w / 2.0 {
        return false;
    }

    if a.x - a.w / 2.0 >= b.x + b.w / 2.0 {
        return false;
    }

    true
}

pub struct Game {
    player: Player,
    platforms: Vec<Platform>,
    enemies: Vec<Enemy>,
    controls: Vec<Texture>,
    elapsed_time: f32,
    spawn_time: f32,
    rand: ThreadRng,
    score: Score,
}

impl Game {
    pub fn new(display: &Display) -> Self {
        let p = Player::new(display);

        let mut platforms: Vec<Platform> = vec![];

        let mut starting_platform = Platform::new(display, Size::XLarge);
        starting_platform.set_position(LEFT + 100.0, -50.0);
        platforms.push(starting_platform);

        for i in 0..3 {
            platforms.push(Platform::new(display, Size::from_u32(i % 3)));
        }

        platforms[3].set_position(510.0, -100.0);
        platforms[2].set_position(800.0, -150.0);
        platforms[1].set_position(1060.0, 50.0);

        let mut controls: Vec<Texture> = vec![];

        controls.push(Texture::new("./res/controls1.png", display));
        controls[0].scale(0.8);
        controls[0].set_position(-210.0, 160.0);

        controls.push(Texture::new("./res/controls2.png", display));
        controls[1].scale(0.8);
        controls[1].set_position(510.0, 160.0);

        let mut enemies: Vec<Enemy> = vec![Enemy::new(display, Species::Flying)];
        enemies[0].set_position(RIGHT, 0.0);

        Game {
            player: p,
            platforms: platforms,
            enemies: enemies,
            controls: controls,
            elapsed_time: 0.0,
            spawn_time: 0.0,
            rand: rand::thread_rng(),
            score: Score::new(display)
        }
    }

    pub fn update(&mut self, input: &mut InputManager, display: &Display, dt: f32) {
        self.player.update(input, dt);

        if input.key_went_up(VirtualKeyCode::T){
            self.score.increment(display);
        }

        for i in 0..self.platforms.len() {
            self.platforms[i].update(display, dt);
        }

        for i in 0..self.enemies.len() {
            self.enemies[i].update(dt);
            self.enemies[i].translate(-120.0 * dt, 0.0);
            if self.enemies[i].x <= LEFT - self.enemies[i].width {
                let x = self.rand.gen_range(RIGHT..SCREEN_WIDTH);
                let y = self.rand.gen_range(BOTTOM+40.0..TOP-40.0);
                self.enemies[i].set_position(x, y);
            }
        }

        for i in 0..self.platforms.len() {
            if self.platforms[i].x + self.platforms[i].width / 2.0 < (-SCREEN_WIDTH) {
                let mut x: f32;
                let mut y: f32;
                let w = self.platforms[i].width;
                let h = self.platforms[i].height;
                loop {
                    let mut intersects = false;
                    y = self.rand.gen_range(BOTTOM + 100.0..TOP - 200.0);
                    x = self.rand.gen_range(RIGHT + 100.0..SCREEN_WIDTH + RIGHT);
                    for j in 0..self.platforms.len() {
                        let x0 = self.platforms[j].x;
                        let y0 = self.platforms[j].y;
                        let w0 = self.platforms[j].width;
                        let h0 = self.platforms[j].height;
                        if overlap(
                            Rect {
                                x: x,
                                y: y,
                                w: w,
                                h: h,
                            },
                            Rect {
                                x: x0,
                                y: y0,
                                w: w0,
                                h: h0,
                            },
                        ) && i != j
                        {
                            intersects = true;
                            break;
                        }
                    }
                    if !intersects {
                        break;
                    }
                }
                let p = self.rand.gen_range(0..10);
                if p < 5 {
                    self.platforms[i].set_type(Type::Fish);
                } else if p >= 5 && p < 8 {
                    self.platforms[i].set_type(Type::Enemy);
                } else {
                    self.platforms[i].set_type(Type::Plain);
                }
                self.platforms[i].set_position(x, y);
            }
        }

        for i in 0..self.platforms.len() {
            if self.player.x + self.player.width / 2.
                >= self.platforms[i].x - self.platforms[i].width / 2.
                && self.player.x - self.player.width / 2.
                    <= self.platforms[i].x + self.platforms[i].width / 2.
                && self.player.y - self.player.height / 2. + self.player.velocity[1]
                    <= self.platforms[i].y + self.platforms[i].height / 2.
                && self.player.y - self.player.height / 2.
                    >= self.platforms[i].y + self.platforms[i].height / 2.0
            {
                self.player.velocity[1] = 0.0;
            }

            if self.player.x + self.player.width / 2.
                >= self.platforms[i].x - self.platforms[i].width / 2.
                && self.player.x - self.player.width / 2.
                    <= self.platforms[i].x + self.platforms[i].width / 2.
            {
                self.player.on_platform = true;
                break;
            } else {
                self.player.on_platform = false;
            }
        }

        if self.elapsed_time > 999. {
            self.elapsed_time = 1.0;
        }
        self.elapsed_time += dt;
        self.spawn_time += dt;

        if self.spawn_time >= SPAWN_DELAY {
            self.enemies.push(Enemy::new(display, Species::Flying));
            let x = self.rand.gen_range(RIGHT..SCREEN_WIDTH);
            let y = self.rand.gen_range(BOTTOM..TOP);
            let i = self.enemies.len() - 1;
            self.enemies[i].set_position(x, y);
            self.spawn_time = 0.0;
        }

        for i in 0..self.controls.len() {
            let t = self.elapsed_time * 1.5;
            let y = t.sin() * 0.04;

            self.controls[i].translate(0.0, y);
        }

        if self.player.right {
            for i in 0..self.platforms.len() {
                self.platforms[i].translate(-80.0 * dt, 0.0);
            }
            for i in 0..self.controls.len() {
                self.controls[i].translate(-80.0 * dt, 0.0);
            }
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        for i in 0..self.controls.len() {
            self.controls[i].draw(target, program);
        }
        for i in (0..=self.platforms.len() - 1).rev() {
            self.platforms[i].draw(target, program);
        }

        self.player.draw(target, program);

        for i in 0..self.enemies.len() {
            self.enemies[i].draw(target, program);
        }

        self.score.draw(target, program);
    }
}
