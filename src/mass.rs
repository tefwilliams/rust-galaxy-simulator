use crate::helpers::{distance_between, Vector3D};

#[derive(Copy, Clone)]
pub struct Particle {
    pub mass: f32,
    pub position: [f32; 3],
}

pub struct MovingParticle {
    pub particle: Particle,
    pub velocity: [f32; 3],
}

pub trait MovingParticles {
    fn get_positions(&self) -> Vec<String>;
}

impl MovingParticles for &[MovingParticle] {
    fn get_positions(&self) -> Vec<String> {
        self.iter()
            .map(|p| p.particle.position.to_string())
            .flatten()
            .collect()
    }
}

// TODO - need better names
// and probably new files
pub trait MassCollection {
    fn to_single_mass(self) -> Particle;
}

impl MassCollection for &[Particle] {
    fn to_single_mass(self) -> Particle {
        let total_mass = self.iter().map(|m| m.mass).sum();

        let weighted_positions = self.iter().fold([0.0, 0.0, 0.0], |acc, p| {
            acc.add(p.position.multiply_by(p.mass))
        });

        Particle {
            // Average of positions weighted by mass
            position: weighted_positions.divide_by(total_mass),
            mass: total_mass,
        }
    }
}

pub fn get_total_energy(particles: &[MovingParticle]) -> f32 {
    let kinetic_energy: f32 = particles
        .iter()
        .map(|p| 0.5 * p.particle.mass * p.velocity.self_dot())
        .sum();

    let potential_energy: f32 = particles
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| {
            particles.iter().skip(i + 1).map(move |p2| {
                -p1.particle.mass * p2.particle.mass
                    / distance_between(p1.particle.position, p2.particle.position)
            })
        })
        .sum();

    kinetic_energy + potential_energy
}
