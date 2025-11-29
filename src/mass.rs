use crate::helpers::{distance_between, Vector3D, Vector3DCollection};

#[derive(Clone)]
pub struct Particle {
    pub mass: f32,
    pub position: [f32; 3],
}

pub struct MovingParticle {
    pub particle: Particle,
    pub velocity: [f32; 3],
}

pub trait MovingParticles {
    fn to_particles(&self) -> Vec<Particle>;
    fn get_positions(&self) -> Vec<String>;
}

impl MovingParticles for Vec<MovingParticle> {
    fn to_particles(&self) -> Vec<Particle> {
        return self.iter().map(|p| p.particle.clone()).collect();
    }

    fn get_positions(&self) -> Vec<String> {
        return self
            .iter()
            .map(|p| p.particle.position.to_string())
            .flatten()
            .collect();
    }
}

// TODO - need better names
// and probably new files
pub trait MassCollection {
    fn to_single_mass(&self) -> Particle;
}

impl MassCollection for &Vec<Particle> {
    fn to_single_mass(&self) -> Particle {
        let total_mass = self.into_iter().map(|m| m.mass).sum();

        return Particle {
            // Average of positions weighted by mass
            position: self
                .into_iter()
                .map(|m| m.position.multiply_by(m.mass))
                .sum_elements()
                .divide_by(total_mass),
            mass: total_mass,
        };
    }
}

pub trait MassCollectionUnwrap {
    fn reference_to_outer(self) -> Vec<Particle>;
}

impl MassCollectionUnwrap for Vec<&Particle> {
    fn reference_to_outer(self) -> Vec<Particle> {
        return self.into_iter().map(|m| m.clone()).collect();
    }
}

pub fn get_total_energy(particles: &Vec<MovingParticle>) -> f32 {
    let kinetic_energy: f32 = particles
        .iter()
        .map(|p| 0.5 * p.particle.mass * p.velocity.magnitude().powi(2))
        .sum();

    let potential_energy: f32 = particles
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| {
            particles.iter().skip(i + 1).map(move |p2| {
                -p1.particle.mass * p2.particle.mass
                    / distance_between(&p1.particle.position, &p2.particle.position)
            })
        })
        .sum();

    return kinetic_energy + potential_energy;
}
