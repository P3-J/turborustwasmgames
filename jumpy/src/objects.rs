use turbo::borsh::*;
use turbo::prelude::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Blocker {
    pub x: i8,
    pub y: i8,
    pub w: i8,
    pub h: i8,
}

impl Blocker {

    pub fn new(x: i8, y: i8, w: i8, h: i8) -> Self {
        Self {
            x: x,
            y: y,
            w: w,
            h: h,
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

        self.x -= 1;
    }

    pub fn check_for_col(&mut self, pos: (i8, i8)) -> bool {
        if pos.0 >= self.x && pos.0 <= self.x + self.w {
            if pos.1 >= self.y && pos.1 <= self.y + self.h {
                
                return true;
            };
        };
        return false;
    }
}
