use nannou::prelude::*;
pub struct Circle {
    pub position: Vec2,
    pub radius: f32,
    pub color: Srgb<u8>,
}
impl Circle {
    pub fn new(x: f32, y: f32, r: f32, c: Srgb<u8>) -> Self {
        Circle {
            position: vec2(x, y),
            radius: r,
            color: c,
        }
    }
    pub fn colliding(&self, c: &Circle, w: f32, h: f32) -> bool {
        self.position.distance(c.position) <= self.radius + c.radius
            || ((self.position.x.abs() + self.radius) as f32) > w / 2.0
            || ((self.position.y.abs() + self.radius) as f32) > h / 2.0
    }
}
