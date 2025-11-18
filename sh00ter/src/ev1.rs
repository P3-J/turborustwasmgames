use turbo::borsh::*;
use turbo::prelude::*;

use crate::bullet::Bullet;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]

// spacecraft script

pub struct Ev1 {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Ev1 {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            w: 16,
            h: 32,
        }
    }

    pub fn state_action(&mut self) {
        self.draw();
    }

    pub fn draw(&self) {
        sprite!(
            "ev1",
            x = self.x,
            y = self.y,
            w = self.w,
            h = self.h,
            rotation = 90,
        );
        // debug rect
        return;
        rect!(
            w = 32,
            h = 16,
            x = self.x - 8,
            y = self.y + 7,
            color = 0x1301ffff,
        );
        
    }

    pub fn move_dir(&mut self, pos: (i8, i8)) {

        let f_x = self.x + pos.0 as i32;
        let f_y = self.y + pos.1 as i32;

        if f_x <= 5 || f_x >= 140 {
            return;
        }

        self.x = f_x;
        self.y = f_y;
    }

    pub fn get_state(&self) ->  (i32, i32, i32, i32) {
        // with hardcoded offset from rect rotation
        return (
            self.x - 8,
            self.y + 7,
            self.w,
            self.h,
        )
    }

    pub fn move_pos(&mut self, pos: (i32, i32)) {
        self.x = pos.0;
        self.y = pos.1;
    }

    pub fn shoot(&self) -> Bullet {
        Bullet::new(self.x + 8, self.y + 10)
    }

}
