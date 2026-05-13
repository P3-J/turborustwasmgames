use bevy::prelude::*;

const FOOD_SIZE: Vec2 = Vec2::new(25.0, 25.0);
const FOOD_COLOR: Color = Color::srgb(0.74117647, 0.57647059, 0.97647059);

#[derive(Component)]
pub struct Food {
    size: Vec2,
}

pub struct FoodPlugin;
impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, gen_food);
        app.add_systems(Update, check_for_collision);
    }
}

fn gen_food(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(FOOD_SIZE.x, FOOD_SIZE.y))),
        MeshMaterial2d(mats.add(FOOD_COLOR)),
        Transform::from_xyz(150.0, 150.0, 0.0),
        Food {
            size: Vec2::new(FOOD_SIZE.x, FOOD_SIZE.y),
        },
    ));
}

fn check_for_collision(
    mut commands: Commands,
    p: Single<&mut Transform, (Without<Food>, With<Mesh2d>)>,
    mut foods: Query<(Entity, &Transform, &Food)>,
) {
    for (entity, trans, food) in &mut foods {
        if (&p.translation.x - &trans.translation.x).abs() < (62.5 + &food.size.x) / 2.0
            && (&p.translation.y - &trans.translation.y).abs() < (62.5 + &food.size.y) / 2.0
        {
            commands.entity(entity).despawn();
        }
    }
}
