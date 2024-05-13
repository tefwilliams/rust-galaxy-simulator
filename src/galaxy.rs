use crate::{accelerations::get_acceleration, helpers::Vector3D};
use rand::thread_rng;
use rand_distr::{Distribution, Exp, Normal, Uniform};
use std::f64::consts::PI;

const SCALE_FACTOR: f64 = 1.0;

pub fn create_galaxy(galaxy_size: u32) -> (Vec<[f32; 3]>, Vec<[f32; 3]>) {
    // Sets positions for points
    let positions = create_positions(galaxy_size);

    // Finds the initial velocity of each point
    let velocities = initialize_velocities(&positions);

    return (positions, velocities);
}

fn create_positions(galaxy_size: u32) -> Vec<[f32; 3]> {
    let mut rng = thread_rng();

    let radial_generator = Exp::new(SCALE_FACTOR).unwrap();
    let angular_generator = Uniform::new(0.0, 2.0 * PI);

    // Generates radial and angular components in 2D with provided
    // distributions and converts them to Cartesian coordinates
    let cartesian = (0..galaxy_size)
        .map(|_| {
            (
                radial_generator.sample(&mut rng) as f32,
                angular_generator.sample(&mut rng) as f32,
            )
        })
        .map(|(r, p)| (r * p.cos(), r * p.sin()));

    // Adds 'noise' to all 3D to generate points
    let mut rng = thread_rng();
    let normal_generator = Normal::new(0.0, SCALE_FACTOR / 5.0).unwrap();

    return cartesian
        .map(|(x, y)| {
            [
                normal_generator.sample(&mut rng) as f32 + x,
                normal_generator.sample(&mut rng) as f32 + y,
                normal_generator.sample(&mut rng) as f32,
            ]
        })
        .collect();
}

fn initialize_velocities(positions: &Vec<[f32; 3]>) -> Vec<[f32; 3]> {
    return positions
        .into_iter()
        .map(|pos| {
            // Gets initial acceleration of the point
            let acceleration = get_acceleration(pos, positions, 0.0);

            // Calculates the speed of particle for circular motion
            let velocity_magnitude = {
                let position_magnitude = pos.magnitude();
                let acceleration_magnitude = acceleration.magnitude();

                (position_magnitude * acceleration_magnitude).sqrt()
            };

            // Calculates unit vectors in 3D
            let velocity_direction = {
                let mut rng = thread_rng();
                let normal_generator = Normal::new(0.0, 0.1).unwrap();

                [
                    -acceleration[1],
                    acceleration[0],
                    acceleration[2] * normal_generator.sample(&mut rng) as f32,
                ]
            };

            // Generates velocities, correcting for magnitude
            // change due to addition of z-component
            let normalisation = velocity_direction.magnitude();
            return velocity_direction.multiply_by(velocity_magnitude / normalisation);
        })
        .collect();
}
