use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    _w: f32,
    _h: f32,
    _number: usize,
    rects: Vec<Rectangle>,
}
struct Rectangle {
    position: Vec2,
    size: Vec2,
    color: Hsva,
}
impl Rectangle {
    fn new(p: Vec2, s: Vec2, c: Hsva) -> Self {
        Rectangle {
            position: p,
            size: s,
            color: c,
        }
    }
}
fn model(app: &App) -> Model {
    let _ = app.new_window().size(400, 400).view(view).build().unwrap();
    let width: f32 = 200.0;
    let height: f32 = 200.0;
    let num: usize = 20000;
    Model {
        _w: width,
        _h: height,
        _number: num,
        rects: {
            let mut r = Vec::new();
            for _i in 0..num {
                let pos = vec2(random_range(-width, width), random_range(-height, height));
                let s = vec2(random_range(15.0, 50.0), random_range(5.0, 25.0));
                r.push(Rectangle::new(
                    pos,
                    s,
                    hsva(random_range(0.55, 0.65), 1.0, 1.0, random_range(0.0, 0.1)),
                ));
            }
            r
        },
    }
}
fn update(_app: &App, _model: &mut Model, _update: Update) {}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(WHITE);
    for r in model.rects.iter() {
        draw.rect().xy(r.position).wh(r.size).color(r.color);
    }
    draw.to_frame(app, &frame).unwrap();
}
