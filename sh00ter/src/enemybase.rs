use turbo::borsh::*;
use turbo::prelude::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]


pub struct Enemy {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    color: u32,
}


impl Enemy {

    pub fn new() -> Self {
        Self {
            x: 20,
            y: 70,
            w: 20,
            h: 20,
            color: 0x301050ff,
        }
    }

    pub fn draw_self(&self) {
        rect!(
            x = self.x,
            y = self.y,
            w = self.w,
            h = self.h,
            color = self.color,
        )
    }

}


