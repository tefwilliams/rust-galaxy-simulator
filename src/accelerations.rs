use rayon::iter::{IntoParallelIterator, ParallelIterator};

// TODO - move to constant file
// also does this need to be a const?
pub const GALAXY_SIZE: usize = 2_usize.pow(9);

pub type Vector3D = [f32; 3];

pub fn get_new_positions(
    positions: &Vec<Vector3D>,
    velocities: &Vec<Vector3D>,
    time_step: f32,
    softening_factor: f32,
) -> Vec<Vector3D> {
    return (0..GALAXY_SIZE)
        .into_par_iter()
        .map(|i| {
            let acceleration = get_acceleration(i, &positions, softening_factor);

            return [
                (velocities[i][0] + acceleration[0] * time_step) * time_step,
                (velocities[i][1] + acceleration[1] * time_step) * time_step,
                (velocities[i][2] + acceleration[2] * time_step) * time_step,
            ];
        })
        .collect();
}

pub fn get_acceleration(body: usize, positions: &Vec<Vector3D>, softening_factor: f32) -> Vector3D {
    let postion = positions[body];

    return positions
        .into_iter()
        .fold([0_f32, 0_f32, 0_f32], |acc, other_pos| {
            let dx = other_pos[0] - postion[0];
            let dy = other_pos[1] - postion[1];
            let dz = other_pos[2] - postion[2];

            let dr_squared = dx * dx + dy * dy + dz * dz + softening_factor * softening_factor;
            let dr_power_n32 = 1_f32 / (dr_squared.sqrt() * dr_squared);

            return [
                acc[0] + dx * dr_power_n32,
                acc[1] + dy * dr_power_n32,
                acc[2] + dz * dr_power_n32,
            ];
        });
}
