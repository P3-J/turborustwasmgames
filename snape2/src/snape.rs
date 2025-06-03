use turbo::borsh::*;
use turbo::prelude::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]

pub struct Snape {
    head_pos: (i32, i32),
    tail_last_pos: (i32, i32),
    body: Vec<(i32, i32)>,
}

impl Snape {

    pub fn new(h_pos: (i32, i32)) -> Self {
        let h_pos: (i32, i32) = (h_pos.0, h_pos.1);
        Snape {
            head_pos: h_pos,
            tail_last_pos: (0, 0),
            body: vec![],
        }
    }

    pub fn draw(&mut self) {
        rect!(
            x = self.head_pos.0,
            y = self.head_pos.1,
            w = 1,
            h = 1,
            color = 0xebff99ff,
        );

        for body in self.body.iter(){
            rect!(
                x = body.0,
                y = body.1,
                w = 1,
                h = 1,
            )
        }
    }

    pub fn move_snape(&mut self, dir: (i32, i32)) -> bool {
        self.tail_last_pos = (self.head_pos.0, self.head_pos.1);
        if self.body.len() > 0{
            self.move_body_parts();
        } 
        self.head_pos.0 = (self.head_pos.0 + dir.0 + 9) % 9;
        self.head_pos.1 = (self.head_pos.1 + dir.1 + 9) % 9;
        
        // check for collision
        for body in self.body.iter(){
            if body == &self.head_pos {
                return true;
            }
        }
        return false;
    }

    pub fn increase_size(&mut self){
        self.body.push(self.tail_last_pos);
        
    }

    fn move_body_parts(&mut self){
        let mut last_pos: (i32, i32) = (self.head_pos.0, self.head_pos.1);
        for body in self.body.iter_mut() {
            let my_pos: (i32, i32) = *body;
            *body = last_pos;
             last_pos = (my_pos.0, my_pos.1);
        }
        self.tail_last_pos = self.body[0];
    }

    pub fn get_head_pos(&mut self) -> (i32, i32) {
        return self.head_pos
    }

}

