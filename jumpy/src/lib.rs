mod player;
mod objects;
use player::Player;
use objects::Blocker;

turbo::init!{

    struct GameState {
        frame: u32,
        player: Player,
        blockers: Vec<Blocker>,
    } = Self {
        frame: 0,
        player: Player::new(),
        blockers: vec![],
    }

}


turbo::go!({
    clear(0x40d1c8ff);
    let mut state = GameState::load();
    state.frame += 1;

    if state.frame % 120 == 0{
        state = gen_blocker(state);
    }

    let mut colliding = false;
    state.blockers.retain_mut(|p| {
        if p.x > 0{
            p.draw_self();
            let is_col = p.check_for_col((state.player.x + 10 , state.player.y + 10));
            if is_col {colliding = true;}
            return true
        }
        return false
    });

    state = check_movement(state);

    state.player.player_state_machine();
    draw_ground();


    if (colliding){
        state = GameState {
            frame: 0,
            player: Player::new(),
            blockers: vec![],
        }
    }

    state.save();
});



fn draw_ground() {
    rect! (
        w = 256,
        h = 30,
        x = 0,
        y = 114,
        color = 0x00F54927FF
    );
}

fn check_movement(mut state: GameState) -> GameState {
    if gamepad(0).start.just_pressed() || pointer().pressed(){
        state.player.jump(true);
    }
    return state;
}


fn gen_blocker(mut state: GameState) -> GameState {

    let obj: Blocker = Blocker::new( 120, 110, 10, 10);

    state.blockers.push(obj);

    return state;

}


