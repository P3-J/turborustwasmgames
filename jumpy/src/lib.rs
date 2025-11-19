mod player;
mod objects;
use player::Player;
use objects::Blocker;
use turbo::{os::server::command::user_id, *};



#[turbo::game]
struct GameState {
    frame: u32,
    player: Player,
    blockers: Vec<Blocker>,
}

impl GameState {

    pub fn new() -> Self {
        Self{
            frame: 0,
            player: Player::new(),
            blockers: vec![],
        }
    }

    pub fn update(&mut self){
        text!("Hello, world!!!");

        if let Some(conn) = PingPongChannel::subscribe("default") { 
            while let Ok(_msg) = conn.recv() { 
                log!("Received pong from server!");
            } 

            if gamepad::get(0).start.just_pressed() { 
                let _ = conn.send(&Ping); 
                log!("Sent ping to the server!"); 
            } 
        } 
    }

}


#[turbo::serialize]
pub struct Ping;
 
#[turbo::serialize]
pub struct Pong;
 
#[turbo::os::channel(program = "pingpong", name = "main")]
pub struct PingPongChannel{
    players: Vec<String>,
}

impl ChannelHandler for PingPongChannel { 
    type Recv = Ping; // incoming from client
    type Send = String; // outgoing to client
    fn new() -> Self { 
        Self {
            players: vec![],
        }
        
    } 
    fn on_data(&mut self, user_id: &str, data: Self::Recv) -> Result<(), std::io::Error> { 
        log!("Got {:?} from {:?}", data, user_id); 
        Self::send(user_id, "test".to_string());
        Self::send(user_id, self.players.iter().count().to_string())
    } 

    fn on_connect(&mut self, _user_id: &str) -> Result<(), std::io::Error> {
        self.players.push(_user_id.to_string());
        Ok(())
    }

} 


/* 
turbo::go!({
    clear(0x40d1c8ff);
    let mut state = GameState::load();
    state.frame += 1;

    if state.frame % 120 == 0{
        state = gen_blocker(state);
    }

    let mut colliding = false;
    state.blockers.retain_mut(|p| {
        if p.x > 0{
            p.draw_self();
            let is_col = p.check_for_col((state.player.x + 10 , state.player.y + 10));
            if is_col {colliding = true;}
            return true
        }
        return false
    });

    state = check_movement(state);

    state.player.player_state_machine();
    draw_ground();


    if (colliding){
        state = GameState {
            frame: 0,
            player: Player::new(),
            blockers: vec![],
        }
    }

    state.save();
}); */

fn draw_ground() {
    rect! (
        w = 256,
        h = 30,
        x = 0,
        y = 114,
        color = 0x00F54927FF
    );
}

fn check_movement(mut state: GameState) -> GameState {
    let gp = gamepad::get(0);

    if gp.start.just_pressed(){
        state.player.jump(true);
    }
    return state;
}


fn gen_blocker(mut state: GameState) -> GameState {

    let obj: Blocker = Blocker::new( 120, 110, 10, 10);

    state.blockers.push(obj);

    return state;

}

