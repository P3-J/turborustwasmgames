mod bird;
mod pipes;
use bird::Bird;
use pipes::Pipe;

turbo::init! {
    
    struct GameState {
        frame: u32,
        pipes: Vec<Pipe>,
        score: u32,
        birdy: Bird,
    } = Self {
        frame: 0,
        pipes: vec![],
        score: 0,
        birdy: Bird::new(),
    }
}

turbo::go!({
    clear(0x40d1c8ff);
    let mut state = GameState::load();
    state.frame += 1;

    state = check_movement(state);

    if (state.frame % 60 == 0){
        state = check_pipe_gen(state);
    }
    //log!("pipecount {:?}", state.pipes.len());

    if (state.pipes.len() > 0 && state.pipes[0].x < 138. && !state.pipes[0].passed){
        state.pipes[0].passed_pipe();
        state.score += 1;
    }

    let mut colliding = false;
    state.pipes.retain_mut(|p| {
        if p.x > -10.{
            p.draw_self();
            let is_col = p.check_for_col((state.birdy.x , state.birdy.y));
            if is_col {colliding = true;}
            return true
        }
        return false
    });

    state.birdy.set_sprite("floppy");
    state.birdy.apply_gravity();

    if (colliding || state.birdy.y > 144. || state.birdy.y < 0.) {
        state = GameState{
            frame: 0,
            pipes: vec![],
            score: 0,
            birdy: Bird::new(), 
        }
    }
    rect! (
        w = 1,
        h = 1,
        x = state.birdy.x,
        y = state.birdy.y,
        color = 0x006400FF
    );
    text!("Score: {}", state.score; x = 10, y = 10, font = "large", color = 0xffffffff);

    state.save();
});


fn check_movement(mut state: GameState) -> GameState {
    if gamepad(0).start.just_pressed() || pointer().pressed(){
        state.birdy.jump(true);
    }
    return state;
}


fn gen_pipe_pair(mut state: GameState) -> GameState {

    let n = 1 + (rand() % 45);

    let top_pipe = Pipe::new(244., 0., 30., 30. + n as f32);
    let bot_pipe: Pipe = Pipe::new(244., 80. + n as f32, 30., 60.);

    state.pipes.push(top_pipe);
    state.pipes.push(bot_pipe);

    return state;
}

fn check_pipe_gen(mut state: GameState) -> GameState {

    
    if (state.pipes.len() == 0){
        state = gen_pipe_pair(state);
    }
    if (state.pipes.len() <= 3 && state.pipes[0].x < 158.) {
        state = gen_pipe_pair(state);
    }
    return state;
}
