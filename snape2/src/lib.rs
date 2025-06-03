mod snape;
use snape::Snape;

turbo::init! {
    struct GameState {
        frame: u32,
        snape: Snape,
        dir: (i32, i32),
        food: (i32, i32),
        frame_speed_buff: u32,
    } = Self {
        frame: 0,
        snape: Snape::new((5,5)),
        dir: (1, 0),
        food: generate_random_board_pos(),
        frame_speed_buff: 0,
    }
}

turbo::go!({
    let mut state = GameState::load();
    clear(0x1001fff);
    state.frame += 1;
    state = get_input(state);

    let mut restart = false;
    
    state = draw_and_check_food(state);
    if state.frame % (30 - state.frame_speed_buff) == 0{
        restart = state.snape.move_snape(state.dir);
    }
    state.snape.draw();

    if restart {
        state = restart_game();
    }

    state.save();
});

fn restart_game() -> GameState {
    return GameState {
        frame: 0,
        snape: Snape::new((5,5)),
        dir: (1, 0),
        food: generate_random_board_pos(),
        frame_speed_buff: 0,
    }
}

fn get_input(mut state: GameState) -> GameState {

    if gamepad(0).up.just_pressed() {
        state.dir = (0, -1);
    }
    if gamepad(0).down.just_pressed() {
        state.dir = (0, 1);
    }
    if gamepad(0).left.just_pressed() {
        state.dir = (-1, 0);
    }
    if gamepad(0).right.just_pressed() {
        state.dir = (1, 0);
    }
    return state;
}


fn draw_and_check_food(mut state: GameState) -> GameState {

    if state.food == state.snape.get_head_pos(){
        state.snape.increase_size();
        state.food = generate_random_board_pos();
        state.frame_speed_buff += 1;
    }

    rect!(
            x = state.food.0,
            y = state.food.1,
            w = 1,
            h = 1,
            color = 0xeb4034ff,
    );
    return state;
}

fn generate_random_board_pos() -> (i32, i32) {
    let x: i32 = 0 + (rand() % 9) as i32;
    let y: i32 = 0 + (rand() % 9) as i32;
    return (x, y)
}
