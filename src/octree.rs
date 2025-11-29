use itertools::{repeat_n, Itertools};

use crate::{
    accelerations::get_acceleration_due_to_particle,
    helpers::{distance_between, Vector3D},
    mass::{MassCollection, MassCollectionUnwrap, Particle},
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
        position: &[f32; 3],
        softening_factor: f32,
        theta: f32,
    ) -> [f32; 3] {
        if self.size / self.particle.position.subtract(&position).magnitude() < theta
            || self.number_of_items == 1
        {
            return get_acceleration_due_to_particle(
                // TODO - avoid self-interaction
                &position,
                &self.particle,
                softening_factor,
            );
        }

        return self.children.iter().fold([0.0, 0.0, 0.0], |acc, child| {
            return acc.add(&child.get_acceleration(position, softening_factor, theta));
        });
    }
}

pub fn generate_node(size: f32, node_center: &[f32; 3], particles: &Vec<Particle>) -> OctreeNode {
    return OctreeNode {
        size,
        particle: particles.to_single_mass(),
        number_of_items: particles.len() as u32,
        children: if particles.len() > 1 {
            generate_children(size, node_center, particles)
        } else {
            Vec::new()
        },
    };
}

fn generate_children(
    size: f32,
    node_center: &[f32; 3],
    particles: &Vec<Particle>,
) -> Vec<OctreeNode> {
    let new_size = size / 2_f32;

    // Get all 8 child node centres
    let node_centers = repeat_n([-new_size, new_size], 3)
        .multi_cartesian_product()
        .map(|vec| node_center.add(&vec.try_into().unwrap()))
        .collect::<Vec<[f32; 3]>>();

    // Get index of node that pos is closest to
    let get_node_index = |pos: &[f32; 3]| {
        (0_usize..8_usize)
            .min_by(|i, j| {
                distance_between(&pos, &node_centers[*i])
                    .total_cmp(&distance_between(&pos, &node_centers[*j]))
            })
            .unwrap()
    };

    return particles
        .into_iter()
        .map(|m| (get_node_index(&m.position), m))
        .into_group_map()
        .into_iter()
        .filter_map(|(node_index, node_masses)| {
            return (node_masses.len() > 0).then_some(generate_node(
                new_size,
                &node_centers[node_index],
                &node_masses.reference_to_outer(),
            ));
        })
        .collect();
}
