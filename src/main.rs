use csv::Writer;
use std::time::Instant;

mod accelerations;
mod galaxy;
mod helpers;
mod octree;

fn main() {
    const N_STEPS: usize = 1000;
    const GALAXY_SIZE: u32 = 2_u32.pow(12);

    let (mut positions, mut velocities) = galaxy::create_galaxy(GALAXY_SIZE);

    let start = Instant::now();
    let points_record = (0..N_STEPS).map(|_| {
        (positions, velocities) = accelerations::get_new_positions_barnes_hut(
            &positions,
            &velocities,
            10_f32.powi(-5),
            10_f32.powi(-4),
            1.0,
        );

        return positions.to_vec();
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
