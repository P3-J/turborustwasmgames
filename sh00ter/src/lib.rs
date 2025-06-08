
turbo::init! {
    struct GameState {
        frame: u32,
        tpos: (i32, i32),
    } = Self {
        frame: 0,
        tpos: (5, 10),
    }
}



turbo::go!({
    let mut state = GameState::load();
    state.frame += 1;

    state.tpos.0 = 0;

    text!(
        "Welcome to Max",
        x = state.tpos.0,
        y = state.tpos.1,
    );

    state.save();

});
