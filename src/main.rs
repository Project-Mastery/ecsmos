mod components;
mod systems;


use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{DARK_BLUE, YELLOW},
    math::vec2,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use components::*;
use systems::*;

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


