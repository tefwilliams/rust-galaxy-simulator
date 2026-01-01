use crate::{
    helpers::Vector3D,
    mass::{MassCollection, Particle},
};

pub struct Octree {
    pub nodes: Vec<OctreeNode>,
}

impl Octree {
    pub fn build(particles: &[Particle], center: [f32; 3], size: f32) -> Self {
        // println!("Initializing octree...");
        let mut tree = Self {
            nodes: Vec::with_capacity(particles.len()),
        };

        // for p in particles.iter() {
        //     println!(
        //         "Particle mass: {}, position: ({}, {}, {})",
        //         p.mass, p.position[0], p.position[1], p.position[2]
        //     );
        // }

        // Add starting root node
        tree.nodes.push(OctreeNode::default());

        tree.build_recursive(0, particles, center, size);
        tree
    }

    fn build_recursive(
        &mut self,
        node_idx: usize,
        particles: &[Particle],
        center: [f32; 3],
        size: f32,
    ) {
        // println!(
        //     "Building node at index {} with {} particles...",
        //     node_idx,
        //     particles.len()
        // );
        self.nodes[node_idx] = OctreeNode {
            size,
            particle: particles.to_single_mass(),
            first_child_index: 0,
        };

        // println!(
        //     "Creating node index: {}, position: {}, mass: {}, child index: {}",
        //     node_idx,
        //     self.nodes[node_idx].particle.position[0],
        //     self.nodes[node_idx].particle.mass,
        //     self.nodes[node_idx].first_child_index
        // );

        if particles.len() <= 1 {
            return;
        }

        // Reserve 8 contiguous slots at the end of the array
        let start_idx = self.nodes.len();
        self.nodes[node_idx].first_child_index = start_idx as u32;
        self.nodes.extend((0..8).map(|_| OctreeNode::default()));

        // Bucket particles into octants
        let mut buckets: Vec<Vec<Particle>> = vec![Vec::new(); 8];
        for p in particles {
            buckets[get_octant(center, p.position)].push(*p);
        }

        // Recurse: Process children immediately (DFS)
        let new_size = size * 0.5;
        for i in 0..8 {
            let new_center = get_child_center(center, new_size, i);
            self.build_recursive(start_idx + i, buckets[i].as_slice(), new_center, new_size);
        }
    }

    pub fn get_acceleration(
        &self,
        idx: usize,
        position: [f32; 3],
        softening_factor: f32,
        theta: f32,
    ) -> [f32; 3] {
        let node = &self.nodes[idx];

        //         println!(
        //             "
        // Particle ({}, {}, {}),
        // Visiting Node: {}, position: ({}, {}, {}), mass: {}, child index: {}",
        //             position[0],
        //             position[1],
        //             position[2],
        //             idx,
        //             node.particle.position[0],
        //             node.particle.position[1],
        //             node.particle.position[2],
        //             node.particle.mass,
        //             node.first_child_index
        //         );

        if node.is_placeholder() {
            return [0.0; 3];
        }

        if node.size * node.size
            < theta * theta * node.particle.position.subtract(position).self_dot()
            || node.is_leaf()
        {
            let acc = node.get_acceleration(position, softening_factor);
            // println!(
            //     "Approximating node {} as single mass, acc: ({}, {}, {})",
            //     idx, acc[0], acc[1], acc[2]
            // );

            return acc;
        }

        // println!("Splitting node {}", idx);

        let mut acc = [0.0; 3];
        for i in 0..8 {
            let child_acc = self.get_acceleration(
                (node.first_child_index as usize) + i,
                position,
                softening_factor,
                theta,
            );
            // println!(
            //     "Node index: {}, acc: ({}, {}, {})",
            //     idx, , child_acc[0], child_acc[1], child_acc[2]
            // );
            acc[0] += child_acc[0];
            acc[1] += child_acc[1];
            acc[2] += child_acc[2];
        }

        // println!(
        //     "Node {} total acc: ({}, {}, {})",
        //     idx, acc[0], acc[1], acc[2]
        // );
        acc
    }
}

// --- Helpers ---

fn get_octant(center: [f32; 3], position: [f32; 3]) -> usize {
    let mut idx = 0;
    if position[0] >= center[0] {
        idx |= 1;
    }
    if position[1] >= center[1] {
        idx |= 2;
    }
    if position[2] >= center[2] {
        idx |= 4;
    }
    idx
}

fn get_child_center(c: [f32; 3], offset: f32, i: usize) -> [f32; 3] {
    [
        c[0] + if (i & 1) != 0 { offset } else { -offset },
        c[1] + if (i & 2) != 0 { offset } else { -offset },
        c[2] + if (i & 4) != 0 { offset } else { -offset },
    ]
}

#[derive(Debug, Default)]
pub struct OctreeNode {
    pub size: f32,
    pub particle: Particle,
    pub first_child_index: u32,
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

    pub fn is_placeholder(&self) -> bool {
        self.particle.mass == 0.0
    }

    pub fn is_leaf(&self) -> bool {
        self.first_child_index == 0
    }
}
