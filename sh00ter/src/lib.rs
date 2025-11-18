mod ev1;
mod enemybase;
mod bullet;

turbo::init! {
    struct GameState {
        frame: u32,
        init_boot: bool,
        craft: ev1::Ev1,
        enemies: Vec<enemybase::Enemy>,
        bullets: Vec<bullet::Bullet>,
    } = Self {
        frame: 0,
        init_boot: true,
        craft: ev1::Ev1::new(),
        enemies: vec![],
        bullets: vec![],
    }
}



turbo::go!({
    let mut state = GameState::load();
    state.frame += 1;

    if state.init_boot {
        _init_start_up(&mut state);
    }

    state.bullets.retain_mut(|p| {
        p.draw_self();
        let state = p.get_state();

        if state.1 < 40{
            return false;
        }
        return true
    });

    // enemy col and drawing
    state.enemies.retain_mut(|p| {
        p.draw_self();
        let mut colliding = false;

        for bullet in &state.bullets {
            let bul_state = bullet.get_state();
            colliding = check_if_colliding(
                p.x, p.y, p.w, p.h,
                bul_state.0, bul_state.1, bul_state.2, bul_state.3
            );
            if colliding {
                break
            }
        }
        return !colliding
    });

    check_for_input(&mut state);
    state.craft.state_action();

    state.save();

});

fn _init_start_up(state: &mut GameState) {
   state.craft.move_pos((80,50));
   state.init_boot = false;

   state.enemies.push(enemybase::Enemy::new());
}


fn check_for_input(gm: &mut GameState) {

    if gamepad(0).left.pressed() {
        gm.craft.move_dir((-1, 0));
    }
    if gamepad(0).right.pressed(){
        gm.craft.move_dir((1, 0));
    }
    if gamepad(0).up.pressed() {
        gm.craft.move_dir((0, -1));
    }
    if gamepad(0).down.pressed(){
        gm.craft.move_dir((0, 1));
    }

    if gamepad(0).a.just_pressed() {
        let bul = gm.craft.shoot();
        gm.bullets.push(bul);
    }
}

fn check_if_colliding(
    x1: i32, y1: i32, w1: i32, h1: i32,
    x2: i32, y2: i32, w2: i32, h2: i32
    ) -> bool {
        !(x1 + w1 < x2 || x1 > x2 + w2 || y1 + h1 < y2 || y1 > y2 + h2)
}
