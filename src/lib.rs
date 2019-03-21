use web_dom::*;
mod game;
use crate::game::*;
use ref_thread_local::RefThreadLocal;
#[macro_use]
extern crate ref_thread_local;

ref_thread_local! {
    static managed GAME_STATE: GameState = GameState::new();
}

#[no_mangle]
pub fn callback(listener: EventListener, event: Event) {
    let game_state = &mut *GAME_STATE.borrow_mut();
    game_state.route_event(listener, event);
}

#[no_mangle]
pub fn main() -> () {
    let game_state = &mut *GAME_STATE.borrow_mut();
    game_state.init();
}
