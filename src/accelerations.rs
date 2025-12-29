use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    helpers::Vector3D,
    mass::{MovingParticle, Particle},
    octree::Octree,
    // octree::generate_node,
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
                particle.particle.position,
                particles
                    .iter()
                    .map(|p| p.particle)
                    .collect::<Vec<Particle>>()
                    .as_slice(),
                softening_factor,
            );

            let new_vel = acceleration.multiply_by(time_step).add(particle.velocity);
            let new_pos = new_vel
                .multiply_by(time_step)
                .add(particle.particle.position);

            (new_pos, new_vel)
        })
        .unzip()
}

pub fn update_particles_barnes_hut(
    particles: &mut [MovingParticle],
    time_step: f32,
    softening_factor: f32,
    theta: f32,
) {
    let size = particles
        .iter()
        .map(|p| p.particle.position.magnitude())
        .max_by(|pos1, pos2| pos1.total_cmp(pos2))
        .unwrap();

    let octree = Octree::build(
        particles
            .iter()
            .map(|p| p.particle)
            .collect::<Vec<Particle>>()
            .as_slice(),
        [0.0, 0.0, 0.0],
        size,
    );

    // for node in octree.nodes.iter() {
    //     println!(
    //         "Node size: {}, first_child_index: {}, particle mass: {}",
    //         node.size, node.first_child_index, node.particle.mass
    //     );
    // }
    //  generate_node(
    //     size,
    //     &[0.0, 0.0, 0.0],
    //     particles
    //         .iter()
    //         .map(|p| p.particle)
    //         .collect::<Vec<Particle>>()
    //         .as_slice(),
    // );

    particles.par_iter_mut().for_each(|p| {
        let acceleration = octree.get_acceleration(0, p.particle.position, softening_factor, theta);

        // Update velocity and position directly
        p.velocity = acceleration.multiply_by(time_step).add(p.velocity);
        p.particle.position = p.velocity.multiply_by(time_step).add(p.particle.position);
    });
}

pub fn get_acceleration(
    position: [f32; 3],
    particles: &[Particle],
    softening_factor: f32,
) -> [f32; 3] {
    particles.iter().fold([0_f32, 0_f32, 0_f32], |acc, mass| {
        acc.add(get_acceleration_due_to_particle(
            position,
            mass,
            softening_factor,
        ))
    })
}

pub fn get_acceleration_due_to_particle(
    position: [f32; 3],
    particle: &Particle,
    softening_factor: f32,
) -> [f32; 3] {
    // TODO - will this work?
    if particle.position == position {
        return [0.0, 0.0, 0.0];
    }

    let distance = particle.position.subtract(position);

    let dr_squared = distance.self_dot() + softening_factor.powi(2);
    let dr_power_n32 = dr_squared.sqrt() * dr_squared;

    distance.multiply_by(particle.mass / dr_power_n32)
}
