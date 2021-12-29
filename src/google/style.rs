use std::fmt::{self, Debug, Formatter};

use crate::google::JavaScript;


#[derive(Debug, Copy, Clone)]
pub enum Color {
	RGB(u8, u8, u8),
	RGBA(u8, u8, u8, u8),
	HSL(u16, u8, u8),
	HSLA(u16, u8, u8, u8),
	Black,
	Silver,
	Gray,
	White,
	Maroon,
	Red,
	Purple,
	Fuchsia,
	Green,
	Lime,
	Olive,
	Yellow,
	Navy,
	Blue,
	Teal,
	Aqua,
}


impl JavaScript for Color {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match *self {
			Color::RGB(r, g, b) => write!(f, "\"#{:02x}{:02x}{:02x}\"", r, g, b),
			Color::RGBA(r, g, b, a) => write!(f, "\"#{:02x}{:02x}{:02x}{:02x}\"", r, g, b, a),
			Color::HSL(h, s, l) => write!(f, "\"hsl({h}, {s}%, {l}%)\"", h = h, s = 100.0 * f64::from(s) / 255.0, l = 100.0 * f64::from(l) / 255.0),
			Color::HSLA(h, s, l, a) => write!(f, "\"hsla({}, {}%, {}%, {}%)\"", h = h, s = 100.0 * f64::from(s) / 255.0, l = 100.0 * f64::from(l) / 255.0, a = 100.0 * f64::from(a) / 255.0),
			named => format!("{:?}", named).to_lowercase().fmt_js(f),
		}
	}
}


#[derive(Debug, Copy, Clone)]
pub enum StrokePosition {
	/// The stroke is centered on the polygon's path, with half the stroke inside the polygon and half the stroke outside the polygon.
	Center,
	/// The stroke lies inside the polygon.
	Inside,
	/// The stroke lies outside the polygon.
	Outside,
}


impl JavaScript for StrokePosition {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			StrokePosition::Center => f.write_str("google.maps.StrokePosition.CENTER"),
			StrokePosition::Inside => f.write_str("google.maps.StrokePosition.INSIDE"),
			StrokePosition::Outside => f.write_str("google.maps.StrokePosition.OUTSIDE"),
		}
	}
}


#[derive(Default, Debug, Copy, Clone)]
pub struct PolylineStyle {
	pub(crate) stroke_color: Option<Color>,
	pub(crate) stroke_opacity: Option<f32>,
	pub(crate) stroke_weight: Option<usize>,
}


impl PolylineStyle {
	/// Create a new empty (default) polyline style.
	#[must_use]
	pub fn new() -> Self {
		PolylineStyle::default()
	}
	
	/// The stroke color.
	#[must_use]
	pub fn color(mut self, value: Color) -> Self {
		self.stroke_color = Some(value);
		self
	}
	
	/// The stroke opacity between 0.0 and 1.0.
	#[must_use]
	pub fn opacity(mut self, value: f32) -> Self {
		self.stroke_opacity = Some(value);
		self
	}
	
	/// The stroke width in pixels.
	#[must_use]
	pub fn width(mut self, value: usize) -> Self {
		self.stroke_weight = Some(value);
		self
	}
}


impl From<Color> for PolylineStyle {
	fn from(c: Color) -> Self {
		PolylineStyle::default().color(c)
	}
}


#[derive(Default, Debug, Copy, Clone)]
pub struct PolygonStyle {
	pub(crate) fill_color: Option<Color>,
	pub(crate) fill_opacity: Option<f32>,
	pub(crate) stroke_position: Option<StrokePosition>,
	pub(crate) stroke_color: Option<Color>,
	pub(crate) stroke_opacity: Option<f32>,
	pub(crate) stroke_weight: Option<usize>,
}


impl PolygonStyle {
	/// Create a new empty (default) polygon style.
	#[must_use]
	pub fn new() -> Self {
		PolygonStyle::default()
	}
	
	/// Set both `fill_color` and `stroke_color`.
	#[must_use]
	pub fn color(mut self, value: Color) -> Self {
		self.fill_color = Some(value);
		self.stroke_color = Some(value);
		self
	}
	
	/// Set both `fill_opacity` and `stroke_opacity`.
	#[must_use]
	pub fn opacity(mut self, value: f32) -> Self {
		self.fill_opacity = Some(value);
		self.stroke_opacity = Some(value);
		self
	}
	
	/// The fill color.
	#[must_use]
	pub fn fill_color(mut self, value: Color) -> Self {
		self.fill_color = Some(value);
		self
	}
	
	/// The fill opacity between 0.0 and 1.0.
	#[must_use]
	pub fn fill_opacity(mut self, value: f32) -> Self {
		self.fill_opacity = Some(value);
		self
	}
	
	/// The stroke position. Defaults to [`StrokePosition::Center`]. This property is not supported on Internet Explorer 8 and earlier.
	#[must_use]
	pub fn stroke_position(mut self, value: StrokePosition) -> Self {
		self.stroke_position = Some(value);
		self
	}
	
	/// The stroke color.
	#[must_use]
	pub fn stroke_color(mut self, value: Color) -> Self {
		self.stroke_color = Some(value);
		self
	}
	
	/// The stroke opacity between 0.0 and 1.0.
	#[must_use]
	pub fn stroke_opacity(mut self, value: f32) -> Self {
		self.stroke_opacity = Some(value);
		self
	}
	
	/// The stroke width in pixels.
	#[must_use]
	pub fn stroke_width(mut self, value: usize) -> Self {
		self.stroke_weight = Some(value);
		self
	}
}


impl From<Color> for PolygonStyle {
	fn from(c: Color) -> Self {
		PolygonStyle::default().color(c)
	}
}
