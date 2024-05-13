use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

pub fn get_new_positions(
    positions: &Vec<[f32; 3]>,
    velocities: &Vec<[f32; 3]>,
    time_step: f32,
    softening_factor: f32,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>) {
    return positions
        .into_par_iter()
        .zip(velocities)
        .map(|(pos, vel)| {
            let acceleration = get_acceleration(&pos, &positions, softening_factor);

            let new_vel = [
                vel[0] + acceleration[0] * time_step,
                vel[1] + acceleration[1] * time_step,
                vel[2] + acceleration[2] * time_step,
            ];

            let new_pos = [
                pos[0] + new_vel[0] * time_step,
                pos[1] + new_vel[1] * time_step,
                pos[2] + new_vel[2] * time_step,
            ];

            return (new_pos, new_vel);
        })
        .unzip();
}

pub fn get_acceleration(
    position: &[f32; 3],
    positions: &Vec<[f32; 3]>,
    softening_factor: f32,
) -> [f32; 3] {
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
