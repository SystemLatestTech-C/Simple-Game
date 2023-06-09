use ggez::event; // 이벤트 모듈
use ggez::graphics; // 그래픽 모듈
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra as na; // 벡터, 행렬 등의 수학 연산 모듈
use ggez::timer;

use ggez::{Context, GameResult}; // 게임 모듈(실행환경 저장 및 결과 반환)
use std::io;
use std::io::ErrorKind;
use std::io::{Read, Write};
use std::net::Shutdown;
use std::net::TcpStream;
use std::thread;

use crate::constants::*; // constants.rs 파일을 가져옵니다.
use crate::state_func::*; // state_func.rs 파일을 가져옵니다.

static mut socket_client: Option<TcpStream> = None;

unsafe fn connect2server() {
    socket_client = (TcpStream::connect(SERVER_ADDR)
        .map_err(|e| {
            io::Error::new(
                ErrorKind::Other,
                format!("Failed to connect to server: {}", e),
            )
        })
        .ok());
}

pub struct GameState {
    pub state_transition: StateTransition, //스테이트를 변경하고자 할 때 변경될 ENUM 값
    // player1,2 에 대한 posintion 값 feild 설정
    player_1_pos: na::Point2<f32>,
    //Point2는 2차원 공간에서의 점(위치)를 나타냅니다.
    player_2_pos: na::Point2<f32>,
    // ball의 위치와 방향 값 feild 설정
    ball_pos: na::Point2<f32>,
    ball_vel: na::Vector2<f32>,
    //ball의 방향을 나타내야 하기 때문에 2차원 벡터를 나타내는 Vector2타입으로 설정합니다.
    player_1_score: u8, //스코어 텍스트
    player_2_score: u8, //스코어 텍스트 2
    state: i32,
    //server_socket: TcpStream,
    start_timer: f64,
    //현재 화면이 보여지기 전에 (싱글플레이 버튼을 누르면 화면 표시에 다소 시간이 소요) 미리 게임이 동작해서 시작부터 스코어가 1:0 인 오류가 발생함
    //임시로 3초가 지나야 게임의 update가 동작하도록 타이머 설정.
}
impl GameState {
    //MainState 구조체의 인스턴스를 생성하는 함수
    pub unsafe fn new(ctx: &mut Context, state: i32) -> Self {
        if state == 1 || state == 2 {
            connect2server();
        }
        let (screen_w, screen_h) = graphics::drawable_size(ctx); // 현재 게임 윈도우의 너비와 높이 정보를 screen_w, screen_h에 저장
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5); // screen half값 저장
        let mut ball_vel = na::Vector2::new(0.0, 0.0); // ball_vel의 초기화
        randomize_vec(&mut ball_vel, 300.0, 300.0); //randomize_vec 함수를 써서 ball의 속도를 300.0으로 설정합니다.
        GameState {
            state_transition: StateTransition::None,
            player_1_pos: na::Point2::new(RACKET_WIDTH_HALF, screen_h_half), //player1,2의 위치를 스크린 중간높이에 저장
            player_2_pos: na::Point2::new(screen_w - RACKET_WIDTH_HALF, screen_h_half),
            ball_pos: na::Point2::new(screen_w_half, screen_h_half), // ball의 위치를 스크린 가운데로 저장
            ball_vel: ball_vel,                                      //ball_vel 필드 초기화
            player_1_score: 0,
            player_2_score: 0,
            state,
            start_timer: 1.0,
        }
    }
}
impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt;
        if ggez::timer::delta(ctx).as_secs_f32() > 1.0 {
            dt = 0.0;
        } else {
            dt = ggez::timer::delta(ctx).as_secs_f32();
        }
        let (screen_w, screen_h) = graphics::drawable_size(ctx); //스크린 사이즈를 저장하는 변수
        println!("{:?}", self.state);
        if self.state == 1 {
            // host
            println!("{}", self.state);

            //서버 연결 안되었으면 연결 시도하고 종료
            // unsafe {
            //     let connected_to_server = socket_client.is_some();
            //     if !connected_to_server {
            //         connect2server();
            //         return Ok(());
            //     }
            // }

            move_racket(&mut self.player_1_pos, KeyCode::Up, -1.0, ctx);
            move_racket(&mut self.player_1_pos, KeyCode::Down, 1.0, ctx);

            self.ball_pos += self.ball_vel * dt; // 프레임에 상관없이 일정한 속도로 공을 움직이도록 함.

            // 플레이어 1의 라켓 위치, 공의 좌표를 서버에 보냅니다.
            let player_position_bytes = self.player_1_pos.y.to_ne_bytes();
            let ball_x_bytes = self.ball_pos.x.to_ne_bytes();
            let ball_y_bytes = self.ball_pos.y.to_ne_bytes();
            let player_position_ball_pos_data = [
                &player_position_bytes[..],
                &ball_x_bytes[..],
                &ball_y_bytes[..],
            ]
            .concat();

            unsafe {
                if let Some(server_socket) = &mut socket_client {
                    println!("{:?}", player_position_ball_pos_data);
                    server_socket
                        .write_all(&player_position_ball_pos_data)
                        .map_err(|e| {
                            io::Error::new(
                                ErrorKind::Other,
                                format!("Failed to send player position to server: {}", e),
                            )
                        })?;
                }
            }

            // 플레이어 2의 라켓 위치를 서버에서 받습니다.
            let mut buffer = [0u8; 4];
            unsafe {
                if let Some(server_socket) = &mut socket_client {
                    server_socket.read_exact(&mut buffer).map_err(|e| {
                        io::Error::new(
                            ErrorKind::Other,
                            format!("Failed to receive data from server: {}", e),
                        )
                    })?;
                    println!("{:?}", buffer);
                }
            }
            self.player_2_pos.y = f32::from_le_bytes(buffer);

            // 게임 오버
            // 공의 위치를 중앙으로 되돌리고 속도를 랜덤하게 해서 다시 시작함.
            if self.ball_pos.x < 0.0 {
                self.ball_pos.x = screen_w * 0.5;
                self.ball_pos.y = screen_h * 0.5;
                randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
                self.player_2_score += 1;
            }
            if self.ball_pos.x > screen_w {
                self.ball_pos.x = screen_w * 0.5;
                self.ball_pos.y = screen_h * 0.5;
                randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
                self.player_1_score += 1;
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
        } else if self.state == 2 {
            // client

            //서버 연결 안되었으면 연결 시도하고 종료
            // unsafe {
            //     let connected_to_server = socket_client.is_some();
            //     if !connected_to_server {
            //         connect2server();
            //         return Ok(());
            //     }
            // }

            move_racket(&mut self.player_2_pos, KeyCode::Up, -1.0, ctx);
            move_racket(&mut self.player_2_pos, KeyCode::Down, 1.0, ctx);

            // 플레이어 2의 라켓 위치를 서버에 전송합니다.
            let player_2_position = self.player_2_pos.y.to_le_bytes();
            unsafe {
                if let Some(server_socket) = &mut socket_client {
                    server_socket.write_all(&player_2_position).map_err(|e| {
                        io::Error::new(
                            ErrorKind::Other,
                            format!("Failed to send data to server: {}", e),
                        )
                    })?;
                }
            }

            // 플레이어 1의 라켓 위치, 공의 위치를 서버에서 받습니다.
            let mut buffer = [0u8; 12];
            unsafe {
                if let Some(server_socket) = &mut socket_client {
                    server_socket.read_exact(&mut buffer).map_err(|e| {
                        io::Error::new(
                            ErrorKind::Other,
                            format!("Failed to receive data from server: {}", e),
                        )
                    })?;
                }
            }

            self.player_1_pos.y = f32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
            self.ball_pos.x = f32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
            self.ball_pos.y = f32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);

            // 받아온 위치를 기준으로 점수만 체크
            if self.ball_pos.x < 0.0 {
                self.player_2_score += 1;
            }
            if self.ball_pos.x > screen_w {
                self.player_1_score += 1;
            }
        } else {
            // solo
            move_racket(&mut self.player_2_pos, KeyCode::Up, -1.0, ctx);
            move_racket(&mut self.player_2_pos, KeyCode::Down, 1.0, ctx);
            self.player_1_pos.y += if self.ball_pos.y > self.player_1_pos.y {
                1.9
            } else {
                -1.9
            };

            self.ball_pos += self.ball_vel * dt; // 프레임에 상관없이 일정한 속도로 공을 움직이도록 함.

            // 게임 오버
            // 공의 위치를 중앙으로 되돌리고 속도를 랜덤하게 해서 다시 시작함.
            if self.ball_pos.x < 0.0 {
                self.ball_pos.x = screen_w * 0.5;
                self.ball_pos.y = screen_h * 0.5;
                randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
                self.player_2_score += 1;
            }
            if self.ball_pos.x > screen_w {
                self.ball_pos.x = screen_w * 0.5;
                self.ball_pos.y = screen_h * 0.5;
                randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
                self.player_1_score += 1;
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
        }

        //승리 조건 달성 시 END 화면으로
        unsafe {
            if self.player_1_score > 4 {
                if let Some(server_socket) = &mut socket_client {
                    let tmp = [self.player_1_score, self.player_2_score, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
                    server_socket.write_all(&tmp)?;
                    // 소켓을 닫습니다.
                    std::mem::drop(server_socket);

                    // socket_client를 None으로 설정하여 이후 사용을 막습니다.
                    socket_client = None;
                }
                self.state_transition = StateTransition::P1Win;
            } else if self.player_2_score > 4 {
                if let Some(server_socket) = &mut socket_client {
                    let tmp = [self.player_1_score, self.player_2_score, 0, 0];
                    server_socket.write_all(&tmp)?;
                    // 소켓을 닫습니다.
                    std::mem::drop(server_socket);

                    // socket_client를 None으로 설정하여 이후 사용을 막습니다.
                    socket_client = None;
                }
                self.state_transition = StateTransition::P2Win;
            }
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

        // 스코어보드 문자열 만들기
        let scoreboard_str = format!("{} : {}", self.player_1_score, self.player_2_score);

        // 스코어보드 Text 객체 생성
        let scoreboard_text = graphics::Text::new(scoreboard_str);

        // 스코어보드 그리기 위치 설정
        let screen_size = graphics::drawable_size(ctx);
        let scoreboard_pos = na::Point2::new(screen_size.0 / 2.0, 50.0);

        // 스코어보드 그리기
        graphics::draw(
            ctx,
            &scoreboard_text,
            (scoreboard_pos, 0.0, graphics::WHITE),
        )?;

        graphics::present(ctx); // 현재 프레임을 출력, 이 부분이 없으면 최종적으로 화면에 그려지지 않음
        Ok(())
    }
}
