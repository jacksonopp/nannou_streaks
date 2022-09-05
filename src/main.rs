use nannou::{color::named, prelude::*};

fn main() {
    nannou::app(model).event(event).run();
}

mod models;
use models::{Model, create_model};

mod events;

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1920, 1080)
        .view(view)
        .build()
        .unwrap();
    let boundary = app.window_rect();

    create_model(boundary, _window)
}


fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple } => {
            if let Some(event) = simple {
                match event {
                    KeyReleased(Key::C) => events::change_color(model),
                    KeyReleased(Key::R) => events::reset(app, model),
                    KeyReleased(Key::S) => events::save(app),
                    _ => ()
                }
            }
        },

        Event::Update(update) => events::update(app, model, update),
        _ => ()
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(named::CORNSILK);
    }

    for point in &model.points.dots {
        draw.ellipse().radius(point.radius).xy(point.pos).color(point.color);
    }

    draw.to_frame(app, &frame).unwrap();

    if app.keys.down.contains(&Key::R) {
        frame.clear(named::CORNSILK);
    }
}