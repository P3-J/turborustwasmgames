use bevy::{
    color::palettes::basic::BLUE,
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};

use bevy::window::{
    CursorGrabMode, CursorIcon, CursorOptions, PresentMode, SystemCursorIcon, WindowLevel,
    WindowTheme,
};

use crate::food::{Food, FoodPlugin};

mod food;

pub struct LaunchPlugin;
impl Plugin for LaunchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_people, spawn_triangle))
            .insert_resource(GreetTimer(Timer::from_seconds(0.1, TimerMode::Once)))
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
                resizable: false,
                visible: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FoodPlugin)
        .add_plugins(LaunchPlugin)
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("jk".to_string())));
}

fn spawn_triangle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2d, Transform::default()));

    let a = meshes.add(Rectangle::new(62.5, 62.5));

    commands.spawn((
        Mesh2d(a),
        MeshMaterial2d(mats.add(Color::from(BLUE))),
        Transform::default(),
    ));
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
        &trs.rotate_z(0.5);
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

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);
