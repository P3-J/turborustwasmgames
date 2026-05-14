use bevy::prelude::*;
use rand::Rng;

use crate::WorldSize;

const FOOD_SIZE: Vec2 = Vec2::new(25.0, 25.0);
const FOOD_COLOR: Color = Color::srgb(0.74117647, 0.57647059, 0.97647059);

#[derive(Component)]
pub struct Food {
    size: Vec2,
}

#[derive(Message)]
struct FoodEaten;

pub struct FoodPlugin;
impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, gen_food);
        app.add_systems(Update, (check_for_collision, gen_food));
        app.add_message::<FoodEaten>();
    }
}

fn random_pos(world_size: &WorldSize) -> Vec3 {
    let mut rng = rand::rng();
    let x = rng.random_range(-world_size.x / 2.0..world_size.x / 2.0);
    let y = rng.random_range(-world_size.y / 2.0..world_size.y / 2.0);
    Vec3::new(x, y, 0.0)
}

fn gen_food(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<ColorMaterial>>,
    world_size: Res<WorldSize>,
    mut messages: MessageReader<FoodEaten>,
) {
    for _ in messages.read() {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(FOOD_SIZE.x, FOOD_SIZE.y))),
            MeshMaterial2d(mats.add(FOOD_COLOR)),
            Transform::from_translation(random_pos(&world_size)),
            Food {
                size: Vec2::new(FOOD_SIZE.x, FOOD_SIZE.y),
            },
        ));
    }
}

fn check_for_collision(
    mut commands: Commands,
    p: Single<&mut Transform, (Without<Food>, With<Mesh2d>)>,
    mut foods: Query<(Entity, &Transform, &Food)>,
    mut eaten: MessageWriter<FoodEaten>,
) {
    for (entity, trans, food) in &mut foods {
        if (&p.translation.x - &trans.translation.x).abs() < (62.5 + &food.size.x) / 2.0
            && (&p.translation.y - &trans.translation.y).abs() < (62.5 + &food.size.y) / 2.0
        {
            commands.entity(entity).despawn();
            eaten.write(FoodEaten);
        }
    }
}
