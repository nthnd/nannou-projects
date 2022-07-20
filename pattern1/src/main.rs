use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    rows: usize,
    columns: usize,
    w: f32,
    h: f32,
    sw: f32,
}

struct Shape {
    origin: Vec2,
    size: f32,
    orientation: i32,
    color: Rgb8,
}
impl Shape {
    fn new(o: Vec2, s: f32) -> Self {
        Shape {
            origin: o,
            size: s,
            orientation: random_range(0, 4),
            color: if random::<bool>() {
                srgb8(0, 0, 0)
            } else {
                srgb8(255, 255, 255)
            },
        }
    }
    fn draw(&self, draw: &Draw, sw: f32) {
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
                    .w_h(self.size, self.size)
                    .stroke_weight(sw)
                    .color(self.color);
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
                    .stroke_weight(sw)
                    .color(self.color);
            }
        }
    }
    fn generate_pattern(settings: &Settings) -> Vec<Shape> {
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
struct Model {
    shapes: Vec<Shape>,
    settings: Settings,
}

fn model(app: &App) -> Model {
    let _ = app
        .new_window()
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    let s = Settings {
        rows: 10,
        columns: 10,
        w: 400.0,
        h: 400.0,
        sw: 2.0,
    };
    Model {
        shapes: Shape::generate_pattern(&s),
        settings: s,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}
fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    let mut gen_pat = true;
    match key {
        Key::Space => {}
        Key::Up => {
            model.settings.rows += 2;
            model.settings.columns += 2;
        }
        Key::Down => {
            model.settings.rows -= 2;
            model.settings.columns -= 2;
        }
        _ => {
            gen_pat = false;
        }
    }
    if gen_pat {
        model.shapes = Shape::generate_pattern(&model.settings);
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(WHITE);
    for shape in model.shapes.iter() {
        shape.draw(&draw, model.settings.sw);
    }
    draw.to_frame(app, &frame).unwrap();
}
