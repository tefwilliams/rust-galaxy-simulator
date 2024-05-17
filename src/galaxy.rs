use crate::{
    accelerations::get_acceleration,
    helpers::Vector3D,
    mass::{MovingParticle, Particle},
};
use rand::thread_rng;
use rand_distr::{Distribution, Exp, Normal, Poisson, Uniform};
use std::f64::consts::PI;

const SCALE_FACTOR: f64 = 1.0;

pub fn create_galaxy(galaxy_size: u32) -> Vec<MovingParticle> {
    // Sets positions for points
    let particles = create_particles(galaxy_size);

    // Finds the initial velocity of each point
    return initialize_velocities(&particles);
}

fn create_particles(galaxy_size: u32) -> Vec<Particle> {
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

    let mut rng = thread_rng();
    let mass_generator = Poisson::new(0.5).unwrap();

    // Adds 'noise' to all 3D to generate points
    let noise_generator = Normal::new(0.0, SCALE_FACTOR / 5.0).unwrap();

    return cartesian
        .map(|(x, y)| Particle {
            mass: mass_generator.sample(&mut rng) as f32,
            position: [
                noise_generator.sample(&mut rng) as f32 + x,
                noise_generator.sample(&mut rng) as f32 + y,
                noise_generator.sample(&mut rng) as f32,
            ],
        })
        .collect();
}

fn initialize_velocities(particles: &Vec<Particle>) -> Vec<MovingParticle> {
    return particles
        .into_iter()
        .map(|particle| {
            // Gets initial acceleration of the point
            let acceleration = get_acceleration(&particle.position, particles, 0.0);

            // Calculates the speed of particle for circular motion
            let velocity_magnitude = {
                let position_magnitude = particle.position.magnitude();
                let acceleration_magnitude = acceleration.magnitude();

                (position_magnitude * acceleration_magnitude).sqrt()
            };

            // Calculates unit vectors in 3D
            let velocity_direction = {
                let mut rng = thread_rng();
                let noise_generator = Normal::new(0.0, 0.1).unwrap();

                [
                    -acceleration[1],
                    acceleration[0],
                    acceleration[2] * noise_generator.sample(&mut rng) as f32,
                ]
            };

            // Generates velocities, correcting for magnitude
            // change due to addition of z-component
            let normalisation = velocity_direction.magnitude();
            return MovingParticle {
                particle: particle.clone(),
                velocity: velocity_direction.multiply_by(velocity_magnitude / normalisation),
            };
        })
        .collect();
}
