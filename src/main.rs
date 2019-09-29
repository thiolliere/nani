use nannou::prelude::*;
use core::time::Duration;
use nannou::color::Alpha;
use enum_iterator::IntoEnumIterator;

const WINDOW_SIZE: f32 = 800.0;

fn main() {
	nannou::app(init)
		.update(update)
		.simple_window(view)
		.run();
}

struct Model {}

impl Model {
	fn new() -> Self {
		Self {}
	}
}

fn init(app: &App) -> Model {
	app.main_window().set_inner_size_points(WINDOW_SIZE, WINDOW_SIZE);

	Model::new()
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

const PERCENT_VELOCITY: f32 = 0.1;
const PERCENT_POSITION_START: f32 = 0.5;

fn view(app: &App, _model: &Model, frame: &Frame) {
	frame.clear(WHITE);

	let draw = app.draw();

	let percent_position = (app.time * PERCENT_VELOCITY + PERCENT_POSITION_START) % 1.0;
	let percent = if percent_position >= 0.5 {
		percent_position
	} else {
		1.0 - percent_position
	};

	let mut size = WINDOW_SIZE/4.0;
	let mut orientation = true;
	while size >= 1.0 {
		let next_size = size * (1.0 - (1.0 - percent) * 2.0);
		draw_square(&draw, size, next_size, percent, orientation);
		orientation = !orientation;
		size = next_size;
	}

	draw.to_frame(app, &frame).unwrap();
}

fn duration_to_fractional(dur: Duration) -> f32 {
	dur.as_secs() as f32 + dur.subsec_nanos() as f32 / 1_000_000_000.0
}

const LINE_THICKNESS: f32 = 2.0;

fn draw_square(draw: &app::Draw, size: f32, next_size: f32, percent: f32, orientation: bool) {
	for &dx in [-1.0, 1.0].iter() {
		for &dy in [-1.0, 1.0].iter() {
			let start = Point2::new(dx, dy) * size;
			let (side_dir, center_dir) = if (dx.signum() != dy.signum()) ^ orientation {
				(Point2::new(0.0, - dy.signum()), (Point2::new(-dx.signum(), 0.0)))
			} else {
				(Point2::new(- dx.signum(), 0.0), (Point2::new(0.0, -dy.signum())))
			};
			let end = start + side_dir * percent * 2.0 *size + center_dir * (size - next_size);
			draw.line()
				.caps_square()
				.color(BLACK)
				.thickness(LINE_THICKNESS)
				.start(start)
				.end(end);
		}
	}
}
