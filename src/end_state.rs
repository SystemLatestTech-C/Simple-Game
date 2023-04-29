use ggez::event;
use ggez::graphics; // 그래픽 모듈
use ggez::input::keyboard::{self, KeyCode}; // 키보드 모듈
use ggez::nalgebra as na; // 벡터, 행렬 등의 수학 연산 모듈
use ggez::{Context, GameResult};

//점수에 따라 승자를 표시

pub struct EndState {
    win_player: i32, //승자 번호
}

impl EndState {
    pub fn new(ctx: &mut Context, win_player: i32) -> Self {
        EndState { win_player }
    }
}
impl event::EventHandler for EndState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        // 승자 텍스트 생성
        let win_text = format!("Player {} Win!", self.win_player);

        // 텍스트 객체 생성
        let win_text_obj = graphics::Text::new(win_text);

        // 화면 크기 구하기
        let screen_size = graphics::drawable_size(ctx);

        // 텍스트의 크기 구하기
        let text_size = win_text_obj.dimensions(ctx);

        // 텍스트를 화면 중앙에 위치시키기 위한 좌표 계산
        let position = na::Point2::new(
            (screen_size.0 - text_size.0 as f32) / 2.0,
            (screen_size.1 - text_size.1 as f32) / 2.0,
        );

        // 텍스트 그리기
        graphics::draw(ctx, &win_text_obj, (position, 0.0, graphics::WHITE))?;

        graphics::present(ctx)?;
        Ok(())
    }
}
