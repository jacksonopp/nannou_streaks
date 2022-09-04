use nannou::{prelude::*, noise::{Perlin, NoiseFn}};

pub struct Dot {
    pub pos: Point2,
    pub radius: f32,
    pub shrinking_point: f32,
    pub stopping_point: f32,
    pub color: Hsla,
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

            let p = Perlin::new();
            let noise = p.get([pos[0] as f64, pos[1] as f64]);

            let ranged_hue = map_range(noise, 0.0, 1.0, 0.0, 0.2);

            let color = hsla(ranged_hue as f32, 1.0, 0.5, 1.0);

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
