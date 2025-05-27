use turbo::borsh::*;
use turbo::prelude::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Pipe {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub rot: f32,
}

impl Pipe {

    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            x: x,
            y: y,
            w: w,
            h: h,
            rot: 0.,
        }
    }

    pub fn draw_self(&mut self) {
        rect! (
            w = self.w,
            h = self.h,
            x = self.x,
            y = self.y,
            color = 0x006400FF
        );
        self.x -= 1.;
    }

    pub fn check_for_col(&mut self, pos: (f32, f32)) -> bool {
        if pos.0 >= self.x && pos.0 <= self.x + self.w {
            if pos.1 >= self.y && pos.1 <= self.y + self.h {
                
                return true;
            };
        };
        
        return false;
    }
}
