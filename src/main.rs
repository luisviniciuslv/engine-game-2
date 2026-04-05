use bevy::window::{PresentMode, WindowMode, WindowResolution};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Meu Jogo Dinâmico".into(),
                // 1. Defina a resolução física que a JANELA terá ao abrir.
                // Usar valores reais ajuda o driver a alocar memória corretamente.
                resolution: WindowResolution::new(1280.0, 720.0 ),
                // 2. BorderlessFullscreen vai expandir para 2560x1080 no seu monitor
                // de forma segura, pois ele já tem uma base de 1080p iniciada.
                mode: bevy::window::WindowMode::Windowed,//BorderlessFullscreen(MonitorSelection::Primary),
                // 3. PresentMode::AutoVsync ajuda a evitar o erro de "Device Lost" 
                // sincronizando os frames com o monitor.
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default()) // Mostra as linhas das colisões
        .add_systems(Startup, (setup, setup_physics))
        .add_systems(Update, (player_input, toggle_fullscreen))
        .run();
}

#[derive(Component)]
struct Player {
}

fn setup(
    mut commands: Commands,
) {
    let cam = Camera2d::default();
    commands.spawn(cam);
}

fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
   mut query: Query<&mut Velocity, With<Player>>,
) {
    for mut vel in query.iter_mut() {
        let speed = 400.0;
        
        // Movimento Horizontal
        if keys.pressed(KeyCode::KeyA) {
            vel.linvel.x = -speed;
        } else if keys.pressed(KeyCode::KeyD) {
            vel.linvel.x = speed;
        } else {
            vel.linvel.x = 0.0;
        }

        // Pulo
        if keys.just_pressed(KeyCode::KeyW) && vel.linvel.y.abs() < 0.1 {
            vel.linvel.y = 270.0;
        }
    }
}

fn setup_physics(mut commands: Commands) {
    /* --- CHÃO --- */
    commands.spawn((
        // Um corpo 'Fixed' não se move com a gravidade
        RigidBody::Fixed, 
        // Define o tamanho da caixa de colisão do chão
        Collider::cuboid(1000.0, 20.0), 
        Transform::from_xyz(0.0, -300.0, 0.0),
    ));

    /* --- PLAYER --- */
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.2, 0.8),
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        RigidBody::Dynamic, // Sofre gravidade e forças
        Collider::cuboid(32.0, 32.0), // Metade do tamanho visual (raio)
        // Impede que o player saia girando como uma bola ao bater nas quinas
        LockedAxes::ROTATION_LOCKED, 
        Player {},
        Velocity::zero(),
    ));
}

fn toggle_fullscreen(
    keys: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
) {
    if keys.just_pressed(KeyCode::F11) {
        let mut window = window_query.single_mut();
        
        if window.mode == WindowMode::Windowed {
            // Ajusta a resolução ANTES de mudar o modo
            window.resolution.set(2560.0, 1080.0); 
            window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
        } else {
            window.mode = WindowMode::Windowed;
            window.resolution.set(1280.0, 720.0);
        }
    }
}