use turbo::*;

#[turbo::serialize]
pub struct Player {
    pub x: i8,
    pub y: i8,
    w: u8,
    h: u8,
    pub name: String,

    jumping: bool,
    jump_length: u8,
    jump_duration: u8,
}

impl Player {

    
    pub fn new() -> Self {
        Self {
            x: 30,
            y: 105,
            w: 10,
            h: 10,
            name: "xd".to_string(),
            
            jumping: false,
            jump_length: 60,
            jump_duration: 0,
        }
    }

    pub fn draw_self(&mut self) {
        let n = random::bool();

        rect! (
            x = self.x,
            y = self.y,
            w = self.w,
            h = self.h,
            color = if n { 0x006400FF } else {0x001400FF}
        )

    }

    pub fn player_state_machine(&mut self){

        self.draw_self();

        if self.jumping{
            self.jumping();
        } else {
            self.apply_gravity();
        }
    }

    pub fn jump(&mut self, state: bool){
        self.jumping = state;
        self.jump_duration= self.jump_length;
    }

    fn jumping(&mut self){
        if self.jump_duration <= 0 {
            self.jumping = false;
            return;
        }

        self.y -= 1;
        self.jump_duration -= 1;
    }

    fn apply_gravity(&mut self){
        if self.y <= 105{
            self.y += 1;
            return;
        }

    }


    
}
