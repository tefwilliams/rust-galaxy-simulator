use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    helpers::Vector3D,
    mass::{MovingParticle, MovingParticles, Particle},
    octree::generate_node,
};

pub fn get_new_positions(
    particles: &Vec<MovingParticle>,
    time_step: f32,
    softening_factor: f32,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>) {
    return particles
        .into_par_iter()
        .map(|particle| {
            let acceleration = get_acceleration(
                &particle.particle.position,
                &particles.to_particles(),
                softening_factor,
            );

            let new_vel = acceleration.multiply_by(time_step).add(&particle.velocity);
            let new_pos = new_vel
                .multiply_by(time_step)
                .add(&particle.particle.position);

            return (new_pos, new_vel);
        })
        .unzip();
}

pub fn update_particles_barnes_hut(
    particles: &Vec<MovingParticle>,
    time_step: f32,
    softening_factor: f32,
    theta: f32,
) -> Vec<MovingParticle> {
    let size = particles
        .iter()
        .map(|p| p.particle.position.magnitude())
        .max_by(|pos1, pos2| pos1.total_cmp(pos2))
        .unwrap();

    let octree = generate_node(size, &[0.0, 0.0, 0.0], &particles.to_particles());

    return particles
        .into_par_iter()
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
    particles: &Vec<Particle>,
    softening_factor: f32,
) -> [f32; 3] {
    return particles
        .into_iter()
        .fold([0_f32, 0_f32, 0_f32], |acc, mass| {
            // TODO - will this work?
            if mass.position == *position {
                return acc;
            }

            return acc.add(&get_acceleration_due_to_mass(
                position,
                mass,
                softening_factor,
            ));
        });
}

pub fn get_acceleration_due_to_mass(
    position: &[f32; 3],
    mass: &Particle,
    softening_factor: f32,
) -> [f32; 3] {
    let [dx, dy, dz] = mass.position.subtract(position);

    let dr_squared = dx * dx + dy * dy + dz * dz + softening_factor * softening_factor;
    let dr_power_n32 = 1_f32 / (dr_squared.sqrt() * dr_squared);

    return [dx, dy, dz].multiply_by(mass.mass / dr_power_n32);
}
