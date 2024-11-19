use crate::components::*;
use bevy::{color::palettes::css::DARK_BLUE, prelude::*};

pub fn move_player(
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

pub fn velocity_sytem(mut query: Query<(&mut Transform, &Speed), With<Agent>>) {
    for (mut t, Speed(speed)) in &mut query {
        t.translation += Vec3::new(speed.x, speed.y, 0.);
    }
}

pub fn motivation_force(
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

pub fn obstacle_force(
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

pub fn agent_max_speed(max_speed: Res<MaxAgentSpeed>, mut agents: Query<&mut Speed, With<Agent>>) {
    for mut speed in &mut agents {
        speed.0 = speed.0.clamp_length_max(max_speed.0);
    }
}

pub fn draw_repulsion_forces(
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