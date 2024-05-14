use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{helpers::Vector3D, octree::generate_node};

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

            let new_vel = acceleration.multiply_by(time_step).add(vel);
            let new_pos = new_vel.multiply_by(time_step).add(pos);

            return (new_pos, new_vel);
        })
        .unzip();
}

pub fn get_new_positions_barnes_hut(
    positions: &Vec<[f32; 3]>,
    velocities: &Vec<[f32; 3]>,
    time_step: f32,
    softening_factor: f32,
    theta: f32,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>) {
    let size = positions
        .iter()
        .max_by(|pos1, pos2| pos1.magnitude().total_cmp(&pos2.magnitude()))
        .unwrap()
        .magnitude();

    let octree = generate_node(size, &[0.0, 0.0, 0.0], &positions);

    return positions
        .into_par_iter()
        .zip(velocities)
        .map(|(pos, vel)| {
            let acceleration = octree.get_acceleration(&pos, softening_factor, theta);

            let new_vel = acceleration.multiply_by(time_step).add(vel);
            let new_pos = new_vel.multiply_by(time_step).add(pos);

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

            return acc.add(&get_acceleration_due_to_object(
                position,
                // TODO - pass real masses in
                &(1.0, *other_pos),
                softening_factor,
            ));
        });
}

pub fn get_acceleration_due_to_object(
    position: &[f32; 3],
    object: &(f32, [f32; 3]),
    softening_factor: f32,
) -> [f32; 3] {
    let (object_mass, object_position) = object;

    let [dx, dy, dz] = object_position.subtract(position);

    let dr_squared = dx * dx + dy * dy + dz * dz + softening_factor * softening_factor;
    let dr_power_n32 = 1_f32 / (dr_squared.sqrt() * dr_squared);

    return [dx, dy, dz].multiply_by(object_mass / dr_power_n32);
}
