use bevy::{color::palettes::basic::BLUE, prelude::*};

use bevy::window::{
    CursorGrabMode, CursorIcon, CursorOptions, PresentMode, SystemCursorIcon, WindowLevel,
    WindowTheme,
};

pub struct LaunchPlugin;

impl Plugin for LaunchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_people, spawn_triangle))
            .insert_resource(GreetTimer(Timer::from_seconds(1., TimerMode::Once)))
            .add_systems(Update, (rotate).chain());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
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

    let a = meshes.add(Triangle2d::new(
        Vec2::new(250., 0.),
        Vec2::new(5., 0.),
        Vec2::new(2.5, -250.),
    ));

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

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);
