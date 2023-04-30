use ggez; // rust의 게임 라이브러리
use ggez::event; // 이벤트 모듈
use ggez::graphics; // 그래픽 모듈
use ggez::input::keyboard::{self, KeyCode}; // 키보드 모듈
use ggez::{Context, GameResult}; // 게임 모듈(실행환경 저장 및 결과 반환)
use std::env;
use std::io::{Read, Write};
use std::thread;

//use ggez::nalgebra as na; // 벡터, 행렬 등의 수학 연산 모듈
//use std::net::TcpStream;
//use std::io::prelude::*;
//use std::error::Error;
//use std::net::TcpListener;
//use std::io;
//use rand::{self, thread_rng, Rng}; // 랜덤 모듈
//use std::io::{ErrorKind};

mod app_state;
mod constants; // 상수를 관리하는 모듈입니다.
mod end_state;
mod game_state;
mod state_func; // move_racket, randomize_vec 함수를 관리하는 모듈입니다.
mod title_state;

use app_state::AppState;
use constants::*;
use state_func::*;
use title_state::TitleState;

/**
 *   main 함수입니다.
 *   처음 4줄은 이전에 작성한 main과 동일합니다.
 *   그 후, 실행 인자에 따라서 서버+호스트(플레이어1)를 실행할지, 플레이어2를 실행할지를 결정합니다.
 *   cargo run -- host 로 실행할 시 서버를 실행하고 MainState를 사용해서 실행합니다.
 *   cargo run -- player 로 실행할 시 플레이어2를 실행하고 PlayerState를 사용해서 실행합니다.
 *
 *   주의점 : 서버는 호스트가 실행될 때 시작하기 때문에, cargo run -- host 를 먼저 실행해야 합니다.
 */

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Ping-pong", "Name")
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "Ping-Pong"); // 창 제목을 설정합니다.
    let mut state = AppState::new(ctx); //임시로 변경
    event::run(ctx, event_loop, &mut state).unwrap();

    Ok(()) //성공적으로 완료, 반환값 없음
}
