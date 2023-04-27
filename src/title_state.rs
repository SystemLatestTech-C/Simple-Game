use ggez::event;
use ggez::graphics; // 그래픽 모듈
use ggez::input::keyboard::{self, KeyCode}; // 키보드 모듈
use ggez::nalgebra as na; // 벡터, 행렬 등의 수학 연산 모듈
use ggez::{Context, GameResult};

use crate::constants::StateTransition; // 게임 모듈(실행환경 저장 및 결과 반환)

//state를 TitleState로 따로 만들어서 메인 화면의 상태를 관리
pub struct TitleState {
    pub state_transition: StateTransition, //스테이트를 변경하고자 할 때 변경될 ENUM 값
    title_text: graphics::Text,            //타이틀 텍스트
    title_text_pos: na::Point2<f32>,       //타이틀 텍스트 위치
    single_button_text: graphics::Text,    //싱글 플레이 시작 버튼 텍스트
    single_button_pos: na::Point2<f32>,    //싱글 플레이 시작 버튼 위치
    multi_button_text: graphics::Text,     //멀티 플레이 시작 버튼 텍스트
    multi_button_pos: na::Point2<f32>,     //멀티 플레이 시작 버튼 위치
}

impl TitleState {
    pub fn new(ctx: &mut Context) -> Self {
        // 제목 텍스트 설정
        let title_font = graphics::Font::default();
        let title_scale = graphics::Scale::uniform(40.0); // 폰트 크기 설정
        let title_text = graphics::Text::new(
            graphics::TextFragment::new("Ping-Pong")
                .font(title_font)
                .scale(title_scale)
                .color(graphics::WHITE),
        );
        let title_text_width = title_text.width(ctx) as f32;

        // 싱글플레이 버튼 텍스트 설정
        let button_font = graphics::Font::default();
        let button_scale = graphics::Scale::uniform(20.0);
        let single_button_text = graphics::Text::new(
            graphics::TextFragment::new("Single Game")
                .font(button_font)
                .scale(button_scale)
                .color(graphics::WHITE),
        );
        let single_button_text_width = single_button_text.width(ctx) as f32;

        //멀티플레이 버튼 텍스트 설정
        let multi_button_text = graphics::Text::new(
            graphics::TextFragment::new("Multi Game")
                .font(button_font)
                .scale(button_scale)
                .color(graphics::WHITE),
        );
        let multi_button_text_width = multi_button_text.width(ctx) as f32;

        let (screen_w, screen_h) = graphics::drawable_size(ctx); // 현재 게임 윈도우의 너비와 높이 정보를 screen_w, screen_h에 저장

        TitleState {
            state_transition: StateTransition::None, //변경 요청이 없으므로 None 반환
            title_text,                              //타이틀 텍스트
            title_text_pos: na::Point2::new(
                screen_w * 0.5 - title_text_width * 0.5,
                screen_h * 0.3,
            ), //타이틀 텍스트 위치
            single_button_text,                      //버튼 텍스트
            single_button_pos: na::Point2::new(
                screen_w * 0.5 - single_button_text_width * 0.5,
                screen_h * 0.6,
            ), //버튼 위치
            multi_button_text,
            multi_button_pos: na::Point2::new(
                screen_w * 0.5 - multi_button_text_width * 0.5,
                screen_h * 0.7,
            ),
        }
    }
}
impl event::EventHandler for TitleState {
    fn mouse_button_down_event(
        //마우스 클릭 시 발생 이벤트
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        let point = na::Point2::new(x, y); //마우스의 좌표 가져오기

        let single_button_rect = graphics::Rect::new(
            self.single_button_pos.x,
            self.single_button_pos.y,
            self.single_button_text.width(ctx) as f32,
            self.single_button_text.height(ctx) as f32,
        ); //버튼으로 설정할 사각형 영역 생성

        let multi_button_rect = graphics::Rect::new(
            self.multi_button_pos.x,
            self.multi_button_pos.y,
            self.multi_button_text.width(ctx) as f32,
            self.multi_button_text.height(ctx) as f32,
        );

        //설정한 사각형 좌표에 마우스 클릭 좌표가 겹치면 게임 페이지 전환.
        if single_button_rect.contains(point) {
            self.state_transition = StateTransition::ToMain; //현 시점에서는 싱글 플레이 버튼을 누를 시, 호스트 스테이트로 전환하도록 임시로 설정
        }
        if multi_button_rect.contains(point) {
            self.state_transition = StateTransition::ToPlayer; //현 시점에서는 멀티 플레이 버튼을 누를 시, 플레이어 스테이트로 전환하도록 임시로 설정
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK); // 매번 그릴때마다 이전에 그려진 부분은 지워야함. 그래서 바탕색으로 지움.

        let mut draw_param = graphics::DrawParam::new();

        //제목, 싱글플레이 버튼, 멀티플레이 버튼 그리기.
        draw_param.dest = self.title_text_pos.into();
        graphics::draw(ctx, &self.title_text, draw_param)?;

        draw_param.dest = self.single_button_pos.into();
        graphics::draw(ctx, &self.single_button_text, draw_param)?;

        draw_param.dest = self.multi_button_pos.into();
        graphics::draw(ctx, &self.multi_button_text, draw_param)?;

        graphics::present(ctx); // 현재 프레임을 출력, 이 부분이 없으면 최종적으로 화면에 그려지지 않음
        Ok(())
    }
}
