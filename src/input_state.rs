use ggez::event;
use ggez::graphics; // 그래픽 모듈
use ggez::input::keyboard::{self, is_key_pressed, KeyCode}; // 키보드 모듈
use ggez::nalgebra as na; // 벡터, 행렬 등의 수학 연산 모듈
use ggez::{Context, GameResult};
use ggez::event::KeyMods;

use crate::constants::StateTransition; // 게임 모듈(실행환경 저장 및 결과 반환)

//state를 InputState로 따로 만들어서 메인 화면의 상태를 관리
pub struct InputState {
    pub state_transition: StateTransition, //스테이트를 변경하고자 할 때 변경될 ENUM 값
    port: String,
    input_title_text: graphics::Text,            //input타이틀 텍스트
    input_title_text_pos: na::Point2<f32>,       //input타이틀 텍스트 위치
    start_button_text: graphics::Text,     //멀티 플레이 시작 버튼 텍스트
    start_button_post: na::Point2<f32>,     //멀티 플레이 시작 버튼 위치
}

impl InputState {
    pub fn new(ctx: &mut Context) -> Self {
        // 제목 텍스트 설정
        let input_title_font = graphics::Font::default();
        let input_title_scale = graphics::Scale::uniform(40.0); // 폰트 크기 설정
        let input_title_text = graphics::Text::new(
            graphics::TextFragment::new("Input port number state")
                .font(input_title_font)
                .scale(input_title_scale)
                .color(graphics::WHITE),
        );
        let input_title_text_width = input_title_text.width(ctx) as f32;
        // start 버튼 텍스트 설정
        let button_font = graphics::Font::default();
        let button_scale = graphics::Scale::uniform(20.0);
        let start_button_text = graphics::Text::new(
            graphics::TextFragment::new("Start game")
                .font(button_font)
                .scale(button_scale)
                .color(graphics::WHITE),
        );
        let start_button_text_width = start_button_text.width(ctx) as f32;
        let start_button_text_height = start_button_text.height(ctx) as f32;

        let (screen_w, screen_h) = graphics::drawable_size(ctx); // 현재 게임 윈도우의 너비와 높이 정보를 screen_w, screen_h에 저장

        InputState {
            state_transition: StateTransition::None, //변경 요청이 없으므로 None 반환
            input_title_text,                              //input타이틀 텍스트
            port: String::new(),                        // 입력 받을 port number
            input_title_text_pos: na::Point2::new(
                screen_w * 0.5 - input_title_text_width * 0.5,
                screen_h * 0.1,
            ),
            start_button_text,
            start_button_post: na::Point2::new(
                screen_w - (start_button_text_width * 1.2),
                screen_h - (start_button_text_height * 2.0),
            ),
        }
    }
}
impl event::EventHandler for InputState {
    fn text_input_event(&mut self, ctx: &mut Context, character: char){//port number를 키보드로 입력 받기 위한 함수
        if self.port.len() < 6{
            self.port.push(character);
        }
    }
    /*
    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, _repeat: bool) { // enter키와 backspace키 입력 받는 함수
        match keycode {
            KeyCode::Return => {
                // Enter 키가 눌리면 여기에서 입력된 텍스트를 처리합니다.
                self.port.clear();
            },
            KeyCode::Back => {
                // 키보드에서 문자 키가 눌리면 여기에서 입력된 텍스트를 수집합니다.
                self.port.pop();
            },
            _ => {
                // 그 외의 키가 눌리면 여기에서 입력된 텍스트를 수집합니다.

            }
        }
    }*/
    fn mouse_button_down_event(
        //마우스 클릭 시 발생 이벤트
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        let point = na::Point2::new(x, y); //마우스의 좌표 가져오기
        let start_button_rect = graphics::Rect::new(//start 버튼 위치, 크기 정보
            self.start_button_post.x,
            self.start_button_post.y,
            self.start_button_text.width(ctx) as f32,
            self.start_button_text.height(ctx) as f32,
        );

        //설정한 사각형 좌표에 마우스 클릭 좌표가 겹치면 게임 페이지 전환.
        if start_button_rect.contains(point) {
            self.state_transition = StateTransition::Client; //client로 실행 되도록 설정
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        /*
        if is_key_pressed(ctx, KeyCode::Back) && !self.port.is_empty() { //backspace를 누르면 port에 있는 번호 하나씩 pop해서 지우기
            self.port.pop();
        }*/
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK); // 매번 그릴때마다 이전에 그려진 부분은 지워야함. 그래서 바탕색으로 지움.
        //제목, 싱글플레이 버튼, 멀티플레이 버튼 그리기.
        let mut draw_param = graphics::DrawParam::new();
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let input_port = graphics::Text::new(format!("Input port number : {}", self.port));//포트 번호 입력 설명 표시 정보
        let input_port_pos = na::Point2::new(screen_w * 0.2, screen_h * 0.5);

        //포트 번호 입력 표시, start 버튼, input타이틀 그리기
        draw_param.dest = input_port_pos.into();
        graphics::draw(ctx, &input_port, draw_param)?;

        draw_param.dest = self.input_title_text_pos.into();
        graphics::draw(ctx, &self.input_title_text, draw_param)?;


        draw_param.dest = self.start_button_post.into();
        graphics::draw(ctx, &self.start_button_text, draw_param)?;

        graphics::present(ctx); // 현재 프레임을 출력, 이 부분이 없으면 최종적으로 화면에 그려지지 않음
        Ok(())
    }
}
