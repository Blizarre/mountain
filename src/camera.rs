use crate::fixed_int::FixedInt10;

#[derive(Debug)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: FixedInt10,
    pub horizon: i32,
    angle: f32,
    pub cos_angle: f32,
    pub sin_angle: f32,
}

impl Camera {
    pub fn update_angle(self: &mut Self, offset: f32) {
        self.angle += offset;
        self.cos_angle = self.angle.cos();
        self.sin_angle = self.angle.sin();
    }

    pub fn new(x: f32, y: f32, z: FixedInt10, horizon: i32) -> Camera {
        Camera {
            x,
            y,
            z,
            horizon,
            angle: 0.,
            cos_angle: 1.,
            sin_angle: 0.,
        }
    }
}
