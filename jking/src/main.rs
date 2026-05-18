use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::BLUE,
    ecs::relationship::RelationshipSourceCollection,
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};

use bevy::window::{
    CursorGrabMode, CursorIcon, CursorOptions, PresentMode, SystemCursorIcon, WindowLevel,
    WindowTheme,
};
use bevy_flycam::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_trenchbroom::{anyhow, class::QuakeClassSpawnView, prelude::*};

use crate::{
    food::{Food, FoodPlugin},
    scoreboard::ScoreboardPlugin,
};

mod food;
mod scoreboard;

pub struct LaunchPlugin;
impl Plugin for LaunchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_triangle)
            .insert_resource(GreetTimer(Timer::from_seconds(0.1, TimerMode::Once)))
            .insert_resource(WorldSize { x: 500.0, y: 500.0 })
            .register_type::<SpawnType>()
            .add_systems(Update, move_player)
            .add_systems(Update, (rotate).chain());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am gaming!".into(),
                name: Some("bevy.app".into()),
                resolution: (500, 500).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                resizable: true,
                visible: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TrenchBroomPlugins(
            TrenchBroomConfig::new("catNapper")
                .default_solid_scene_hooks(|| SceneHooks::new().convex_collider()),
        ))
        .add_plugins(FoodPlugin)
        .add_plugins(LaunchPlugin)
        .add_plugins(ScoreboardPlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PlayerPlugin)
        .add_systems(Update, ball_query)
        .run();
}

fn spawn_triangle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    //commands.spawn((Camera2d, Transform::default()));
    /*  commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
    )); */

    let a = meshes.add(Rectangle::new(62.5, 62.5));

    commands.spawn((
        Mesh2d(a),
        MeshMaterial2d(mats.add(Color::from(BLUE))),
        Transform::default(),
    ));

    /*   commands.insert_resource(GlobalAmbientLight {
        color: BLUE.into(),
        brightness: 200.0,
        ..default()
    }); */

    commands.spawn((
        PointLight {
            intensity: 100_000.0,
            color: BLUE.into(),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
    ));

    commands.spawn(SceneRoot(asset_server.load("maps/test.map#Scene")));
}

fn rotate(
    mut gtimer: ResMut<GreetTimer>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Mesh2d>>,
) {
    gtimer.0.tick(time.delta());

    if !gtimer.0.is_finished() {
        return;
    } else {
        gtimer.0.reset();
    }

    for mut trs in &mut query {
        let _ = &trs.rotate_z(0.5);
    }
}

fn move_player(
    time: Res<Time>,
    mut query: Single<&mut Transform, (Without<Food>, With<Mesh2d>)>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if key.pressed(KeyCode::KeyA) {
        query.translation.x -= 300.0 * time.delta_secs();
    }
    if key.pressed(KeyCode::KeyD) {
        query.translation.x += 300.0 * time.delta_secs();
    }
    if key.pressed(KeyCode::KeyW) {
        query.translation.y += 300.0 * time.delta_secs();
    }
    if key.pressed(KeyCode::KeyS) {
        query.translation.y -= 300.0 * time.delta_secs();
    }
}

#[point_class(
    hooks(SceneHooks::new().push(Self::trying_to_spawn_ball))
)]
struct BallSpawnPoint;
impl BallSpawnPoint {
    pub fn trying_to_spawn_ball(view: &mut QuakeClassSpawnView) -> anyhow::Result<()> {
        let transform = *view.world.get::<Transform>(view.entity).unwrap();

        view.world.commands().spawn((
            SpawnType {
                sp_enum: SpawnShape::Ball,
            },
            transform,
        ));

        Ok(())
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct SpawnType {
    sp_enum: SpawnShape,
}

#[derive(Reflect)]
enum SpawnShape {
    Ball,
    Square,
}

fn ball_query(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &SpawnType, &Transform), Added<SpawnType>>,
) {
    for (entity, spawn_type, transform) in &query {
        println!("got 1");
        let mesh = match spawn_type.sp_enum {
            SpawnShape::Ball => meshes.add(Sphere::new(1.0)),
            SpawnShape::Square => meshes.add(Cuboid::from_length(2.0)),
        };

        commands.entity(entity).insert((
            Mesh3d(mesh),
            RigidBody::Dynamic,
            Collider::sphere(1.0),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            GlobalTransform::from_xyz(
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
            ),
        ));
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Resource)]
struct WorldSize {
    x: f32,
    y: f32,
}
