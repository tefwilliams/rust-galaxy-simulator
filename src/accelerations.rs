use rayon::iter::{IntoParallelIterator, ParallelIterator};

// TODO - move to constant file
// also does this need to be a const?
pub const GALAXY_SIZE: usize = 2_usize.pow(12);

pub type Vector3D = [f32; 3];

pub fn get_new_positions(
    positions: &mut Vec<Vector3D>,
    velocities: &mut Vec<Vector3D>,
    time_step: f32,
    softening_factor: f32,
) -> Vec<Vector3D> {
    return (0..GALAXY_SIZE)
        .map(|i| {
            let acceleration = get_acceleration(&positions[i], &positions, softening_factor);

            velocities[i][0] += acceleration[0] * time_step;
            velocities[i][1] += acceleration[1] * time_step;
            velocities[i][2] += acceleration[2] * time_step;

            positions[i][0] += velocities[i][0] * time_step;
            positions[i][1] += velocities[i][1] * time_step;
            positions[i][2] += velocities[i][2] * time_step;

            return positions[i];
        })
        .collect();
}

pub fn get_acceleration(
    position: &Vector3D,
    positions: &Vec<Vector3D>,
    softening_factor: f32,
) -> Vector3D {
    return positions
        .into_iter()
        .fold([0_f32, 0_f32, 0_f32], |acc, other_pos| {
            if other_pos == position {
                return acc;
            }

            let dx = other_pos[0] - position[0];
            let dy = other_pos[1] - position[1];
            let dz = other_pos[2] - position[2];

            let dr_squared = dx * dx + dy * dy + dz * dz + softening_factor * softening_factor;
            let dr_power_n32 = 1_f32 / (dr_squared.sqrt() * dr_squared);

            return [
                acc[0] + dx * dr_power_n32,
                acc[1] + dy * dr_power_n32,
                acc[2] + dz * dr_power_n32,
            ];
        });
}
