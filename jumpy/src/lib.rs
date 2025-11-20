mod player;
mod objects;
use player::Player;
use objects::Blocker;
use turbo::{encoding::b64::url_safe::decode, os::{client::channel::ChannelConnection, server::channel::broadcast}, *};



#[turbo::game]
struct GameState {
    frame: u32,
    players: Vec<Player>,
    blockers: Vec<Blocker>,
    playercount: i32,
    selfcount: i32,
    selfplayer: Player,
    firstjoin: bool,
    handshakewaiting: bool,
    event_start: u64,
    selfid: String,
}

impl GameState {

    pub fn new() -> Self {
        Self{
            frame: 0,
            players: vec![],
            blockers: vec![],
            playercount: 0,
            selfcount: 1,
            selfplayer: Player::new(),
            handshakewaiting: true,
            firstjoin: true,
            selfid: "".to_string(),
            event_start: time::now()
        }
    }

    pub fn update(&mut self){
        text!("Hello, world!!!");
        self.frame += 1;

        self.render_players();

        if let Some(my_id) = turbo::os::client::user_id() {

            if (self.handshakewaiting){
                self.selfid = my_id.clone();
                self.add_player(my_id); // add self once authenticated
                self.handshakewaiting = false;
            }
        }

        if let Some(conn) = PingPongChannel::subscribe("default") { 
            while let Ok(_msg) = conn.recv() { 
                self.decode_pong(_msg, &conn);
            } 
            
            if gamepad::get(0).start.just_pressed() { 
                self.check_movement(conn);
            } 

        } 
    }

    fn check_movement(&mut self, conn: ChannelConnection<Pong, Pong>){
        let gp = gamepad::get(0);

        //let _ = conn.send(&Ping); 
        //log!("Sent ping to the server!"); 

        if gp.start.just_pressed(){
            if let Some(me) = self.find_self() {
                me.jump(true);

                let mut sending= Pong {
                    actions: vec![]
                };

                sending.actions.push(Actions { action: actionTypes::jump, value: self.selfcount, text: self.selfid.to_string()});

                let _ = conn.send(&sending); 
            }
        }
    }


    fn find_self(&mut self) -> Option<&mut Player> {
        for p in self.players.iter_mut(){
            if p.name == self.selfid {
                return Some(p);
            }
        }
        None
    }

    fn decode_pong(&mut self, _msg: Pong, conn: &ChannelConnection<Pong, Pong>){
        for actions in _msg.actions{
            match actions.action {
                actionTypes::join => self.join_checks(actions, conn),
                actionTypes::jump => self.other_jump(actions),
                actionTypes::handshake => self.handshake_do(actions),
                _ => log!("uncovered")
            }
        }
    }

    fn other_jump(&mut self, act:Actions){
        
        if act.value == self.selfcount {
            return;
        }

        for p in self.players.iter_mut(){
            if (p.name == act.text){
                p.jump(true);
            }
        }
    }

    fn add_player(&mut self, id: String){
        
        self.playercount += 1;

        let mut newPlayer = Player::new();
        newPlayer.name = id;
        self.players.push(newPlayer);
    }

    fn join_checks(&mut self, act: Actions, conn: &ChannelConnection<Pong, Pong>){

        // self boot
        let mut sending= Pong {
            actions: vec![]
        };

        sending.actions.push(Actions { action: actionTypes::handshake, value: self.selfcount, text: act.text});
        let _ = conn.send(&sending);
        
    }

    fn handshake_do(&mut self, act: Actions){
        self.add_player(act.text);

        log!("got handshake from {}", self.selfid);
    }

    fn render_players(&mut self){

        for player in self.players.iter_mut(){
            player.player_state_machine();
        }

    }
}








#[turbo::serialize]
pub struct Pong {
    actions: Vec<Actions>,
}

#[turbo::serialize]
struct Actions {
    action: actionTypes,
    value: i32,
    text: String,
}

#[turbo::serialize]
enum actionTypes {
    log,
    jump,
    join,
    handshake,
}
 
#[turbo::os::channel(program = "pingpong", name = "main")]
pub struct PingPongChannel{
    players: Vec<String>,
}

impl ChannelHandler for PingPongChannel { 
    type Recv = Pong; // incoming from client
    type Send = Pong; // outgoing to client
    fn new() -> Self { 
        Self {
            players: vec![],
        }
        
    } 

    fn on_connect(&mut self, user_id: &str) -> Result<(), std::io::Error> {

        let mut sending= Pong {
            actions: vec![]
        };

        sending.actions.push(
            Actions { action: actionTypes::join, value: self.players.len() as i32, text: user_id.to_string()}
        );
        Self::send(user_id, sending)
    }

    fn on_data(&mut self, _user_id: &str, _data: Self::Recv) -> Result<(), std::io::Error> {
       
        let _= Self::broadcast(_data);
        log!("got from {}", _user_id);
        return Ok(());
    }

} 


fn handshake_all_clients(cnl :PingPongChannel){
    return;
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



fn gen_blocker(mut state: GameState) -> GameState {

    let obj: Blocker = Blocker::new( 120, 110, 10, 10);

    state.blockers.push(obj);

    return state;

}

