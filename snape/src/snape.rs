use turbo::borsh::*;
use turbo::prelude::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]

pub struct Snape {
    head_pos: (i32, i32),
    body: Vec<(i32, i32)>
}

impl Snape {

    pub fn new(h_pos: (i32, i32)) -> Self {
        let h_pos: (i32, i32) = (h_pos.0, h_pos.1);
        Snape {
            head_pos: h_pos,
            body: vec![h_pos],
        }
    }

    pub fn draw(&mut self) {
        rect!(
            x = self.head_pos.0,
            y = self.head_pos.1,
            w = 1,
            h = 1,
        )
    }

    pub fn move_snape(&mut self, dir: (i32, i32)) {
        self.head_pos.0 = (self.head_pos.0 + dir.0 + 9) % 9;
        self.head_pos.1 = (self.head_pos.1 + dir.1 + 9) % 9;

    }

}

