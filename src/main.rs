use csv::Writer;
use std::time::Instant;

use crate::mass::MovingParticles;

mod accelerations;
mod galaxy;
mod helpers;
mod mass;
mod octree;

fn main() {
    const GALAXY_SIZE: u32 = 2_u32.pow(14);
    const N_RECORDS: usize = 100;
    const STEPS_PER_RECORD: usize = 10;
    const N_STEPS: usize = N_RECORDS * STEPS_PER_RECORD;

    let mut particles = galaxy::create_galaxy(GALAXY_SIZE);

    let start = Instant::now();
    let points_record = (0..N_RECORDS).map(|_| {
        for _ in 0..STEPS_PER_RECORD {
            particles = accelerations::update_particles_barnes_hut(
                &particles,
                10_f32.powi(-6),
                10_f32.powi(-5),
                1.0,
            );
        }

        return particles.get_positions();
    });

    let mut wtr = Writer::from_path("data/points_record.csv").unwrap();

    for record in points_record {
        wtr.serialize(record).ok();
    }

    let energy_record = (0..N_RECORDS).map(|_| {
        for _ in 0..STEPS_PER_RECORD {
            particles = accelerations::update_particles_barnes_hut(
                &particles,
                10_f32.powi(-8),
                10_f32.powi(-5),
                1.0,
            );
        }

        return mass::get_total_energy(&particles);
    });

    let mut egy_wtr = Writer::from_path("data/energy_record.csv").unwrap();

    for record in energy_record {
        egy_wtr.serialize(record).ok();
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
