mod snape;
use snape::Snape;



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

    if state.frame % 30 == 0{
        state.snape.move_snape(state.last_dir);
    }

    state.snape.draw();

    state.save();
});

fn get_input(mut state: GameState) -> GameState {
    if gamepad(0).up.just_pressed() {
        state.last_dir = (0, -1);
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
