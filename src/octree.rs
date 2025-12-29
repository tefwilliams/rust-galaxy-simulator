use itertools::{repeat_n, Itertools};

use crate::{
    accelerations::get_acceleration_due_to_particle,
    helpers::{distance_between, Vector3D},
    mass::{MassCollection, Particle},
};

pub struct OctreeNode {
    size: f32,
    particle: Particle,
    number_of_items: u32,
    children: Vec<OctreeNode>,
}

impl OctreeNode {
    pub fn get_acceleration(
        &self,
        position: [f32; 3],
        softening_factor: f32,
        theta: f32,
    ) -> [f32; 3] {
        if self.size * self.size
            < theta * theta * self.particle.position.subtract(position).self_dot()
            || self.number_of_items == 1
        {
            return get_acceleration_due_to_particle(position, &self.particle, softening_factor);
        }

        let mut ax = 0.0;
        let mut ay = 0.0;
        let mut az = 0.0;

        for i in 0..8 {
            let a = self.children[i].get_acceleration(position, softening_factor, theta);
            ax += a[0];
            ay += a[1];
            az += a[2];
        }

        [ax, ay, az]
    }
}

pub fn generate_node(size: f32, node_center: &[f32; 3], particles: &[Particle]) -> OctreeNode {
    OctreeNode {
        size,
        particle: particles.to_single_mass(),
        number_of_items: particles.len() as u32,
        children: if particles.len() > 1 {
            generate_children(size, node_center, particles)
        } else {
            Vec::new()
        },
    }
}

fn generate_children(size: f32, node_center: &[f32; 3], particles: &[Particle]) -> Vec<OctreeNode> {
    let new_size = size / 2_f32;

    // Get all 8 child node centres
    let child_centers: [[f32; 3]; 8] = core::array::from_fn(|i| {
        [
            node_center[0] + if (i & 1) == 0 { -new_size } else { new_size }, // Bit 0: X
            node_center[1] + if (i & 2) == 0 { -new_size } else { new_size }, // Bit 1: Y
            node_center[2] + if (i & 4) == 0 { -new_size } else { new_size }, // Bit 2: Z
        ]
    });

    // Get index of node that pos is closest to
    let get_node_index = |pos: [f32; 3]| {
        let x = (pos[0] >= node_center[0]) as usize;
        let y = ((pos[1] >= node_center[1]) as usize) << 1; // Wrap the cast
        let z = ((pos[2] >= node_center[2]) as usize) << 2; // Wrap the cast

        x | y | z
    };

    particles
        .iter()
        .map(|&m| (get_node_index(m.position), m))
        .into_group_map()
        .iter()
        .filter_map(|(&node_index, node_masses)| {
            if node_masses.is_empty() {
                return None;
            }

            Some(generate_node(
                new_size,
                &child_centers[node_index],
                node_masses.as_slice(),
            ))
        })
        .collect()
}
