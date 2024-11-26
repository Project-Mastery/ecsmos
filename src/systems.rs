use crate::{components::*, consts::*};
use bevy::{color::palettes::css::DARK_BLUE, math::vec2, prelude::*};

pub fn input_system(
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

pub fn motivation_force_system(
    mut agents: Query<(&mut Speed, &Transform), With<Agent>>,
    objectives: Query<&Transform, With<Objective>>,
) {
    let objective = objectives.get_single();

    if objective.is_err() {
        return;
    }

    let objective = objective.unwrap();

    for (mut speed, transform) in &mut agents {
        let mut direction = objective.translation - transform.translation;

        direction.z = 0.;
        direction = direction.normalize() * AGENT_MAX_SPEED;

        speed.0 += Vec2::new(direction.x, direction.y);
    }
}

pub fn agent_araived_at_destination_system(mut commands: Commands, agents: Query<(Entity, &Transform), With<Agent>>, destinations: Query<(&Transform, &Colider), With<Objective>>){
    for (agent, agent_transform) in &agents{
        let agent_position = agent_transform.translation;

        for (dest_transform, dest_colider) in &destinations{
            let destination_pos = dest_transform.translation;

            match dest_colider {
                Colider::Circle(radius) => {
                    let distance = (destination_pos - agent_position).length() - radius;
                    if distance <= 0. {
                        commands.entity(agent).despawn();
                    }
                },
                _ => todo!()
            }
            
        }
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
            let distance = -(dv.length() - 50. - AGENT_RADIUS);

            dv.z = 0.;
            dv = dv.normalize() * (distance / 50.).exp();
            let dv: Vec2 = Vec2::new(dv.x, dv.y) * 1.5;
            speed.0 += dv;

            let start = Vec2::new(transform.translation.x, transform.translation.y);

            gizmos.arrow_2d(
                start,
                start + dv * 20.0,
                DARK_BLUE,
            );
        }
    }
}

pub fn agent_max_speed_system(mut agents: Query<&mut Speed, With<Agent>>) {
    for mut speed in &mut agents {
        speed.0 = speed.0.clamp_length_max(AGENT_MAX_SPEED);
    }
}

pub fn start_speed_system(mut agents: Query<&mut Speed, With<Agent>>) {
    for mut speed in &mut agents {
        speed.0 = vec2(0.0, 0.0);
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