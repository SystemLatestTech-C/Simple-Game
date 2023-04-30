use ggez::event;
use ggez::graphics; // 그래픽 모듈
use ggez::input::keyboard::{self, KeyCode}; // 키보드 모듈
use ggez::nalgebra as na; // 벡터, 행렬 등의 수학 연산 모듈
use ggez::{Context, GameResult};

use crate::constants::StateTransition; // 게임 모듈(실행환경 저장 및 결과 반환)

//state를 TitleState로 따로 만들어서 메인 화면의 상태를 관리
pub struct StayState {
    pub state_transition: StateTransition, //스테이트를 변경하고자 할 때 변경될 ENUM 값
    multi_title_text: graphics::Text,            //멀티타이틀 텍스트
    multi_title_text_pos: na::Point2<f32>,       //멀티타이틀 텍스트 위치
    make_room_button_text: graphics::Text,    //host 시작 버튼 텍스트
    make_room_button_pos: na::Point2<f32>,    //host 시작 버튼 위치
    input_port_button_text: graphics::Text,     //port 번호 입력 state 시작 버튼 텍스트
    input_port_button_post: na::Point2<f32>,     //port 번호 입력 state 시작 버튼 위치
}

impl StayState {
    pub fn new(ctx: &mut Context) -> Self {
        // 타이틀 텍스트 설정
        let multi_title_font = graphics::Font::default();
        let multi_title_scale = graphics::Scale::uniform(40.0); // 폰트 크기 설정
        let multi_title_text = graphics::Text::new(
            graphics::TextFragment::new("Multiplay state")
                .font(multi_title_font)
                .scale(multi_title_scale)
                .color(graphics::WHITE),
        );
        let multi_title_text_width = multi_title_text.width(ctx) as f32;
        //  host 시작 버튼 텍스트 설정
        let button_font = graphics::Font::default();
        let button_scale = graphics::Scale::uniform(20.0);
        let make_room_button_text = graphics::Text::new(
            graphics::TextFragment::new("Start host")
                .font(button_font)
                .scale(button_scale)
                .color(graphics::WHITE),
        );
        let make_room_button_text_width = make_room_button_text.width(ctx) as f32;
        //port번호 입력 state 시작 버튼 텍스트 설정
        let input_port_button_text = graphics::Text::new(
            graphics::TextFragment::new("Input port number")
                .font(button_font)
                .scale(button_scale)
                .color(graphics::WHITE),
        );
        let input_port_button_text_width = input_port_button_text.width(ctx) as f32;

        let (screen_w, screen_h) = graphics::drawable_size(ctx); // 현재 게임 윈도우의 너비와 높이 정보를 screen_w, screen_h에 저장

        StayState {
            state_transition: StateTransition::None, //변경 요청이 없으므로 None 반환
            multi_title_text,                              //타이틀 텍스트
            multi_title_text_pos: na::Point2::new(
                screen_w * 0.5 - multi_title_text_width * 0.5,
                screen_h * 0.1,
            ),
            make_room_button_text,                      //버튼 텍스트
            make_room_button_pos: na::Point2::new(
                screen_w * 0.5 - (input_port_button_text_width* 0.5 +(make_room_button_text_width)),
                screen_h * 0.5 ,
            ), //버튼 위치
            input_port_button_text,
            input_port_button_post: na::Point2::new(
                screen_w * 0.5 + input_port_button_text_width * 0.5,
                screen_h * 0.5,
            ),
        }
    }
}
impl event::EventHandler for StayState {
    fn mouse_button_down_event(
        //마우스 클릭 시 발생 이벤트
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        let point = na::Point2::new(x, y); //마우스의 좌표 가져오기

        let make_room_button_rect = graphics::Rect::new(
            self.make_room_button_pos.x,
            self.make_room_button_pos.y,
            self.make_room_button_text.width(ctx) as f32,
            self.make_room_button_text.height(ctx) as f32,
        ); //버튼으로 설정할 사각형 영역 생성

        let multi_button_rect = graphics::Rect::new(
            self.input_port_button_post.x,
            self.input_port_button_post.y,
            self.input_port_button_text.width(ctx) as f32,
            self.input_port_button_text.height(ctx) as f32,
        );

        //설정한 사각형 좌표에 마우스 클릭 좌표가 겹치면 게임 페이지 전환.
        if make_room_button_rect.contains(point) {
            self.state_transition = StateTransition::ToWait; //client가 들어올 때 까지 기다리는 state로 전환
        }
        if multi_button_rect.contains(point) {
            self.state_transition = StateTransition::Input; //port 번호를 입력 받는 state로 전환
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK); // 매번 그릴때마다 이전에 그려진 부분은 지워야함. 그래서 바탕색으로 지움.
        //제목, 각종 버튼 그리기.
        let mut draw_param = graphics::DrawParam::new();

        draw_param.dest = self.multi_title_text_pos.into();
        graphics::draw(ctx, &self.multi_title_text, draw_param)?;

        draw_param.dest = self.make_room_button_pos.into();
        graphics::draw(ctx, &self.make_room_button_text, draw_param)?;

        draw_param.dest = self.input_port_button_post.into();
        graphics::draw(ctx, &self.input_port_button_text, draw_param)?;

        graphics::present(ctx); // 현재 프레임을 출력, 이 부분이 없으면 최종적으로 화면에 그려지지 않음
        Ok(())
    }
}
