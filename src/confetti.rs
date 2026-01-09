use bevy::prelude::*;

#[derive(Component)]
struct Confetti {
    x_rotation: f32,
    y_rotation: f32,
    z_rotation: f32,
    y_speed: f32,
}

#[derive(Event)]
pub struct SpawnConfetti {
    pub count: i32,
    pub colors: Vec<&'static str>,
}

pub struct ConfettiPlugin;

impl Plugin for ConfettiPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_confetti);
        app.add_systems(Update, (rotation, fall, cleanup));
    }
}

fn spawn_confetti(
    confettis: On<SpawnConfetti>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let width = window.width();
    let height = window.height();

    let half_width = width / 2.0;

    for _ in 0..confettis.count {
        let x = rand::random::<f32>() * width - half_width;
        let y = rand::random::<f32>() * 200.0 + (height / 2.0);
        let random_index = rand::random_range(0..confettis.colors.len());
        let hex_code = confettis.colors[random_index];

        commands.spawn((
            Confetti {
                x_rotation: rand::random::<f32>() * 10.0 - 7.0,
                y_rotation: rand::random::<f32>() * 10.0 - 7.0,
                z_rotation: rand::random::<f32>() * 10.0 - 7.0,
                y_speed: rand::random::<f32>() * (250.0 - 140.0) + 140.0,
            },
            Mesh2d(meshes.add(Rectangle::new(5.0, 10.0))),
            MeshMaterial2d(materials.add(Color::from(Srgba::hex(hex_code).unwrap()))),
            Transform::from_xyz(x, y, 0.0),
        ));
    }
}

fn rotation(mut query: Query<(&mut Transform, &Confetti)>, time: Res<Time>) {
    for (mut transform, confetti) in &mut query {
        transform.rotate_y(time.delta_secs() / confetti.y_rotation);
        transform.rotate_x(time.delta_secs() / confetti.x_rotation);
        transform.rotate_z(time.delta_secs() / confetti.z_rotation);
    }
}

fn fall(mut query: Query<(&mut Transform, &Confetti)>, time: Res<Time>) {
    for (mut transform, confetti) in &mut query {
        transform.translation.y -= confetti.y_speed * time.delta_secs();
    }
}

fn cleanup(
    mut commands: Commands,
    window: Single<&Window>,
    query: Query<(Entity, &mut Transform), With<Confetti>>,
) {
    let window_height = window.height();

    for (entity, transform) in query.iter() {
        if (window_height / 2.0 + transform.translation.y) <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
