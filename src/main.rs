use ggez;
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use rand::{self, thread_rng, Rng};
const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const RACKET_HIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const PLAYER_SPEED: f32 = 600.0;
const BALL_SPEED: f32 = 400.0;
fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}
fn move_racket(pos: &mut na::Point2<f32>, keycode: KeyCode, y_dir: f32, ctx: &mut Context) {
    let screen_h = graphics::drawable_size(ctx).1;
    let dt = ggez::timer::delta(ctx).as_secs_f32();
    if keyboard::is_key_pressed(ctx, keycode) {
        pos.y += y_dir * PLAYER_SPEED * dt;
    }
    clamp(&mut pos.y, RACKET_HIGHT_HALF, screen_h - RACKET_HIGHT_HALF);
}
fn randomize_vec(vec: &mut na::Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}
struct MainState {
    player_1_pos: na::Point2<f32>,
    player_2_pos: na::Point2<f32>,
    ball_pos: na::Point2<f32>,
    ball_vel: na::Vector2<f32>,
}
impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);
        let mut ball_vel = na::Vector2::new(0.0, 0.0);
        randomize_vec(&mut ball_vel, 50.0, 50.0);
        MainState {
            player_1_pos: na::Point2::new(RACKET_WIDTH_HALF, screen_h_half),
            player_2_pos: na::Point2::new(screen_w - RACKET_WIDTH_HALF, screen_h_half),
            ball_pos: na::Point2::new(screen_w_half, screen_h_half),
            ball_vel: ball_vel,
        }
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        move_racket(&mut self.player_1_pos, KeyCode::W, -1.0, ctx);
        move_racket(&mut self.player_1_pos, KeyCode::S, 1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Up, -1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Down, 1.0, ctx);
        self.ball_pos += self.ball_vel * dt;
        if self.ball_pos.x < 0.0 {
            self.ball_pos.x = screen_w * 0.5;
            self.ball_pos.y = screen_h * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
        }
        if self.ball_pos.x > screen_w {
            self.ball_pos.x = screen_w * 0.5;
            self.ball_pos.y = screen_h * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
        }
        if self.ball_pos.y < BALL_SIZE_HALF {
            self.ball_pos.y = BALL_SIZE_HALF;
            self.ball_vel.y = self.ball_vel.y.abs();
        } else if self.ball_pos.y > screen_h - BALL_SIZE_HALF {
            self.ball_pos.y = screen_h - BALL_SIZE_HALF;
            self.ball_vel.y = -self.ball_vel.y.abs();
        }
        let intersects_player_1 = self.ball_pos.x - BALL_SIZE_HALF
            < self.player_1_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_1_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_1_pos.y + RACKET_WIDTH_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_1_pos.y - RACKET_WIDTH_HALF;
        if intersects_player_1 {
            self.ball_vel.x = self.ball_vel.x.abs();
        }
        let intersects_player_2 = self.ball_pos.x - BALL_SIZE_HALF
            < self.player_2_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_2_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_2_pos.y + RACKET_WIDTH_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_2_pos.y - RACKET_WIDTH_HALF;
        if intersects_player_2 {
            self.ball_vel.x = -self.ball_vel.x.abs();
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK); //background 색
        let racket_rect = graphics::Rect::new(
            -RACKET_WIDTH_HALF,
            -RACKET_HIGHT_HALF,
            RACKET_WIDTH,
            RACKET_HEIGHT,
        );
        let racket_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            racket_rect,
            graphics::WHITE,
        )?;
        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::WHITE,
        )?;
        let mut draw_param = graphics::DrawParam::default();
        draw_param.dest = self.player_1_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;
        draw_param.dest = self.player_2_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;
        draw_param.dest = self.ball_pos.into();
        graphics::draw(ctx, &ball_mesh, draw_param)?;
        graphics::present(ctx);
        Ok(())
    }
}
fn main() -> GameResult {
    // 게임 컨텍스트와 이벤트 루프를 생성합니다.
    //컨텍스트는 게임의 실행 환경, 자원 등의  정보를 담은 객체
    //이벤트 루프는 게임의 메인 루프로 게임 상태를 업데이트하고, 화면을 렌더링하는 역할
    let cb = ggez::ContextBuilder::new("Ping-pong", "Name")
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "Ping-Pong"); // 창 제목을 설정합니다.
    let mut state = MainState::new(ctx); // 게임 상태를 초기화합니다.
    event::run(ctx, event_loop, &mut state).unwrap(); // 이벤트 루프를 실행합니다.
    Ok(()) //성공적으로 완료, 반환값 없음
}
