use ggez; // rust의 게임 라이브러리
use ggez::graphics; // 그래픽 모듈
use ggez::input::keyboard::{self, KeyCode}; // 키보드 모듈
use ggez::nalgebra as na; // 벡터, 행렬 등의 수학 연산 모듈
use rand::{self, thread_rng, Rng}; // 랜덤 모듈
use ggez::{Context, GameResult}; // 게임 모듈(실행환경 저장 및 결과 반환)

use crate::constants::*; // constants.rs 파일을 가져옵니다.

/**
*      move_racket 함수와 randomize_vec 함수를 관리하는 모듈입니다.
*      변경사항은 없습니다.
*/

pub fn move_racket(pos: &mut na::Point2<f32>, keycode: KeyCode, y_dir: f32, ctx: &mut Context) {
    let screen_h = graphics::drawable_size(ctx).1; // 화면의 높이를 가져옵니다.
    let dt = ggez::timer::delta(ctx).as_secs_f32(); // 1프레임당 흐른 시간(델타 타임)을 가져옵니다.

    if keyboard::is_key_pressed(ctx, keycode) {
        // 해당 키가 눌렸는지 확인합니다.
        pos.y += y_dir * PLAYER_SPEED * dt; // 라켓의 y 좌표를 업데이트합니다.
    }
    pos.y = pos.y.clamp(RACKET_HIGHT_HALF, screen_h - RACKET_HIGHT_HALF); // 라켓이 화면 밖으로 나가지 않도록 위치를 조정합니다.
}
pub fn randomize_vec(vec: &mut na::Vector2<f32>, x: f32, y: f32) { // 공의 방향을 랜덤한 방향으로 설정합니다.
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