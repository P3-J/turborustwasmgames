use turbo::borsh::*;
use turbo::prelude::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]

// spacecraft script

pub struct Ev1 {
    x: i32,
    y: i32,
}

impl Ev1 {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }

    pub fn draw(&self) {
        sprite!(
            "ev1",
            x = self.x,
            y = self.y,
            w = 16,
            h = 32,
            rotation = 90,
        );
        // debug rect
        rect!(
            w = 2,
            h = 2,
            x = self.x + 8,
            y = self.y + 16,
            color = 0x1301ffff
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
     pub fn move_pos(&mut self, pos: (i32, i32)) {
        self.x = pos.0;
        self.y = pos.1;
    }

}
