use ggez; // rust의 게임 라이브러리
use ggez::event; // 이벤트 모듈
use ggez::graphics; // 그래픽 모듈
use ggez::input::keyboard::{self, KeyCode}; // 키보드 모듈
use ggez::nalgebra as na; // 벡터, 행렬 등의 수학 연산 모듈
use ggez::{Context, GameResult}; // 게임 모듈(실행환경 저장 및 결과 반환)
use rand::{self, thread_rng, Rng}; // 랜덤 모듈

const RACKET_HEIGHT: f32 = 100.0; // 라켓의 높이
const RACKET_WIDTH: f32 = 20.0; // 라켓의 너비
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5; // 라켓의 너비의 절반
const RACKET_HIGHT_HALF: f32 = RACKET_HEIGHT * 0.5; // 라켓의 높이의 절반
const BALL_SIZE: f32 = 30.0; // 공의 크기
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5; // 공의 크기의 절반
const PLAYER_SPEED: f32 = 600.0; // 플레이어의 속도
const BALL_SPEED: f32 = 300.0; // 공의 속도

fn move_racket(pos: &mut na::Point2<f32>, keycode: KeyCode, y_dir: f32, ctx: &mut Context) {
    let screen_h = graphics::drawable_size(ctx).1; // 화면의 높이를 가져옵니다.
    let dt = ggez::timer::delta(ctx).as_secs_f32(); // 1프레임당 흐른 시간(델타 타임)을 가져옵니다.
    if keyboard::is_key_pressed(ctx, keycode) {
        // 해당 키가 눌렸는지 확인합니다.
        pos.y += y_dir * PLAYER_SPEED * dt; // 라켓의 y 좌표를 업데이트합니다.
    }
    pos.y = pos.y.clamp(RACKET_HIGHT_HALF, screen_h - RACKET_HIGHT_HALF); // 라켓이 화면 밖으로 나가지 않도록 위치를 조정합니다.
}
fn randomize_vec(vec: &mut na::Vector2<f32>, x: f32, y: f32) { // 공의 방향을 랜덤한 방향으로 설정합니다.
    let mut rng = thread_rng(); // 랜덤 모듈의 인스턴스를 생성합니다.
    vec.x = match rng.gen_bool(0.5) { // 0.5의 확률로 true 또는 false를 반환합니다.
        true => x, // true일 경우 x를 반환합니다.
        false => -x, // false일 경우 -x를 반환합니다.
    };
    vec.y = match rng.gen_bool(0.5) {// 0.5의 확률로 true 또는 false를 반환합니다.
        true => y, // true일 경우 y를 반환합니다.
        false => -y, // false일 경우 -y를 반환합니다.
    };
}
struct MainState {
    // player1,2 에 대한 posintion 값 feild 설정
    player_1_pos: na::Point2<f32>, //Point2는 2차원 공간에서의 점(위치)를 나타냅니다.
    player_2_pos: na::Point2<f32>,
    // ball의 위치와 방향 값 feild 설정
    ball_pos: na::Point2<f32>,
    ball_vel: na::Vector2<f32>, //ball의 방향을 나타내야 하기 때문에 2차원 벡터를 나타내는 Vector2타입으로 설정합니다.
}
impl MainState {
    //MainState 구조체의 인스턴스를 생성하는 함수
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);// 현재 게임 윈도우의 너비와 높이 정보를 screen_w, screen_h에 저장
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);// screen half값 저장
        let mut ball_vel = na::Vector2::new(0.0, 0.0);// ball_vel의 초기화
        randomize_vec(&mut ball_vel, 300.0, 300.0); //randomize_vec 함수를 써서 ball의 속도를 300.0으로 설정합니다.
        MainState {
            player_1_pos: na::Point2::new(RACKET_WIDTH_HALF, screen_h_half), //player1,2의 위치를 스크린 중간높이에 저장
            player_2_pos: na::Point2::new(screen_w - RACKET_WIDTH_HALF, screen_h_half),
            ball_pos: na::Point2::new(screen_w_half, screen_h_half),// ball의 위치를 스크린 가운데로 저장
            ball_vel: ball_vel,//ball_vel 필드 초기화
        }
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32(); // 프레임에 상관없이 경과한 시간을 초로 포현
        let (screen_w, screen_h) = graphics::drawable_size(ctx); //스크린 사이즈를 저장하는 변수

        // 키 입력에 따라 라켓을 움직이도록 함.
        // W, S는 player_1, Up, Down은 player_2
        move_racket(&mut self.player_1_pos, KeyCode::W, -1.0, ctx);
        move_racket(&mut self.player_1_pos, KeyCode::S, 1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Up, -1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Down, 1.0, ctx);

        self.ball_pos += self.ball_vel * dt; // 프레임에 상관없이 일정한 속도로 공을 움직이도록 함.

        // 게임 오버
        // 공의 위치를 중앙으로 되돌리고 속도를 랜덤하게 해서 다시 시작함.
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

        // 공이 스크린의 높이(위아래)를 벗어나는 경우
        // ball의 속도의 y값을 반대로 돌림
        if self.ball_pos.y < BALL_SIZE_HALF {
            self.ball_pos.y = BALL_SIZE_HALF;
            self.ball_vel.y = self.ball_vel.y.abs();
        } else if self.ball_pos.y > screen_h - BALL_SIZE_HALF {
            self.ball_pos.y = screen_h - BALL_SIZE_HALF;
            self.ball_vel.y = -self.ball_vel.y.abs();
        }
        // player 타일과 ball 상호작용
        // 플레이어 라켓과 ball이 부딪히는 경우 ball의 속도의 x를 반대로 바꿈
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
        graphics::clear(ctx, graphics::BLACK); // 매번 그릴때마다 이전에 그려진 부분은 지워야함. 그래서 바탕색으로 지움.

        // 플레이어 라켓인 사각형 그리기
        let racket_rect = graphics::Rect::new(
            -RACKET_WIDTH_HALF,
            -RACKET_HIGHT_HALF,
            RACKET_WIDTH,
            RACKET_HEIGHT,
        );
        // 사각형의 오브젝트를 생성하는 함수
        let racket_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            racket_rect,
            graphics::WHITE,
        )?;

        // 공 그리는 부분, 사각형
        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        // 똑같이 사각형 오브젝트 생성
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::WHITE,
        )?;

        // 그래픽 렌더링 방식을 기본적으로 초기화된 값을 사용
        let mut draw_param = graphics::DrawParam::default();

        // player_1의 위치를 기준으로 해서 그림
        draw_param.dest = self.player_1_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;
        // player_2
        draw_param.dest = self.player_2_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;
        // player_3
        draw_param.dest = self.ball_pos.into();
        graphics::draw(ctx, &ball_mesh, draw_param)?;

        graphics::present(ctx); // 현재 프레임을 출력, 이 부분이 없으면 최종적으로 화면에 그려지지 않음
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
