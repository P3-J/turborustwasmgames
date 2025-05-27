use turbo::borsh::*;
use turbo::prelude::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub rot: f32,
    pub jumping: bool,
    pub start_y: f32,
}

impl Bird {
    pub fn new() -> Self {
        Self {
            x: 128.0,
            y: 22.0,
            r: 16.0,
            rot: 0.,
            jumping: false,
            start_y: 0.,
        }
    }
    pub fn set_sprite(&self, name_of : &str) {
        sprite!(name_of,
        x = self.x - self.r,
        y = self.y - 16.0,
        rotation = self.rot,
       );
    }

    pub fn apply_gravity(&mut self) {
        self.y += 0.9;
        self.apply_jump();
    }

    pub fn jump(&mut self, state: bool){
        self.jumping = state;
        self.start_y = self.y;
    }

    fn apply_jump(&mut self){
        if !self.jumping {
            return;
        }

        if self.start_y - self.y > 30.{
            self.jumping = false;
        } else {
            self.y -= 4.;
        }
        
    }


}
