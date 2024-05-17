use crate::helpers::{Vector3D, Vector3DCollection};

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
    fn get_positions(&self) -> Vec<[f32; 3]>;
}

impl MovingParticles for Vec<MovingParticle> {
    fn to_particles(&self) -> Vec<Particle> {
        return self.iter().map(|p| p.particle.clone()).collect();
    }

    fn get_positions(&self) -> Vec<[f32; 3]> {
        return self.iter().map(|p| p.particle.position).collect();
    }
}

// TODO - need better names
// and probably new files
pub trait MassCollection {
    fn to_single_mass(self) -> Particle;
}

impl MassCollection for &Vec<Particle> {
    fn to_single_mass(self) -> Particle {
        let total_mass = self.into_iter().map(|m| m.mass).sum();

        return Particle {
            // Average of positions weighted by mass
            position: self
                .into_iter()
                .map(|m| m.position.multiply_by(m.mass))
                .collect::<Vec<_>>()
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
