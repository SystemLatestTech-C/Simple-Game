use quicksilver::{
    Result,
    geom::{Rectangle, Vector},
    graphics::{Color, Image},
    input::{Key, Keyboard},
    lifecycle::{Asset, Settings, State, Window, run},
    Future, combinators::result,
};
use std::collections::HashSet;
use rand::prelude::*;

const WINDOW_SIZE: (i32, i32) = (640, 480);
const SHIP_IMAGE_PATH: &str = "resources/ship.png";
const BULLET_IMAGE_PATH: &str = "resources/bullet.png";
const INVADER_IMAGE_PATH: &str = "resources/invader.png";
const SHIP_SPEED: f32 = 5.0;
const INVADER_SPEED: f32 = 2.0;
const BULLET_SPEED: f32 = 8.0;

struct Game {
    ship: Asset<Image>,
    bullet: Asset<Image>,
    invader: Asset<Image>,
    bullet_rect: Option<Rectangle>,
    invader_rects: Vec<Rectangle>,
    invader_direction: i32,
    invader_speed: f32,
    invader_move_counter: i32,
    ship_rect: Rectangle,
    bullet_speed: f32,
    bullets: HashSet<Rectangle>,
    score: u32,
}

impl State for Game {
    fn new() -> Result<Game> {
        let ship = Asset::new(Image::load(&SHIPE_IMAGE_PATH));
        let bullet = Asset::new(Image::load(&BULLET_IMAGE_PATH));
        let invader = Asset::new(Image::load(&INVADER_IMAGE_PATH));
        let ship_rect = Rectangle::new(Vector::new((WINDOW_SIZE.0 / 2) as f32, (WINDOW_SIZE.1 - 50) as f32), (32.0, 32.0));
        let game = Game {
            ship,
            bullet,
            invader,
            bullet_rect: None,
            invader_rects: vec![],
            invader_direction: 1,
            invader_speed: INVADER_SPEED,
            invader_move_counter: 0,
            ship_rect,
            bullet_speed: BULLET_SPEED,
            bullets: HashSet::new(),
            score: 0,
        };
        Ok(game)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        window.draw(&self.ship, self.ship_rect);
        for bullet in &self.bullets {
            window.draw(&self.bullet, *bullet);
        }
        for invader in &self.invader_rects {
            window.draw(&self.invader, *invader);
        }
        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        // handle keyboard input
        let keyboard = window.keyboard();
        let mut x_move = 0.0;
        if keyboard.is_down(Key::Left) {
            x_move -= SHIP_SPEED;
        }
        if keyboard.is_down(Key::Right) {
            x_move += SHIP_SPEED;
        }
        self.ship_rect.translate(x_move, 0.0);
    
        // move bullet
        if let Some(rect) = &mut self.bullet_rect {
            rect.translate(0.0, -self.bullet_speed);
            if rect.top() < 0.0 {
                self.bullet_rect = None;
            }
        }
    
        // move invader
        self.invader_move_counter += 1;
        if self.invader_move_counter >= 60 {
            self.invader_move_counter = 0;
            for invader_rect in &mut self.invader_rects {
                invader_rect.translate(self.invader_direction as f32 * self.invader_speed, 0.0);
            }
            let invader_rect = self.invader_rects.first().unwrap();
            if invader_rect.left() < 0.0 || invader_rect.right() > WINDOW_SIZE.0 as f32 {
                self.invader_direction *= -1;
                for invader_rect in &mut self.invader_rects {
                    invader_rect.translate(0.0, 32.0);
                }
            }
        }
    
        // check bullet collision
        let mut to_remove = HashSet::new();
        for invader_rect in &self.invader_rects {
            if let Some(bullet_rect) = &self.bullet_rect {
                if invader_rect.overlaps(bullet_rect) {
                    to_remove.insert(invader_rect.clone());
                    self.bullet_rect = None;
                    self.score += 10;
                }
            }
        }
        for rect in to_remove {
            self.invader_rects.retain(|r| *r != rect);
        }
    
        // spawn bullet
        if self.bullet_rect.is_none() && keyboard.is_down(Key::Space) {
            let rect = Rectangle::new(self.ship_rect.pos + Vector::new(12.0, -32.0), (8.0, 16.0));
            self.bullet_rect = Some(rect);
        }
    
        // spawn invader
        if self.invader_rects.is_empty() {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0, 10) as f32 * 32.0 + 64.0;
            let y = rng.gen_range(0, 3) as f32 * 32.0 + 32.0;
            let rect = Rectangle::new(Vector::new(x, y), (32.0, 32.0));
            self.invader_rects.push(rect);
        }
    
        Ok(())
    }
}
fn main() {
    let settings = Settings {
        size: Vector::new(WINDOW_SIZE.0 as f32, WINDOW_SIZE.1 as f32),
        title: String::from("Space Invader"),
        ..Settings::default()
    };
    run::<Game>("Space Invader", settings);
}    
