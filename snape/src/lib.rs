mod snape;
use snape::Snape;
/* use wasm_bindgen::prelude::*;
use web_sys::{CustomEvent, CustomEventInit, EventTarget};
use js_sys::JSON; */


/* #[wasm_bindgen]
pub fn emit_game_event(event_name: &str, payload_json: &str) {

    let window = web_sys::window().expect("should have a window");

    let mut event_init_dict = CustomEventInit::new();
    event_init_dict.bubbles(true); 
    event_init_dict.composed(true); 

    let detail_value = JSON::parse(payload_json)
        .unwrap_or_else(|err| {
            JsValue::from_str("Invalid JSON payload")
        });

    event_init_dict.detail(&detail_value);
    let event = CustomEvent::new_with_event_init_dict(event_name, &event_init_dict)
        .expect(&format!("Rust: Failed to create CustomEvent: {}", event_name));

    let event_target: &EventTarget = window.as_ref(); 
    event_target.dispatch_event(&event)
        .expect(&format!("Rust: Failed to dispatch custom event: {}", event_name));
}
 */

turbo::init! {
    struct GameState {
        frame: u32,
        snape: Snape,
        last_dir: (i32, i32)
    } = Self {
        frame: 0,
        snape: Snape::new((5,5)),
        last_dir: (1, 0),
    }
}

turbo::go!({
    let mut state = GameState::load();
    state.frame += 1;

    state = get_input(state);

    if state.frame % 120 == 0{
        state.snape.move_snape(state.last_dir);
    }
    state.snape.draw();

    state.save();
});

fn get_input(mut state: GameState) -> GameState {
    if gamepad(0).up.just_pressed() {
        //emit_game_event("GameOver", "");
        log("y");
        state.last_dir = (0, -2);
    }
    if gamepad(0).down.just_pressed() {
        state.last_dir = (0, 1);
    }
    if gamepad(0).left.just_pressed() {
        state.last_dir = (-1, 0);
    }
    if gamepad(0).right.just_pressed() {
        state.last_dir = (1, 0);
    }
    return state;
}
