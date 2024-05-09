use std::time::Instant;
mod accelerations;
mod galaxy;

fn main() {
    const N_STEPS: usize = 1000;

    let (velocities, positions) = {
        // Sets positions for points
        let positions = create_positions();

        // Finds the initial velocity of each point
        let velocities = initialize_velocities(positions);

        return (positions, velocities);
    };

    let points_record = [0; N_STEPS];

    let time_step = 10_f32.powi(-4);
    let softening_factor = 10_f32.powi(-4);

    let start = Instant::now();
    for i in 0..N_STEPS {
        points_record[i] =
            update_positions(positions, velocities, time_step, softening_factor).flatten()
    }

    let runtime = start.elapsed();

    print!(
        "
    
Time to run: {runtime:.3g}s
Time per iteration: {runtime / n_steps:.3g}s

Number of particles: {n_particles}
Number of iterations: {n_steps}
    ",
    );
}
