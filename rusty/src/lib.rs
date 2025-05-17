// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
turbo::init! {
    struct GameState {
        frame: u32,
        last_munch_at: u32,
        cat_x: f32,
        cat_y: f32,
        cat_r: f32,
        cat_rot: f32,
        pancakes: Vec<struct Pancake {
            x: f32,
            y: f32,
            vel: f32,
            radius: f32,
        }>,
        score: u32,
    } = Self {
        frame: 0,
        last_munch_at: 0,
        cat_x: 128.0,
        cat_y: 112.0,
        cat_r: 16.0,
        cat_rot: 0.,
        pancakes: vec![],
        score: 0,
    }
}

turbo::go!({
    let mut state = GameState::load();
    state.frame += 1;

    clear(0x00ffffff);
    let frame = (state.frame as i32) / 10;
    for col in 0..5 {
        for row in 0..3 {
            let x = ((col * 32 + frame) % 272);
            let y = ((row * 32 + frame) % 144);
            sprite!("heart", x = x, y = y);
        }
    }
    
    state = check_movement(state);

    let cat_center = (state.cat_x + state.cat_r, state.cat_y + state.cat_r);


    for pancake in &state.pancakes {
        circ!(x = pancake.x , y = pancake.y, d = pancake.radius, color = 0xdba470ff)
    }

    sprite!("munch_cat",
     x = state.cat_x - state.cat_r,
     y = state.cat_y - 16.0,
     rotation = state.cat_rot,
    );
 

    if rand() % 64 == 0 {
        let pk = Pancake {
            x: (rand() % 256) as f32,
            y: -15.0,
            vel: 3.,
            radius: 10.,
        };
        state.pancakes.push(pk);
    }

    

    
    text!("Score: {}", state.score; x = 10, y = 10, font = "large", color = 0xffffffff);
    // draw
    state.pancakes.retain_mut(|p| {
        p.y += p.vel;


        let dx = cat_center.0 - (p.x + p.radius);
        let dy = cat_center.1 - (p.y + p.radius);

        let distance = (dx * dx + dy * dy).sqrt();

        let max_distance = state.cat_r + p.radius;
        let radii_diff = (state.cat_r - p.radius).abs();

        if radii_diff <= distance && distance <= max_distance {
            state.score += 1;
            return false
        }
        return true;
    });



    state.save();
});


fn check_movement(mut state: GameState) -> GameState {
    if gamepad(0).left.pressed() {
        state.cat_x -= 2.;
    }
    if gamepad(0).right.pressed() {
        state.cat_x += 2.;
    }
    return state;
}
