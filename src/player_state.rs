use ggez::nalgebra as na; // 벡터, 행렬 등의 수학 연산 모듈
use std::net::TcpStream;
use ggez::{Context, GameResult}; // 게임 모듈(실행환경 저장 및 결과 반환)
use std::io;
use std::io::{Read, Write};
use std::io::{ErrorKind};
use ggez::graphics; // 그래픽 모듈
use ggez::event; // 이벤트 모듈
use ggez::input::keyboard::{self, KeyCode}; // 키보드 모듈

use crate::constants::*; // constants.rs 파일을 가져옵니다.
use crate::state_func::*; // state_func.rs 파일을 가져옵니다.

/**
*   플레이어 2 구조체입니다.
*   본래의 MainState와 달라진 점은
*   1. 공의 움직임과 관련된 계산 부분 삭제
*   2. update 함수 내에 서버에 라켓2의 y좌표를 전송하고, 서버로부터 라켓1의 y좌료 + 공의 좌표 받아오는 부분 추가
*   3. draw부분은 동일 (단, 서버를 통해 받아온 라켓1의 y좌표, 공의 좌표를 사용해서 그림을 그립니다.)
*/


pub struct PlayerState {
    // player1,2 에 대한 posintion 값 feild 설정
    player_1_pos: na::Point2<f32>, //Point2는 2차원 공간에서의 점(위치)를 나타냅니다.
    player_2_pos: na::Point2<f32>,
    // ball의 위치와 방향 값 feild 설정
    ball_pos: na::Point2<f32>,
    ball_vel: na::Vector2<f32>, //ball의 방향을 나타내야 하기 때문에 2차원 벡터를 나타내는 Vector2타입으로 설정합니다.
    server_socket: TcpStream,   //서버와 통신하기 위한 소켓
}
impl PlayerState {
    //PlayerState 구조체의 인스턴스를 생성하는 함수
    pub fn new(ctx: &mut Context) -> Self {
        let server_socket = TcpStream::connect(SERVER_ADDR).map_err(|e| { //서버와 연결을 시도합니다.
            io::Error::new(ErrorKind::Other, format!("Failed to connect to server: {}", e))
        }).unwrap();

        let (screen_w, screen_h) = graphics::drawable_size(ctx);// 현재 게임 윈도우의 너비와 높이 정보를 screen_w, screen_h에 저장
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);// screen half값 저장
        let mut ball_vel = na::Vector2::new(0.0, 0.0);// ball_vel의 초기화
        //randomize_vec(&mut ball_vel, 300.0, 300.0); //randomize_vec 함수를 써서 ball의 속도를 300.0으로 설정합니다.
        PlayerState {
            player_1_pos: na::Point2::new(RACKET_WIDTH_HALF, screen_h_half), //player1,2의 위치를 스크린 중간높이에 저장
            player_2_pos: na::Point2::new(screen_w - RACKET_WIDTH_HALF, screen_h_half),
            ball_pos: na::Point2::new(screen_w_half, screen_h_half),// ball의 위치를 스크린 가운데로 저장
            ball_vel: ball_vel,//ball_vel 필드 초기화
            server_socket,
        }
    }
}
impl event::EventHandler for PlayerState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32(); // 프레임에 상관없이 경과한 시간을 초로 포현
        let (screen_w, screen_h) = graphics::drawable_size(ctx); //스크린 사이즈를 저장하는 변수

        // 키 입력에 따라 라켓을 움직이도록 함.
        // W, S는 player_1, Up, Down은 player_2
        //move_racket(&mut self.player_1_pos, KeyCode::W, -1.0, ctx);
        //move_racket(&mut self.player_1_pos, KeyCode::S, 1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Up, -1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Down, 1.0, ctx);

        // 플레이어 2의 라켓 위치를 서버에 전송합니다.
        let player_2_position = self.player_2_pos.y.to_le_bytes();
        self.server_socket.write_all(&player_2_position).map_err(|e| {
            io::Error::new(ErrorKind::Other, format!("Failed to send data to server: {}", e))
        })?;

        // 플레이어 1의 라켓 위치, 공의 위치를 서버에서 받습니다.
        let mut buffer = [0u8; 12];
        self.server_socket.read_exact(&mut buffer).map_err(|e| {
            io::Error::new(ErrorKind::Other, format!("Failed to receive data from server: {}", e))
        })?;

        self.player_1_pos.y = f32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        self.ball_pos.x = f32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
        self.ball_pos.y = f32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);

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