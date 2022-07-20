use crate::Settings;
use nannou::prelude::*;
pub struct Shape {
    pub origin: Vec2,
    pub size: f32,
    pub orientation: i32,
}
impl Shape {
    pub fn new(o: Vec2, s: f32) -> Self {
        Shape {
            origin: o,
            size: s,
            orientation: random_range(0, 5),
        }
    }
    pub fn draw(&self, draw: &Draw, sw: f32) {
        match self.orientation {
            0 => {
                draw.line()
                    .weight(sw)
                    .start(vec2(self.origin.x, self.origin.y + self.size))
                    .end(vec2(self.origin.x + self.size, self.origin.y));
                draw.line()
                    .weight(sw)
                    .start(vec2(self.origin.x, self.origin.y - self.size))
                    .end(vec2(self.origin.x - self.size, self.origin.y));
            }

            1 => {
                draw.line()
                    .weight(sw)
                    .start(vec2(self.origin.x, self.origin.y + self.size))
                    .end(vec2(self.origin.x - self.size, self.origin.y));
                draw.line()
                    .weight(sw)
                    .start(vec2(self.origin.x, self.origin.y - self.size))
                    .end(vec2(self.origin.x + self.size, self.origin.y));
            }
            3 => {
                draw.line()
                    .weight(sw)
                    .start(vec2(self.origin.x, self.origin.y + self.size))
                    .end(vec2(self.origin.x, self.origin.y - self.size));
                draw.line()
                    .weight(sw)
                    .start(vec2(self.origin.x + self.size, self.origin.y))
                    .end(vec2(self.origin.x - self.size, self.origin.y));
                draw.rect()
                    .xy(self.origin)
                    .w_h(self.size * 0.33, self.size)
                    .stroke_weight(sw);
            }

            4 => {
                draw.line()
                    .weight(sw)
                    .start(vec2(self.origin.x, self.origin.y + self.size))
                    .end(vec2(self.origin.x, self.origin.y - self.size));
                draw.line()
                    .weight(sw)
                    .start(vec2(self.origin.x + self.size, self.origin.y))
                    .end(vec2(self.origin.x - self.size, self.origin.y));
                draw.rect()
                    .xy(self.origin)
                    .w_h(self.size, self.size * 0.33)
                    .stroke_weight(sw);
            }
            _ => {
                draw.line()
                    .weight(sw)
                    .start(vec2(self.origin.x, self.origin.y + self.size))
                    .end(vec2(self.origin.x, self.origin.y - self.size));
                draw.line()
                    .weight(sw)
                    .start(vec2(self.origin.x + self.size, self.origin.y))
                    .end(vec2(self.origin.x - self.size, self.origin.y));
                draw.ellipse()
                    .xy(self.origin)
                    .radius(self.size * 0.5)
                    .stroke_weight(sw);
            }
        }
    }
    pub fn generate_pattern(settings: &Settings) -> Vec<Shape> {
        let mut shape = Vec::<Shape>::new();
        for j in 1..settings.rows {
            for i in 1..settings.columns {
                shape.push(Shape::new(
                    vec2(
                        (i as f32 / settings.columns as f32) * settings.w - settings.w * 0.5,
                        (j as f32 / settings.rows as f32) * settings.h - settings.h * 0.5,
                    ),
                    (settings.w / settings.columns as f32) * 0.5,
                ));
            }
        }
        shape
    }
}
