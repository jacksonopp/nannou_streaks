use crate::models::{create_model, Model};
use nannou::{color::named, prelude::*, };

/// Handle the update event
/// * `_app` - currently unsused, but a reference to the application
/// * `model` - a mutable reference to the model
/// * `_update` - currently unsed, but the update event
pub fn update(_app: &App, model: &mut Model, _update: Update) {
    model.points.dots.iter_mut().for_each(|point| {
        let rand_scalar = random_range(-0.1, 0.1);
        if point.pos[1] > point.stopping_point {
            point.radius += rand_scalar;
            point.pos -= Vec2::new(rand_scalar, 1.0);

            // Give it an even streaker look
            // let new_alpha = point.color.alpha + random_range(-0.1, 0.1);
            // let new_color = rgba(point.color.red, point.color.green, point.color.blue, new_alpha);
            // point.color = new_color;

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

/// Resets the application
pub fn reset(app: &App, model: &mut Model) {
    

    let draw = app.draw();
    draw.background().color(named::CORNSILK);
    let boundary = app.window_rect();

    let win_id = model.window;

    let new_model = create_model(boundary, win_id);

    model.points = new_model.points;
}

/// Change the color to a random value
pub fn change_color(model: &mut Model) {
    let new_color = rgba(random_f32(), random_f32(), random_f32(), random_f32());
    model.points.dots.iter_mut().for_each(|point| {
        point.color = new_color;
    })
}

/// Save as a png
pub fn save(app: &App) {
    let rand = random_range(0, 100000);
    let rand_str = format!("{}", rand);
    app.main_window()
        .capture_frame(app.exe_name().unwrap() + rand_str.as_str() + ".png");
}
