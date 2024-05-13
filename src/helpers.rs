pub type Vector3D = [f32; 3];

pub fn magnitude(vector: &Vector3D) -> f32 {
    return vector.iter().map(|e| e * e).sum::<f32>().sqrt();
}

