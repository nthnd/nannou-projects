use nannou::prelude::*;
mod circle;
use circle::Circle;
const W: f32 = 480.0;
const H: f32 = 540.0;
const ATTEMPTS: u32 = 6000;
const MIN_R: f32 = 4.0;
const MAX_R: f32 = 64.0;
const COLORS: [(u8, u8, u8); 9] = [
    (55, 6, 23),
    (106, 4, 15),
    (157, 2, 8),
    (208, 0, 0),
    (220, 47, 2),
    (232, 93, 4),
    (244, 140, 6),
    (250, 163, 7),
    (255, 186, 8),
];
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    circles: Vec<Circle>,
    minimum_radius: f32,
    maximum_radius: f32,
    density: u32,
}
fn get_new_circles(minimum: f32, maximum: f32, density: u32) -> Vec<Circle> {
    let mut c = Vec::new();
    for _i in 0..density {
        let radius = map_range(random::<f32>(), 0.0, 1.0, minimum, maximum);
        let curr_color = COLORS[map_range(radius, minimum, maximum, 0, COLORS.len()) as usize];
        let new_circle = Circle::new(
            random::<f32>() * W - W / 2.0,
            random::<f32>() * H - H / 2.0,
            radius,
            srgb8(curr_color.0, curr_color.1, curr_color.2),
        );
        let mut has_collided = false;
        for cir in c.iter() {
            if new_circle.colliding(cir, W, H) {
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
    Model {
        circles: get_new_circles(MIN_R, MAX_R, ATTEMPTS),
        minimum_radius: MIN_R,
        maximum_radius: MAX_R,
        density: ATTEMPTS,
    }
}
fn update(_app: &App, _model: &mut Model, _update: Update) {}
fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.circles =
                get_new_circles(model.minimum_radius, model.maximum_radius, model.density);
        }
        Key::Down => {
            model.minimum_radius -= 2.0;
            model.circles =
                get_new_circles(model.minimum_radius, model.maximum_radius, model.density);
        }
        Key::Up => {
            model.minimum_radius += 2.0;
            model.circles =
                get_new_circles(model.minimum_radius, model.maximum_radius, model.density);
        }
        Key::Left => {
            model.maximum_radius -= 2.0;
            model.circles =
                get_new_circles(model.minimum_radius, model.maximum_radius, model.density);
        }
        Key::Right => {
            model.maximum_radius += 2.0;
            model.circles =
                get_new_circles(model.minimum_radius, model.maximum_radius, model.density);
        }
        Key::D => {
            model.density -= 100;
            model.circles =
                get_new_circles(model.minimum_radius, model.maximum_radius, model.density);
        }
        Key::I => {
            model.density += 100;
            model.circles =
                get_new_circles(model.minimum_radius, model.maximum_radius, model.density);
        }
        _ => {}
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(srgb8(0, 0, 0));
    for c in model.circles.iter() {
        draw.ellipse()
            .xy(c.position)
            .radius(c.radius)
            .color(c.color);
    }
    draw.to_frame(app, &frame).unwrap();
}
