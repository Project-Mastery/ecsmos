use crate::{components::*, consts::*};
use bevy::{
    color::palettes::css::{BLUE, DARK_BLUE, DARK_RED, GREEN, PURPLE, RED, YELLOW},
    math::{vec2, vec3},
    prelude::*, transform,
};

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
    mut agents: Query<(&mut MotivationForce, &Transform, &Speed), With<Agent>>,
    objectives: Query<&Transform, With<Objective>>,
) {
    let objective = objectives.get_single();

    if objective.is_err() {
        return;
    }

    let objective = objective.unwrap();

    for (mut motivation_force, transform, agent_speed) in &mut agents {
        let direction = (objective.translation - transform.translation)
            .with_z(0.)
            .normalize()
            * AGENT_DESIRED_SPEED;

        let final_force =
            AGENT_DESIRED_SPEED * direction - vec3(agent_speed.0.x, agent_speed.0.y, 0.);

        motivation_force.0 = Vec2::new(final_force.x, final_force.y);
    }
}

pub fn agent_araived_at_destination_system(
    mut commands: Commands,
    agents: Query<(Entity, &Transform), With<Agent>>,
    destinations: Query<(&Transform, &Colider), With<Objective>>,
) {
    for (agent, agent_transform) in &agents {
        let agent_position = agent_transform.translation;

        for (dest_transform, dest_colider) in &destinations {
            let destination_pos = dest_transform.translation;

            match dest_colider {
                Colider::Circle(radius) => {
                    let distance = (destination_pos - agent_position).length() - radius;
                    if distance <= 0. {
                        commands.entity(agent).despawn();
                    }
                }
                _ => todo!(),
            }
        }
    }
}

pub fn obstacle_force(
    mut agents: Query<(&mut ObstacleForce, &Transform), With<Agent>>,
    obstacles: Query<&Transform, With<Obstacle>>,
) {

    for (mut force, _)in &mut agents{
        force.0 = vec2(0., 0.)
    }
    
    for (mut obstacle_force, agent_transform) in &mut agents {
        for obstacle_transform in &obstacles {
            let effective_distance = (obstacle_transform.translation.with_z(0.)
                - agent_transform.translation.with_z(0.))
            .length()
                - AGENT_RADIUS
                - 50.;
            let effective_distance = effective_distance / PIXELS_PER_METER;

            let a = 2000.;
            let b = 0.08;
            let k = 120000.;
            let kappa = 240000.;
            let g = 0.;

            let n = (agent_transform.translation - obstacle_transform.translation)
                .with_z(0.)
                .normalize();
            let t = Vec3::new(-n.y, n.x, 0.);

            let repulsive_factor = a * (-effective_distance / b).exp();
            let contact_factor = k * g * effective_distance;

            let pushing_force = (repulsive_factor + contact_factor) * n;
            let sliding_force = kappa * g * effective_distance * t;

            let final_force = pushing_force + sliding_force;

            obstacle_force.0 += Vec2::new(final_force.x, final_force.y);
        }
    }
}

pub fn apply_repulsive_forces(mut agents: Query<(&mut RepulsiveForce, &Transform), With<Agent>>) {
    
    for (mut force, _)in &mut agents{
        force.0 = vec2(0., 0.)
    }
    
    let mut combinations = agents.iter_combinations_mut();

    let a = 2000.;
    let b = 0.08;
    let k = 120000.;
    let kappa = 240000.;
    let g = 0.;

    while let Some([(mut force_1, transform_1), (mut force_2, transform_2)]) = combinations.fetch_next() {

        let pixel_distance = (transform_2.translation - transform_1.translation).with_z(0.).length() - 2. * AGENT_RADIUS;
        let effective_distance = (pixel_distance / PIXELS_PER_METER);


        let n = (transform_1.translation - transform_2.translation)
        .with_z(0.)
        .normalize();

        let t = Vec3::new(-n.y, n.x, 0.);

        let repulsive_factor = a * (-effective_distance / b).exp();
        let contact_factor = k * g * effective_distance;

        let pushing_force = (repulsive_factor + contact_factor) * n;
        let sliding_force = kappa * g * effective_distance * t;

        let final_force = pushing_force + sliding_force;

        force_1.0 += Vec2::new(final_force.x, final_force.y);
        force_2.0 += -Vec2::new(final_force.x, final_force.y);
    }
}

pub fn agent_max_speed_system(mut agents: Query<&mut Speed, With<Agent>>) {
    for mut speed in &mut agents {
        speed.0 = speed.0.clamp_length_max(AGENT_DESIRED_SPEED);
    }
}

pub fn start_speed_system(mut agents: Query<&mut Speed, With<Agent>>) {
    for mut speed in &mut agents {
        speed.0 = vec2(0.0, 0.0);
    }
}

pub fn apply_social_foces(
    mut agents: Query<(&mut Speed, &ObstacleForce, &MotivationForce, &RepulsiveForce), With<Agent>>,
) {
    for (mut agent_speed, obstacle_force, motivation_force, repulsive_force) in &mut agents {
        let previous_speed = agent_speed.0.clone();

        agent_speed.0 = previous_speed + motivation_force.0 + (obstacle_force.0 + repulsive_force.0) / AGENT_MASS;
    }
}

pub fn show_social_forces(
    mut gizmos: Gizmos,
    mut agents: Query<(&Transform, &ObstacleForce, &MotivationForce, &RepulsiveForce, &Speed), With<Agent>>,
) {
    for (agent_transform, obstacle_force, motivation_force, repulsive_force, agent_speed) in &mut agents {
        let start = Vec2::new(agent_transform.translation.x, agent_transform.translation.y);

        gizmos.arrow_2d(
            start,
            start + Vec2::new(obstacle_force.0.x, obstacle_force.0.y),
            BLUE,
        );

        let effective_motivation_force = (motivation_force.0 + agent_speed.0) * AGENT_MASS;

        gizmos.arrow_2d(
            start,
            start + Vec2::new(effective_motivation_force.x, effective_motivation_force.y),
            RED,
        );

         
        let final_force = obstacle_force.0 + effective_motivation_force + repulsive_force.0;

        gizmos.arrow_2d(
            start,
            start + Vec2::new(final_force.x, final_force.y),
            GREEN,
        );

        gizmos.arrow_2d(
            start,
            start + Vec2::new(repulsive_force.0.x, repulsive_force.0.y),
            PURPLE,
        );
    }
}
