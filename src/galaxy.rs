use std::f32::consts::PI;
use rand::distributions::uniform::{UniformFloat, UniformSampler};


// TODO - move to constant file
const GALAXY_SIZE: usize = 2_usize.pow(15);


pub fn create_galaxy() -> ([f32; GALAXY_SIZE], [f32; GALAXY_SIZE]) {
    // Sets positions for points
    let positions = create_positions();

    // Finds the initial velocity of each point
    let velocities = initialize_velocities(positions);

    return (positions, velocities);
}


fn create_positions() -> [f32; GALAXY_SIZE] {
    // Generates radial and angular components in 2D
    let points_r = Exp(size, n);
    let points_p = UniformFloat::new(0, 2_f32 * PI);

    // Converts to Cartesian
    points_x = points_r * np.cos(points_p)
    points_y = points_r * np.sin(points_p)

    // Adds 'noise' to all 3D to generate points
    noise_magnitude = size / 5

    points = np.empty((n, 3), dtype=np.float64)
    points[:, 0] = np.random.normal(0, noise_magnitude, n) + points_x
    points[:, 1] = np.random.normal(0, noise_magnitude, n) + points_y
    points[:, 2] = np.random.normal(0, noise_magnitude, n)

    return points
}


fn initialize_velocities(positions: [f32; GALAXY_SIZE]) -> [f32; GALAXY_SIZE] {
    // Gets initial acceleration of each point
    initial_accelerations = get_accelerations(positions, 0)

    // Calculates the speed of particle for circular motion
    position_magnitudes = np.linalg.norm(positions, axis=1)
    acceleration_magnitudes = np.linalg.norm(initial_accelerations, axis=1)
    velocity_magnitudes = np.sqrt(position_magnitudes * acceleration_magnitudes)

    // Calculates unit vectors in 3D
    radial_vectors = initial_accelerations[:, 0:2]
    vertical_vectors = np.random.normal(0, 0.1, len(positions))

    velocity_directions = np.zeros_like(positions)
    velocity_directions[:, 0] = - radial_vectors[:, 1]
    velocity_directions[:, 1] = radial_vectors[:, 0]
    velocity_directions[:, 2] = vertical_vectors

    // Adds normalisation for correct speed (due to addition of z-component)
    normalisation = np.linalg.norm(velocity_directions, axis=1)

    // Generates velocities
    return velocity_magnitudes[:, None] * velocity_directions / normalisation[:, None]
}