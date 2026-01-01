use csv::Writer;
use std::time::Instant;

use crate::{
    mass::{MovingParticles, Particle},
    octree::Octree,
};

mod accelerations;
mod galaxy;
mod helpers;
mod mass;
mod octree;

fn main() {
    // let particles = vec![
    //     Particle {
    //         mass: 1.0,
    //         position: [5.0, 5.0, 5.0],
    //     },
    //     Particle {
    //         mass: 1.0,
    //         position: [-5.0, -5.0, -5.0],
    //     },
    //     Particle {
    //         mass: 1.0,
    //         position: [6.0, 6.0, 6.0],
    //     },
    // ];

    // let tree = Octree::build(&particles, [0.0, 0.0, 0.0], 10.0);

    // println!("Tree nodes: {}", tree.nodes.len());

    // for node in tree.nodes.iter() {
    //     println!(
    //         "Node size: {}, first_child_index: {}, particle mass: {}",
    //         node.size, node.first_child_index, node.particle.mass
    //     );
    // }

    const GALAXY_SIZE: u32 = 2_u32.pow(14);
    const N_RECORDS: usize = 1000;
    const STEPS_PER_RECORD: usize = 10;
    const N_STEPS: usize = N_RECORDS * STEPS_PER_RECORD;

    let mut particles = galaxy::create_galaxy(GALAXY_SIZE);

    // for p in particles.iter() {
    //     println!(
    //         "Mass: {}, Position: ({}, {}, {})",
    //         p.particle.mass, p.particle.position[0], p.particle.position[1], p.particle.position[2]
    //     );
    // }

    // println!("Initial energy: {}", mass::get_total_energy(&particles));

    let start = Instant::now();
    let points_record = (0..N_RECORDS).map(|step| {
        for _ in 0..STEPS_PER_RECORD {
            accelerations::update_particles_barnes_hut(&mut particles, 1e-6, 1e-3, 1.0);
        }

        println!("Completed step {} of {}", (step + 1), N_RECORDS);

        // let energy = mass::get_total_energy(&particles);
        // println!("Total energy: {}", energy);

        particles.as_slice().get_positions()
    });

    let mut wtr = Writer::from_path("data/points_record.csv").unwrap();

    for record in points_record {
        wtr.serialize(record).ok();
    }

    let runtime = start.elapsed();
    let seconds = runtime.as_millis();

    print!(
        "
Time to run: {seconds}ms
Number of particles: {GALAXY_SIZE}
Number of iterations: {N_STEPS}

",
    );
}
