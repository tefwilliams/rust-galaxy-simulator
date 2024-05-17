use csv::Writer;
use std::time::Instant;

use crate::mass::MovingParticles;

mod accelerations;
mod galaxy;
mod helpers;
mod mass;
mod octree;

fn main() {
    const N_STEPS: usize = 100;
    const GALAXY_SIZE: u32 = 2_u32.pow(12);

    let mut particles = galaxy::create_galaxy(GALAXY_SIZE);

    let start = Instant::now();
    let points_record = (0..N_STEPS).map(|_| {
        particles = accelerations::update_particles_barnes_hut(
            &particles,
            10_f32.powi(-5),
            10_f32.powi(-4),
            1.0,
        );

        return particles.get_positions();
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
