use nannou::{prelude::*, noise::{Perlin, NoiseFn}};

pub struct Dot {
    pub pos: Point2,
    pub radius: f32,
    pub shrinking_point: f32,
    pub stopping_point: f32,
    pub color: Rgba,
}

pub struct Dots {
    pub dots: Vec<Dot>
}

pub struct Model {
    pub window: WindowId,
    pub points: Dots,
}

impl Model {
    pub fn new(_w: WindowId, d: Dots) -> Model {
        Model { window: _w, points: d }
    }
}

pub fn create_model(boundary: Rect, _window: WindowId) -> Model {
    let points = Dots::new(boundary);
    Model::new(_window, points)
}


impl Dots {
    pub fn new(boundary: Rect) -> Dots {
        let left_p = (boundary.left() + 20.0) as i32;
        let right_p = (boundary.right() - 20.0) as i32;
        let density = 5;

        let mut dots = vec![];

        for i in (left_p..right_p).step_by(density) {
            let rand_scalar = random_range(-2.0, 2.0);
            let rand_x_pos = i as f32 + rand_scalar;
            let pos = Point2::new(rand_x_pos, boundary.top() - random_range(20.0, 25.0));
            let stopping_point = boundary.bottom() + random_range(5.0, 20.0);
            let shrinking_point = boundary.bottom() + random_range(50.0, 100.0);

            let color = rgba(0.863, 0.078, 0.235, random_range(0.0, 1.0));

            let dot = Dot {
                pos,
                radius: random_range(2.0, 3.2),
                shrinking_point,
                stopping_point,
                color,
            };
            dots.push(dot);
        }
        Dots { dots }

    }
}
