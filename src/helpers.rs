pub trait Vector3D {
    fn self_dot(&self) -> f32;
    fn magnitude(&self) -> f32;
    fn add(&self, other: Self) -> Self;
    fn subtract(&self, other: Self) -> Self;
    fn multiply_by(&self, number: f32) -> Self;
    fn divide_by(&self, number: f32) -> Self;
    fn to_string(&self) -> Vec<String>;
}

impl Vector3D for [f32; 3] {
    fn self_dot(&self) -> f32 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    fn magnitude(&self) -> f32 {
        self.self_dot().sqrt()
    }

    fn add(&self, other: [f32; 3]) -> [f32; 3] {
        [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
    }

    fn subtract(&self, other: [f32; 3]) -> [f32; 3] {
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
