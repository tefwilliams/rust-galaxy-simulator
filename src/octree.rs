use itertools::{repeat_n, Itertools};
use rand_distr::num_traits::ToPrimitive;

use crate::{
    accelerations::get_acceleration_due_to_particle,
    helpers::{distance_between, Vector3D},
    mass::{MassCollection, Particle},
};

pub struct Octree {
    pub nodes: Vec<OctreeNode>,
}

impl Octree {
    pub fn build(particles: &[Particle], center: [f32; 3], size: f32) -> Self {
        let mut tree = Self {
            nodes: Vec::with_capacity(particles.len()),
        };

        tree.nodes.push(OctreeNode {
            particle: particles.to_single_mass(),
            size,
            first_child_index: 0,
        });

        tree.build_recursive(0, particles, center, size);
        tree
    }

    fn build_recursive(
        &mut self,
        node_index: usize,
        particles: &[Particle],
        center: [f32; 3],
        size: f32,
    ) -> usize {
        if particles.len() > 1 {
            let mut octants: [Vec<Particle>; 8] = Default::default();

            for p in particles {
                octants[get_node_index(p.position, center)].push(p.clone());
            }

            let first_child_index = self.nodes.len() as i32;
            self.nodes[node_index].first_child_index = first_child_index;

            let child_size = size / 2.0;
            let child_centers = get_child_centers(&center, child_size);

            for i in 0..8 {
                self.nodes.push(OctreeNode {
                    particle: octants[i].to_single_mass(),
                    size: child_size,
                    first_child_index: 0,
                });
            }

            // 5. Recursively build all 8 children
            // We must do this even for empty octants to keep the 0..8 indexing consistent

            for i in 0..8 {
                self.build_recursive(node_index + i, &octants[i], child_centers[i], child_size);
            }
        }

        node_index
    }

    pub fn get_acceleration(
        &self,
        idx: usize,
        position: [f32; 3],
        softening_factor: f32,
        theta: f32,
    ) -> [f32; 3] {
        let node = &self.nodes[idx];
        // println!(
        //     "Node index: {}, size: {}, first_child_index: {}",
        //     idx, node.size, node.first_child_index
        // );

        if node.size * node.size
            < theta * theta * node.particle.position.subtract(position).self_dot()
            || node.first_child_index == 0
        {
            return node.get_acceleration(position, softening_factor);
        }

        let mut acc = [0.0; 3];
        for i in 0..8 {
            // println!(
            //     "Node index: {}, size: {}, first_child_index: {}",
            //     idx, node.size, node.first_child_index
            // );
            let child_acc = self.get_acceleration(
                (node.first_child_index as usize) + i,
                position,
                softening_factor,
                theta,
            );
            acc[0] += child_acc[0];
            acc[1] += child_acc[1];
            acc[2] += child_acc[2];
        }
        acc
    }
}

pub struct OctreeNode {
    pub size: f32,
    pub particle: Particle,
    pub first_child_index: i32,
}

impl OctreeNode {
    pub fn get_acceleration(&self, position: [f32; 3], softening_factor: f32) -> [f32; 3] {
        // TODO - will this work?
        if self.particle.position == position {
            return [0.0, 0.0, 0.0];
        }

        let distance = self.particle.position.subtract(position);

        let dr_squared = distance.self_dot() + softening_factor.powi(2);
        let dr_power_n32 = dr_squared.sqrt() * dr_squared;

        distance.multiply_by(self.particle.mass / dr_power_n32)
    }
}

// pub fn generate_node(size: f32, node_center: &[f32; 3], particles: &[Particle]) -> OctreeNode {
//     OctreeNode {
//         size,
//         particle: particles.to_single_mass(),
//         children: if particles.len() > 1 {
//             generate_children(size, node_center, particles)
//         } else {
//             Vec::new()
//         },
//     }
// }

fn get_child_centers(node_center: &[f32; 3], new_size: f32) -> [[f32; 3]; 8] {
    core::array::from_fn(|i| {
        [
            node_center[0] + if (i & 1) == 0 { -new_size } else { new_size }, // Bit 0: X
            node_center[1] + if (i & 2) == 0 { -new_size } else { new_size }, // Bit 1: Y
            node_center[2] + if (i & 4) == 0 { -new_size } else { new_size }, // Bit 2: Z
        ]
    })
}

fn get_node_index(position: [f32; 3], node_center: [f32; 3]) -> usize {
    let x = (position[0] >= node_center[0]) as usize;
    let y = ((position[1] >= node_center[1]) as usize) << 1; // Wrap the cast
    let z = ((position[2] >= node_center[2]) as usize) << 2; // Wrap the cast

    x | y | z
}

// fn generate_children(size: f32, node_center: &[f32; 3], particles: &[Particle]) -> Vec<OctreeNode> {
//     let new_size = size / 2_f32;

//     // Get all 8 child node centres
//     let child_centers: [[f32; 3]; 8] = core::array::from_fn(|i| {
//         [
//             node_center[0] + if (i & 1) == 0 { -new_size } else { new_size }, // Bit 0: X
//             node_center[1] + if (i & 2) == 0 { -new_size } else { new_size }, // Bit 1: Y
//             node_center[2] + if (i & 4) == 0 { -new_size } else { new_size }, // Bit 2: Z
//         ]
//     });

//     // Get index of node that pos is closest to
//     let get_node_index = |pos: [f32; 3]| {
//         let x = (pos[0] >= node_center[0]) as usize;
//         let y = ((pos[1] >= node_center[1]) as usize) << 1; // Wrap the cast
//         let z = ((pos[2] >= node_center[2]) as usize) << 2; // Wrap the cast

//         x | y | z
//     };

//     particles
//         .iter()
//         .map(|&m| (get_node_index(m.position), m))
//         .into_group_map()
//         .iter()
//         .filter_map(|(&node_index, node_masses)| {
//             if node_masses.is_empty() {
//                 return None;
//             }

//             Some(generate_node(
//                 new_size,
//                 &child_centers[node_index],
//                 node_masses.as_slice(),
//             ))
//         })
//         .collect()
// }
