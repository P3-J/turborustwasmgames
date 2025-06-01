mod snape;
use snape::Snape;

turbo::init! {
    struct GameState {
        frame: u32,
        snape: Snape,
        last_dir: (i32, i32),
        food: (i32, i32),
        frame_speed_buff: u32,
    } = Self {
        frame: 0,
        snape: Snape::new((5,5)),
        last_dir: (1, 0),
        food: (-1, -1),
        frame_speed_buff: 0,
    }
}

turbo::go!({
    let mut state = GameState::load();
    clear(0x1001fff);
    state.frame += 1;
    state = get_input(state);

    let mut restart = false;
    
    state = generate_food(state);
    if state.frame % (30 - state.frame_speed_buff) == 0{
        restart = state.snape.move_snape(state.last_dir);
    }
    
    state.snape.draw();

    if restart {
        state = GameState {
            frame: 0,
            snape: Snape::new((5,5)),
            last_dir: (1, 0),
            food: (-1, -1),
            frame_speed_buff: 0,
        }
    }

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

fn generate_food(mut state: GameState) -> GameState {

    if state.food.0 == -1{
        let x: i32 = 0 + (rand() % 9) as i32;
        let y: i32 = 0 + (rand() % 9) as i32;
        state.food = (x, y);
    }

    if (state.food == state.snape.get_head_pos()){
        state.snape.increase_size();
        state.food = (-1, -1);
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
