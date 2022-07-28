use nannou::prelude::*;
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub dead: bool,
    pub min_vel: f32,
    pub max_vel: f32,
    bounds: Rect,
}
impl Particle {
    pub fn new(r: &Rect) -> Self {
        let minv = 0.5;
        let maxv = 5.5;
        Particle {
            position: vec2(random_range(r.left(), r.right()), r.top() + 10.0),
            velocity: vec2(random_range(-minv, minv), random_range(-maxv, -minv)),
            bounds: *r,
            dead: false,
            max_vel: maxv,
            min_vel: minv,
        }
    }
    pub fn update(&mut self, r: &Rect) -> bool {
        self.bounds = *r;
        self.position += self.velocity;
        if self.position.x > self.bounds.right() || self.position.x < self.bounds.left() {
            self.velocity.x = -self.velocity.x;
        }
        if self.position.y < self.bounds.bottom() {
            self.dead = true;
        }
        self.dead
    }
}
