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
    let _ = app.new_window().size(400, 400).view(view).build().unwrap();
    let app_rect = Rect::from_w_h(400.0, 400.0);
    let mut p = Vec::new();
    for _i in 0..100 {
        p.push(Particle::new(
            vec2(random_range(-200.0, 200.0), random_range(-200.0, 200.0)),
            random_range(0.0, 3000.0),
            &app_rect,
        ));
    }
    Model { particles: p }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    for p in model.particles.iter_mut() {
        p.apply_force(vec2(random_range(-30.0, 30.0), random_range(-30.0, 30.0)));
        p.check_bounds();
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(srgba(0.0, 0.0, 0.0, 0.1));
    for p in model.particles.iter() {
        draw.ellipse()
            .xy(p.position)
            .radius(map_range(p.mass, 0.0, 3000.0, 2.0, 10.0));
        for np in model.particles.iter() {
            let dis = p.position.distance(np.position).abs();
            if dis < 50.0 {
                draw.line().start(p.position).end(np.position).color(srgba(
                    1.0,
                    1.0,
                    1.0,
                    map_range(dis, 0.0, 50.0, 0.0, 1.0),
                ));
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
