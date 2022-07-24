use nannou::prelude::*;
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
    terminal_v: Vec2,
    acceleration: Vec2,
    bounds: Rect,
}
impl Particle {
    pub fn new(p: Vec2, m: f32, win: &Rect) -> Self {
        Particle {
            position: p,
            velocity: vec2(random_range(-2.5, 2.5), random_range(-2.5, 2.5)),
            mass: m,
            terminal_v: vec2(2.5, 2.5),
            acceleration: vec2(0.0, 0.0),
            bounds: *win,
        }
    }
    pub fn apply_force(&mut self, f: Vec2) {
        self.acceleration = f / self.mass;
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp(-self.terminal_v, self.terminal_v);
        self.position += self.velocity;
    }
    pub fn check_bounds(&mut self) {
        if self.position.x <= self.bounds.left() || self.position.x > self.bounds.right() {
            self.velocity.x = -self.velocity.x;
        }
        if self.position.y <= self.bounds.bottom() || self.position.y > self.bounds.top() {
            self.velocity.y = -self.velocity.y;
        }
    }
}
