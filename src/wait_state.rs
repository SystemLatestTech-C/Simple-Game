use ggez::event;
use ggez::graphics; // 그래픽 모듈
use ggez::input::keyboard::{self, KeyCode}; // 키보드 모듈
use ggez::nalgebra as na; // 벡터, 행렬 등의 수학 연산 모듈
use ggez::{Context, GameResult};

use crate::constants::StateTransition; // 게임 모듈(실행환경 저장 및 결과 반환)

//state를 TitleState로 따로 만들어서 메인 화면의 상태를 관리
pub struct WaitState {
    pub state_transition: StateTransition, //스테이트를 변경하고자 할 때 변경될 ENUM 값
    wait_title_text: graphics::Text,            //타이틀 텍스트
    wait_title_text_pos: na::Point2<f32>,       //타이틀 텍스트 위치
    port: String,                               //host의 port 번호
}

impl WaitState {
    pub fn new(ctx: &mut Context) -> Self {
        // 제목 텍스트 설정
        let wait_title_font = graphics::Font::default();
        let wait_title_scale = graphics::Scale::uniform(30.0); // 폰트 크기 설정
        let wait_title_text = graphics::Text::new(
            graphics::TextFragment::new("Waiting for another player")
                .font(wait_title_font)
                .scale(wait_title_scale)
                .color(graphics::WHITE),
        );
        let wait_title_text_width = wait_title_text.width(ctx) as f32;


        let (screen_w, screen_h) = graphics::drawable_size(ctx); // 현재 게임 윈도우의 너비와 높이 정보를 screen_w, screen_h에 저장

        WaitState{
            state_transition: StateTransition::None, //변경 요청이 없으므로 None 반환
            wait_title_text,                              //타이틀 텍스트
            wait_title_text_pos: na::Point2::new(
                screen_w * 0.5 - wait_title_text_width * 0.5,
                screen_h * 0.7,
            ),
            port:String::new(), //일단 host의 port 번호는 빈칸
        }
    }
}
impl event::EventHandler for WaitState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK); // 매번 그릴때마다 이전에 그려진 부분은 지워야함. 그래서 바탕색으로 지움.
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let mut draw_param = graphics::DrawParam::new();

        //wait 타이틀에 대한 텍스트 정보
        let wait_title_font = graphics::Font::default();
        let port_text_scale = graphics::Scale::uniform(20.0); // 폰트 크기 설정
        let port_text = graphics::Text::new(
            graphics::TextFragment::new(format!("My port number is : {}", self.port))//포트 번호 출력
                .font(wait_title_font)
                .scale(port_text_scale)
                .color(graphics::WHITE),
        );
        let port_text_width = port_text.width(ctx) as f32;
        let port_text_pos = na::Point2::new(
            screen_w * 0.5 - port_text_width * 0.5,
            screen_h * 0.3,
        );

        //포트 번호, wait 타이틀 그리기.
        draw_param.dest = port_text_pos.into();
        graphics::draw(ctx, &port_text, draw_param)?;

        draw_param.dest = self.wait_title_text_pos.into();
        graphics::draw(ctx, &self.wait_title_text, draw_param)?;

        graphics::present(ctx); // 현재 프레임을 출력, 이 부분이 없으면 최종적으로 화면에 그려지지 않음
        Ok(())
    }
}
