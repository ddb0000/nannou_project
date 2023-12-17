use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {}

fn model (app: &App) -> Model {
    app.new_window().view(view).build().unwrap();
    Model {}
}

fn update (_app: &App, _model: &mut Model, _update: Update) {
}

fn view (app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    draw.to_frame(app, &frame).unwrap();
}
