use turbo::borsh::*;
use turbo::prelude::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]

pub struct Bullet {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Bullet {

    pub fn new(X: i32, Y: i32) -> Self {
        Self {
            x: X,
            y: Y,
            w: 2,
            h: 2,
        }
    }

    pub fn draw_self(&mut self) {
        self.y -= 1;
        rect!(
            x = self.x,
            y = self.y,
            h = self.h,
            w = self.w
        )
    }

    pub fn get_state(&self) ->  (i32, i32, i32, i32) {
        return (
            self.x,
            self.y,
            self.h,
            self.w,
        )
    }
}
