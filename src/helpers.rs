pub trait Vector3D {
    fn magnitude(&self) -> f32;
    fn add(&self, other: &Self) -> Self;
    fn subtract(&self, other: &Self) -> Self;
    fn multiply_by(&self, number: f32) -> Self;
    fn divide_by(&self, number: f32) -> Self;
}

impl Vector3D for [f32; 3] {
    fn magnitude(&self) -> f32 {
        return self.iter().map(|e| e * e).sum::<f32>().sqrt();
}

    fn add(&self, other: &[f32; 3]) -> [f32; 3] {
        return [self[0] + other[0], self[1] + other[1], self[2] + other[2]];
    }

    fn subtract(&self, other: &[f32; 3]) -> [f32; 3] {
        return [self[0] - other[0], self[1] - other[1], self[2] - other[2]];
    }

    fn multiply_by(&self, number: f32) -> [f32; 3] {
        return [self[0] * number, self[1] * number, self[2] * number];
    }

    fn divide_by(&self, number: f32) -> [f32; 3] {
        return [self[0] / number, self[1] / number, self[2] / number];
    }
}

