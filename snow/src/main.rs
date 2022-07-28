use nannou::prelude::*;
mod particle;
use particle::Particle;

fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    particles: Vec<Particle>,
}
fn model(app: &App) -> Model {
    let _ = app.new_window().view(view).build().unwrap();
    let r = app.window_rect();
    let mut p = Vec::new();
    for _i in 0..2000 {
        p.push(Particle::new(&r));
    }
    Model { particles: p }
}
fn update(app: &App, model: &mut Model, _update: Update) {
    let r = app.window_rect();
    model.particles.iter_mut().for_each(|p| {
        if p.update(&r) {
            *p = Particle::new(&r)
        }
    });
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(BLACK);
    for p in model.particles.iter() {
        draw.ellipse()
            .xy(p.position)
            .radius(map_range(
                p.velocity.length(),
                p.min_vel,
                p.max_vel,
                1.0,
                3.0,
            ))
            .color(rgba(
                1.0,
                1.0,
                1.0,
                map_range(p.velocity.length(), p.min_vel, p.max_vel, 0.0, 1.0),
            ));
    }
    draw.to_frame(app, &frame).unwrap();
}
