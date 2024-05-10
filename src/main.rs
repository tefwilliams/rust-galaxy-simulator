use csv::Writer;
use std::time::Instant;
mod accelerations;
mod galaxy;

const GALAXY_SIZE: usize = accelerations::GALAXY_SIZE;

fn main() {
    const N_STEPS: usize = 1;

    let (mut positions, mut velocities) = galaxy::create_galaxy();

    let start = Instant::now();
    let points_record = (0..N_STEPS).map(|_| {
        accelerations::get_new_positions(
            &mut positions,
            &mut velocities,
            10_f32.powi(-4),
            0_f32.powi(-4),
        )
    });

    let mut wtr = Writer::from_path("points_record.csv").unwrap();

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
