use crate::graphs::colors::COLORS;
use crate::GraphData;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use seed::*;
use std::collections::HashMap;
use std::ops::Range;

pub fn draw(
	canvas_id: &str,
	history: &[(u64, GraphData)],
	map: impl Fn(&GraphData) -> f64,
	range: Range<f64>
) -> Option<()> {
	let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
	let root = backend.into_drawing_area();
	let font: FontDesc = ("sans-serif", 20.0).into();


	root.fill(&WHITE).ok()?;

	let mut chart = ChartBuilder::on(&root)
		.margin(20)
		.x_label_area_size(30)
		.y_label_area_size(30)
		.build_cartesian_2d(0..history.last().unwrap().0, range)
		.ok()?;

	// This line will hang if y range is 0.0 .. 0.0, this is a plotters bug probably
	chart
		.configure_mesh()
		.disable_x_mesh()
		.disable_y_mesh()
		.x_labels(10)
		.y_labels(3)
		.draw()
		.ok()?;

	chart
		.draw_series(LineSeries::new(
			history.iter().map(|(tick, data)| (*tick, map(data))),
			&COLORS[5],
		))
		.ok()?;
	root.present().ok()
}
