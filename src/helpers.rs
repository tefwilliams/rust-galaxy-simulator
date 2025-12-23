pub trait Vector3D {
    fn self_dot(&self) -> f32;
    fn magnitude(&self) -> f32;
    fn add(&self, other: &Self) -> Self;
    fn subtract(&self, other: &Self) -> Self;
    fn multiply_by(&self, number: f32) -> Self;
    fn divide_by(&self, number: f32) -> Self;
    fn to_string(&self) -> Vec<String>;
}

impl Vector3D for [f32; 3] {
    fn self_dot(&self) -> f32 {
        self.iter().map(|e| e * e).sum::<f32>()
    }

    fn magnitude(&self) -> f32 {
        self.self_dot().sqrt()
    }

    fn add(&self, other: &[f32; 3]) -> [f32; 3] {
        [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
    }

    fn subtract(&self, other: &[f32; 3]) -> [f32; 3] {
        [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
    }

    fn multiply_by(&self, number: f32) -> [f32; 3] {
        [self[0] * number, self[1] * number, self[2] * number]
    }

    fn divide_by(&self, number: f32) -> [f32; 3] {
        [self[0] / number, self[1] / number, self[2] / number]
    }

    fn to_string(&self) -> Vec<String> {
        self.into_iter()
            .map(|number| format!("{:+.2e}", number))
            .collect()
    }
}

pub fn distance_between(vector1: &[f32; 3], vector2: &[f32; 3]) -> f32 {
    return vector1.subtract(vector2).magnitude();
}

pub trait Vector3DCollection {
    fn sum_elements(self) -> [f32; 3];
}

impl<I> Vector3DCollection for I
where
    I: Iterator<Item = [f32; 3]>,
{
    fn sum_elements(self) -> [f32; 3] {
        self.fold([0.0, 0.0, 0.0], |acc, vec| acc.add(&vec))
    }
}
