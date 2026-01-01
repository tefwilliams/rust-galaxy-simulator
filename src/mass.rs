use crate::helpers::Vector3D;

#[derive(Copy, Clone, Debug, Default)]
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
