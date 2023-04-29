use ggez; // rust의 게임 라이브러리
use ggez::event; // 이벤트 모듈
use ggez::input::keyboard::{self, KeyCode}; // 키보드 모듈
use ggez::{Context, GameResult}; // 게임 모듈(실행환경 저장 및 결과 반환)

use crate::constants::StateTransition;
use crate::end_state::EndState;
use crate::game_state::GameState;
use crate::title_state::TitleState;

// AppState는 게임의 현재 State를 인자로 가지는 ENUM을 갖고, ENUM의 값에 해당하는 스테이트의 이벤트핸들러 함수를 수행합니다.
// 즉 동작에 따라 씬을 전환하는 역할을 수행하는, event::run에서 사용될 최상위 스테이트입니다.
// 상태를 추가할 때, CurrentState ENUM과 StateTransition ENUM에 해당 상태를 추가하고,
//AppState의 이벤트 핸들러에서 해당 스테이트에 수행할 match문의 항목을 추가합니다.
//각각의 스테이트 구조체는 (예를 들면) StateTransition을 필드로 가지고, 상황에 따라 StateTransition의 값을 바꿀 수 있습니다.
//AppState에서는 현재 State의 StateTranstion 필드를 확인하고, 그 값에 따라서 현재 스테이트를 다른 스테이트로 변환합니다.

//Appstate 구조체. 현재 스테이트를 인자로 가짐.
pub struct AppState {
    current_state: CurrentState,
}

//기본 스테이트는 타이틀 화면.
impl AppState {
    pub fn new(ctx: &mut Context) -> Self {
        AppState {
            current_state: CurrentState::Title(TitleState::new(ctx)),
        }
    }

    //각 State의 Stage Transition을 인자로 받아서, 현재 페이지(State)를 인자로 받은 스테이트로 변경하는 함수
    pub fn change_state(&mut self, ctx: &mut Context, state_transition: StateTransition) {
        match state_transition {
            StateTransition::Solo => {
                self.current_state = CurrentState::Game(GameState::new(ctx, 0));
            }
            StateTransition::Multi => {
                self.current_state = CurrentState::Game(GameState::new(ctx, 1));
            }
            StateTransition::P1Win => {
                self.current_state = CurrentState::End(EndState::new(ctx, 1));
            }
            StateTransition::P2Win => {
                self.current_state = CurrentState::End(EndState::new(ctx, 2));
            }
            _ => {}
        }
    }
}

//현재 게임의 스테이트. 추가 가능.
pub enum CurrentState {
    Title(TitleState),
    Game(GameState),
    End(EndState),
}

impl event::EventHandler for AppState {
    fn mouse_button_down_event(
        //마우스 클릭 시 발생 이벤트
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        match &mut self.current_state {
            CurrentState::Title(title_state) => {
                // title_state를 사용하여 마우스 클릭 로직을 수행합니다.
                title_state.mouse_button_down_event(ctx, button, x, y); //title_state의 마우스 클릭 로직 실행
            }
            CurrentState::Game(game_state) => {
                // game_state를 사용하여 마우스 클릭 로직을 수행합니다.
            }
            CurrentState::End(end_state) => {
                // end_state를 사용하여 마우스 클릭 로직을 수행합니다.
            }
        };
    }

    //CurrentState의 종류에 따른 업데이트 로직 수행
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match &mut self.current_state {
            CurrentState::Title(title_state) => {
                // title_state를 사용하여 업데이트 로직을 수행합니다.
            }
            CurrentState::Game(game_state) => {
                // game_state를 사용하여 업데이트 로직을 수행합니다.
                game_state.update(ctx).unwrap();
            }
            CurrentState::End(end_state) => {
                // end_state를 사용하여 업데이트 로직을 수행합니다.
            }
        }

        //현재 스테이트의, 스테이트 변경 요청을 체크 후 가져오기
        let state_transition = match &mut self.current_state {
            CurrentState::Title(title_state) => title_state.state_transition,
            CurrentState::Game(game_state) => game_state.state_transition,
            CurrentState::End(end_state) => StateTransition::None,
        };

        //스테이트 변경 요청에 따라 스테이트를 변경
        self.change_state(ctx, state_transition);

        Ok(())
    }

    //CurrentState의 종류에 따른 렌더링 로직 수행
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &mut self.current_state {
            CurrentState::Title(title_state) => {
                // title_state를 사용하여 렌더링 로직을 수행합니다.
                title_state.draw(ctx).unwrap();
            }
            CurrentState::Game(game_state) => {
                // game_state를 사용하여 렌더링 로직을 수행합니다.
                game_state.draw(ctx).unwrap();
            }
            CurrentState::End(end_state) => {
                // end_state를 사용하여 업데이트 로직을 수행합니다.
                end_state.draw(ctx).unwrap();
            }
        }
        Ok(())
    }
}
