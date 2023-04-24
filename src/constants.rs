/**
 *   마지막 부분에 SERVER_ADDR 상수 추가. 이는 서버의 주소 + 포트번호를 나타냅니다.
 *
 */

pub const RACKET_HEIGHT: f32 = 100.0; // 라켓의 높이
pub const RACKET_WIDTH: f32 = 20.0; // 라켓의 너비
pub const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5; // 라켓의 너비의 절반
pub const RACKET_HIGHT_HALF: f32 = RACKET_HEIGHT * 0.5; // 라켓의 높이의 절반
pub const BALL_SIZE: f32 = 30.0; // 공의 크기
pub const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5; // 공의 크기의 절반
pub const PLAYER_SPEED: f32 = 600.0; // 플레이어의 속도
pub const BALL_SPEED: f32 = 300.0; // 공의 속도
pub const SERVER_ADDR: &str = "127.0.0.1:8080"; // 서버의 주소

//스테이트를 변경하고자 할 때 반환하는 ENUM 값

#[derive(Clone, Copy)]
pub enum StateTransition {
    None,
    ToTitle,
    ToMain,
    ToPlayer,
}
