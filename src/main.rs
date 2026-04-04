use bevy::{color::palettes::basic::PURPLE, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_input, apply_physics).chain()) 
        .run();
}

#[derive(Component)]
struct Player {
    velocity: Vec2,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(128.)),
        Player { velocity: Vec2::ZERO },
    ));
}

fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &Transform)>,
) {
    let Ok((mut player, transform)) = query.single_mut() else { return; };
    let speed = 400.0;
    let jump_force = 350.0;

    // Movimento Horizontal
    let mut x_dir = 0.0;
    if keys.pressed(KeyCode::KeyA) { x_dir -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { x_dir += 1.0; }
    player.velocity.x = x_dir * speed;

    // Pulo (apenas se estiver no chão)
    if keys.just_pressed(KeyCode::KeyW) && transform.translation.y <= 0.0 {
        player.velocity.y = jump_force;
    }
}

fn apply_physics(
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let dt = time.delta_secs();
    let gravity = -1500.0;

    for (mut player, mut transform) in query.iter_mut() {
        // 1. Aplica gravidade à velocidade
        if transform.translation.y > 0.0 || player.velocity.y > 0.0 {
            player.velocity.y += gravity * dt;
        } else {
            player.velocity.y = 0.0;
            transform.translation.y = 0.0;
        }

        // 2. Aplica a velocidade à posição
        transform.translation.x += player.velocity.x * dt;
        transform.translation.y += player.velocity.y * dt;

        // 3. Trava no chão
        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
            player.velocity.y = 0.0;
        }
    }
}