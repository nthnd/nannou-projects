use nannou::prelude::*;

mod circle;
use circle::Circle;
// global colors
const COLORS: [(u8, u8, u8); 3] = [(255, 231, 154), (255, 214, 122), (250, 191, 85)];

struct Settings {
    w: f32,
    h: f32,
    attempts: u32,
    min_r: f32,
    max_r: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    circles: Vec<Circle>,
    settings: Settings,
}

fn get_new_circles(s: &Settings) -> Vec<Circle> {
    let mut c = Vec::new();
    for _i in 0..s.attempts {
        let radius = map_range(random::<f32>(), 0.0, 1.0, s.min_r, s.max_r);
        let curr_color = COLORS[map_range(radius, s.min_r, s.max_r, 0, COLORS.len()) as usize];
        let new_circle = Circle::new(
            random::<f32>() * s.w - s.w / 2.0,
            random::<f32>() * s.h - s.h / 2.0,
            radius,
            srgb8(curr_color.0, curr_color.1, curr_color.2),
        );
        let mut has_collided = false;
        for cir in c.iter() {
            if new_circle.colliding(cir, s.w, s.h) {
                has_collided = true;
            }
        }
        if !has_collided {
            c.push(new_circle);
        }
    }
    c
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    let s = Settings {
        w: 400.0,
        h: 400.0,
        attempts: 5000,
        min_r: 4.0,
        max_r: 64.0,
    };
    Model {
        circles: get_new_circles(&s),
        settings: s,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    let mut get_new = true;
    match key {
        Key::Space => {}
        Key::Down => {
            model.settings.min_r -= 2.0;
        }
        Key::Up => {
            model.settings.min_r += 2.0;
        }
        Key::Left => {
            model.settings.max_r -= 2.0;
        }
        Key::Right => {
            model.settings.max_r += 2.0;
        }
        Key::D => {
            model.settings.attempts -= 100;
        }
        Key::I => {
            model.settings.attempts += 100;
        }
        _ => {
            get_new = false;
        }
    }
    if get_new {
        model.circles = get_new_circles(&model.settings);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(srgb8(110, 131, 78));
    for c in model.circles.iter() {
        draw.ellipse()
            .xy(c.position)
            .radius(c.radius)
            .color(c.color);
    }
    draw.to_frame(app, &frame).unwrap();
}
