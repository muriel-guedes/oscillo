#[derive(Copy, Clone, Default, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}
impl Color {
    pub const TRANSPARENT:  Self = Self::new(0., 0., 0., 0.);
    pub const BLACK: Self = Self::new(0., 0., 0., 1.);
    pub const WHITE: Self = Self::new(1., 1., 1., 1.);
    pub const GRAY: Self = Self::new(0.5, 0.5, 0.5, 1.);
    pub const RED:   Self = Self::new(1., 0., 0., 1.);
    pub const GREEN: Self = Self::new(0., 1., 0., 1.);
    pub const BLUE:  Self = Self::new(0., 0., 1., 1.);
    pub const YELLOW:  Self = Self::new(1., 1., 0., 1.);
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl Into<Color> for (f32, f32, f32, f32) {
    fn into(self) -> Color {
        Color { r: self.0, g: self.1, b: self.2, a: self.3 }
    }
}

impl Into<[f32;4]> for Color {
    fn into(self) -> [f32;4] {
        [self.r, self.g, self.b, self.a]
    }
}
impl Into<Vec<f32>> for Color {
    fn into(self) -> Vec<f32> {
        vec![self.r, self.g, self.b, self.a]
    }
}