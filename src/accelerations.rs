// TODO - move to constant file
const GALAXY_SIZE: usize = 2_usize.pow(15);

type Vector = [f32; 3];
type GalaxyVector = [Vector; GALAXY_SIZE];

pub fn get_new_positions(
    positions: &mut GalaxyVector,
    velocities: &mut GalaxyVector,
    time_step: f32,
    softening_factor: f32,
) -> GalaxyVector {
    let accelerations = get_accelerations(positions, softening_factor);

    for i in 0..GALAXY_SIZE {
        for j in 0..3 {
            velocities[i][j] += accelerations[i][j] * time_step;
            positions[i][j] += velocities[i][j] * time_step;
        }
    }

    return *positions;
}

fn get_accelerations(positions: &GalaxyVector, softening_factor: f32) -> GalaxyVector {
    let mut accelerations = [[0_f32; 3]; GALAXY_SIZE];

    for i in 0..GALAXY_SIZE {
        for j in 0..GALAXY_SIZE {
            if i == j {
                continue;
            };

            let dx = positions[j][0] - positions[i][0];
            let dy = positions[j][1] - positions[i][1];
            let dz = positions[j][2] - positions[i][2];

            let dr_squared = dx * dx + dy * dy + dz * dz + softening_factor * softening_factor;
            let dr_power_n32 = 1_f32 / (dr_squared.sqrt() * dr_squared);

            accelerations[i][0] += dx * dr_power_n32;
            accelerations[i][1] += dy * dr_power_n32;
            accelerations[i][2] += dz * dr_power_n32;
        }
    }

    return accelerations;
}
