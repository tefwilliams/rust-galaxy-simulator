use crate::{
    accelerations::{get_acceleration, Vector3D},
    GALAXY_SIZE,
};
use randomkit::{
    dist::{Exponential, Normal, Uniform},
    Rng, Sample,
};
use std::f64::consts::PI;

const SCALE_FACTOR: f64 = 1_f64;

pub fn create_galaxy() -> (Vec<Vector3D>, Vec<Vector3D>) {
    // Sets positions for points
    let positions = create_positions();

    // Finds the initial velocity of each point
    let velocities = initialize_velocities(&positions);

    return (positions, velocities);
}

fn create_positions() -> Vec<Vector3D> {
    let mut rng1 = Rng::from_seed(1);

    let radial_generator = Exponential::new(SCALE_FACTOR).unwrap();
    let angular_generator = Uniform::new(0_f64, 2_f64 * PI).unwrap();

    // Generates radial and angular components in 2D with provided
    // distributions and converts them to Cartesian coordinates
    let cartesian = (0..GALAXY_SIZE)
        .map(|_| {
            (
                radial_generator.sample(&mut rng1) as f32,
                angular_generator.sample(&mut rng1) as f32,
            )
        })
        .map(|(r, p)| (r * p.cos(), r * p.sin()));

    // Adds 'noise' to all 3D to generate points
    let mut rng2 = Rng::from_seed(2);

    let normal_generator = Normal::new(0_f64, SCALE_FACTOR / 5_f64).unwrap();
    return cartesian
        .map(|(x, y)| {
            [
                normal_generator.sample(&mut rng2) as f32 + x,
                normal_generator.sample(&mut rng2) as f32 + y,
                normal_generator.sample(&mut rng2) as f32,
            ]
        })
        .collect();
}

fn initialize_velocities(positions: &Vec<Vector3D>) -> Vec<Vector3D> {
    return (0..GALAXY_SIZE)
        .map(|i| {
            // Gets initial acceleration of the point
            let acceleration = get_acceleration(i, positions, 0_f32);

            // Calculates the speed of particle for circular motion
            let position_magnitude: f32 = positions[i].iter().sum();
            let acceleration_magnitude: f32 = acceleration.iter().sum();
            let velocity_magnitude = (position_magnitude * acceleration_magnitude).sqrt();

            // Calculates unit vectors in 3D
            let mut rng = Rng::from_seed(1);
            let normal_generator = Normal::new(0_f64, 0.1).unwrap();

            let velocity_direction = [
                -acceleration[1],
                acceleration[0],
                normal_generator.sample(&mut rng) as f32,
            ];

            // Adds normalisation for correct speed (due to addition of z-component)
            let normalisation: f32 = velocity_direction.iter().sum();

            // Generates velocities
            return [
                velocity_magnitude * velocity_direction[0] / normalisation,
                velocity_magnitude * velocity_direction[1] / normalisation,
                velocity_magnitude * velocity_direction[2] / normalisation,
            ];
        })
        .collect();
}
