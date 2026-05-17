use bevy::prelude::*;

#[derive(Resource)]
pub struct Scoreboard {
    pub score: usize,
}

#[derive(Component)]
pub struct ScoreText;

pub struct ScoreboardPlugin;
impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scoreboard { score: 0 });
        app.add_systems(Startup, create_highscore_text);
        app.add_systems(Update, update_score);
    }
}

fn create_highscore_text(mut commands: Commands, score: Res<Scoreboard>) {
    commands.spawn((
        Text2d::new(score.score.to_string()),
        ScoreText,
        Transform::from_xyz(0.0, 150.0, 0.0),
    ));
}

fn update_score(score: Res<Scoreboard>, mut query: Single<&mut Text2d, With<ScoreText>>) {
    query.0 = score.score.to_string();
}
