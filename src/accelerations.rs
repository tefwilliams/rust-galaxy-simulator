use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    helpers::Vector3D,
    mass::{MovingParticle, MovingParticles, Particle},
    octree::generate_node,
};

pub fn get_new_positions(
    particles: &[MovingParticle],
    time_step: f32,
    softening_factor: f32,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>) {
    particles
        .into_par_iter()
        .map(|particle| {
            let acceleration = get_acceleration(
                &particle.particle.position,
                particles.to_particles().as_slice(),
                softening_factor,
            );

            let new_vel = acceleration.multiply_by(time_step).add(&particle.velocity);
            let new_pos = new_vel
                .multiply_by(time_step)
                .add(&particle.particle.position);

            return (new_pos, new_vel);
        })
        .unzip()
}

pub fn update_particles_barnes_hut(
    particles: &[MovingParticle],
    time_step: f32,
    softening_factor: f32,
    theta: f32,
) -> Vec<MovingParticle> {
    let size = particles
        .iter()
        .map(|p| p.particle.position.magnitude())
        .max_by(|pos1, pos2| pos1.total_cmp(pos2))
        .unwrap();

    let octree = generate_node(size, &[0.0, 0.0, 0.0], particles.to_particles().as_slice());

    return particles
        .par_iter()
        .map(|p| {
            let acceleration =
                octree.get_acceleration(&p.particle.position, softening_factor, theta);

            let new_vel = acceleration.multiply_by(time_step).add(&p.velocity);
            let new_pos = new_vel.multiply_by(time_step).add(&p.particle.position);

            return MovingParticle {
                particle: Particle {
                    position: new_pos,
                    ..p.particle
                },
                velocity: new_vel,
            };
        })
        .collect();
}

pub fn get_acceleration(
    position: &[f32; 3],
    particles: &[&Particle],
    softening_factor: f32,
) -> [f32; 3] {
    return particles
        .into_iter()
        .fold([0_f32, 0_f32, 0_f32], |acc, mass| {
            // TODO - will this work?
            if mass.position == *position {
                return acc;
            }

            return acc.add(&get_acceleration_due_to_particle(
                position,
                mass,
                softening_factor,
            ));
        });
}

pub fn get_acceleration_due_to_particle(
    position: &[f32; 3],
    particle: &Particle,
    softening_factor: f32,
) -> [f32; 3] {
    let distance = particle.position.subtract(position);

    let dr_squared = distance.self_dot() + softening_factor.powi(2);
    let dr_power_n32 = dr_squared.sqrt() * dr_squared;

    return distance.multiply_by(particle.mass / dr_power_n32);
}
