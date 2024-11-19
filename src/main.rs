use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{DARK_BLUE, YELLOW},
    math::vec2,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, motivation_force)
        .add_systems(Update, obstacle_force)
        .add_systems(Update, agent_max_speed.before(velocity_sytem))
        .add_systems(Update, velocity_sytem.after(move_player))
        .add_systems(Update, draw_repulsion_forces);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(MaxAgentSpeed(4.));

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Agent,
        Speed(vec2(0., 0.)),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
            material: materials.add(Color::hsl(360. * 0.6, 0.95, 0.7)),
            transform: Transform::from_xyz(-300.0, 50.0, 0.1),
            ..default()
        },
    ));

    // commands.spawn((
    //     Objective,
    //     MaterialMesh2dBundle {
    //         mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
    //         material: materials.add(Color::hsl(360. * 0.9, 0.95, 0.7)),
    //         transform: Transform::from_xyz(300.0, 0.0, 0.0),
    //         ..default()
    //     },
    // ));

    let mut t = Transform::from_xyz(0.0, 0.0, -0.5);
    t.rotate_z(1.0 / 4.0 * PI);

    commands.spawn((
        Obstacle,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
            material: materials.add(Color::srgb(0., 0., 0.)),
            transform: t,
            ..default()
        },
    ));

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(1000.0, 700.0))),
        material: materials.add(Color::srgb(214., 219., 223.)),
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..default()
    });
}

#[derive(Component)]
struct Agent;

#[derive(Component)]
struct Speed(Vec2);

#[derive(Component)]
struct Objective;

#[derive(Component)]
struct Obstacle;

#[derive(Resource)]
struct MaxAgentSpeed(f32);

fn move_player(
    mut transforms: Query<&mut Transform, With<Agent>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut t in transforms.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            direction.y += 1.0
        }
        if keys.pressed(KeyCode::KeyS) {
            direction.y -= 1.0
        }
        if keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0
        }
        if keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0
        }

        if direction.length() > 0.0 {
            t.translation += 6.0 * direction.normalize()
        }
    }
}

fn velocity_sytem(mut query: Query<(&mut Transform, &Speed), With<Agent>>) {
    for (mut t, Speed(speed)) in &mut query {
        t.translation += Vec3::new(speed.x, speed.y, 0.);
    }
}

fn motivation_force(
    mut agents: Query<(&mut Speed, &Transform), With<Agent>>,
    objectives: Query<&Transform, With<Objective>>,
) {
    let objective = objectives.get_single();

    if objective.is_err() {
        return;
    }

    let objective = objective.unwrap();

    for (mut speed, transform) in &mut agents {
        let mut dv = objective.translation - transform.translation;

        dv.z = 0.;
        dv = dv.normalize() * 0.5;

        speed.0 += Vec2::new(dv.x, dv.y);
    }
}

fn obstacle_force(
    mut gizmos: Gizmos,
    mut agents: Query<(&mut Speed, &Transform), With<Agent>>,
    obstacles: Query<&Transform, With<Obstacle>>,
) {
    for (mut speed, transform) in &mut agents {
        for obstacle in &obstacles {
            let mut dv = transform.translation - obstacle.translation;
            let distance = -(dv.length() - 50.);

            dv.z = 0.;
            dv = dv.normalize() * (distance / 10.0).exp();

            speed.0 += Vec2::new(dv.x, dv.y);

            let start = Vec2::new(transform.translation.x, transform.translation.y);

            let vec2: Vec2 = Vec2::new(dv.x, dv.y);
            gizmos.arrow_2d(
                start,
                start + vec2 * 5.0,
                DARK_BLUE,
            );
        }
    }
}

fn agent_max_speed(max_speed: Res<MaxAgentSpeed>, mut agents: Query<&mut Speed, With<Agent>>) {
    for mut speed in &mut agents {
        speed.0 = speed.0.clamp_length_max(max_speed.0);
    }
}

fn draw_repulsion_forces(
    mut gizmos: Gizmos,
    agents: Query<&Transform, With<Agent>>,
    obstacles: Query<&Transform, With<Obstacle>>,
) {
    // for transform in &agents {
    //     for obstacle in &obstacles {
    //         let start = Vec2::new(transform.translation.x, transform.translation.y);
    //         let vector3 = (transform.translation - obstacle.translation).normalize() * 100.;
    //         let vec2: Vec2 = Vec2::new(vector3.x, vector3.y);
    //         gizmos.arrow_2d(
    //             start,
    //             start + vec2,
    //             DARK_BLUE,
    //         );
    //     }
    // }
}
