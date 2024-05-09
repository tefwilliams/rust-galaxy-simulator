// TODO - move to constant file
const GALAXY_SIZE: usize = 2_usize.pow(15);

type Vector = [f32; 3];
type GalaxyVector = [Vector; GALAXY_SIZE];

pub fn get_new_positions(
    positions: &GalaxyVector,
    velocities: &GalaxyVector,
    time_step: f32,
    softening_factor: f32,
) -> GalaxyVector {
    let accelerations = get_accelerations(positions, softening_factor);

    for i in 0..velocities.len() {
        for j in 0..3 {
            velocities[i][j] += accelerations[i][j] * time_step;
            positions[i][j] += velocities[i][j] * time_step;
        }
    }

    return positions;
}

fn get_accelerations(positions: &GalaxyVector, softening_factor: f32) -> GalaxyVector {
    // for position in positions {
    //     dx = positions[j, 0] - positions[i, 0]
    //     dy = positions[j, 1] - positions[i, 1]
    //     dz = positions[j, 2] - positions[i, 2]
    //     drSquared = dx * dx + dy * dy + dz * dz + softening_factor * softening_factor
    //     drPowerN32 = nb.float32(1.0) / (math.sqrt(drSquared) * drSquared)
    //     accelerations[i, 0] += dx * drPowerN32
    //     accelerations[i, 1] += dy * drPowerN32
    //     accelerations[i, 2] += dz * drPowerN32
    // }

    return positions;
}
