use nannou::prelude::*;
mod shape;
use shape::Shape;

fn main() {
    nannou::app(model).update(update).run();
}

pub struct Settings {
    rows: usize,
    columns: usize,
    w: f32,
    h: f32,
    sw: f32,
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
        rows: 20,
        columns: 20,
        w: 500.0,
        h: 500.0,
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
