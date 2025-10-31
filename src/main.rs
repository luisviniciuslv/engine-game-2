use bevy::{color::palettes::basic::PURPLE, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_input)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform::default().with_scale(Vec3::splat(128.)), Player
    ));
}


fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let speed = 300.0; // pixels por segundo
    let mut dir = 0.0;
    if keys.pressed(KeyCode::KeyA) {
        dir -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        dir += 1.0;
    }

    if dir != 0.0 {
        let dt = time.delta_secs();
        for mut transform in query.iter_mut() {
            transform.translation.x += dir * speed * dt;
        }
    }
}

fn pysichs_system(
    time: Res<Time>,
    mut query: Query<&Transform, With<Player>>,
) {
    let y_pos = query.single().translation.y;
}
