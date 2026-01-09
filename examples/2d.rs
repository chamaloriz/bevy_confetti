use bevy::prelude::*;

use bevy_confetti::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, ConfettiPlugin));
    app.add_systems(Startup, setup);
    app.add_systems(Update, spawn_confetties);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_confetties(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        commands.trigger(SpawnConfetti {
            count: 500,
            colors: vec!["#07C8F9", "#09A6F3", "#0A85ED", "#0C63E7", "#0D41E1"],
        });
    }
}
