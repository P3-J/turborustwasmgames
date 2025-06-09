mod ev1;

turbo::init! {
    struct GameState {
        frame: u32,
        init_boot: bool,
        craft: ev1::Ev1,
    } = Self {
        frame: 0,
        init_boot: true,
        craft: ev1::Ev1::new(),
    }
}



turbo::go!({
    let mut state = GameState::load();
    state.frame += 1;

    if state.init_boot {
        _init_start_up(&mut state);
    }

    let dir = check_for_input();
    state.craft.draw();
    state.craft.move_dir(dir);

    state.save();

});

fn _init_start_up(state: &mut GameState) {
   state.craft.move_pos((80,50));
   state.init_boot = false;
}


fn check_for_input() -> (i8, i8) {
    if gamepad(0).left.pressed() {
        camera::set_z(1.0);
        return (-1, 0);
    }
    if gamepad(0).right.pressed(){
        camera::set_z(2.);
        return (1, 0);
    }
    (0, 0)
}
