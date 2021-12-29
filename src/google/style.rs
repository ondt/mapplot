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


#[derive(Debug, Copy, Clone)]
pub enum StrokePosition {
	/// The stroke is centered on the polygon's path, with half the stroke inside the polygon and half the stroke outside the polygon.
	Center,
	/// The stroke lies inside the polygon.
	Inside,
	/// The stroke lies outside the polygon.
	Outside,
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
	pub fn weight(mut self, value: usize) -> Self {
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
	pub fn stroke_weight(mut self, value: usize) -> Self {
		self.stroke_weight = Some(value);
		self
	}
}


impl From<Color> for PolygonStyle {
	fn from(c: Color) -> Self {
		PolygonStyle::default().color(c)
	}
}
