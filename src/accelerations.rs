
// TODO - move to constant file
const GALAXY_SIZE: usize = 2_usize.pow(15);


pub fn get_new_positions(positions: [f32; (3, GALAXY_SIZE)], velocities: [f32; (3, GALAXY_SIZE)], time_step: f32, softening_factor: f32) -> [i32; (3, GALAXY_SIZE)] {
    let accelerations = get_accelerations(positions, softening_factor);

    let velocities += accelerations * time_step;
    let positions += velocities * time_step;

    return positions
}


fn get_accelerations(positions: [f32; (3, GALAXY_SIZE)], softening_factor: f32) -> [f32; (3, GALAXY_SIZE)] {
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

    return positions
}