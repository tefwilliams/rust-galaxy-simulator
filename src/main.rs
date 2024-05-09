use std::time::Instant;
mod accelerations;
mod galaxy;

const GALAXY_SIZE: usize = accelerations::GALAXY_SIZE;

fn main() {
    const N_STEPS: usize = 1000;

    let (velocities, positions) = galaxy::create_galaxy();

    let mut points_record = Box::new([[[0_f32; 3]; accelerations::GALAXY_SIZE]; N_STEPS]);

    let time_step = 10_f32.powi(-4);
    let softening_factor = 10_f32.powi(-4);

    let start = Instant::now();
    for i in 0..N_STEPS {
        let new_positions =
            accelerations::get_new_positions(&positions, &velocities, time_step, softening_factor);

        for j in 0..accelerations::GALAXY_SIZE {
            for k in 0..3 {
                points_record[i][j][k] = new_positions[j][k];
            }
        }
    }

    let runtime = start.elapsed();
    let seconds = runtime.as_secs();

    print!(
        "
    
Time to run: {seconds}s
Number of particles: {GALAXY_SIZE}
Number of iterations: {N_STEPS}
    ",
    );
}
