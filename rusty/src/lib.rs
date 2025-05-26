

#[derive(BorshSerialize, BorshDeserialize,PartialEq, Debug, Clone)] //Turbo structs need to derive these 5 traits to serialize properly
struct Bird {
    x: f32,
    y: f32,
    r: f32,
    rot: f32,
    jumping: bool,
    start_y: f32,
}

impl Bird {
    fn new() -> Self {
        Self {
            x: 128.0,
            y: 22.0,
            r: 16.0,
            rot: 0.,
            jumping: false,
            start_y: 0.,
        }
    }

    fn get_center(&self) -> (f32, f32) {
        (self.x + self.r, self.y + self.r)
    }

    fn set_sprite(&self, name_of : &str) {
        sprite!(name_of,
        x = self.x - self.r,
        y = self.y - 16.0,
        rotation = self.rot,
       );
    }

    fn apply_gravity(&mut self) {
        self.y += 0.9;
        self.apply_jump();
    }

    fn jump(&mut self, state: bool){
        self.jumping = state;
        self.start_y = self.y;
    }

    fn apply_jump(&mut self){
        if !self.jumping {
            return;
        }

        if (self.start_y - self.y > 30.){
            self.jumping = false;
        } else {
            self.y -= 4.;
        }
        
    }


}




turbo::init! {
    struct GameState {
        frame: u32,
        pancakes: Vec<struct Pancake {
            x: f32,
            y: f32,
            vel: f32,
            radius: f32,
        }>,
        score: u32,
        birdy: Bird,
    } = Self {
        frame: 0,
        pancakes: vec![],
        score: 0,
        birdy: Bird::new(),
    }

}

turbo::go!({
    let mut state = GameState::load();
    state.frame += 1;

    state = check_movement(state);

    let cat_center = state.birdy.get_center();
    state.birdy.set_sprite("munch_cat");
    state.birdy.apply_gravity();


    for pancake in &state.pancakes {
        circ!(x = pancake.x , y = pancake.y, d = pancake.radius, color = 0xdba470ff)
    }

    state.save();
});


fn check_movement(mut state: GameState) -> GameState {
    if gamepad(0).left.pressed() {
        state.birdy.x -= 2.;
    }
    if gamepad(0).right.pressed() {
        state.birdy.x += 2.;
    }

    if gamepad(0).start.pressed(){
        state.birdy.jump(true);
    }
    return state;
}
