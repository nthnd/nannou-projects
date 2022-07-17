use nannou::noise::*;
use nannou::prelude::*;
const ROWS: i8 = 6;
const COLUMNS: i8 = 6;
const WIDTH: f32 = 360.0;
const HEIGHT: f32 = 360.0;
const DAMPENER: f64 = 0.05;
struct Rectangle {
    position: Vec2,
    size: Vec2,
    rotation: f32,
}
impl Rectangle {
    pub fn new(x: f32, y: f32, r: f32, w: f32, h: f32) -> Self {
        Rectangle {
            position: vec2(x, y),
            size: vec2(w, h),
            rotation: r,
        }
    }
}
fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
struct Model {
    rects: Vec<Rectangle>,
    noise: Perlin,
}
fn model(_app: &App) -> Model {
    let mut r = Vec::new();
    for j in 0..=COLUMNS {
        for i in 0..=ROWS {
            r.push(Rectangle::new(
                (j as f32 / COLUMNS as f32) * WIDTH - WIDTH / 2.0,
                (i as f32 / ROWS as f32) * HEIGHT - HEIGHT / 2.0,
                0.0,
                50.0,
                5.0,
            ));
        }
    }

    Model {
        rects: r,
        noise: Perlin::new(),
    }
}
fn update(app: &App, model: &mut Model, _update: Update) {
    let t: f64 = app.elapsed_frames() as f64 / 60.0 as f64 * DAMPENER;
    for j in 0..=COLUMNS {
        for i in 0..=ROWS {
            model.rects[(i as usize * COLUMNS as usize) + j as usize].rotation =
                (model.noise.get([j as f64, i as f64, t]) - 0.5) as f32 * 5.0;
        }
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.rect()
        .w_h(WIDTH + 50.0, HEIGHT + 50.0)
        .color(srgba(1.0, 1.0, 1.0, 0.01));
    for re in model.rects.iter() {
        draw.rect()
            .xy(re.position)
            .wh(re.size)
            .rotate(re.rotation)
            .color(BLACK);
    }
    draw.ellipse().color(RED).radius(20.0);
    draw.to_frame(app, &frame).unwrap();
}
