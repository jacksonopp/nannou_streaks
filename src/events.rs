use crate::model::{create_model, Model};
use nannou::{color::named, prelude::*, };

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    model.points.dots.iter_mut().for_each(|point| {
        let rand_scalar = random_range(-0.1, 0.1);
        if point.pos[1] > point.stopping_point {
            point.radius += rand_scalar;
            point.pos -= Vec2::new(rand_scalar, 1.0);

            // let boundary = app.window_rect();

            // let new_color = 

            // Shrink it!
            if point.pos[1] < point.shrinking_point {
                let new_radius = point.radius - 0.1;

                if new_radius > 0.0 {
                    point.radius = new_radius;
                } else {
                    point.radius = 0.0
                }
            }
        }
    })
}

pub fn reset(app: &App, model: &mut Model) {
    let draw = app.draw();
    draw.background().color(named::CORNSILK);
    let boundary = app.window_rect();

    let win_id = model.window;

    let new_model = create_model(boundary, win_id);

    model.points = new_model.points;
}

pub fn change_color(model: &mut Model) {
    let new_color = hsla(random_f32(), random_f32(), random_f32(), random_f32());
    model.points.dots.iter_mut().for_each(|point| {
        point.color = new_color;
    })
}

pub fn save(app: &App) {
    let rand = random_range(0, 100000);
    let rand_str = format!("{}", rand);
    app.main_window()
        .capture_frame(app.exe_name().unwrap() + rand_str.as_str() + ".png");
}
