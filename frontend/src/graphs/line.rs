use crate::graphs::constants::*;
use crate::GraphData;
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use seed::*;
use std::collections::{HashMap, VecDeque};
use std::ops::Range;

pub fn draw(
	backend: &mut DrawingArea<CanvasBackend, Shift>,
	history: &[(u64, GraphData)],
	map: impl Fn(&GraphData) -> f64,
	y_range: Range<f64>,
	title: &str,
	forget: bool,
) -> Option<()> {
	let font: FontDesc = ("sans-serif", 20.0).into();

	let last = history.last()?.0;
	let index = history
		.iter()
		.enumerate()
		.find(|(_, (tick, _))| tick + MAX_HISTORY > last)
		.map(|x| x.0);

	let skip = match (forget, index) {
		(true, Some(index)) => index,
		_ => 0,
	};

	let x_range = history.get(skip)?.0..history.last()?.0;

	let mut chart = ChartBuilder::on(&backend)
		.margin(20)
		.caption(title, font)
		.x_label_area_size(30)
		.y_label_area_size(30)
		.build_cartesian_2d(x_range, y_range)
		.ok()?;

	// This line will hang if y range is 0.0 .. 0.0, this is a plotters bug probably
	chart
		.configure_mesh()
		.disable_x_mesh()
		.disable_y_mesh()
		.x_labels(10)
		.y_labels(5)
		.draw()
		.ok()?;

	let step = match forget {
		true => ((history.len() as u64).min(MAX_HISTORY) / MAX_COLS) + 1,
		false => (history.len() as u64 / MAX_COLS) + 1,
	} as usize;

	chart
		.draw_series(LineSeries::new(
			history
				.iter()
				.skip(skip)
				.step_by(step)
				.map(|(tick, data)| (*tick, map(data))),
			&COLORS[6],
		))
		.ok()?;

	Some(())
}
