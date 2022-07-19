use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

const ROWS: usize = 10;
const COLUMNS: usize = 10;
const W: f32 = 400.0;
const H: f32 = 400.0;
const SW: f32 = 2.0;

struct Shape {
    origin: Vec2,
    size: f32,
    orientation: bool,
}
impl Shape {
    fn new(o: Vec2, s: f32) -> Self {
        Shape {
            origin: o,
            size: s,
            orientation: random::<bool>(),
        }
    }
    fn draw(&self, draw: &Draw) {
        if self.orientation {
            draw.line()
                .weight(SW)
                .start(vec2(self.origin.x, self.origin.y + self.size))
                .end(vec2(self.origin.x + self.size, self.origin.y));
            draw.line()
                .weight(SW)
                .start(vec2(self.origin.x, self.origin.y - self.size))
                .end(vec2(self.origin.x - self.size, self.origin.y));
        } else {
            draw.line()
                .weight(SW)
                .start(vec2(self.origin.x, self.origin.y + self.size))
                .end(vec2(self.origin.x - self.size, self.origin.y));
            draw.line()
                .weight(SW)
                .start(vec2(self.origin.x, self.origin.y - self.size))
                .end(vec2(self.origin.x + self.size, self.origin.y));
        }
    }
    fn generate_pattern() -> Vec<Shape> {
        let mut shape = Vec::<Shape>::new();
        for j in 1..ROWS {
            for i in 1..COLUMNS {
                shape.push(Shape::new(
                    vec2(
                        (i as f32 / COLUMNS as f32) * W - W * 0.5,
                        (j as f32 / ROWS as f32) * H - H * 0.5,
                    ),
                    (W / COLUMNS as f32) * 0.5,
                ));
            }
        }
        shape
    }
}
struct Model {
    shapes: Vec<Shape>,
}

fn model(app: &App) -> Model {
    let _ = app
        .new_window()
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    Model {
        shapes: Shape::generate_pattern(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}
fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.shapes = Shape::generate_pattern();
        }
        _ => {}
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(WHITE);
    for shape in model.shapes.iter() {
        shape.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
